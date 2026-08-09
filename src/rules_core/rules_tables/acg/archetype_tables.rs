//! Advanced Class Guide (ACG) archetype-swap catalog. SD28-E30
//! (`epic-32-archetype-swap`) tier-1 table 2 -- the largest in-scope
//! book, landed after UPsi's 15-record proof table to check whether
//! that table's shape and TYPE-vs-ABILITY disagreement rate generalize.
//! See `ultimate_psionics::archetype_tables`'s own module doc comment
//! for the full struct rationale (two-tier record shape, `replaces`/
//! `grants` kept as separate lists, the `ABILITY:` grant-token shapes
//! this extraction handles, the `§46`/`§48`/`§49` text-shape triad).
//!
//! **UPsi's TYPE/ABILITY-disagreement finding generalizes.** 87 master
//! records, 378 total `TYPE:`-replaced slots vs 337 total `ABILITY:`-
//! granted features -- equal counts in only 30 of 87 (34%), the same
//! direction as UPsi's own corrected rate (13%, 2 of 15) though not the
//! same magnitude -- the exact rate is book-dependent, confirmed across
//! two books, not assumed to be a fixed constant.
//!
//! **333 of 337 sub-feature grants (99%) resolved to real `DESC:`/
//! `BENEFIT:` text.** All 4 shortfalls are the "found but textless"
//! kind, named individually: `Mutagenic Mauler ~ Discovery`,
//! `Snakebite Striker ~ Sneak Attack`, `Snakebite Striker ~ Maneuver
//! Training`, `Divine Hunter ~ Class Skills`.
//!
//! **The `§46`/`§48`/`§49` text-shape triad, run against this book's own
//! archetype `.MOD` rows.** Sampled several master archetypes' own
//! `.MOD` suppression rows directly: same shape as UPsi's -- pure
//! `FACT:<Class>_CF_<Slot>|true` flag-setters, no `DESC:`/`BENEFIT:` at
//! all. Clean; none of the three hazards applied.
//!
//! **This table is data only.** No `pilot_compute.rs` integration lands
//! in this slice -- see `decisions.md §51`/`forward-scope-register.md
//! §C4.8` for why that half is blocked on an explicit scope decision.
//!
//! Every field below is copied verbatim from the real corpus row (source:
//! `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/
//! advanced_class_guide/acg_abilities_class.lst`), generated
//! programmatically by a one-off extraction script, not hand-transcribed.

use super::super::archetype_swap::{ArchetypeGrant, ArchetypeSwapEntry};

/// Full ACG archetype-swap catalog: 87 real, distinct master records, in
/// source order. Built once and cached for the process lifetime.
pub fn archetype_swap_tables() -> &'static [ArchetypeSwapEntry] {
    static TABLE: std::sync::OnceLock<Vec<ArchetypeSwapEntry>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        vec![
        // Alchemist Archetype ~ Inspired Chemist -- acg_abilities_class.lst:2315
        ArchetypeSwapEntry {
            key: "Alchemist Archetype ~ Inspired Chemist",
            subject: "Alchemist",
            archetype_name: "Inspired Chemist",
            description: Some("Akin to a mindchemist, inspired chemists use a type of cognatogen that instead of increasing their mental ability scores grants them inspiration like an investigator. This inspiration also grants an inspired chemist amazing powers to avoid danger, but takes a toll on both physical power and health."),
            source_page: Some("p.75"),
            prerequisites: Some(&["PRECLASS:1,Alchemist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Alchemist Archetype ~ Inspired Chemist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.AlchemistMutagen]"]),
            replaces: Some(&["AlchemistMutagen"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Inspired Chemist ~ Bonus Feats", at_level: 1, description: Some("An inspired chemist can select Skill Focus (Disable Device, Disguise, Heal, any Knowledge skill, Sense Motive, Spellcraft, or Use Magic Device) in place of a discovery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Inspired Chemist ~ Bonus Investigator Talents", at_level: 1, description: Some("An inspired chemist can select any two investigator talents in place of a discovery, but can only use these talents while under the effect of an inspiring cognatogen."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Inspired Chemist ~ Inspiring Cognatogen", at_level: 1, description: Some("At 1st level, an inspired chemist learns how to create an inspiring cognatogen, as the inspiring cognatogen discovery. This ability replaces the mutagen class ability. (This means that an inspired chemist cannot create mutagens unless he selects the mutagen discovery)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Inspired Chemist ~ Bonus Languages", at_level: 1, description: Some("An inspired chemist can learn three languages in place of a discovery."), benefit: None },
            ],
        },
        // Arcanist Archetype ~ Blade Adept -- acg_abilities_class.lst:2373
        ArchetypeSwapEntry {
            key: "Arcanist Archetype ~ Blade Adept",
            subject: "Arcanist",
            archetype_name: "Blade Adept",
            description: Some("A small number of arcanists learn to use blades as part of their spellcasting and in combat. While these blade adepts are not as capable with a sword as a true master duelist, their combination of swordplay and arcane power makes them quite deadly."),
            source_page: Some("p.76"),
            prerequisites: Some(&["PRECLASS:1,Arcanist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Arcanist Archetype ~ Blade Adept],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ArcanistExploit1,TYPE.ArcanistExploit3,TYPE.ArcanistExploit9]"]),
            replaces: Some(&["ArcanistExploit1", "ArcanistExploit3", "ArcanistExploit9"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Blade Adept ~ Sword Bond", at_level: 1, description: Some("At 1st level, a blade adept develops a bond with a blade and infuses it with arcane power. This ability works like a wizard's arcane bond ability save that the blade adept must bond to a one-handed piercing or slashing melee weapon. Additionally, the blade adept gains proficiency with the weapon if it is a simple or martial weapon. This ability replaces the arcanist exploits gained at 1st and 9th levels."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Blade Adept ~ Sentient Sword", at_level: 3, description: Some("You have a reservoir of mystical arcane energy that you draw upon to fuel your powers and enhance your weapon. This arcane resevoir has %1 points. The resevoir refreshes once per day when you prepare your spells. You can expend 1 point from your arcane resevoir as a swift action to grant any weapon you are holding a +%2 enhancement bonus for 1 minute. These bonuses can be added to the weapon, stacking with existing weapon enhancement to a maximum of +5. Multiple uses of this ability do not stack with themselves.|ArcaneReservoirPool|ArcaneReservoirPoolBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Blade Adept ~ Adept Exploits", at_level: 1, description: Some("A blade adept can select from the following additional exploits. Eldritch Blade, Magus Arcana, Spell Strike, Student of the Blade, and Weapon Specialization."), benefit: None },
            ],
        },
        // Arcanist Archetype ~ Blood Arcanist -- acg_abilities_class.lst:2374
        ArchetypeSwapEntry {
            key: "Arcanist Archetype ~ Blood Arcanist",
            subject: "Arcanist",
            archetype_name: "Blood Arcanist",
            description: Some("Though most arcanists possess only a rudimentary innate arcane gift, the blood arcanist has the full power of a bloodline to draw upon."),
            source_page: Some("p.77"),
            prerequisites: Some(&["PRECLASS:1,Arcanist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Arcanist Archetype ~ Blood Arcanist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ArcanistExploit1,TYPE.ArcanistExploit3,TYPE.ArcanistExploit9,TYPE.ArcanistExploit15,TYPE.ArcanistMagicalSupremacy]"]),
            replaces: Some(&["ArcanistExploit1", "ArcanistExploit3", "ArcanistExploit9", "ArcanistExploit15", "ArcanistMagicalSupremacy"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Blood Arcanist ~ Bloodline", at_level: 1, description: Some("A blood arcanist selects one bloodline from those available through the sorcerer bloodline Class Feature.Arcanist Class Feature. The blood arcanist gains the bloodline arcana and bloodline powers of that bloodline, treating her arcanist level as her sorcerer level. The blood arcanist does not gain the class skill, bonus feats, or bonus spells from her bloodline. If the blood arcanist takes levels in another class that grants a bloodline, the bloodlines must be the same type, even if that means that the bloodline of one of her classes must change. Subject to GM discretion, the blood arcanist can change her former bloodline to make them conform. This ability replaces the arcanist exploits gained at 1st, 3rd, 9th, and 15th levels, as well as magical supremacy. A blood arcanist cannot select the bloodline development arcanist exploit."), benefit: None },
            ],
        },
        // Arcanist Archetype ~ Brown-Fur Transmuter -- acg_abilities_class.lst:2375
        ArchetypeSwapEntry {
            key: "Arcanist Archetype ~ Brown-Fur Transmuter",
            subject: "Arcanist",
            archetype_name: "Brown-Fur Transmuter",
            description: Some("Frequently called \"brown-furs,\" these transmutationfocused arcanists are known for transforming themselves into animals. What few realize is that these specialized arcanists excel at turning themselves-and others-into all kinds of creatures."),
            source_page: Some("p.77"),
            prerequisites: Some(&["PRECLASS:1,Arcanist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Arcanist Archetype ~ Brown-Fur Transmuter],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ArcanistExploit3,TYPE.ArcanistExploit9,TYPE.ArcanistMagicalSupremacy]"]),
            replaces: Some(&["ArcanistExploit3", "ArcanistExploit9", "ArcanistMagicalSupremacy"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Brown-Fur Transmuter ~ Powerful Change", at_level: 3, description: Some("At 3rd level, the brown-fur transmuter learns to harness the power of her magic to empower her transmutations. Whenever the brown-fur transmuter casts a transmutation spell using one of her arcanist spell slots, she can expend 1 point from her arcane reservoir as a free action to bolster the spell. If the spell grants a bonus to an ability score, the bonus then increases by 2. If it grants a bonus to more than one ability score, only one of the ability scores gains this bonus. The brownfur transmuter cannot expend more than 1 point from her arcane reservoir in this way. This ability replaces the arcanist exploit gained at 3rd level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Brown-Fur Transmuter ~ Share Transmutation", at_level: 9, description: Some("At 9th level, the brown-fur transmuter can target others with her transmutation spells. A brown-fur transmuter can expend 1 point from her arcane reservoir to change any transmutation spell with a range of personal to a range of touch. Such a spell automatically fails on unwilling creatures. This ability replaces the arcanist exploit gained at 9th level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Brown-Fur Transmuter ~ Transmutation Supremacy", at_level: 20, description: Some("At 20th level, the brown-fur transmuter learns to fully master the power of transmutation. Whenever she casts a transmutation spell, it is treated as it were affected by the Extend Spell feat without altering the casting time or slot used. (She cannot then alter its duration again with the Extend Spell feat). Whenever she uses her powerful change ability, the bonus increases by 4 instead of 2. Her share transmutation ability can now target a willing creature within 30 feet. This ability replaces magical supremacy."), benefit: None },
            ],
        },
        // Arcanist Archetype ~ Eldritch Font -- acg_abilities_class.lst:2376
        ArchetypeSwapEntry {
            key: "Arcanist Archetype ~ Eldritch Font",
            subject: "Arcanist",
            archetype_name: "Eldritch Font",
            description: Some("For some arcanists, the power bubbling up from within is nearly too much to contain. They become adept at shaping this magical energy without needing to bind it up in spells."),
            source_page: Some("p.77"),
            prerequisites: Some(&["PRECLASS:1,Arcanist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Arcanist Archetype ~ Eldritch Font],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ArcanistExploit3,TYPE.ArcanistExploit7,TYPE.ArcanistExploit13,TYPE.ArcanistMagicalSupremacy]"]),
            replaces: Some(&["ArcanistExploit3", "ArcanistExploit7", "ArcanistExploit13", "ArcanistMagicalSupremacy"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Eldritch Font ~ Font of Power", at_level: 1, description: Some("An eldritch font gains one additional spell slot for each level of arcanist spell she can cast. However, the number of spells of each level that she can prepare reduces by 1. If this reduces her spells prepared for a level to 0, she still gains spell slots of that level, which can be consumed using the consume spells class feature to fuel her arcane reservoir or cast spells using metamagic feats."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Eldritch Font ~ Eldritch Surge", at_level: 3, description: Some("An eldritch font can, as a swift action, pour more power into her spells and abilities. She can add 2 to the caster level and DC of a spell, or increase her effective arcanist level by 2 when using an arcanist exploit. She becomes fatigued upon using this ability. If she is already fatigued, she becomes exhausted. If she's already exhausted, or something would prevent her from becoming fatigued or exhausted, she cannot use this ability. This ability does not stack with spending points from her arcane reservoir to increase the spell's caster level or DC (as the arcane reservoir class feature). Only rest can remove fatigue or exhaustion caused by an eldritch surge-spells and abilities have no effect."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Eldritch Font ~ Improved Surge", at_level: 7, description: Some("An eldritch font can use her eldritch surge ability to reroll an attack roll associated with a spell or arcanist exploit, or to reroll all of the damage dice associated with a spell or arcanist exploit. In the case of attack rolls, this ability must be used after the die is rolled but before the results are revealed. The eldritch font must take the results of the reroll, even if they are lower."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Eldritch Font ~ Greater Surge", at_level: 13, description: Some("An eldritch font can use her eldritch surge ability to force a creature to reroll a saving throw against one spell or arcanist exploit and take the lower value. The eldritch font must declare the use of this ability before the result of that creature's saving throw is revealed. If the spell or arcanist exploit affects more than one target, only one target is affected by this ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Eldritch Font ~ Bottomless Well", at_level: 20, description: Some("An eldritch font can spend 1 hour studying her spellbook to refuel herself. Doing so allows her to prepare new spells and regain upto %1 points of arcane reservoir. She can use this ability multiple times per day, however she still only regains spell slots once per day.|floor(ArcanistLVL/2)"), benefit: None },
            ],
        },
        // Arcanist Archetype ~ Elemental Master -- acg_abilities_class.lst:2377
        ArchetypeSwapEntry {
            key: "Arcanist Archetype ~ Elemental Master",
            subject: "Arcanist",
            archetype_name: "Elemental Master",
            description: Some("Arcanists with an affinity for elemental forces sometimes focus on one and display its power in everything they do."),
            source_page: Some("p.78"),
            prerequisites: Some(&["PRECLASS:1,Arcanist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Arcanist Archetype ~ Elemental Master],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ArcanistExploit1,TYPE.ArcanistExploit3,TYPE.ArcanistExploit9,TYPE.ArcanistExploit11,TYPE.ArcanistExploit15]"]),
            replaces: Some(&["ArcanistExploit1", "ArcanistExploit3", "ArcanistExploit9", "ArcanistExploit11", "ArcanistExploit15"]),
            grants: &[],
        },
        // Arcanist Archetype ~ Occultist -- acg_abilities_class.lst:2378
        ArchetypeSwapEntry {
            key: "Arcanist Archetype ~ Occultist",
            subject: "Arcanist",
            archetype_name: "Occultist",
            description: Some("Not all arcanists peer inward to discern the deepest secrets of magic. Some look outward, connecting with extraplanar creatures and bartering for secrets, power, and favor."),
            source_page: Some("p.78"),
            prerequisites: Some(&["PRECLASS:1,Arcanist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Arcanist Archetype ~ Occultist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ArcanistExploit1,TYPE.ArcanistExploit7,TYPE.ArcanistMagicalSupremacy]"]),
            replaces: Some(&["ArcanistExploit1", "ArcanistExploit7", "ArcanistMagicalSupremacy"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Occultist ~ Planar Spells", at_level: 1, description: Some("[NOT IMPLEMENTED] An occultist adds all planar ally spells to her spell list (using her arcanist level as the cleric level), and treats plane shift as a 5th-level arcanist spell."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Occultist ~ Conjurer's Focus", at_level: 1, description: Some("An occultist can spend 1 point from her arcane reservoir to cast summon monster I. She can cast this spell as a standard action and the summoned creatures remain for 1 minute per level (instead of 1 round per level). At 3rd level and every 2 levels thereafter, the power of this ability increases by one spell level, allowing her to summon more powerful creatures (to a maximum of summon monster IX at 17th level), at the cost of an additional point from her arcane spell reserve per spell level. An occultist cannot have more than one summon monster spell active in this way at one time. If this ability is used again, any existing summon monster immediately ends."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Occultist ~ Planar Contact", at_level: 7, description: Some("An occultist can cast augury once per day and contact other plane once per week, using her arcanist level as her caster level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Occultist ~ Perfect Summoner", at_level: 20, description: Some("An occultist can use her conjurer's focus without spending points from her arcane reservoir, and the creatures summoned last until dismissed."), benefit: None },
            ],
        },
        // Arcanist Archetype ~ School Savant -- acg_abilities_class.lst:2379
        ArchetypeSwapEntry {
            key: "Arcanist Archetype ~ School Savant",
            subject: "Arcanist",
            archetype_name: "School Savant",
            description: Some("Some arcanists specialize in a school of magic and trade flexibility for focus. School savants are able to prepare more spells per day than typical arcanists, but their selection is more limited."),
            source_page: Some("p.78"),
            prerequisites: Some(&["PRECLASS:1,Arcanist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Arcanist Archetype ~ School Savant],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ArcanistExploit1,TYPE.ArcanistExploit3,TYPE.ArcanistExploit7]"]),
            replaces: Some(&["ArcanistExploit1", "ArcanistExploit3", "ArcanistExploit7"]),
            grants: &[],
        },
        // Arcanist Archetype ~ Spell Specialist -- acg_abilities_class.lst:2380
        ArchetypeSwapEntry {
            key: "Arcanist Archetype ~ Spell Specialist",
            subject: "Arcanist",
            archetype_name: "Spell Specialist",
            description: Some("Where most arcanists are broad in their study of magic, a spell specialist has her power focused in a few spells. Spell specialists are able to warp and twist the magic of their signature spells in ways other casters cannot."),
            source_page: Some("p.78"),
            prerequisites: Some(&["PRECLASS:1,Arcanist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Arcanist Archetype ~ Spell Specialist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ArcanistExploit1,TYPE.ArcanistExploit7,TYPE.ArcanistExploit13,TYPE.ArcanistExploit19]"]),
            replaces: Some(&["ArcanistExploit1", "ArcanistExploit7", "ArcanistExploit13", "ArcanistExploit19"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Spell Specialist ~ Signature Spells", at_level: 1, description: Some("At 1st level and each time a spell specialist gains a new spell level, she chooses a signature spell. The spell specialist can cast this spell without preparing it, in the same way a sorcerer casts spells spontaneously. Even though the spell specialist does not need to prepare this spell, it still counts against the number of spells she can prepare, reducing the number of spells of each level she can prepare each day by 1. A spell specialist can swap out a single signature spell for another spell of the same level when gaining a class level. The DC for signature spells increases by 1. The spell specialist gains a +2 bonus on concentration checks when casting signature spells; this bonus increases to +4 at 10th level. Additionally, a spell specialist can apply one of the following additional effects by spending 1 point from her arcane reservoir."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spell Specialist ~ Dismiss", at_level: 1, description: Some("A spell specialist can dismiss a signature spell as a swift action instead of a standard action. Alternatively, the spell specialist can dismiss a signature spell that has a duration but isn't normally dismissible."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spell Specialist ~ Spell Bender", at_level: 1, description: Some("The spell specialist can bend the line of a spell that has a line area of effect up to 90 degrees at any single point along the line's length."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spell Specialist ~ Spellwarp", at_level: 1, description: Some("A spell specialist can reduce the radius of a spread or burst effect or shorten the length of a cone. All changes must occur in 5-foot increments, to a minimum of 5 feet. Alternatively, the spell specialist can change the area of effect of a cone spell to a line with a length equal to the spell's range. This ability replaces the arcanist exploits gained at 1st, 7th, 13th, and 19th levels."), benefit: None },
            ],
        },
        // Arcanist Archetype ~ White Mage -- acg_abilities_class.lst:2382
        ArchetypeSwapEntry {
            key: "Arcanist Archetype ~ White Mage",
            subject: "Arcanist",
            archetype_name: "White Mage",
            description: Some("A white mage is an arcanist touched by a divine power and gifted with the ability to heal others."),
            source_page: Some("p.79"),
            prerequisites: Some(&["PRECLASS:1,Arcanist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Arcanist Archetype ~ White Mage],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ArcanistExploit1,TYPE.ArcanistExploit9]"]),
            replaces: Some(&["ArcanistExploit1", "ArcanistExploit9"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "White Mage ~ Spontaneous Healing", at_level: 1, description: Some("A white mage can expend 1 point from her arcane reservoir to use one of her spell slots to cast a cure spell (any spell with \"cure\" in its name) from the cleric spell list as if it were on her spell list and prepared. The spell must be of a level the arcanist can cast. At 10th level, the white mage can expend 5 points from her arcane reservoir and a spell slot of at least 5th level to cast breath of life."), benefit: None },
            ],
        },
        // Bard Archetype ~ Flame Dancer -- acg_abilities_class.lst:2684
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Flame Dancer",
            subject: "Bard",
            archetype_name: "Flame Dancer",
            description: Some("A flame dancer studies the movements of fire, adding its grace to his repertoire. He seeks truth in fire's burning essence, and uses his performance to unleash the power of fire against those who dare oppose him."),
            source_page: Some("p.81"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Flame Dancer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardCountersong,TYPE.BardInspireCompetence,TYPE.BardSuggestion,TYPE.BardDirgeOfDoom]"]),
            replaces: Some(&["BardCountersong", "BardInspireCompetence", "BardSuggestion", "BardDirgeOfDoom"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Flame Dancer ~ Fire Dance", at_level: 1, description: Some("A fire dancer learns to protect himself and others from the pain of fire, and discovers how to control the flame while enduring its heat. Each round of the fire dance, he rolls a Perform (dance or sing) check. Any ally within 30 feet of the bard that has caught on fire or is affected by a fire effect or extreme heat can use the bard's Perform check result in place of its saving throw against that fire. Any ally within 30 feet of the bard who is suffering from heatstroke can ignore the fatigue from heat exposure so long as the bard maintains this performance."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Flame Dancer ~ Song of the Fiery Gaze", at_level: 3, description: Some("A fire dancer can allow allies to see through flames without any distortion. Any ally within 30 feet of the bard who can hear the performance can see through fire, fog, and smoke without penalty as long as the light is sufficient to allow him to see normally, as with the base effect of the gaze of flames oracle revelation (APG p.47)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Flame Dancer ~ Fire Break", at_level: 6, description: Some("A fire dancer's performance can bend flames away from others. Any ally within 30 feet of the bard who can hear or see the bardic performance gains resist fire 20 as long as the performance is maintained. At 11th level, this resistance increases to 30."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Flame Dancer ~ Fan the Flames", at_level: 8, description: Some("A fire dancer adds burning hands, flaming sphere, and fireball to his list of bard spells known (as 1st-, 2nd-, and 3rd-level spells, respectively)."), benefit: None },
            ],
        },
        // Bard Archetype ~ Voice of the Wild -- acg_abilities_class.lst:2685
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Voice of the Wild",
            subject: "Bard",
            archetype_name: "Voice of the Wild",
            description: Some("Most bards are inspired by the art of civilization, yet the voice of the wild's muse is the grandeur and beauty of nature. The voice of the wild has discovered some of nature's magical secrets, and can use his performance to bring out the bestial side in his allies."),
            source_page: Some("p.81"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Voice of the Wild],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardBardicKnowledge,TYPE.BardCountersong,TYPE.BardVersatilePerformance,TYPE.BardJackOfAllTrades,TYPE.BardInspireCompetence,TYPE.BardDirgeOfDoom,TYPE.BardInspireHeroics]"]),
            replaces: Some(&["BardBardicKnowledge", "BardCountersong", "BardVersatilePerformance", "BardJackOfAllTrades", "BardInspireCompetence", "BardDirgeOfDoom", "BardInspireHeroics"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Voice of the Wild ~ Wild Knowledge", at_level: 1, description: Some("A voice of the wild adds %1 to all Knowledge (nature) checks. He can use Knowledge (geography) and Knowledge (nature) untrained.|max(floor(BardLVL/2))"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Voice of the Wild ~ Nature Magic", at_level: 1, description: Some("[NOT IMPLEMENTED] At 1st level, a voice of the wild can select a 1st-level druid or ranger spell as a bard spell known instead of a spell from the bard spell list. The voice of the wild can select another druid or ranger spell (of any bard spell level he can cast) at 4th, 7th, 10th, 13th, and 16th levels. If he chooses to learn a new spell in place of an old one, he can exchange a druid or ranger spell for another."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Voice of the Wild ~ Song of the Wild", at_level: 3, description: Some("The voice of the wild can use bardic performance to grant an animal aspect to an ally, as if using the hunter's animal focus class feature. The ally must be able to hear or see the performance. The voice of the wild uses his bard level as his hunter level for determining the effect of the animal aspect. The bard can affect a second ally with this performance at 10th level and a third at 17th level."), benefit: None },
            ],
        },
        // Bloodrager Archetype ~ Blood Conduit -- acg_abilities_class.lst:2702
        ArchetypeSwapEntry {
            key: "Bloodrager Archetype ~ Blood Conduit",
            subject: "Bloodrager",
            archetype_name: "Blood Conduit",
            description: Some("Blood conduits learn to channel their arcane might directly through their flesh, without the need for mystical words or gestures."),
            source_page: Some("p.82"),
            prerequisites: Some(&["PRECLASS:1,Bloodrager=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bloodrager Archetype ~ Blood Conduit],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BloodragerFastMovement,TYPE.BloodragerBloodlineFeats,TYPE.BloodragerUncannyDodge,TYPE.BloodragerImprovedUncannyDodge,TYPE.BloodragerIndomitableWill]"]),
            replaces: Some(&["BloodragerFastMovement", "BloodragerUncannyDodge", "BloodragerImprovedUncannyDodge", "BloodragerIndomitableWill"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Blood Conduit ~ Contact Specialist", at_level: 1, description: Some("At 1st level, a blood conduit selects a bonus feat from the following: Improved Bull Rush, Improved Grapple, Improved Reposition, Improved Trip, and Improved Unarmed Strike. He does not need to meet the prerequisites to take this feat. He also adds those feats to his list of bloodline feats. This ability replaces fast movement and alters bloodline feats."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Blood Conduit ~ Spell Conduit", at_level: 5, description: Some("At 5th level, as long as a blood conduit is wearing light or no armor, he can deliver bloodrager spells with a range of touch through bodily contact. When he succeeds at a combat maneuver check to bull rush, grapple, pin, reposition, or trip an opponent, or makes an unarmed strike against an enemy, he can as a swift action cast a touch spell on the creature that he affected with the combat maneuver, requiring no further touch attack roll. If this spell would usually require a successful touch attack, his successful combat maneuver check counts as this attack."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Blood Conduit ~ Reflexive Conduit", at_level: 14, description: Some("At 14th level, a blood conduit can discharge his power into foes that attempt bodily contact with him. While wearing light or no armor, when the blood conduit is subject to a combat maneuver check made to bull rush, grapple, pin, reposition, or trip him, as an immediate action he can target his attacker with a bloodrager spell that has a range of touch. If the spell would normally require a touch attack, a blood conduit can attempt a combat maneuver check for this attack instead."), benefit: None },
            ],
        },
        // Bloodrager Archetype ~ Bloodrider -- acg_abilities_class.lst:2703
        ArchetypeSwapEntry {
            key: "Bloodrager Archetype ~ Bloodrider",
            subject: "Bloodrager",
            archetype_name: "Bloodrider",
            description: Some("In the world's wild lands, a mount is an advantage in both everyday life and the dealing of death. In many barbarian tribes, the true stature of a warrior is determined by his skill and ferocity on horseback. Other tribes measure it in skill atop whatever terrible mounts their people employ. A number of bloodragers are not only skilled in the art of mounted combat, but have learned to channel their arcane energies directly into their mount."),
            source_page: Some("p.82"),
            prerequisites: Some(&["PRECLASS:1,Bloodrager=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bloodrager Archetype ~ Bloodrider],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BloodragerFastMovement,TYPE.BloodragerUncannyDodge,TYPE.BloodragerImprovedUncannyDodge,TYPE.BloodragerBloodlineFeat9]"]),
            replaces: Some(&["BloodragerFastMovement", "BloodragerUncannyDodge", "BloodragerImprovedUncannyDodge", "BloodragerBloodlineFeat9"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Bloodrider ~ Fast Rider", at_level: 1, description: Some("The bloodrider is adept at pushing his mount to its limit. The speed of any mount the bloodrager rides increases by 10 feet."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bloodrider ~ Feral Mount", at_level: 5, description: Some("At 5th level, the bloodrider gains the service of a feral mount. This ability functions as the druid's animal companion, using the bloodrager's level - 4 as his effective druid level. This companion must be one he is capable of riding and suitable as a mount. A Medium bloodrider can select a camel or a horse. A Small bloodrider can select a pony or wolf, but can also select a boar or riding dog if he is at least 8th level. Whenever a bloodrider is bloodraging, this feral mount gains a +2 morale bonus to its Strength."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bloodrider ~ Blood Bond", at_level: 9, description: Some("At 9th level, the bloodrider and his feral mount gain a closer bond that allows the bloodrider to augment his mount based on his bloodline. While the bloodrider is bloodraging and on his feral mount, he grants the mount all the immunities and resistances he gains from bloodline powers. Furthermore, whenever the bloodrager is affected by a spell or spell-like ability with the range of personal while on the feral mount, the feral mount also gains the benef it of that spell. This ability replaces the bloodline feat gained at 9th level."), benefit: None },
            ],
        },
        // Bloodrager Archetype ~ Greenrager -- acg_abilities_class.lst:2704
        ArchetypeSwapEntry {
            key: "Bloodrager Archetype ~ Greenrager",
            subject: "Bloodrager",
            archetype_name: "Greenrager",
            description: Some("Typically, nature finds its greatest harmony with divine magic, but sometimes a connection with the natural world manifest itself through the arcane current in the veins of the bloodragers called greenragers. These bloodragers funnel their eldritch heritage into abilities that allow them to call powerful allies from nature and empower them with their bloodrage."),
            source_page: Some("p.83"),
            prerequisites: Some(&["PRECLASS:1,Bloodrager=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bloodrager Archetype ~ Greenrager],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BloodragerBloodSanctuary.BloodragerBloodlineFeat6.BloodragerBloodlineFeat9]"]),
            replaces: Some(&["BloodragerBloodSanctuary", "BloodragerBloodlineFeat6", "BloodragerBloodlineFeat9"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Greenrager ~ Unfettered Fury", at_level: 3, description: Some("A greenrager's fury allows him to move through undergrowth with frightening speed and grace. This functions as the druid's woodland stride class feature, but only while the greenrager is bloodraging."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Greenrager ~ Summoning Rager", at_level: 6, description: Some("A greenrager's magic unlocks the secrets of summoning allies from nature. He adds summon nature's ally I to his list of 1st-level bloodrager spells known as a bonus spell, as if it were a bonus bloodrager spell. At 7th level, he adds summon nature's ally II as a bonus 2nd-level bloodrager spell, at 10th level he adds summon nature's ally III as a bonus 3rd level bloodrager spell, and at 13th level he adds summon nature's ally IV as a bonus 4th-level bloodrager spell."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Greenrager ~ Furious Summoning", at_level: 9, description: Some("Creatures summoned by the bloodrager's summon nature's ally spell gain a +%1 morale bonus to Strength and Constitution and gain the druid's woodland stride ability.|min(8,(floor((CASTERLEVEL-2)/9)+2)*2)"), benefit: None },
            ],
        },
        // Bloodrager Archetype ~ Metamagic Rager -- acg_abilities_class.lst:2705
        ArchetypeSwapEntry {
            key: "Bloodrager Archetype ~ Metamagic Rager",
            subject: "Bloodrager",
            archetype_name: "Metamagic Rager",
            description: Some("While metamagic is difficult for many bloodragers to utilize, a talented few are able to channel their bloodrage in ways that push their spells to impressive ends."),
            source_page: Some("p.83"),
            prerequisites: Some(&["PRECLASS:1,Bloodrager=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bloodrager Archetype ~ Metamagic Rager],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BloodragerImprovedUncannyDodge]"]),
            replaces: Some(&["BloodragerImprovedUncannyDodge"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Metamagic Rager ~ Meta-Rage", at_level: 5, description: Some("[NOT IMPLEMENTED] A metamagic rager can sacrifice additional rounds of bloodrage to apply a metamagic feat he knows to a bloodrager spell. This costs a number of rounds of bloodrage equal to twice what the spell's adjusted level would normally be with the metamagic feat applied (minimum 2 rounds). The metamagic rager does not have to be bloodraging to use this ability. The metamagic effect is applied without increasing the level of the spell slot expended, though the casting time is increased as normal. The metamagic rager can apply only one metamagic feat he knows in this manner with each casting. Additionally, when the metamagic rager takes a bloodline feat, he can choose to take a metamagic feat instead."), benefit: None },
            ],
        },
        // Bloodrager Archetype ~ Rageshaper -- acg_abilities_class.lst:2707
        ArchetypeSwapEntry {
            key: "Bloodrager Archetype ~ Rageshaper",
            subject: "Bloodrager",
            archetype_name: "Rageshaper",
            description: Some("All bloodragers blend the unpredictable surge of arcane power with the savage fury of battle lust. For most, their rage is a conduit for the eldritch power locked in their heritage, but for a rageshaper, the latent magical energies in his blood bring about physical transformations and facilitate the blending of arcana and aggression into a deadly synthesis that few other barbarians (or even other bloodragers) can match."),
            source_page: Some("p.84"),
            prerequisites: Some(&["PRECLASS:1,Bloodrager=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bloodrager Archetype ~ Rageshaper],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BloodragerBloodSanctuary,TYPE.BloodragerImprovedUncannyDodge]"]),
            replaces: Some(&["BloodragerBloodSanctuary", "BloodragerImprovedUncannyDodge"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Rageshaper ~ Bestial Aspect", at_level: 4, description: Some("Whenever a rageshaper gains a natural attack through the use of a polymorph spell, he can increase the damage done by that attack by one die. If the spell grants multiple natural attacks, the rageshaper must choose one kind of natural attack for the ability to enhance. At 9th level, if the rageshaper's altered form grants him a new mode of movement, that movement's base speed increases by 10 feet. This is an enhancement bonus. If the rageshaper's bloodrage powers already grant natural attacks or alternate modes of movement, then the bonuses granted by bestial aspect also apply to these bloodrage powers."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Rageshaper ~ Furious Transformation", at_level: 5, description: Some("A rageshaper can attempt to bolster any transmutation spell from the bloodrager spell list with the polymorph subschool while bloodraging. The rageshaper must succeed at a concentration check as if casting defensively to modify the spell; otherwise, the spell is wasted. If he succeeds, the spell is treated as if the rageshaper had the Extend Spell metamagic feat. The rageshaper must be the spell's intended target or cast a spell with a range of personal to gain this effect. If the rageshaper casts a spell that is linked to the rageshaper's own bloodline, such as a rageshaper with the elemental bloodline who casts elemental body I, the spell is automatically extended without forcing the concentration check, so long as the rageshaper casts it while in a bloodrage."), benefit: None },
            ],
        },
        // Bloodrager Archetype ~ Spelleater -- acg_abilities_class.lst:2708
        ArchetypeSwapEntry {
            key: "Bloodrager Archetype ~ Spelleater",
            subject: "Bloodrager",
            archetype_name: "Spelleater",
            description: Some("Where other bloodragers learn to avoid or shrug off minor damage of all sorts, spelleaters tap into the power of their bloodline in order to heal damage as it comes, and can even cannibalize their own magical energy to heal more damage and continue taking the fight to the enemy."),
            source_page: Some("p.85"),
            prerequisites: Some(&["PRECLASS:1,Bloodrager=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bloodrager Archetype ~ Spelleater],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BloodragerUncannyDodge,TYPE.BloodragerDamageReduction,TYPE.BloodragerImprovedUncannyDodge]"]),
            replaces: Some(&["BloodragerUncannyDodge", "BloodragerDamageReduction", "BloodragerImprovedUncannyDodge"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Spelleater ~ Blood of Life", at_level: 1, description: Some("A spelleater's blood empowers him to slowly recover from his wounds. While bloodraging, a spelleater gains fast healing %1 (Bestiary p.300). If the spelleater gains an increase to damage reduction from a bloodline, feat, or other ability, he is considered to have an effective damage reduction of 0, and the increase is added to this effective damage reduction.|max(1,min(6,floor((CASTERLEVEL-1)/3)))"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spelleater ~ Spell Eating", at_level: 5, description: Some("A spelleater can consume spell slots for an extra dose of healing. As a swift action, the spelleater can consume one unused bloodrager spell slot to heal 1d8 damage for each level of the spell slot consumed."), benefit: None },
            ],
        },
        // Bloodrager Archetype ~ Steelblood -- acg_abilities_class.lst:2709
        ArchetypeSwapEntry {
            key: "Bloodrager Archetype ~ Steelblood",
            subject: "Bloodrager",
            archetype_name: "Steelblood",
            description: Some("Most bloodragers prefer light armor, but some learn the secret of using heavy armors. These steelbloods plod around the battlefield inspiring fear and delivering carnage from within a steel shell."),
            source_page: Some("p.85"),
            prerequisites: Some(&["PRECLASS:1,Bloodrager=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bloodrager Archetype ~ Steelblood],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BloodragerArmorProficiency,TYPE.BloodragerFastMovement,TYPE.BloodragerUncannyDodge,TYPE.BloodragerImprovedUncannyDodge,TYPE.BloodragerDamageReduction]"]),
            replaces: Some(&["BloodragerArmorProficiency", "BloodragerFastMovement", "BloodragerUncannyDodge", "BloodragerImprovedUncannyDodge", "BloodragerDamageReduction"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Steelblood ~ Weapon and Armor Proficiency", at_level: 1, description: Some("A steelblood gains proficiency in heavy armor. A steelblood can cast bloodrager spells while wearing heavy armor without incurring an arcane spell failure chance."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Steelblood ~ Indomitable Stance", at_level: 1, description: Some("A steelblood gains a +1 bonus on combat maneuver checks, to CMD against overrun combat maneuvers, and on Reflex saving throws against trample attacks. He also gains a +1 bonus to his AC against charge attacks and on attack and damage rolls against charging creatures."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Steelblood ~ Armored Swiftness", at_level: 2, description: Some("A steelblood moves faster in medium and heavy armor. When wearing medium or heavy armor, a steelblood can move 5 feet faster than normal in that armor, to a maximum of his unencumbered speed."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Steelblood ~ Armor Training", at_level: 5, description: Some("A steelblood learns to be more maneuverable while wearing armor. Whenever he is wearing armor, he reduces the armor check penalty by 1 (to a maximum of 0) and increases the maximum Dexterity bonus allowed by his armor by 1. Every 4 levels thereafter (9th, 13th, and 17th), these bonuses increase by 1, to a maximum 4-point reduction of the armor check penalty and a +4 increase of the maximum Dexterity bonus. This ability stacks with the fighter class feature of the same name."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Steelblood ~ Blood Deflection", at_level: 7, description: Some("As an immediate action a steelblood can sacrifice a bloodrager spell slot to gain a deflection bonus to AC equal to the level of the spell sacrificed. The deflection bonus lasts until the start of his next turn. This ability can be applied after an attack roll is made against the steelblood, allowing the steelblood to convert a hit into a miss if the deflection bonus is high enough."), benefit: None },
            ],
        },
        // Brawler Archetype ~ Exemplar -- acg_abilities_class.lst:2845
        ArchetypeSwapEntry {
            key: "Brawler Archetype ~ Exemplar",
            subject: "Brawler",
            archetype_name: "Exemplar",
            description: Some("A versatile soldier who inspires her companions with her fighting prowess, an exemplar is at home on the front lines of battles anywhere."),
            source_page: Some("p.86"),
            prerequisites: Some(&["PRECLASS:1,Brawler=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Brawler Archetype ~ Exemplar],[!PREABILITY:1,CATEGORY=Archetype,TYPE.Brawler,TYPE.BrawlerUnarmedStrike,TYPE.BrawlerACBonus,TYPE.BrawlerManeuverTraining,TYPE.BrawlerBrawlersStrike,TYPE.BrawlerCloseWeaponMastery]"]),
            replaces: Some(&["BrawlerUnarmedStrike", "BrawlerManeuverTraining", "BrawlerACBonus", "BrawlerBrawlersStrike", "BrawlerCloseWeaponMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Exemplar Brawler ~ Call to Arms", at_level: 1, description: Some("An exemplar can expend a use of martial flexibility to rouse her allies into action. All allies within 30 feet are no longer flat-footed, even if they are surprised."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Exemplar Brawler ~ Inspiring Prowess", at_level: 3, description: Some("An exemplar gains the ability to use certain bardic performances. She can use this ability %1 rounds per day. The exemplar's effective bard level for this ability is equal to her brawler level - 2. Instead of the Perform skill, she activates this ability with impressive flourishes and displays of martial talent (this uses visual components). This ability otherwise functions as bardic performance; feats and other effects that affect bardic performance (such as the Extra Performance feat) apply to it.|BardicPerformanceDuration"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Exemplar Brawler ~ Field Instruction", at_level: 5, description: Some("%1 times per day, as a standard action, an exemplar can grant a teamwork feat to all allies within 30 feet who can see and hear her. This teamwork feat must be one the exemplar knows or has gained with the martial flexibility ability. Allies retain the use of this teamwork feat for %2 rounds. If the granted teamwork feat is one gained from martial flexibility, this duration ends immediately if the exemplar loses access to that feat. Allies don't need to meet the prerequisites of this teamwork feat. This ability otherwise counts as the cavalier's tactician class feature; feats and other effects which affect tactician (such as Practiced Tactician) apply to it.|FieldTrainingTimes|FieldTrainingDuration"), benefit: None },
            ],
        },
        // Brawler Archetype ~ Mutagenic Mauler -- acg_abilities_class.lst:2846
        ArchetypeSwapEntry {
            key: "Brawler Archetype ~ Mutagenic Mauler",
            subject: "Brawler",
            archetype_name: "Mutagenic Mauler",
            description: Some("Not content with perfecting her body with natural methods, a mutagenic mauler resorts to alchemy to unlock the primal beast within."),
            source_page: Some("p.86"),
            prerequisites: Some(&["PRECLASS:1,Brawler=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Brawler Archetype ~ Mutagenic Mauler],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BrawlerACBonus,TYPE.BrawlerMartialFlexibility]"]),
            replaces: Some(&["BrawlerMartialFlexibility", "BrawlerACBonus"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Mutagenic Mauler Brawler ~ Mutagen", at_level: 1, description: Some("You know how to create a mutagen that you can imbibe in order to heighten your physical prowess at the cost of your personality. It takes 1 hour to brew a dose of mutagen, and once brewed, it remains potent until used. You can only maintain one dose of mutagen at a time - if you brews a second dose, any existing mutagen becomes inert. A mutagen that is not in your possession becomes inert until it is picked up by either an mutagenic mauler or a mutagenic mauler. When you brew a mutagen, you select one physical ability score - either Strength, Dexterity, or Constitution. It's a standard action to drink a mutagen. Upon being imbibed, the mutagen causes you to grow bulkier and more bestial, granting you a +2 natural armor bonus and a +4 alchemical bonus to the selected ability score for %1 minutes. In addition, while the mutagen is in effect, you take a -2 penalty to one of your mental ability scores. If the mutagen enhances your Strength, it applies a penalty to your Intelligence. If it enhances your Dexterity, it applies a penalty to your Wisdom. If it enhances your Constitution, it applies a penalty to your Charisma. If a non-mutagenic mauler or non-mutagenic mauler drinks a mutagen, they must make a DC %2 Fortitude save or become nauseated for 1 hour - a non-mutagenic mauler or non-mutagenic mauler can never gain the benefit of a mutagen, but an mutagenic mauler or a mutagenic mauler can gain the effects of another mutagenic mauler or mutagenic mauler's mutagen if drunk. (Although if the other mutagenic mauler or mutagenic mauler creates a different mutagen, the effects of the \"stolen\" mutagen immediately cease.) The effects of a mutagen do not stack. Whenever an mutagenic mauler or a mutagenic mauler drinks a mutagen, the effects of any previous mutagen immediately end. Additionally, a mutagenic mauler gains +%3 weapon damage when she attacks in melee while her mutagen is active.|MutagenicMaulerMutagenDuration|MutagenicMaulerMutagenDC|MutagenicMaulerDamage"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mutagenic Mauler Brawler ~ Beastmorph", at_level: 4, description: Some("A mutagenic mauler gains additional abilities when using her mutagen. She gains low-light vision and a +%1 enhancement bonus to her base speed.|BeastmorphSpeed|PREVAREQ:BeastmorphProgression,1"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mutagenic Mauler Brawler ~ Discovery", at_level: 10, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Mutagenic Mauler Brawler ~ Greater Mutagen", at_level: 12, description: Some("The mutagenic mauler's mutagen now grants a +4 natural armor bonus, a +6 alchemical bonus to one physical ability score (Strength, Dexterity, or Constitution), and a +4 alchemical bonus to a second physical ability score. The mutagenic mauler takes a -2 penalty on both associated mental ability scores as long as the mutagen persists."), benefit: None },
            ],
        },
        // Brawler Archetype ~ Shield Champion -- acg_abilities_class.lst:2847
        ArchetypeSwapEntry {
            key: "Brawler Archetype ~ Shield Champion",
            subject: "Brawler",
            archetype_name: "Shield Champion",
            description: Some("Stalwart in battle, a shield champion has perfected an entire martial discipline relying on only her hand-to-hand fighting skills and her ever-present shield. What she forgoes in weapon versatility and improved combat maneuvering, she makes up for in her ability to turn her defense into a weapon."),
            source_page: Some("p.86"),
            prerequisites: Some(&["PRECLASS:1,Brawler=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Brawler Archetype ~ Shield Champion],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BrawlerManeuverTraining,TYPE.BrawlerWeaponProficiencies,TYPE.BrawlerBrawlersStrike]"]),
            replaces: Some(&["BrawlerManeuverTraining", "BrawlerWeaponProficiencies", "BrawlerBrawlersStrike"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Shield Champion Brawler ~ Weapon and Armor Proficiency", at_level: 1, description: Some("A shield champion is proficient with all simple weapons and with shields as weapons. She is also proficient with light armor, and with bucklers, light shields, and heavy shields. This replaces the brawler's weapon and armor proficiencies."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shield Champion Brawler ~ Throw Shield", at_level: 3, description: Some("A shield champion can throw a heavy or light shield as a normal (non-improvised) thrown weapon with a range increment of 10 feet or the shield's range increment, whichever is greater. The thrown shield deals the same damage as a shield bash, and any damage increases from shield spikes apply to this attack. A shield champion is treated as having the Far Shot feat for the purpose of determining range increment penalties for throwing a shield."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shield Champion Brawler ~ Champion Defense", at_level: 15, description: Some("%1 times per day when she would be reduced to 0 or fewer hit points by damage in combat from a weapon, unarmed, or natural attack, the shield champion can attempt to absorb all the damage with her shield. To use this ability, the shield champion must attempt a Fortitude saving throw, with the DC equal to the damage dealt. If it succeeds, she takes only half damage from the blow; otherwise, she takes full damage. She must be aware of the attack and able to react to it in order to use this ability-if she is denied her Dexterity bonus to AC, she can't use this ability.|ChampionDefenseTimes"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shield Champion Brawler ~ Returning Shield", at_level: 5, description: Some("A shield champion can throw a shield so it ricochets off her target (and possibly other solid objects) to return to her at the end of her turn. This ability functions whether or not the shield champion hits her opponent or moves on her turn. The shield deals no damage to targets it bounces off other than the original target of the shield champion's attack. Other circumstances can prevent the shield from returning to the shield champion, such as an opponent using a readied action to catch the shield, or the shield sticking to a mimic's adhesive. The shield champion can opt to not have a thrown shield return to her, in which case it falls to the ground as it normally would. If the shield has the returning weapon special ability, she can use either that or this ability. If a shield champion has additional attacks from a high base attack bonus, these additional attacks can be ricochets off an earlier target. The distance to each additional target adds to the total range of the shield, and range penalties apply, but there are no additional penalties for attacking in this manner. Because ricocheting attacks are treated as separate attacks, effects and modifiers that only apply to one attack roll (such as true strike) only apply to the first attack and not to the others. A shield champion can throw a shield as part of a brawler's flurry."), benefit: None },
            ],
        },
        // Brawler Archetype ~ Snakebite Striker -- acg_abilities_class.lst:2848
        ArchetypeSwapEntry {
            key: "Brawler Archetype ~ Snakebite Striker",
            subject: "Brawler",
            archetype_name: "Snakebite Striker",
            description: Some("With her lightning quickness and guile, a snakebite striker keeps her foes' attention focused on her, because any one of her feints might be an actual attack. By giving up some of a brawler's versatility, she increases her damage potential and exposes opponents to deadly and unexpected strikes."),
            source_page: Some("p.86"),
            prerequisites: Some(&["PRECLASS:1,Brawler=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Brawler Archetype ~ Snakebite Striker],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BrawlerManeuverTraining,TYPE.BrawlerMartialFlexibility,TYPE.BrawlerClassSkills]"]),
            replaces: Some(&["BrawlerManeuverTraining", "BrawlerMartialFlexibility", "BrawlerClassSkills"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Snakebite Striker Brawler ~ Sneak Attack", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Snakebite Striker Brawler ~ Class Skills", at_level: 1, description: Some("The snakebite striker gains Bluff and Stealth as class skills, but does not gain Intimidate as a class skill."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Snakebite Striker Brawler ~ Snake Feint", at_level: 3, description: Some("A snakebite striker who uses a standard action to move can combine that move with a feint. If she is able to feint as a move action (such as from having the Improved Feint feat), she can combine a move action to move with her feint."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Snakebite Striker Brawler ~ Opportunist", at_level: 11, description: Some("%1 times per round the snakebite striker can make an attack of opportunity against an opponent who has just been struck for damage in melee by another character. This attack counts as an attack of opportunity for that round. She cannot use this ability more than %1 times per round, even if she has the Combat Reflexes feat or a similar ability.|SnakebiteStrikerOpportunistTimes"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Snakebite Striker Brawler ~ Maneuver Training", at_level: 15, description: None, benefit: None },
            ],
        },
        // Brawler Archetype ~ Steel-Breaker -- acg_abilities_class.lst:2849
        ArchetypeSwapEntry {
            key: "Brawler Archetype ~ Steel-Breaker",
            subject: "Brawler",
            archetype_name: "Steel-Breaker",
            description: Some("The steel-breaker studies destruction and practices it as an art form. She knows every defense has a breaking point, and can shatter those defenses with carefully planned strikes."),
            source_page: Some("p.86"),
            prerequisites: Some(&["PRECLASS:1,Brawler=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Brawler Archetype ~ Steel-Breaker],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BrawlerManeuverTraining,TYPE.BrawlerBrawlersStrike,TYPE.BrawlerClassSkills]"]),
            replaces: Some(&["BrawlerManeuverTraining", "BrawlerBrawlersStrike"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Steel-Breaker Brawler ~ Class Skills", at_level: 1, description: Some("The steel-breaker gains Knowledge (engineering) as a class skill, and does not gain Knowledge (dungeoneering) as a class skill."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Steel-Breaker Brawler ~ Sunder Training", at_level: 3, description: Some("A steel-breaker receives additional training in sunder combat maneuvers. She gains a +%1 bonus when attempting a sunder combat maneuver checks and a +%1 bonus to her CMD when defending against this maneuver.|SunderTrainingSunderBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Steel-Breaker Brawler ~ Exploit Weakness", at_level: 5, description: Some("As a swift action a steel-breaker can observe a creature or object to find its weak point by succeeding at a Wisdom check, adding her brawler level against a DC of 10 + the object's hardness or the target's CR. If it succeeds, the steel-breaker gains a +2 bonus on attack rolls until the end of her turn, and any attacks she makes until the end of her turn ignore the creature or object's DR or hardness. A steel-breaker can instead use this ability as a swift action to analyze the movements and expressions of one creature within 30 feet, granting a bonus on Sense Motive checks and Reflex saving throws, as well as a dodge bonus to AC against that opponent equal to %1 until the start of her next turn.|BrawlerLVL/2"), benefit: None },
            ],
        },
        // Brawler Archetype ~ Strangler -- acg_abilities_class.lst:2850
        ArchetypeSwapEntry {
            key: "Brawler Archetype ~ Strangler",
            subject: "Brawler",
            archetype_name: "Strangler",
            description: Some("A strangler is trained to choke the life out of her victims with her vise-like grip. Some stranglers are self-taught and are little more than brutish murderers, unhinged sociopaths, or opportunistic alley-bashers. Others are members of murder cults or specialized schools of assassination, trained since the cradle to kill with their bare hands."),
            source_page: Some("p.86"),
            prerequisites: Some(&["PRECLASS:1,Brawler=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Brawler Archetype ~ Strangler],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BrawlerKnockout,TYPE.BrawlerBrawlersFlurry,TYPE.BrawlerUnarmedStrike,TYPE.BrawlerAwesomeBlow,TYPE.BrawlerImprovedAwesomeBlow,TYPE.BrawlerACBonus]"]),
            replaces: Some(&["BrawlerBrawlersFlurry", "BrawlerUnarmedStrike", "BrawlerACBonus", "BrawlerKnockout", "BrawlerAwesomeBlow", "BrawlerImprovedAwesomeBlow"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Strangler Brawler ~ Class Skills", at_level: 1, description: Some("A strangler gains Stealth as a class skill."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Strangler Brawler ~ Strangle", at_level: 1, description: Some("A strangler deals +%1d6 sneak attack damage whenever she succeeds at a grapple check to damage or pin an opponent. The strangler is always considered flanking her target for the purpose of using this ability.|SneakAttackDice"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Strangler Brawler ~ Practiced Strangler", at_level: 2, description: Some("When a strangler has the grappled condition, she does not take a -4 penalty to Dexterity and does not lose her Dexterity bonus to AC."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Strangler Brawler ~ Sleeper Hold", at_level: 4, description: Some("%1 times per day, a strangler can instantly render a pinned opponent unconscious. If you perform a successful grapple combat maneuver check against the pinned opponent, the target must succeed at a Fortitude saving throw (DC %2) or fall unconscious for 1d6 rounds. Each round on its turn, the unconscious target may attempt a new saving throw to end the effect as a full-round action that does not provoke attacks of opportunity. Creatures that do not need to breathe, are immune to critical hits, or are immune to nonlethal damage are immune to this ability.|SleeperHoldTimes|SleeperHoldDC"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Strangler Brawler ~ Neckbreaker", at_level: 16, description: Some("A strangler can attempt to instantly kill a pinned opponent. This works like the sleeper hold ability, but imposes a -5 penalty on her grapple combat maneuver check. If the opponent succeeds at its Fortitude save, the strangler deals damage as if she had attempted the grapple check to damage her opponent; if the opponent fails its Fortitude save, it dies. Creatures that are immune to critical hits are immune to this ability."), benefit: None },
            ],
        },
        // Brawler Archetype ~ Wild Child -- acg_abilities_class.lst:2851
        ArchetypeSwapEntry {
            key: "Brawler Archetype ~ Wild Child",
            subject: "Brawler",
            archetype_name: "Wild Child",
            description: Some("The wild child works with his sworn animal friend to conquer the challenges that lay before them. This kinship could come from being lost in the wilderness and raised by animals or growing up with an exotic pet."),
            source_page: Some("p.86"),
            prerequisites: Some(&["PRECLASS:1,Brawler=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Brawler Archetype ~ Wild Child],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BrawlerManeuverTraining,TYPE.BrawlerBonusFeats,TYPE.BrawlerCloseWeaponMastery]"]),
            replaces: Some(&["BrawlerBonusFeats", "BrawlerCloseWeaponMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Wild Child Brawler ~ Class Skills", at_level: 1, description: Some("A wild child gains Heal as a class skill."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wild Child Brawler ~ Animal Companion", at_level: 1, description: Some("A wild child forms a bond with a loyal companion that accompanies the wild child on his adventures. A wild child can begin play with any of the animals available to a druid. The wild child uses his brawler level as his effective druid level for determining the abilities of his animal companion."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wild Child Brawler ~ Hunter's Tricks", at_level: 5, description: Some("A wild child can expend a use of martial flexibility to use a trick from the ranger skirmisher archetype. Each time he activates this ability, the wild child can use a different hunter trick. He cannot choose any tricks that rely on ranged attacks. Activating this ability is not an action, but using the trick might require the wild child to use an action of a different type."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wild Child Brawler ~ Maneuver Training", at_level: 3, description: Some("Whenever the wild child chooses a new maneuver for this ability, his animal companion also learns a trick to make use of this combat maneuver. For example, if the wild child chooses maneuver training (dirty trick), his animal companion also learns a trick that allows it to use the dirty trick combat maneuver. He cannot choose any tricks that rely on ranged attacks. This bonus trick doesn't count against the animal companion's total tricks known and does not take any time or checks to train. Should the wild child gain a new animal companion (such as if the previous one dies), this new animal companion begins with the same number of bonus tricks."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wild Child Brawler ~ Wild Tricks", at_level: 5, description: Some("The wild child has learned a number of tricks to aid his allies and his animal companion, as well as to hinder his opponents. He cannot choose any tricks that rely on ranged attacks. The wild child can use these tricks %1 times per day. This ability otherwise follows the rules of the hunter's tricks ability, including all action costs.|WildTricksTimes"), benefit: None },
            ],
        },
        // Cavalier Archetype ~ Daring Champion -- acg_abilities_class.lst:2974
        ArchetypeSwapEntry {
            key: "Cavalier Archetype ~ Daring Champion",
            subject: "Cavalier",
            archetype_name: "Daring Champion",
            description: Some("While many cavaliers are the champions of old fighting forms, some younger, more daring cavaliers mix a martial style influenced by the lighter armored and more flamboyant swashbuckler forms with the dedication of cavalier orders."),
            source_page: Some("p.90"),
            prerequisites: Some(&["PRECLASS:1,Cavalier=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Cavalier Archetype ~ Daring Champion],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CavalierWeaponProficiencies,TYPE.CavalierArmorProficiencies,TYPE.CavalierMount,TYPE.CavalierCavaliersCharge,TYPE.CavalierExpertTrainer,TYPE.CavalierMightyCharge,TYPE.CavalierSupremeCharge]"]),
            replaces: Some(&["CavalierWeaponProficiencies", "CavalierArmorProficiencies", "CavalierMount", "CavalierCavaliersCharge", "CavalierExpertTrainer", "CavalierMightyCharge", "CavalierSupremeCharge"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Daring Champion ~ Weapon and Armor Proficiency", at_level: 1, description: Some("Daring champions are proficient with all simple and martial weapons, light and medium armor, and bucklers. This replaces the cavalier's weapon and armor proficiencies."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Daring Champion ~ Champion's Finesse", at_level: 1, description: Some("A daring champion gains the benefits of the Weapon Finesse feat with light or one-handed piercing melee weapons, and he can use Charisma in place of Intelligence for the purpose of combat feats prerequisites. A daring champion also counts as having the Weapon Finesse feat for the purpose of meeting feat requirements."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Daring Champion ~ Nimble", at_level: 3, description: Some("A daring champion gains a +%1 dodge bonus to AC when wearing light or no armor and carrying no more than a light load. Anything that causes the daring champion to lose his Dexterity bonus to AC also causes him to lose this dodge bonus.|DaringChampionAC"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Daring Champion ~ Panache and Deeds", at_level: 4, description: Some("A daring champion gains the swashbuckler's panache class feature, along with the following swashbuckler deeds: dodging panache, precise strike, and swashbuckler initiative."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Daring Champion ~ Advanced Deeds", at_level: 11, description: Some("A daring champion gains the following swashbuckler deeds: superior feint, targeted strike, subtle blade, and dizzying defense."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Daring Champion ~ Champion's Weapon Mastery", at_level: 20, description: Some("When a daring champion threatens a critical hit with a light or onehanded piercing melee weapon, that critical is automatically confirmed. Furthermore, the critical modifier of those weapons increases by 1 (x2 becomes x3, for example)."), benefit: None },
            ],
        },
        // Cleric Archetype ~ Ecclesitheurge -- acg_abilities_class.lst:3008
        ArchetypeSwapEntry {
            key: "Cleric Archetype ~ Ecclesitheurge",
            subject: "Cleric",
            archetype_name: "Ecclesitheurge",
            description: Some("Eschewing physical armor for protection via the strength of his faith, an ecclesitheurge focuses on the miracles his deity bestows and the breadth of that deity's dominion."),
            source_page: Some("p.91"),
            prerequisites: Some(&["PRECLASS:1,Cleric=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Cleric Archetype ~ Ecclesitheurge],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ClericWeaponProficiencies,TYPE.ClericArmorProficiencies,TYPE.ClericChannelEnergy3,TYPE.ClericDomains]"]),
            replaces: Some(&["ClericWeaponProficiencies", "ClericArmorProficiencies", "ClericChannelEnergy3"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Ecclesitheurge ~ Weapon and Armor Proficiency", at_level: 1, description: Some("An ecclesitheurge is proficient with the club, dagger, heavy crossbow, light crossbow, and quarterstaff, but he's not proficient with any type of armor or shield. This replaces the cleric's weapon and armor proficiencies."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ecclesitheurge ~ Ecclesitheurge's Vow", at_level: 1, description: Some("An ecclesitheurge makes a vow to his deity to be protected solely by his faith, not by armor or shields. An ecclesitheurge who wears armor or uses a shield is unable to use his blessing of the faithful ability, use cleric domain powers, or cast cleric spells."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ecclesitheurge ~ Domain Mastery", at_level: 1, description: Some("[NOT IMPLEMENTED] When an ecclesitheurge chooses his cleric domains, he designates one as his primary domain and the other as his secondary domain. An ecclesitheurge can use his non-domain spell slots to prepare spells from his primary domain's spell list. Each day when he prepares spells, an ecclesitheurge can select a different domain granted by his deity to gain access to that domain's spell list instead of his secondary domain spell list. He does not lose access to his actual secondary domain's granted powers or gain access to the other domain's granted powers. For example, an ecclesitheurge of Sarenrae with Glory and his primary domain and Good as his secondary domain can choose to gain access to the Healing domain; until the next time he prepares spells, he uses the Healing domain spell list as his secondary domain spell list instead of the Good domain spell list, but still keeps the granted powers of the Good domain and does not gain the granted powers of the Healing domain. This ability alters the normal domain ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ecclesitheurge ~ Bonded Holy Symbol", at_level: 3, description: Some("An ecclesitheurge forms a powerful bond with a holy symbol of his deity, which functions identically to a wizard's bonded object except it can be used to cast cleric and domain spells (instead of wizard spells) and the ecclesitheurge can grant his bonded holy symbol only magic abilities appropriate for a holy symbol or a neck slot item. As with a wizard's bonded item, an ecclesitheurge can add additional magic abilities to his bonded holy symbol as if he had the required item creation feat (typically Craft Wondrous Item), provided he meets the feat's level prerequisites. For example, an ecclesitheurge with a bonded holy symbol who wants to add a wondrous amulet ability, like amulet of natural armor, to his bonded holy symbol must be at least 3rd level to do so. The magic properties of a bonded holy symbol, including any magic abilities the ecclesitheurge added to the object, function for only the ecclesitheurge. If a bonded holy symbol's owner dies or the item is replaced, the object loses all enhancements the ecclesitheurge added using this ability. This ability replaces the increase to channel energy gained at 3rd level."), benefit: None },
            ],
        },
        // Druid Archetype ~ Feral Shifter -- acg_abilities_class.lst:3021
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Feral Shifter",
            subject: "Druid",
            archetype_name: "Feral Shifter",
            description: Some("A feral shifter internalizes her communion with and mastery over animals. Instead of forming a bond with an animal companion or an aspect of nature, she alters her own essence or being as homage to the noble creatures of the wild. More in tune with transformation and animal bodies than a normal druid, a feral shifter blurs the line between humanoid and beast."),
            source_page: Some("p.92"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Feral Shifter],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidNatureBond,TYPE.DruidVenomImmunity,TYPE.DruidAThousandFaces,TYPE.DruidTimelessBody]"]),
            replaces: Some(&["DruidNatureBond", "DruidVenomImmunity", "DruidAThousandFaces", "DruidTimelessBody"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Feral Shifter ~ Animal Focus", at_level: 1, description: Some("As a swift action, a feral shifter can take on the aspect of an animal, gaining a bonus or special ability based on the type of animal emulated. This functions as the hunter's animal focus class feature (see page 27). The feral shifter can use this ability for %1 minutes per day. This duration does not need to be consecutive, but it must be spent in 1-minute increments. She can emulate only one animal at a time.|DruidLVL"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Shifter ~ Second Animal Focus", at_level: 9, description: Some("When a feral shifter uses her animal focus ability, she selects two different animal aspects for herself instead of one."), benefit: None },
            ],
        },
        // Druid Archetype ~ Nature Fang -- acg_abilities_class.lst:3022
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Nature Fang",
            subject: "Druid",
            archetype_name: "Nature Fang",
            description: Some("A nature fang is a druid who stalks and slays those who despoil nature, kill scarce animals, or introduce diseases to unprotected habitats. She gives up a close empathic connection with the natural world to become its deadly champion and avenger."),
            source_page: Some("p.92"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Nature Fang],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidNatureSense,TYPE.DruidWildEmpathy,TYPE.DruidWoodlandStride,TYPE.DruidWildShape,TYPE.DruidResistNaturesLure,TYPE.DruidVenomImmunity]"]),
            replaces: Some(&["DruidNatureSense", "DruidWildEmpathy", "DruidWoodlandStride", "DruidWildShape", "DruidResistNaturesLure", "DruidVenomImmunity"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Nature Fang ~ Studied Target", at_level: 1, description: Some("A nature fang gains the slayer's studied target class feature. A nature fang can study an opponent to gain a +%1 bonus on Bluff, Knowledge, and Sense Motive; a +%1 bonus on Perception, and Survival checks attempted against that opponent; and a +%1 bonus on weapon attack and damage rolls against it. The DCs of nature fang class abilities against that opponent increase by %1. If a slayer deals sneak attack damage to a target, he can study that target as an immediate action, allowing him to apply his studied target bonuses against that target (including to the normal weapon damage roll). A nature fang can only maintain these bonuses against one opponents at a time; these bonuses remain in effect until either the opponent is dead or the nature fang studies a new target. The nature fang may discard this connection to a studied target as a free action, allowing him to study another target in its place.|NatureFangStudiedTargetBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Nature Fang ~ Slayer Talent", at_level: 4, description: Some("At 4th level and every 2 levels thereafter, a nature fang selects a slayer talent. Starting at 12th level, she can select an advanced slayer talent in place of a slayer talent. She uses her druid level as her slayer level to determine what talents she can select."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Nature Fang ~ Sneak Attack", at_level: 4, description: Some("A nature fang gains sneak attack +1d6. This functions as the rogue sneak attack ability. If the nature fang gets a sneak attack bonus from another source, the bonuses on damage stack."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Nature Fang ~ Swift Studied Target", at_level: 9, description: Some("A nature fang can study an opponent as a move or swift action."), benefit: None },
            ],
        },
        // Druid Archetype ~ Wild Whisperer -- acg_abilities_class.lst:3023
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Wild Whisperer",
            subject: "Druid",
            archetype_name: "Wild Whisperer",
            description: Some("A wild whisperer is an expert at studying, predicting, and explaining animal behavior. She is less interested in plants, fey, and other aspects of the natural world, and uses her gifts to tame or relocate dangerous beasts and soothe the hurts of wounded and sickly creatures."),
            source_page: Some("p.92"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Wild Whisperer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidWoodlandStride,TYPE.DruidTracklessStep,TYPE.DruidResistNaturesLure,TYPE.DruidWildShape]"]),
            replaces: Some(&["DruidWoodlandStride", "DruidTracklessStep", "DruidResistNaturesLure", "DruidWildShape6", "DruidWildShape8"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Wild Whisperer ~ Inspiration", at_level: 2, description: Some("At 2nd level, a wild whisperer gains an inspiration pool as the investigator class ability (see page 31). A wild whisperer uses her druid level as her investigator level when determining the effects of this ability. She has an inspiration pool equal to %1. Instead of free uses of inspiration on Knowledge, Linguistics, or Spellcraft skill checks, the wild whisperer can use inspiration on Handle Animal, Heal, Knowledge (geography), Knowledge (nature), Ride, Sense Motive, and Survival skill checks without expending a use of inspiration, provided she's trained in the relevant skill. She can also use inspiration on any wild empathy check without expending a use of inspiration. A wild whisperer's inspiration pool refreshes each day when she prepares spells.|WildWhispererInspirationPoolBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wild Whisperer ~ Wild Shape", at_level: 4, description: Some("A wild whisperer gains the wild shape ability, but she never gains access to any forms beyond Small and Medium animal forms, as beast shape I."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wild Whisperer ~ Natural Expertise", at_level: 6, description: Some("A wild whisperer's powers of observation give her an advantage when she's fighting natural creatures. When using inspiration on an attack roll against an animal or a vermin or on a saving throw against an effect from an animal or a vermin, a wild whisperer has to expend only one use of inspiration instead of two."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wild Whisperer ~ Investigator Talent", at_level: 8, description: Some("A wild whisperer selects an investigator talent (see page 32)."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Martial Master -- acg_abilities_class.lst:3044
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Martial Master",
            subject: "Fighter",
            archetype_name: "Martial Master",
            description: Some("There are those who learn the fighting arts though countless hours of repetition and training, while others seem to pick up new stances and forms as if they were born to them."),
            source_page: Some("p.93"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ Martial Master],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterWeaponTraining,TYPE.FighterWeaponMastery]"]),
            replaces: Some(&["FighterWeaponTraining", "FighterWeaponMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Martial Master ~ Martial Flexibility", at_level: 5, description: Some("The martial master can use a move action to gain the benefit of a combat feat he doesn't possess. The martial master must otherwise meet all the feat's prerequisites. |PREVARLT:FighterLVL,9"), benefit: None },
            ],
        },
        // Fighter Archetype ~ Mutation Warrior -- acg_abilities_class.lst:3045
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Mutation Warrior",
            subject: "Fighter",
            archetype_name: "Mutation Warrior",
            description: Some("While most fighters rely on physical fitness and rigorous training to achieve martial superiority, a few prefer to create and imbibe dangerous concoctions that mutate them into fearsome creatures."),
            source_page: Some("p.93"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ Mutation Warrior],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterArmorTraining1,TYPE.FighterArmorTraining2,TYPE.FighterArmorTraining3,TYPE.FighterArmorTraining4,TYPE.FighterArmorMastery]"]),
            replaces: Some(&["FighterArmorTraining1", "FighterArmorTraining2", "FighterArmorTraining3", "FighterArmorTraining4", "FighterArmorMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Mutation Warrior ~ Mutagen", at_level: 3, description: Some("A mutation warrior discovers how to create a mutagen that he can imbibe in order to heighten his physical prowess at the cost of his personality. This ability functions as the alchemist's mutagen ability (APG p.28), using his fighter level as his alchemist level. This ability replaces armor training 1."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mutation Warrior ~ Mutagen Discovery", at_level: 7, description: Some("At 7th level and every 4 levels thereafter, the mutation warrior can choose one of the following alchemist discoveries to augment his abilities: feral mutagen, grand mutagen, greater mutagen, infuse mutagen, nauseating flesh, preserve organs, rag doll mutagen, spontaneous healing, tentacle, vestigial arm, wings. The mutagen warrior uses his fighter level as his effective alchemist level for the purpose of whether he qualifies for these discoveries."), benefit: None },
            ],
        },
        // Hunter Archetype ~ Divine Hunter -- acg_abilities_class.lst:3138
        ArchetypeSwapEntry {
            key: "Hunter Archetype ~ Divine Hunter",
            subject: "Hunter",
            archetype_name: "Divine Hunter",
            description: Some("While most hunters heed the call of nature and fight to protect its bounty, some are inspired to serve a higher power. These divine hunters use faith to aid them in their struggles, and their faith infuses their animal companions, making these companions champions of their deities."),
            source_page: Some("p.96"),
            prerequisites: Some(&["PRECLASS:1,Hunter=1", "PREMULT:1,[PREMULT:2,[PREDEITYALIGN:LE],[PREALIGN:LE,LN,NE]],[PREMULT:2,[PREDEITYALIGN:LN],[PREALIGN:LN,LE,LG,TN]],[PREMULT:2,[PREDEITYALIGN:LG],[PREALIGN:LG,NG,LN]],[PREMULT:2,[PREDEITYALIGN:NE],[PREALIGN:NE,LE,CE,TN]],[PREMULT:2,[PREDEITYALIGN:TN],[PREALIGN:TN,LN,CN,NG,NE]],[PREMULT:2,[PREDEITYALIGN:NG],[PREALIGN:NG,TN,LG,CG]],[PREMULT:2,[PREDEITYALIGN:CE],[PREALIGN:CE,NE,CN]],[PREMULT:2,[PREDEITYALIGN:CN],[PREALIGN:CN,TN,CG,CE]],[PREMULT:2,[PREDEITYALIGN:CG],[PREALIGN:CG,CN,NG]]", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Hunter Archetype ~ Divine Hunter],[!PREABILITY:1,CATEGORY=Archetype,TYPE.HunterClassSkills,TYPE.HunterTeamworkFeats,TYPE.HunterHunterTactics]"]),
            replaces: Some(&["HunterClassSkills", "HunterTeamworkFeats", "HunterHunterTactics"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Divine Hunter ~ Class Skills", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Divine Hunter ~ Domain", at_level: 3, description: Some("[PARTIALLY IMPLEMENTED] A divine hunter learns to call upon the power of her deity. The divine hunter must select one domain from those available to her deity. She gains the granted powers of this domain, using her hunter level - 2 as her cleric level for determining when the powers are gained and what effects they have. Once she chooses this domain, it cannot be changed. If the divine hunter selects the animal domain, she does not gain a second animal companion upon reaching an effective cleric level of 4th. When the divine hunter would gain that ability, her animal companion instead gains two ability score increases (gaining +1 to two different ability scores or +2 to one ability score). If her animal companion dies or is released, when she gains a new one, it benefits from this ability score increase. In addition, the divine hunter adds the 1st-level domain spell from her domain to her list of spells known. She adds the 2nd-level domain spell at 6th level, the 3rd-level domain spell at 9th level, the 4th-level domain spell at 12th level, the 5th-level domain spell at 15th level, and the 6th-level domain spell at 18th level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Divine Hunter ~ Otherworldly Companion", at_level: 3, description: Some("A hunter's companion takes on otherworldly features. If the divine hunter is good (or worships a good deity), the animal companion gains the celestial template. If the hunter is evil (or worships an evil deity), the animal companion gains the fiendish template. If the hunter is neutral and worships a neutral deity, she must choose either the celestial or fiendish template; once this choice is made, it cannot be changed. The companion's CR is considered to be equal to its Hit Dice for the purpose of the celestial or fiendish template."), benefit: None },
            ],
        },
        // Hunter Archetype ~ Feral Hunter -- acg_abilities_class.lst:3139
        ArchetypeSwapEntry {
            key: "Hunter Archetype ~ Feral Hunter",
            subject: "Hunter",
            archetype_name: "Feral Hunter",
            description: Some("A feral hunter has forged a bond with nature that's so strong that she doesn't merely channel the aspects of animals-she actually becomes an animal herself. Though she lacks an animal companion, a feral hunter is in tune with the beast lurking within her flesh and spirit, and lives in a near-wild state of being. A feral hunter often resembles a lycanthrope, but her power comes from her own nature and is not influenced by moonlight or silver."),
            source_page: Some("p.96"),
            prerequisites: Some(&["PRECLASS:1,Hunter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Hunter Archetype ~ Feral Hunter],[!PREABILITY:1,CATEGORY=Archetype,TYPE.HunterAnimalCompanion,TYPE.HunterAnimalFocus,TYPE.HunterHunterTactics,TYPE.HunterSpeakWithMaster,TYPE.HunterPreciseCompanion,TYPE.HunterBonusTricks,TYPE.HunterImprovedEmpatheticLink,TYPE.HunterGreaterEmpatheticLink,TYPE.HunterMasterOfTheWild,TYPE.HunterRaiseAnimalCompanion,TYPE.HunterTeamworkFeat6,TYPE.HunterTeamworkFeat9,TYPE.HunterTeamworkFeat12,TYPE.HunterTeamworkFeat15,TYPE.HunterTeamworkFeat18]"]),
            replaces: Some(&["HunterAnimalCompanion", "HunterAnimalFocus", "HunterHunterTactics", "HunterSpeakWithMaster", "HunterPreciseCompanion", "HunterBonusTricks", "HunterImprovedEmpatheticLink", "HunterGreaterEmpatheticLink", "HunterMasterOfTheWild", "HunterRaiseAnimalCompanion", "HunterTeamworkFeat6", "HunterTeamworkFeat9", "HunterTeamworkFeat12", "HunterTeamworkFeat15", "HunterTeamworkFeat18"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Feral Hunter ~ Solitary", at_level: 1, description: Some("Unlike most hunters, a feral hunter does not gain an animal companion."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Hunter ~ Feral Focus", at_level: 1, description: Some("A feral hunter gains a limited ability to change her shape into hybrid animal forms. This functions as the animal focus class feature, except that the hunter always applies the animal aspect to herself, and there is no limit to this ability's duration. She can end this ability as a free action. When a feral hunter uses this ability, her body takes on cosmetic aspects of an animal, such as furry skin, longer nails, elongated teeth, and oddly colored eyes; these changes do not grant her any abilities other than what is stated in the animal focus, and end when she takes on a different aspect or ends the ability. This physical change is a polymorph effect, though the effects of the animal focus are not."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Hunter ~ Precise Summoned Animal", at_level: 2, description: Some("This functions like the precise companion class ability, except the hunter grants all her teamwork feats to all animals she summons with summon nature's ally."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Hunter ~ Wild Shape", at_level: 4, description: Some("A feral hunter gains the ability to change shape. This ability functions like the druid wild shape ability, except the hunter can take only animal forms (not elemental or plant forms). The hunter's effective druid level is equal to her class level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Hunter ~ Summon Pack", at_level: 6, description: Some("Whenever a feral hunter casts a summon nature's ally spell to summon one or more animals, she summons one additional animal of the same type. The summoned creature or creatures must be animals and must be of the same type as the hunter's current aspect or of a similar type (bears for bear aspect, dogs or wolves for wolf aspect, great cats for the tiger aspect, and so on). The additional creature immediately vanishes if the hunter chooses a different aspect or ends his feral focus ability. She can increase the duration of any one summon nature's ally spell afected by this ability to 1 minute per level. She can have only one spell with a duration increased by this ability active at a time."), benefit: None },
            ],
        },
        // Hunter Archetype ~ Packmaster -- acg_abilities_class.lst:3140
        ArchetypeSwapEntry {
            key: "Hunter Archetype ~ Packmaster",
            subject: "Hunter",
            archetype_name: "Packmaster",
            description: Some("Some hunters form bonds with packs of well-trained creatures. Whether such a hunter is a northern berserker running with a pack of timber wolves or a savage warrior dashing through the jungle alongside her herd of dimetrodons, the packmaster revels in the thrill of the hunt and the glory of the kill. A packmaster is more comfortable in groups than alone, and although her animal companions may be weaker than a typical hunter's, what they lack in strength they make up for in numbers."),
            source_page: Some("p.96"),
            prerequisites: Some(&["PRECLASS:1,Hunter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Hunter Archetype ~ Packmaster],[!PREABILITY:1,CATEGORY=Archetype,TYPE.HunterAnimalCompanion,TYPE.HunterAnimalFocus,TYPE.HunterTeamworkFeats,TYPE.HunterSecondAnimalFocus,TYPE.HunterMasterHunter]"]),
            replaces: Some(&["HunterAnimalCompanion", "HunterAnimalFocus", "HunterTeamworkFeats", "HunterSecondAnimalFocus", "HunterMasterHunter"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Packmaster ~ Pack Bond", at_level: 1, description: Some("A packmaster can have more than one animal companion, but she must divide her effective druid level between her companions to determine the abilities of each one. For example, a 4th-level packmaster can have one 4th-level animal companion, two 2nd-level companions, one 3rd-level companion and one 1st-level companion, or four 1st-level companions. A packmaster's precise companion, woodland stride, and teamwork feats each apply to only one of her animal companions at a time."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Packmaster ~ Pack Focus", at_level: 1, description: Some("[NOT IMPLEMENTED] This ability functions like animal focus, with the following exceptions. A packmaster can apply her animal aspect to only one of her animal companions at a time without it counting against the number of minutes per day she can use that ability. When using animal focus on herself or her other animal companion, the ability counts against her minutes per day as normal. She can have only two animal aspects in effect at a time-one that counts against her minutes per day and one that doesn't - and they can't both target the same companion. Unless both her companions are dead, the hunter can't apply the companion's aspect to herself (and thereby gain the benefit of its unlimited duration)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Packmaster ~ Teamwork Feat", at_level: 3, description: Some("At 3rd level or any level at which a packmaster would gain a bonus teamwork feat, she can instead increase the number of her animal companions that gain the benefits of her precise companion, woodland stride, and teamwork feats by 1. The current number of animal companions that can benefit from precise companion is %1. She can select this ability multiple times. This ability alters teamwork feats.|PackmasterPreciseCompanionAnimals"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Packmaster ~ Second Pack Focus", at_level: 8, description: Some("[NOT IMPLEMENTED] At 8th level, the hunter gains an ability that functions like the second animal focus, but the hunter can either assign each companion one aspect each or assign both aspects to the same companion. The foci on the companions don't need to be the same, nor do they need to be the same as the one assigned to the packmaster. This ability replaces second animal focus."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Packmaster ~ Master Of The Pack", at_level: 20, description: Some("At 20th level, a packmaster and her animal companions can always move at full speed while using Survival to follow tracks without penalty. Each day when a packmaster gains new spells for the day, she chooses one animal focus to be active on herself or one of her animal companions for the entire day (if all of her animal companions are dead, she instead chooses two animal foci to be active on herself for the entire day). This focus is in addition to her pack focus class ability. This ability replaces master hunter."), benefit: None },
            ],
        },
        // Hunter Archetype ~ Primal Companion Hunter -- acg_abilities_class.lst:3141
        ArchetypeSwapEntry {
            key: "Hunter Archetype ~ Primal Companion Hunter",
            subject: "Hunter",
            archetype_name: "Primal Companion Hunter",
            description: Some("Most hunters are skilled at awakening the primal beasts inside themselves. However, some can instead activate the primal essence within their animal companion. These primal companion hunters bestow upon their companions the ability to suddenly manifest new and terrifying powers-throwbacks to long-extinct beasts, bizarre mutations from extreme environments, or new abilities crafted from generations of selective breeding."),
            source_page: Some("p.96"),
            prerequisites: Some(&["PRECLASS:1,Hunter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Hunter Archetype ~ Primal Companion Hunter],[!PREABILITY:1,CATEGORY=Archetype,TYPE.HunterAnimalFocus,TYPE.HunterSecondAnimalFocus,TYPE.HunterMasterHunter]"]),
            replaces: Some(&["HunterAnimalFocus", "HunterSecondAnimalFocus", "HunterMasterHunter"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Primal Companion Hunter ~ Primal Transformation", at_level: 1, description: Some("(NOT IMPLEMENTED) At first level, a primal companion hunter can awaken a primal creature from within his animal companion as a swift action. The animal companion gains a pool of 2 evolution points (Advanced Player's Guide 60) that can be used to temporarily give the companion evolutions as if it were an eidolon. A primal companion hunter uses her hunter level to determine her effective summoner level for the purpose of qualifying for evolutions and determining their effect. At 8th level, the number of evolution points in her pool increases to 4, and at 15th level, it increases to 6. Activating these evolutions on the animal companion is a swift action. A primal companion hunter can use this ability for 1 minute per day per hunter level. This duration need not to be consecutive, but it must be spent in 1-minute increments. An animal companion transformed in this way cannot exceed the maximum number of attacks available to the eidolon of a summoner whose class level equals that of the hunter. While transformed in this way, the animal companion's type changes to magical beast, though the primal companion hunter still treats it as an animal for the purpose of the Handle Animal skill. If a primal companion hunter's animal companion is dead, she can apply these evolutions to herself instead of to her animal companion. Uses of this ability count toward the hunter's maximum daily duration of evolution use. This ability replaces animal focus."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Primal Companion Hunter ~ Primal Surge", at_level: 8, description: Some("(NOT IMPLEMENTED) At 8th level, once per day as a swift action, a primal companion hunter can touch her animal companion and grant it one evolution that costs up to 4 evolution points. The companion must meet the prerequisites of the selected evolution. Unlike the evolutions from primal transformation, this evolution is not set; it can be changed each time the hunter uses this ability. Using primal surge activates the primal transformation ability on the companion if it isn't already active. This effect lasts until the hunter ends the primal transformation. This does not allow a companion to exceed its maximum number of natural attacks. This ability can grant only one evolution at a time, even if the chosen evolution could be selected multiple times. This ability can grant an evolution that allows additional evolution points to be spent to upgrade that evolution (such as damage reduction or flight), and any points left over can be spent on such upgrades. This ability cannot be used to grant an upgrade to an evolution that the companion already possesses. This ability replaces second animal focus."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Primal Companion Hunter ~ Primal Master", at_level: 20, description: Some("(NOT IMPLEMENTED) At 20th level, a primal companion hunter becomes in tune with his primal nature. He can activate his companion's primal aspect as a free action. When using primal surge, he can grant his companion two evolutions instead of one (each costing up to 4 evolution points). This ability replaces master hunter."), benefit: None },
            ],
        },
        // Hunter Archetype ~ Verminous Hunter -- acg_abilities_class.lst:3142
        ArchetypeSwapEntry {
            key: "Hunter Archetype ~ Verminous Hunter",
            subject: "Hunter",
            archetype_name: "Verminous Hunter",
            description: Some("A verminous hunter calls on the ceaseless, single-minded dedication of vermin to hunt and overwhelm her prey. Where other hunters invoke the cunning, animalistic powers of the alpha predators, she calls on the powers of the lowest life forms, reaching out to the spider instead of the monkey, the mantis instead of the snake, or the moth instead of the owl."),
            source_page: Some("p.96"),
            prerequisites: Some(&["PRECLASS:1,Hunter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Hunter Archetype ~ Verminous Hunter],[!PREABILITY:1,CATEGORY=Archetype,TYPE.HunterAnimalCompanion,TYPE.HunterSecondAnimalFocus,TYPE.HunterWildEmpathy,TYPE.HunterAnimalFocus,TYPE.HunterWoodlandStride,TYPE.HunterMasterHunter]"]),
            replaces: Some(&["HunterAnimalCompanion", "HunterWildEmpathy", "HunterAnimalFocus", "HunterSecondAnimalFocus", "HunterWoodlandStride", "HunterMasterHunter"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Verminous Hunter ~ Vermin Companion", at_level: 1, description: Some("A verminous hunter must choose a vermin companion instead of an animal companion. The hunter tactics class ability allows a verminous hunter to grant her teamwork feats to a mindless vermin companion."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Verminous Hunter ~ Vermin Empathy", at_level: 1, description: Some("A verminous hunter gains the wild empathy ability, but can use it only to influence vermin (not animals or magical beasts)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Verminous Hunter ~ Vermin Focus", at_level: 1, description: Some("You can take on the aspect of a vermin as a swift action, gaining a bonus or special ability based on the type of vermin emulated. The ability is usable %1 minutes per day. The vermin companion gets a constant benefit.|HunterVerminFocusMinutes"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Verminous Hunter ~ Swarm Stride", at_level: 5, description: Some("A verminous hunter learns to move through vermin without danger. He can safely pass through swarms of vermin and does not take swarm damage while within a vermin swarm's space. In addition, he is immune to a swarm's distraction ability. If the hunter or his animal companion attacks a swarm, they lose this protection against only that swarm."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Verminous Hunter ~ Second Vermin Focus", at_level: 8, description: Some("Whenever a hunter uses her animal focus ability, she selects two different animal aspects for herself instead of one, and can assign two aspects to her companion instead of one."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Verminous Hunter ~ Master Hunter", at_level: 20, description: Some("A verminous hunter becomes a master verminous hunter. She can always move at full speed while using Survival to follow tracks without penalty. Each day when the hunter regains her spell slots, she chooses one vermin focus to be active on herself for the entire day. This focus is in addition to using her vermin focus class ability."), benefit: None },
            ],
        },
        // Inquisitor Archetype ~ Sacred Huntsmaster -- acg_abilities_class.lst:3285
        ArchetypeSwapEntry {
            key: "Inquisitor Archetype ~ Sacred Huntsmaster",
            subject: "Inquisitor",
            archetype_name: "Sacred Huntsmaster",
            description: Some("Some inquisitors create a strong bond with an animal companion, and they hunt and punish threats to the faith as an awe-inspiring duo. When they work together as one, there are few that dare to stand in their way."),
            source_page: Some("p.96"),
            prerequisites: Some(&["PRECLASS:1,Inquisitor=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Inquisitor Archetype ~ Sacred Huntsmaster],[!PREABILITY:1,CATEGORY=Archetype,TYPE.InquisitorJudgements,TYPE.InquisitorSoloTactics,TYPE.InquisitorSecondJudgement,TYPE.InquisitorThirdJudgement,TYPE.InquisitorSlayer,TYPE.InquisitorTrueJudgement]"]),
            replaces: Some(&["InquisitorJudgements", "InquisitorSoloTactics", "InquisitorSecondJudgement", "InquisitorThirdJudgement", "InquisitorSlayer", "InquisitorTrueJudgement"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Sacred Huntsmaster ~ Animal Companion", at_level: 1, description: Some("A hunter forms a bond with an animal companion. A hunter may begin play with any of the animals on the druid list. This animal is a loyal companion that accompanies the hunter on her adventures. The hunter's effective druid level is equal to her hunter level. If a character receives an animal companion from more than one source, her effective druid levels stack for the purposes of determining the statistics and abilities of the companion. If a hunter releases her companion from service or her animal companion perishes, she may gain a new one by performing a ceremony requiring 24 uninterrupted hours of prayer in the environment where the sought companion typically lives."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sacred Huntsmaster ~ Hunter Tactics", at_level: 3, description: Some("A sacred huntsmaster automatically grants her teamwork feats to her animal companion. The companion doesn't need to meet the prerequisites of these teamwork feats."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sacred Huntsmaster ~ Animal Focus", at_level: 4, description: Some("A sacred huntsmaster can take on the aspects of an animal as a swift action, gaining a bonus or special ability based on the type of animal emulated. The ability is usable %1 minutes per day. The animal companion gets a constant benefit. This ability works as the hunter's animal focus class feature, with her inquisitor level serving as her hunter level.|HunterAnimalFocusMinutes"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sacred Huntsmaster ~ Improved Empathic Link", at_level: 8, description: Some("The sacred huntsmaster gains an empathic link with her animal companion. This functions like an empathic link with a familiar, except the sacred huntsmaster can also see through a companion's eyes as a swift action, maintaining this connection as long as she likes (as long as the companion is within 1 mile) and ending it as a free action. The sacred huntsmaster is blinded while maintaining this connection."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sacred Huntsmaster ~ Raise Animal Companion", at_level: 16, description: Some("A sacred huntsmaster gains raise animal companion as a spelllike ability (not restricted to her own animal companion). Using this ability gives the sacred huntsmaster a negative level that cannot be overcome in any way (including by restoration), but automatically ends after 24 hours. This functions as resurrection instead of raise dead, but otherwise operates as normal."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sacred Huntsmaster ~ Second Animal Focus", at_level: 17, description: Some("Whenever a sacred huntsmaster uses her animal focus ability, she selects two different animal aspects for herself instead of one, and can assign two aspects to her companion instead of one. As with the companion's previous aspect, the second one does not count against the minutes per day a sacred huntsmaster can take on an aspect. If her animal companion is dead and the sacred huntsmaster has applied that aspect to herself, that aspect counts toward her maximum of two aspects at once. The sacred huntsmaster can still only apply one of her dead companion's to herself, not both."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sacred Huntsmaster ~ Greater Empathic Link", at_level: 20, description: Some("The range of a sacred huntsmaster's empathic link with her animal companion increases to 10 miles. If the animal companion is within 1 mile, it and the sacred huntsmaster can communicate telepathically."), benefit: None },
            ],
        },
        // Inquisitor Archetype ~ Sanctified Slayer -- acg_abilities_class.lst:3286
        ArchetypeSwapEntry {
            key: "Inquisitor Archetype ~ Sanctified Slayer",
            subject: "Inquisitor",
            archetype_name: "Sanctified Slayer",
            description: Some("While all inquisitors root out enemies of the faith, in many orders and churches there's a select group of these religious hunters devoted to one goal, and one goal alone-to terminate the enemies of the faith wherever they can be found. Sometimes these sanctified slayers are given special dispensation to commit ruthless murders for the faith's greater good. Other times, they're simply willing to take the initiative to revel in the zeal of such grisly work."),
            source_page: Some("p.96"),
            prerequisites: Some(&["PRECLASS:1,Inquisitor=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Inquisitor Archetype ~ Sanctified Slayer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.InquisitorJudgements,TYPE.InquisitorTrueJudgement]"]),
            replaces: Some(&["InquisitorJudgements", "InquisitorTrueJudgement"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Sanctified Slayer ~ Studied Target", at_level: 1, description: Some("A sanctified slayer gains the slayer's studied target class feature. She uses her inquisitor level as her effective slayer level to determine the effects of studied target."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sanctified Slayer ~ Sneak Attack", at_level: 4, description: Some("If a sanctified slayer catches an opponent unable to defend itself effectively from her attack, she can strike a vital spot to deal extra damage. The sanctified slayer's attack deals additional damage anytime her target would be denied a Dexterity bonus to AC (whether or not the target actually has a Dexterity bonus), or when the sanctified slayer flanks her target. This additional damage is 1d6 at 4th level, and increases by 1d6 every 3 levels thereafter. Should a sanctified slayer score a critical hit with the sneak attack, this extra damage is not multiplied. Ranged attacks can count as sneak attacks only if the target is within 30 feet. With a weapon that deals nonlethal damage (such as a sap, whip, or an unarmed strike), a sanctified slayer can make a sneak attack that deals nonlethal damage instead of lethal damage. She cannot use a weapon that deals lethal damage to deal nonlethal damage in a sneak attack, even with the usual -4 penalty. A sanctified slayer must be able to see the target well enough to pick out a vital spot and must be able to reach such a spot. A sanctified slayer cannot use sneak attack while striking a creature with concealment. This ability replaces the later iterations of the judgment ability. The current sneak damage is %1d6|InvestigatorSantifiedSlayerSneakDice"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sanctified Slayer ~ Talented Slayer", at_level: 8, description: Some("At 8th, 16th, 17th, and 20th levels, a sanctified slayer can gain a single slayer talent, including those from the list of rogue talents that a slayer can take, but not an advanced slayer talent."), benefit: None },
            ],
        },
        // Investigator Archetype ~ Empiricist -- acg_abilities_class.lst:3305
        ArchetypeSwapEntry {
            key: "Investigator Archetype ~ Empiricist",
            subject: "Investigator",
            archetype_name: "Empiricist",
            description: Some("Champions of deductive reasoning and logical insight, empiricists put their faith in facts, data, confirmed observations, and consistently repeatable experiments- these things are their currency of truth."),
            source_page: Some("p.100"),
            prerequisites: Some(&["PRECLASS:1,Investigator=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Investigator Archetype ~ Empiricist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.InvestigatorPoisonLore,TYPE.InvestigatorPoisonResistance,TYPE.InvestigatorSwiftAlchemy,TYPE.InvestigatorTrueInspiration]"]),
            replaces: Some(&["InvestigatorPoisonLore", "InvestigatorPoisonResistance", "InvestigatorSwiftAlchemy", "InvestigatorTrueInspiration"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Empiricist ~ Ceaseless Observation", at_level: 2, description: Some("An empiricist's ability to notice the minutiae of almost everything that happens around him allows him to make shrewd and insightful calculations about people and even inanimate objects. At 2nd level, an empiricist uses his Intelligence modifier instead of the skill's typical ability for all Disable Device, Perception, Sense Motive, and Use Magic Device checks. He can also use his Intelligence modifier instead of Charisma on any Diplomacy checks made to gather information. This ability replaces poison lore and poison resistance."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Empiricist ~ Unfailing Logic", at_level: 4, description: Some("An empiricist's grasp of facts and data teaches him to anchor himself in reality, granting resistance to even the most potent illusions. At 4th level, an empiricist gains a +2 insight bonus on all Will saving throws against illusion spells or spell-like abilities that allow a save to disbelieve their effects. In addition he can spend one point from his inspiration pool as an immediate action to use his Intelligence bonus instead of her Wisdom bonus on all such saves for one round. At 8th level, the empiricist's insight bonus increases to +4. At 16th level, he gains immunity to all illusion spells and spell-like abilities that allow a save to disbelieve the effects. This ability replaces swift alchemy."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Empiricist ~ Master Intellect", at_level: 20, description: Some("An empiricist's powers of reason and deduction become almost superhuman, and he is able to use them in nearly all aspects of life. At 20th level, an empiricist can use inspiration on all skills (even ones he is not trained in) and all ability checks (including initiative checks) without spending inspiration. This ability replaces true inspiration."), benefit: None },
            ],
        },
        // Investigator Archetype ~ Infiltrator -- acg_abilities_class.lst:3306
        ArchetypeSwapEntry {
            key: "Investigator Archetype ~ Infiltrator",
            subject: "Investigator",
            archetype_name: "Infiltrator",
            description: Some("An infiltrator specializes in investigating or disrupting groups from within. He uses his specialized set of skills and alchemical abilities to morph into the shape of the people or creatures whose company he's infiltrating, or even specific individuals."),
            source_page: Some("p.100"),
            prerequisites: Some(&["PRECLASS:1,Investigator=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Investigator Archetype ~ Infiltrator],[!PREABILITY:1,CATEGORY=Archetype,TYPE.InvestigatorTrapfinding,TYPE.InvestigatorPoisonLore,TYPE.InvestigatorPoisonResistance]"]),
            replaces: Some(&["InvestigatorTrapfinding", "InvestigatorPoisonLore", "InvestigatorPoisonResistance"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Infiltrator ~ Master of Disguise", at_level: 1, description: Some("An infiltrator can use disguise with great results. When disguising himself as a different gender, race, age category, or size category, an infiltrator reduces the penalties for each by -2. For example, if the infiltrator disguises himself as a female two age categories older than himself, he would take a -2 to the check instead of a -6. Also, an infiltrator can disguise himself with 1d3 minutes of work (instead of the normal 1d3 x 10 minutes of work). This ability replaces trapfinding."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Infiltrator ~ Voice Mimicry", at_level: 2, description: Some("At 2nd level, an infiltrator learns to mimic voices and sounds around him. Using this ability requires a special Disguise check, and creatures hearing the voice can make a Perception check to discover the ruse. An infiltrator can attempt to emulate any creature or other sound he's heard clearly for at least 1 minute. The bonuses or penalties to this special Disguise check are modified in the following ways, all of which are cumulative. +5 bonus for a non-specific voice, -2 each for different gender, race, or age category, and -5 for different size category. Also, the creature making the Perception check gains a bonus based on its familiarity with specific voices, just as if it were confronted with a normal disguise (Core Rulebook 95). This ability is a language-dependent effect, meaning that if a creature cannot hear or understand what the infiltrator is saying, the ruse fails. Magic items, feats, and traits that affect typical disguises do not affect this disguise check. At 8th level, the infiltrator gains the effects of its master of disguise class feature on this special use of the Disguise skill (no penalty for a voice of a different gender, race, or age category, and only a -3 penalty for a different size category). This ability replaces poison lore."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Infiltrator ~ Mimic Mastery", at_level: 2, description: Some("At 2nd level, when an infiltrator uses disguise self or any polymorph extracts on himself, he is treated as 2 investigator levels higher for the purpose of determining the duration of that extract's effect. He can use these extracts to take the appearance of specific individuals of the type of form he chooses, gaining a +10 to Disguise checks even if that extract does not typically grant such a bonus. Furthermore, these extracts grant the infiltrator a +10 bonus to Disguise checks made as part of his voice mimicry ability. This ability replaces poison resistance."), benefit: None },
            ],
        },
        // Investigator Archetype ~ Mastermind -- acg_abilities_class.lst:3307
        ArchetypeSwapEntry {
            key: "Investigator Archetype ~ Mastermind",
            subject: "Investigator",
            archetype_name: "Mastermind",
            description: Some("Although some investigators use their honed senses and cunning insight for personal gain, no one excels at such endeavors like the mastermind. Typically, these investigators dwell at the centers of complex networks of lies, minions, or simply precious information, from which they dispense commands, threats, and rumors, all carefully crafted to increase the power of their peculiar empires. While masterminds often act as the heads of illicit organizations like criminal families, thieves' guilds, or corruption-riddled bureaucracies, they aren't always evil."),
            source_page: Some("p.101"),
            prerequisites: Some(&["PRECLASS:1,Investigator=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Investigator Archetype ~ Mastermind],[!PREABILITY:1,CATEGORY=Archetype,TYPE.InvestigatorTrapfinding,TYPE.InvestigatorTrapSense,TYPE.InvestigatorySwiftAlchemy,TYPE.InvestigatorTalent9,TYPE.InvestigatorInspiration]"]),
            replaces: Some(&["InvestigatorTrapfinding", "InvestigatorTrapSense", "InvestigatorySwiftAlchemy", "InvestigatorTalent9"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Mastermind ~ Mastermind's Inspiration", at_level: 1, description: Some("A mastermind can use inspiration on any Diplomacy and Intimidate checks without spending a use of inspiration, but can't do so for Linquistics or Spellcraft checks. This ability alters inspiration."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mastermind ~ A Quiet Word", at_level: 1, description: Some("A mastermind's reputation precedes him. At 1st level, once per day a mastermind can spend 10 minutes preparing an ally to make a single Diplomacy or Intimidate check (mastermind's choice when preparing the ally) within the next 24-hour period at the mastermind's behest. This skill check uses the mastermind's skill ranks instead of the ally's. The mastermind's affected ally still uses its own ability bonus for the check. Furthermore, when a mastermind uses this ability, he can expend one use of inspiration to give the ally use of the inspiration die when making the check. At 3rd level, a mastermind can use this ability an additional time each day, and the number of times he can use this ability per day increases by 1 for every third level thereafter. Multiple uses of this ability on the same ally grant that ally the benefit on additional Diplomacy or Intimidate checks. At 12th level, a mastermind can use this ability to bestow on his ally the use of his skill ranks, similar to the 1stlevel effects of this ability but with a wider range of skills to choose from. The mastermind can only bestow this for a skill check that he can use inspiration on without expending uses of inspiration. For instance, a mastermind with the underworld inspiration talent can bolster his ally's Bluff, Disable Device, Disguise, or Sleight of Hand skill checks, selecting a single skill for each use of the ability. This ability replaces trapfinding and trap sense."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mastermind ~ Mastermind Defense", at_level: 4, description: Some("At 4th level, a mastermind can, as an immediate action, expend two uses of inspiration to make an inspired defense. He rolls his inspiration die and applies the result as a penalty on an attack roll made against him. If the mastermind has the combat inspiration talent, he can expend one use of inspiration instead of two. This ability replaces swift alchemy."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mastermind ~ Impregnable Mind", at_level: 9, description: Some("At 9th level, a mastermind's secrecy, obscurity, and mental conditioning reach superhuman levels. He becomes immune to any divination spell, spell-like ability, or effect that allows a saving throw (though he can still allow a divination effect to affect him if he wishes). Even divination effects that do not allow a saving throw have difficulty piercing a mastermind's barriers, as a mastermind can now choose to think in any language he speaks. Unless the opponent reading a mastermind's thoughts speaks all of the mastermind's languages, attempts at thought reading automatically fail. This ability replaces the investigator talent gained at 9th level."), benefit: None },
            ],
        },
        // Investigator Archetype ~ Sleuth -- acg_abilities_class.lst:3308
        ArchetypeSwapEntry {
            key: "Investigator Archetype ~ Sleuth",
            subject: "Investigator",
            archetype_name: "Sleuth",
            description: Some("A sleuth is an investigator who relies on good fortune and guile rather than alchemy. Having no mystical energy intrinsic to her, she must forgo the more magical aspects of alchemy to solve her mysteries with wits, gumption, and the fickle consideration of luck."),
            source_page: Some("p.101"),
            prerequisites: Some(&["PRECLASS:1,Investigator=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Investigator Archetype ~ Sleuth],[!PREABILITY:1,CATEGORY=Archetype,TYPE.InvestigatorAlchemy,TYPE.InvestigatorSwiftAlchemy]"]),
            replaces: Some(&["InvestigatorAlchemy", "InvestigatorSwiftAlchemy"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Sleuth ~ Sleuth's Luck", at_level: 1, description: Some("A sleuth gains a fluctuating pool of luck, measuring her ability to get out of scrapes. At the start of each day, a sleuth has a number of luck points equal to her Charisma modifier (minimum 1). Her luck goes up or down throughout the day, but usually cannot go higher than her Charisma modifier (minimum 1), though feats, magic items, and spells that grant either grit or panache points can also grant a sleuth luck points equal to the amount of grit or panache they grant. A sleuth spends luck to accomplish deeds (see below), and regains luck in the following ways. Rolling a Natural 20 on a Knowledge or Sense Motive Check: While on an investigation, a sleuth regains luck by uncovering secrets. Rolling a natural 20 on these skill checks while actively investigating causes the sleuth to regain 1 luck point. Such skill checks made for more mundane reasons (such as normal research or using Sense Motive to gain information during normal bargaining or while gambling) do not regain luck for the sleuth. Determining which skill checks qualify is up to the GM. Rolling a 6 or Higher on an Inspiration Roll: When a sleuth rolls an inspiration die while on an investigation and the die comes up a 6 or higher, she regains 1 luck point. (If she rolls multiple inspiration dice at a time, she regains 1 luck point if the total is 6 or higher.) Like regaining luck via a natural 20 on a qualifying skill check, inspiration checks made for more mundane reasons don't qualify for regaining luck. The GM is the final arbitrator for what rolls qualify. It's possible for a investigator to regain 2 luck points on the same Skill check, if both the qualifying skill check is a natural 20 and any inspiration roll made is a 6 or higher. This and the 1st-level deeds below replace alchemy. A sleuth cannot take the alchemist discovery talent or any investigator talent that affects alchemy."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sleuth ~ Deeds", at_level: 1, description: Some("Sleuths spend luck points to accomplish deeds. Most deeds grant a sleuth some momentary bonus or effect, but there are some that provide longer-lasting effects. Some deeds stay in effect as long as a sleuth has at least 1 luck point. At 1st level, a sleuth gains the following deeds."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deed ~ Daring", at_level: 1, description: Some("At 1st level, a sleuth can spend 1 luck point when he makes an Acrobatics, Climb, Escape Artist, Fly, Ride, or Swim check to roll d6 and add the result to the check. He can choose to add this die after he rolls. If the d6 roll is a natural 6, he rolls another d6 and adds it to the check. He can continue to do this as long as he rolls natural 6s, up to a number of times equal to his Intelligence modifier (minimum 1)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deed ~ Opportunistic Evasion", at_level: 1, description: Some("At 1st level, when a sleuth succeeds at a Reflex saving throw against an effect that still deals damage on a successful save, he can spend 1 luck point as an immediate action to instead take no damage for that effect."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deed ~ Sleuth's Initiative", at_level: 1, description: Some("At 1st level, as long as the sleuth has at least 1 luck point, he gains a +2 bonus on initiative checks. Furthermore, if he has the Quick Draw feat, his hands are free and unrestrained, and the weapon is not hidden, he can draw a single light or one-handed melee weapon as part of the initiative check. (Unlike with swashbuckler deeds, this does not have to be a piercing weapon.)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deed ~ Make It Count", at_level: 4, description: Some("When a sleuth uses studied strike, he can spend 1 luck point to apply an investigator talent that he doesn't already know and which affects studied strike to his studied strike. He must be able to fulfill that talent's prerequisites."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deed ~ Run Like Hell", at_level: 4, description: Some("At 4th level, a sleuth can spend 1 luck point to gain a +20-foot bonus to his speed for 1 minute. Furthermore, while under the effect of this bonus, if he moves more than his normal speed in a round, he gains a +4 bonus to AC until the start of his next turn."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sleuth ~ Second Chance", at_level: 4, description: Some("At 4th level, when a sleuth rolls an inspiration die or uses daring deed, he can spend 1 luck point to reroll either the inspiration or the daring deed die. If he rolls a 6 or higher on this reroll, he does not regain a luck point, and no matter what he rolls on either roll, he must keep the reroll result, even if it is lower."), benefit: None },
            ],
        },
        // Investigator Archetype ~ Spiritualist -- acg_abilities_class.lst:3309
        ArchetypeSwapEntry {
            key: "Investigator Archetype ~ Spiritualist",
            subject: "Investigator",
            archetype_name: "Spiritualist",
            description: Some("While most investigators look to the physical world to gain their knowledge, there are those who seek out knowledge beyond the pale. Those who think that the dead tell no tales are quickly proven wrong by the spiritualist. Instead of toying with chemicals and reagents to find clues, he talks directly to the spirit world to uncover the ways and means of skullduggery and the desperate acts committed in the heat of dark passions."),
            source_page: Some("p.102"),
            prerequisites: Some(&["PRECLASS:1,Investigator=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Investigator Archetype ~ Spiritualist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.InvestigatorAlchemy,TYPE.InvestigatorPoisonLore,TYPE.InvestigatorPoisonResistance,TYPE.InvestigatorTrapSense,TYPE.InvestigatorSwiftAlchemy,TYPE.InvestigatorPoisonImmunity]"]),
            replaces: Some(&["InvestigatorAlchemy", "InvestigatorPoisonLore", "InvestigatorPoisonResistance", "InvestigatorTrapSense", "InvestigatorSwiftAlchemy", "InvestigatorPoisonImmunity"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Spiritualist ~ Commune with Spirits", at_level: 1, description: Some("Instead of relying on alchemical research to find clues, a spiritualist relies on communion with the world beyond death. A spiritualist gains the ability to use comprehend languages, detect secret doors,"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spiritualist ~ Spirit Sense", at_level: 2, description: Some("When a spiritualist attempts a saving throw against an ability or an effect delivered by an incorporeal creature, he can expend one use of his inspiration instead of two to augment that saving throw. If the spiritualist has the combat inspiration investigator talent, he need not spend any inspiration to augment such saving throws."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spiritualist ~ Strong Life", at_level: 2, description: Some("At 2nd level, his dealings with the other side and knowledge of what lies beyond strengthens the spiritualist's will. He gains a +%1 bonus on saving throws against death effects and negative energy damage.|StrongLifeBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spiritualist ~ Sixth Sense", at_level: 3, description: Some("A spiritualist can spend one of his uses of the commune with spirits ability to reroll a single saving throw that he has failed. He must take the results of the reroll, even if it is lower."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spiritualist ~ Whispering Spirits", at_level: 4, description: Some("The spiritualist can open a conduit with the spirit world that can aid him in combat for a short time. He can expend one use of his commune with spirits ability in order to gain a +%1 insight bonus to both AC and saving throws for 1 minute.|max(1,WIS)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spiritualist ~ Touched by the Beyond", at_level: 11, description: Some("The spiritualist's ability to touch the beyond grants him further protection against the dangers of death and negative energy. The spiritualist becomes immune to death effects, and he takes half damage from negative energy."), benefit: None },
            ],
        },
        // Investigator Archetype ~ Steel Hound -- acg_abilities_class.lst:3310
        ArchetypeSwapEntry {
            key: "Investigator Archetype ~ Steel Hound",
            subject: "Investigator",
            archetype_name: "Steel Hound",
            description: Some("Black powder and firearms are a natural extension of the alchemical experimentation that investigators use on a regular basis. Steel hounds are investigators who have taken to using firearms in place of the more mundane weapons their counterparts favor."),
            source_page: Some("p.103"),
            prerequisites: Some(&["PRECLASS:1,Investigator=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Investigator Archetype ~ Steel Hound],[!PREABILITY:1,CATEGORY=Archetype,TYPE.InvestigatorWeaponProficiencies,TYPE.InvestigatorArmorProficiencies,TYPE.InvestigatorPoisonLore,TYPE.InvestigatorSwiftAlchemy]"]),
            replaces: Some(&["InvestigatorWeaponProficiencies", "InvestigatorArmorProficiencies", "InvestigatorPoisonLore", "InvestigatorSwiftAlchemy"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Steel Hound ~ Weapon and Armor Proficiency", at_level: 1, description: Some("Steel hounds are proficient with simple weapons, plus the rapier, the sap, and one type of firearm selected at 1st level. They are proficient in light armor, but not shields. This replaces the investigator's weapon and armor proficiencies."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Steel Hound ~ Packing Heat", at_level: 2, description: Some("At 2nd level, the steel hound gains both the Amateur Gunslinger and Gunsmithing feats as bonus feats. He also gains a battered gun identical to the one gained by the gunslinger. This ability replaces poison lore."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Steel Hound ~ Investigator Talents", at_level: 2, description: Some("The steel hound can select the Extra Grit or Rapid Reload feats in place of an investigator talent."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Steel Hound ~ Shot in the Dark", at_level: 4, description: Some("At 4th level, a steel hound gains the following deed. This deed works and interacts with grit the same way as gunslinger deeds, but only the steel hound can use it. If the steel hound also has levels in gunslinger, he can spend grit points from that class to use this deed."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deed ~ Blind Shot", at_level: 4, description: Some("A steel hound can spend 1 grit point to ignore all miss chances due to concealment when making firearm attacks. This effect lasts until the end of his turn. This ability allows the steel hound to ignore concealment, but does not reveal or allow him to see the enemy. This ability replaces swift alchemy."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Steel Hound ~ Talented Shot", at_level: 11, description: Some("At 11th level, a steel hound can select a gunslinger deed in place of an investigator talent. He can select from any deed available to a gunslinger of his investigator level - 4."), benefit: None },
            ],
        },
        // Monk Archetype ~ Kata Master -- acg_abilities_class.lst:3436
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Kata Master",
            subject: "Monk",
            archetype_name: "Kata Master",
            description: Some("The kata master takes the visual aspect of his martial art to its logical extreme, harnessing her flowing movements and skilled maneuvers as a psychological weapon against her enemies. A kata master forsakes the mental discipline of her more contemplative brethren in favor of these flamboyant exhibitions. She often performs in staged fights and tournaments, utilizing stylized forms to amaze the audience and shock and dismay her opponents."),
            source_page: Some("p.105"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Kata Master],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkStunningFist,TYPE.MonkStillMind,TYPE.MonkWholenessOfBody,TYPE.MonkQuiveringPalm,TYPE.MonkKiPool]"]),
            replaces: Some(&["MonkStunningFist", "MonkStillMind", "MonkWholenessOfBody", "MonkQuiveringPalm"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Kata Master ~ Panache", at_level: 1, description: Some("A kata master gains the swashbuckler's panache class ability. At the start of each day, a kata master gains %1 panache points. Her panache goes up or down throughout the day, but usually cannot go higher than %1. A kata master gains the swashbuckler's derring-do and dodging panache deeds. A kata master can use an unarmed strike or monk special weapon in place of a light or one-handed piercing melee weapon for granted swashbuckler class features and deeds.|max(1,CHA)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Kata Master ~ Menacing Swordplay", at_level: 3, description: Some("A kata master gains the swashbuckler's menacing swordplay deed."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Kata Master ~ Ki Pool", at_level: 4, description: Some("A kata master can treat ki points as panache points for any swashbuckler deed gained through this archetype. This ability modifies ki pool."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Kata Master ~ Targeted Strike", at_level: 7, description: Some("A kata master gains the swashbuckler's targeted strike deed."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Kata Master ~ Dizzying Defense", at_level: 15, description: Some("A kata master gains the dizzying defense swashbuckler deed."), benefit: None },
            ],
        },
        // Monk Archetype ~ Wildcat -- acg_abilities_class.lst:3437
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Wildcat",
            subject: "Monk",
            archetype_name: "Wildcat",
            description: Some("A wildcat is a student of the school of hard knocks, who dedicates himself to learning how to take down foes by any means necessary. A wildcat isn't afraid to smash a tankard over a foe's head, stomp an opponent's foot, gouge an eye, or generally create mayhem to gain any possible advantage."),
            source_page: Some("p.105"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Wildcat],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkStillMind,TYPE.MonkKiPool,TYPE.MonkHighJump,TYPE.MonkSlowFall,TYPE.MonkImprovedEvasion,TYPE.MonkAbundantStep,TYPE.MonkDiamondSoul,TYPE.MonkEmptyBody]"]),
            replaces: Some(&["MonkStillMind", "MonkKiPool", "MonkHighJump", "MonkSlowFall", "MonkImprovedEvasion", "MonkAbundantStep", "MonkDiamondSoul", "MonkEmptyBody"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Wildcat ~ Ready for Anything", at_level: 3, description: Some("A wildcat gains a +2 bonus on initiative checks and Perception checks to act in a surprise round."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wildcat ~ Brawler Maneuver Training", at_level: 4, description: Some("A wildcat gains additional training with the dirty trick combat maneuver (APG p.320) and other combat maneuvers as he gains levels."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wildcat ~ Improvised Weapon Mastery", at_level: 4, description: Some("Starting at 4th level, a wildcat's damage with improvised weapons increases. When wielding an improvised weapon, he uses the unarmed strike damage of a monk four levels lower instead of the base damage for that weapon (minimum monk level 1). For example, a 6th-level Medium wildcat wielding a broken bottle deals 1d6 points of damage instead of the weapon's normal 1d4. If the weapon normally deals more damage than this, its damage is unchanged. This increase in damage does not affect any other aspect of the weapon. The wildcat can decide to use the weapon's base damage instead of his adjusted unarmed strike damage. This must be declared before the attack roll is attempted. This ability replaces all instances of slow fall."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wildcat ~ Bonus Feat", at_level: 6, description: Some("A wildcat adds the following feats to his list of bonus feats at 6th level: Improved Dirty Trick, Improved Reposition, Improved Steal. He adds the following feats to his list of bonus feats at 10th level: Quick Dirty Trick, Quick Reposition, Quick Steal."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wildcat ~ Knockout", at_level: 9, description: Some("A wildcat can unleash a devastating attack that can instantly knock a target unconscious. He must announce this intent before making his attack roll. If the wildcat's strike is successful and the target takes damage from the blow, the target must succeed at a Fortitude saving throw (DC %1) or fall unconscious for 1d6 rounds. Each round on its turn, the unconscious target can attempt a new saving throw to end the effect; this is a fullround action that does not provoke attacks of opportunity. Creatures immune to critical hits or nonlethal damage are immune to this ability.|10+floor(MonkLVL/2)+max(STR,DEX)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wildcat ~ Turn the Tables", at_level: 13, description: Some("Opponents provoke an attack of opportunity from the wildcat whenever they fail at a combat maneuver against the wildcat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wildcat ~ Dirty Blow", at_level: 19, description: Some("When a wildcat succeeds at a dirty trick combat maneuver, he can deal his unarmed strike damage to that opponent."), benefit: None },
            ],
        },
        // Oracle Archetype ~ Psychic Searcher -- acg_abilities_class.lst:3493
        ArchetypeSwapEntry {
            key: "Oracle Archetype ~ Psychic Searcher",
            subject: "Oracle",
            archetype_name: "Psychic Searcher",
            description: Some("A psychic searcher is devoted to revealing the hidden within the world around her by sensing and communing with residual mental energy, haunts, and fragments of living spirits that can dwell in objects or rooms."),
            source_page: Some("p.106"),
            prerequisites: Some(&["PRECLASS:1,Oracle=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Oracle Archetype ~ Psychic Searcher],[!PREABILITY:1,CATEGORY=Archetype,TYPE.OracleMysterySpell2,TYPE.OracleMysterySpell4,TYPE.OracleMysterySpell6,TYPE.OracleMysterySpell8,TYPE.OracleMysterySpell12,TYPE.OracleMysterySpell16,TYPE.OracleMysterySpell18,TYPE.OracleRevelation3]"]),
            replaces: Some(&["OracleMysterySpell2", "OracleMysterySpell4", "OracleMysterySpell6", "OracleMysterySpell8", "OracleMysterySpell12", "OracleMysterySpell16", "OracleMysterySpell18", "OracleRevelation3"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Psychic Searcher ~ Bonus Spells", at_level: 1, description: Some("Several of the oracle's normal mystery bonus spells are replaced."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Psychic Searcher ~ Inspiration", at_level: 2, description: Some("A psychic searcher gains an inspiration pool, as the investigator class ability. A psychic searcher uses her oracle level as her investigator level to determine the effects of this ability. A psychic searcher has the ability to augment skill checks and ability checks through her brilliant inspiration.  The psychic searcher has an inspiration pool equal to %1.  A psychic searcher's inspiration pool refreshes each day,  usually when she refreshes her spells. As a free action, she can expend one use of inspiration from her pool to add %2d%3 to the result of that check, including any on which she takes 10 or 20. This choice is made after the check is rolled and before the results are revealed. A psychic searcher can only use inspiration once per check or roll.  She gains free uses of inspiration on Diplomacy, Knowledge (arcana, history, local, nobility, planes, religion), and Sense Motive skill checks without spending a use of inspiration, provided she's trained in that skill. A psychic searcher's inspiration pool is based on her Charisma modifier, not Intelligence modifier.|PsychicSearcherInspirationPoolBonus|PsychicSearcherInspirationDice|PsychicSearcherInspirationDieSize"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Psychic Searcher ~ Psychic Talent", at_level: 3, description: Some("[NOT IMPLEMENTED] A psychic searcher's mastery of her supernatural insight grows, granting her a new investigator talent from the following list: amazing inspiration, eidetic recollection, empathy, inspired alertness, item lore, perceptive tracking (except using Sense Motive instead of Perception or Survival), rogue talent (only for hard to fool), and tenacious inspiration. Whenever a psychic searcher can select a new revelation, she can instead select an investigator or rogue talent from the above list."), benefit: None },
            ],
        },
        // Oracle Archetype ~ Spirit Guide -- acg_abilities_class.lst:3494
        ArchetypeSwapEntry {
            key: "Oracle Archetype ~ Spirit Guide",
            subject: "Oracle",
            archetype_name: "Spirit Guide",
            description: Some("Through her exploration of the universe's mysteries, a spirit guide opens connections to the spirit world and forms bonds with the entities that inhabit it."),
            source_page: Some("p.106"),
            prerequisites: Some(&["PRECLASS:1,Oracle=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Oracle Archetype ~ Spirit Guide],[!PREABILITY:1,CATEGORY=Archetype,TYPE.OracleMysteryClassSkills,TYPE.OracleRevelation3,TYPE.OracleRevelation7,TYPE.OracleRevelation15]"]),
            replaces: Some(&["OracleMysteryClassSkills", "OracleRevelation3", "OracleRevelation7", "OracleRevelation15"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Spirit Guide ~ Class Skills", at_level: 1, description: Some("A spirit guide gains all Knowledge skills as class skills. This replaces the bonus class skills gained from the oracle's mystery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spirit Guide ~ Bonded Spirit", at_level: 3, description: Some("[NOT IMPLEMENTED] A spirit guide can form a temporary bond with a spirit, as the shaman's wandering spirit class feature (see page 37). She must make this selection each day when she refreshes her spells. A spirit guide cannot bond with a spirit that is incompatible with her alignment, ethos, or mystery (GM's discretion). A spirit guide gains one hex of her choice from the list of hexes available from that spirit. She uses her oracle level as her shaman level, and she switches Wisdom for Charisma and vice versa for the purpose of determining the hex's effects. At 4th level, she adds the bonded spirit's spirit magic spells to her oracle spells known for that day, but only of spell levels she can cast. At 7th level, she gains the spirit ability of her current bonded spirit. At 15th level, she gains the greater spirit ability of her current bonded spirit. This ability replaces the revelations gained at 3rd, 7th, and 15th levels."), benefit: None },
            ],
        },
        // Oracle Archetype ~ Warsighted -- acg_abilities_class.lst:3495
        ArchetypeSwapEntry {
            key: "Oracle Archetype ~ Warsighted",
            subject: "Oracle",
            archetype_name: "Warsighted",
            description: Some("A warsighted's unique gifts are not in strange magical revelations, but in her ability to adapt in the midst of a battle with new fighting techniques. The warsighted is a master of combat, as dedicated as a fighter and as flexible as a brawler."),
            source_page: Some("p.106"),
            prerequisites: Some(&["PRECLASS:1,Oracle=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Oracle Archetype ~ Warsighted],[!PREABILITY:1,CATEGORY=Archetype,TYPE.OracleRevelation1,TYPE.OracleRevelation7,TYPE.OracleRevelation11,TYPE.OracleRevelation15]"]),
            replaces: Some(&["OracleRevelation1", "OracleRevelation7", "OracleRevelation11", "OracleRevelation15"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Warsighted ~ Martial Flexibility", at_level: 1, description: Some("The warsighted can use a move action to gain the benefit of a combat feat he doesn't possess. The warsighted must otherwise meet all the feat's prerequisites. |PREVARLT:OracleLVL,7"), benefit: None },
            ],
        },
        // Paladin Archetype ~ Holy Guide -- acg_abilities_class.lst:3513
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Holy Guide",
            subject: "Paladin",
            archetype_name: "Holy Guide",
            description: Some("A holy guide believes that it's his sacred calling to clear the roads of bandits between towns as well as to escort travelers to safety. He must enforce the rule of law in the wilderness and help those that cannot defend themselves against the many dangers of the area."),
            source_page: Some("p.107"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ Holy Guide],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinMercy3,TYPE.PaladinMercy5]"]),
            replaces: Some(&["PaladinMercy3", "PaladinMercy6"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Holy Guide ~ Class Skills", at_level: 1, description: Some("A holy guide gains Knowledge (geography) and Survival as class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Holy Guide ~ Favored Terrain", at_level: 3, description: Some("A holy guide chooses a favored terrain from the ranger favored terrains table. This otherwise functions like the ranger ability of the same name. This ability replaces the mercy gained at 3rd level. Every time a holy guide would be able to select another mercy, he can instead select another favored terrain and increase his bonuses for one existing favored terrain, just like a ranger."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Holy Guide ~ Teamwork Feat", at_level: 6, description: Some("A holy guide gains a teamwork feat as a bonus feat. He must meet the prerequisites for this feat. As a standard action, He can expend one use of smite evil to grant this feat to all allies within 30 feet who can see and hear him. Allies retain the use of this bonus feat for %1 rounds. Allies do not need to meet the prerequisites of this bonus feat. Evil creatures do not gain the benefit of this teamwork feat, even if the paladin considers them allies.|3+floor(PaladinLVL/2)"), benefit: None },
            ],
        },
        // Paladin Archetype ~ Temple Champion -- acg_abilities_class.lst:3514
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Temple Champion",
            subject: "Paladin",
            archetype_name: "Temple Champion",
            description: Some("A temple champion is a powerful warrior dedicated to a good or lawful deity. She thinks of herself primarily as a servant of her deity and secondarily as an agent of her deity's church. She has a refined understanding of a specific aspect of that faith and gives up standard paladin spellcasting in favor of a warpriest's domainbased blessings and granted powers."),
            source_page: Some("p.107"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ Temple Champion],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinSpells,TYPE.PaladinDivineBond,TYPE.PaladinAuraOfJustice]"]),
            replaces: Some(&["PaladinSpells", "PaladinDivineBond", "PaladinAuraOfJustice"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Temple Champion ~ Spells", at_level: 1, description: Some("A temple champion does not gain access to paladin spells, and does not have a paladin caster level or spell list. This is not considered a spellcasting class."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Temple Champion ~ Domain Granted Power", at_level: 4, description: Some("At 4th level, a temple champion selects one domain granted by her deity (or a domain suitable for her ethos or goals, subject to GM approval). The temple champion gains the 1st-level granted power of that domain and uses her paladin level as her cleric level for determining the effects of that granted power. Any Wisdom-based aspects of that granted power instead use the temple champion's Charisma. The temple champion does not gain access to that domain's spell list."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Temple Champion ~ Blessing", at_level: 5, description: Some("At 5th level, a temple champion gains the minor blessing (as the warpriest class feature) of the domain she selected at 4st level. She uses her paladin level as her warpriest level for determining the effects of that blessing. Any Wisdom-based aspects of that blessing instead use the temple champion's Charisma. At 11th level, she gains the major blessing of her chosen domain. This ability replaces divine bond and aura of justice."), benefit: None },
            ],
        },
        // Ranger Archetype ~ Divine Tracker -- acg_abilities_class.lst:3533
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Divine Tracker",
            subject: "Ranger",
            archetype_name: "Divine Tracker",
            description: Some("Blessed by his deity, a divine tracker hunts down those he deems deserving of his retribution. His weapon is likely to find purchase in his favored enemy."),
            source_page: Some("p.108"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ Divine Tracker],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RangerWildEmpathy,TYPE.RangerHuntersBond]"]),
            replaces: Some(&["RangerWildEmpathy", "RangerHuntersBond"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Divine Tracker ~ Alignment", at_level: 1, description: Some("A divine tracker's alignment must be within one step of his deity's, along either the law/chaos axis or the good/evil axis."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Divine Tracker ~ Favored Weapon", at_level: 1, description: Some("A divine tracker becomes proficient with the favored weapon of his deity. If his deity's favored weapon is unarmed strike, he instead gains Improved Unarmed Strike as a bonus feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Divine Tracker ~ Blessings", at_level: 4, description: Some("A divine tracker forms a close bond with his deity's ethos. He selects two warpriest domains from among the domains granted by his deity, and gains the minor blessings of those domains. A divine tracker can select an alignment domain (Chaos, Evil, Good, or Law) only if his alignment matches that domain. If a divine tracker isn't devoted to a particular deity, he still selects two blessings to represent his spiritual inclinations and abilities, subject to GM approval. The restriction on alignment domains still applies. A divine tracker uses his ranger level as his warpriest level to determine the effect of the blessing. At 13th level, a divine tracker gains the major blessing from both of his domains. This ability replaces hunter's bond."), benefit: None },
            ],
        },
        // Ranger Archetype ~ Hooded Champion -- acg_abilities_class.lst:3534
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Hooded Champion",
            subject: "Ranger",
            archetype_name: "Hooded Champion",
            description: Some("The hooded champion lives on the periphery of civilized lands, and is often at odds with the forces of law and order. He is frequently a hero of oppressed peoples, lurking in the woods near their homes and trying to right the injustices inflicted upon them by the wealthy and powerful."),
            source_page: Some("p.108"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ Hooded Champion],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FavoredEnemy1,TYPE.RangerWildEmpathy,TYPE.RangerEndurance,TYPE.RangerEvasion,TYPE.RangerImprovedEvasion]"]),
            replaces: Some(&["FavoredEnemy1", "RangerWildEmpathy", "RangerEndurance", "RangerEvasion", "RangerImprovedEvasion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Hooded Champion ~ Panache", at_level: 1, description: Some("The hooded champion gains the swashbuckler's panache class feature. His panache goes up or down throughout the day, but usually cannot go higher than %1. He regains panache through critical hits and killing blows when using a bow of any kind, rather than when using a light or one-handed piercing melee weapon. If the hooded champion has this ability and the panache ability from another class or archetype, the panache points from the two sources do not stack, but the hooded champion regains panache in any way either class feature allows him to.At the start of each day, a hooded champion gains %1 panache points.|max(1,CHA)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Hooded Champion ~ Deeds", at_level: 1, description: Some("The hooded champion gains the swashbuckler's derring-do and dodging panache deeds, as well as the following deeds. For all deeds, treat the hooded champion's ranger level as his swashbuckler level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Hooded Champion ~ Dead Aim", at_level: 1, description: Some("The hooded champion can spend 1 panache point when making a single ranged attack (not a full attack) with a bow to make a ranged touch attack instead. The target must be in the bow's first range increment."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Hooded Champion ~ Hooded Champion's Initiative", at_level: 3, description: Some("While the hooded champion has at least 1 panache point, he gains a +2 bonus on initiative checks. In addition, if he has the Quick Draw feat, his hands are free and unrestrained, and his weapon isn't hidden, he can draw a single bow as part of the initiative check."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Hooded Champion ~ Combat Style", at_level: 2, description: Some("The hooded champion must select the archery combat style."), benefit: None },
            ],
        },
        // Ranger Archetype ~ Wild Hunter -- acg_abilities_class.lst:3535
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Wild Hunter",
            subject: "Ranger",
            archetype_name: "Wild Hunter",
            description: Some("A wild hunter seeks to emulate the animals around him to keep him safe while he tracks his prey. Instead of studying the traits and behaviors of a favored enemy, a wild hunter studies those of various animals, incorporating those attributes into his hunting strategy."),
            source_page: Some("p.108"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ Wild Hunter],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RangerFavoredEnemy,TYPE.RangerWoodlandStride,TYPE.RangerSwiftTracker]"]),
            replaces: Some(&["RangerFavoredEnemy", "RangerWoodlandStride", "RangerSwiftTracker"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Wild Hunter ~ Animal Focus", at_level: 1, description: Some("As a swift action, a wild hunter can take on the aspect of an animal, gaining a bonus or special ability based on the type of animal emulated. This functions as the hunter's animal focus class feature (see page 27), though this only applies to the wild hunter and not an animal companion. The wild hunter can use this ability for %1 minutes per day. This duration does not need to be consecutive, but it must be spent in 1-minute increments. He can only emulate one animal at a time.|HunterAnimalFocusMinutes"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wild Hunter ~ Shared Focus", at_level: 7, description: Some("[NOT IMPLEMENTED] A wild hunter can share his current animal focus with one creature appropriate to his hunter's bond. If the wild hunter's bond is with an animal companion, the companion automatically gains the benefits of the wild hunter's current animal focus. If the wild hunter's bond is with his companions, as a swift action he can select one bonded ally to gain the benefits of the hunter's current animal focus; this lasts until the hunter's bond ends, the animal focus ends for the wild hunter, or the wild hunter selects a different companion."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Counterfeit Mage -- acg_abilities_class.lst:3558
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Counterfeit Mage",
            subject: "Rogue",
            archetype_name: "Counterfeit Mage",
            description: Some("Charlatans and stage magicians use slight of hand to fake magic. A counterfeit mage goes a step further, parroting the motions and activation phrases used by arcane casters to activate wands or other magical accoutrements. While counterfeit mages rarely fool a real wizard, their command of the arcane is enough to convince most lay people."),
            source_page: Some("p.109"),
            prerequisites: Some(&["PREMULT:1,[PRECLASS:1,Rogue=1],[PREFACT:1,ABILITIES,ActAsClass_Rogue=true]", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Counterfeit Mage],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RogueTrapfinding,TYPE.RogueTalent4]"]),
            replaces: Some(&["RogueTrapfinding", "RogueTalent4"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Counterfeit Mage ~ Magical Expertise", at_level: 1, description: Some("A counterfeit mage adds +%1 his level to Disable Device checks to disarm magical traps, Perception checks to find magical traps, and Use Magic Device checks to activate scrolls and wands. A counterfeit mage can use Disable Device to disarm magic traps.|floor(RogueLVL/2)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Counterfeit Mage ~ Signature Wand", at_level: 1, description: Some("A counterfeit mage can spend 1 hour practicing with a wand to designate it as his signature wand. He can draw that wand as a free action, and can activate it without having to succeed at a Use Magic Device check. He can change his signature wand once per day."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Counterfeit Mage ~ Wand Adept", at_level: 1, description: Some("A counterfeit mage can use his Dexterity modifier in place of his Charisma modifier when attempting Use Magic Device checks to activate wands."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Underground Chemist -- acg_abilities_class.lst:3559
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Underground Chemist",
            subject: "Rogue",
            archetype_name: "Underground Chemist",
            description: Some("Underground chemists are part of the rotting, fetid underbelly of the alchemical world. While underground chemists can't hold a candle to dedicated alchemists, they're tricky and dangerous with alchemical substances and potions."),
            source_page: Some("p.109"),
            prerequisites: Some(&["PREMULT:1,[PRECLASS:1,Rogue=1],[PREFACT:1,ABILITIES,ActAsClass_Rogue=true]", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Underground Chemist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RogueEvasion,TYPE.RogueTalent4,TYPE.RogueAdvancedTalents]"]),
            replaces: Some(&["RogueEvasion", "RogueTalent4", "RogueAdvancedTalents"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Underground Chemist ~ Chemical Weapons", at_level: 1, description: Some("An underground chemist is able to retrieve an alchemical item as if drawing a weapon. She adds her Intelligence modifier to damage dealt with splash weapons, including any splash damage. She adds +%1 to Craft (alchemy) checks.|floor(RogueLVL/2)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Underground Chemist ~ Precise Splash Weapons", at_level: 1, description: Some("An underground chemist can deal sneak attack damage with splash weapons. The attack must be her first attack that round, qualify for dealing sneak attack damage (such as against a flat-footed target), and be directed at a creature rather than a square."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Underground Chemist ~ Discovery", at_level: 1, description: Some("An underground chemist can select one of the following alchemist discoveries (APG p.28) in place of a rogue talent: concentrate poison, dilution, enhance potion, extend potion, mummification, nauseating flesh, poison conversion, preserve organs, spontaneous healing, or sticky poison. She uses her rogue level as her alchemist level for determining the effects of her discoveries and whether she is able to select one."), benefit: None },
            ],
        },
        // Shaman Archetype ~ Animist -- acg_abilities_class.lst:3586
        ArchetypeSwapEntry {
            key: "Shaman Archetype ~ Animist",
            subject: "Shaman",
            archetype_name: "Animist",
            description: Some("Even among mystical practitioners, the animist has a strange perspective and even stranger magic. The animist perceives that all things have a spirit, including objects, constructs, illnesses, buildings, and the environment."),
            source_page: Some("p.110"),
            prerequisites: Some(&["PRECLASS:1,Shaman=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Shaman Archetype ~ Animist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ShamanHex18,TYPE.ShamanManifestation,TYPE.ShamanSpirit,TYPE.ShamanHex8,TYPE.ShamanHex10,TYPE.ShamanHex12,TYPE.ShamanHex2]"]),
            replaces: Some(&["ShamanHex18", "ShamanManifestation", "ShamanSpiritMagic", "ShamanHex8", "ShamanHex10", "ShamanHex12", "ShamanHex2"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Animist ~ Animist Spirit Magic", at_level: 1, description: Some("[NOT IMPLEMENTED] The animist adds the following spells to the list of spells he can cast using spirit magic: speak with animals (1st), skinsend (2nd), speak with plants (3rd), malfunction (4th), dream (5th), speak with stone (6th), control construct (7th), trap the soul (8th), soul bind (9th). This ability replaces the spirit magic spells gained from the shaman's spirit."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Animist ~ Wrangle Condition", at_level: 2, description: Some("The animist interacts directly with unwholesome spirits of the ills that trouble him or his people. The animist can attempt to persuade the spirit to leave its victim alone, thus performing a miraculous healing.  The animist can attempt a Diplomacy check to persuade the condition's spirit to leave. Each category of condition has an accompanying Diplomacy DC (see below). The animist cannot take 10 or 20 on the check or receive aid from any creature except another shaman. Failure by 5 or less means the condition spirit is willing to leave the target and transfer to the animist for the rest of the duration or until cured; this happens only if the animist is willing. Failure by 10 or more means that not only is the target inflicted with the condition, but the animist is as well (whether or not he is willing) for the remaining duration or until cured. Minor Conditions (DC 15): Fatigued, shaken, and sickened. Major Conditions (DC 20): Dazed and staggered. Severe Conditions (DC 25): Exhausted, frightened, and nauseated. Dire Conditions (DC 30): Blinded, deafened, paralyzed, and stunned. The animist can use this ability %1 times per day. Using this ability is a standard action that requires no contact or shared language with the target, but it relies on audible components and the target must be within 30 feet. If a target is afflicted with multiple conditions, the animist targets one condition at a time (of her choosing). The animist cannot use this ability on himself. At 5th level, the animist can use this ability to exorcise minor conditions without needing to attempt a Diplomacy check. He can dispatch major conditions without a roll at 9th level, severe conditions at 13th level, and dire conditions at 17th level. Doing so still requires one use of this ability per condition.|floor(ShamanLVL/2)+WIS"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Animist ~ Exorcism", at_level: 8, description: Some("The animist can attempt to end effects that control a creature or object, such as magic jar, possess object, dominate person, and dominate monster, or possessing entities such as ghosts. This is a full-round action that requires the animist to touch the target. The possessing or dominating creature must attempt a Will save with a DC of %1. Failure means that the controlling effect ends or the entity is immediately cast out of the target, as appropriate. If the possessing entity is exorcised, it cannot attempt to dominate or possess that target again for 24 hours. The animist can exorcise the same entity out of different targets, but once the entity successfully saves against the animist's exorcism, it cannot be affected by this ability again for 24 hours.|10+floor(ShamanLVL/2)+WIS"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Animist ~ Dominate Spirit", at_level: 10, description: Some("The animist can attempt to possess creatures or objects as magic jar or possess object. When the animist is attempting to possess the creature, his familiar acts as the jar and must be within 10 feet when the animist attempts to possess a creature. The animist's soul can perceive his surroundings through the familiar's senses, clearly evaluate potential targets for possession, and communicate with his familiar telepathically. The familiar retains its autonomy while acting as the jar, and is not harmed by being used as a vessel for the animist's soul. The animist can use this ability %1 times per day.|floor((ShamanLVL-6)/4)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Animist ~ Contact with the Spirit World", at_level: 12, description: Some("The animist can interact with incorporeal entities as if his unarmed strikes and melee weapons had the ghost touch weapon special ability. While using this ability, he can also see nearby ethereal entities and likewise strike at them as if they were incorporeal. He can use this ability for %1 rounds per day. The rounds need not be consecutive.|ShamanLVL"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Animist ~ Etherealness", at_level: 18, description: Some("The animist can cast etherealness once per day as a spell-like ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Animist ~ Spirit Shaman", at_level: 20, description: Some("The animist can use ethereal jaunt as at will as a spell-like ability, and cast astral projection once per day as a spell-like ability."), benefit: None },
            ],
        },
        // Shaman Archetype ~ Possessed Shaman -- acg_abilities_class.lst:3587
        ArchetypeSwapEntry {
            key: "Shaman Archetype ~ Possessed Shaman",
            subject: "Shaman",
            archetype_name: "Possessed Shaman",
            description: Some("For a possessed shaman, merely communing with the spirit world is insufficient. Instead, she invites the spirits to share her body, granting them the chance to experience corporeal existence. In return, they grant her their skills and protect her from otherworldly influence."),
            source_page: Some("p.111"),
            prerequisites: Some(&["PRECLASS:1,Shaman=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Shaman Archetype ~ Possessed Shaman],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ShamanSpiritMagic,TYPE.ShamanHex2,TYPE.ShamanWanderingHex6]"]),
            replaces: Some(&["ShamanSpiritMagic", "ShamanHex2", "ShamanWanderingHex6"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Possessed Shaman ~ Shared Skill", at_level: 1, description: Some("[NOT IMPLEMENTED] At 1st level, a possessed shaman selects two skills. Both of these skills must use the same ability score. The possessed shaman treats these skills as if she had a number of ranks in them equal to her shaman level, and uses her Wisdom modifier in place of the ability modifier the skills would normally use. If either of the skills are class skills, she receives the usual +3 bonus on those skill checks for having ranks in those skills. These ranks do not stack with her other skill ranks (only the higher number of ranks applies). This ability replaces spirit magic."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Possessed Shaman ~ Crowded Vessel", at_level: 2, description: Some("Whenever a possessed shaman fails a saving throw against a charm or compulsion spell or effect, she can attempt a new saving throw (using the original DC) at the end of her next turn as the spirits inside her attempt to regain control. If the saving throw is successful, the charm or compulsion effect ends. If the saving throw fails, she is affected as normal for the remainder of the duration."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Possessed Shaman ~ Wandering Skills", at_level: 6, description: Some("[NOT IMPLEMENTED] At 6th level, a possessed shaman is able to make room for another spirit. When choosing her wandering spirit for the day, the possessed shaman chooses one skill. The possessed shaman treats this skill as if she had a number of ranks in it equal to her shaman level, and uses her Wisdom modifier in place of the ability modifier the skill would normally use. If the skill is a class skill, she receives the usual +3 bonus on skill checks for having ranks in that skill. Each time the possessed shaman changes her wandering spirit, she can also change the skill gained through this ability. These ranks do not stack with her other skill ranks (only the higher number of ranks applies). This ability replaces the wandering hex gained at 6th level."), benefit: None },
            ],
        },
        // Shaman Archetype ~ Speaker for the Past -- acg_abilities_class.lst:3588
        ArchetypeSwapEntry {
            key: "Shaman Archetype ~ Speaker for the Past",
            subject: "Shaman",
            archetype_name: "Speaker for the Past",
            description: Some("A speaker for the past is a shaman who specifically serves as the voice for spirits from her people's history. A speaker for the past is often an advocate of the ancestors of a specific group, the voice of experience, and a powerful resource that allows the past to aid the present."),
            source_page: Some("p.111"),
            prerequisites: Some(&["PRECLASS:1,Shaman=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Shaman Archetype ~ Speaker for the Past],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ShamanWanderingSpirit,TYPE.ShamanWanderingHex]"]),
            replaces: Some(&["ShamanSpiritAnimal", "ShamanWanderingSpirit", "ShamanWanderingHex"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Speaker for the Past ~ Mysteries of the Past", at_level: 1, description: Some("A speaker for the past gains Linguistics, Knowledge (history), Knowledge (local), Perception, and Use Magic Device as class skills. She also adds the spells from the ancestorUM and timeUM oracle mysteries to her class spell list (as the cleric level for those spells). This ability replaces the shaman's familiar. The speaker for the past must choose a time when she must spend 1 hour each day in quiet contemplation to regain her daily allotment of spells, but does not need to commune with a familiar during this time. Because she has no familiar, she does not gain a spirit familiar feature from any spirit she bonds with."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Speaker for the Past ~ Revelations of the Past", at_level: 4, description: Some("At 4th, 6th, 12th, 14th, and 20th levels, the speaker for the past can select a revelation from the ancestor or time mysteries. She uses her shaman level as her oracle level for these revelations, and uses her Wisdom modifier in place of her Charisma modifier for the purposes of the revelation. This ability replaces wandering spirit and wandering hex."), benefit: None },
            ],
        },
        // Shaman Archetype ~ Spirit Warden -- acg_abilities_class.lst:3589
        ArchetypeSwapEntry {
            key: "Shaman Archetype ~ Spirit Warden",
            subject: "Shaman",
            archetype_name: "Spirit Warden",
            description: Some("Not all spirits deserve reverence and respect. Some are twisted and despicable. It's a spirit warden's duty to end these spirits' existence."),
            source_page: Some("p.112"),
            prerequisites: Some(&["PRECLASS:1,Shaman=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Shaman Archetype ~ Spirit Warden],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ShamanHex10,TYPE.ShamanHex2,TYPE.ShamanSpirit]"]),
            replaces: Some(&["ShamanHex10", "ShamanHex2", "ShamanSpiritMagic"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Spirit Warden ~ Unnatural Mien", at_level: 1, description: Some("The spirit warden's dealings with the spirit world give her an unsettling demeanor. Diplomacy and Handle Animal are not class skills for a spirit warden. Intimidate is added as a class skill, and she gains a +2 bonus on Intimidate checks to demoralize a foe."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spirit Warden ~ Restless Magic", at_level: 1, description: Some("[NOT IMPLEMENTED] The spirit warden adds the following spells to the list of spells she can cast using spirit magic: detect undead (1st), command undead (2nd), halt undead (3rd), death ward (4th), possess object (5th), undead to death (6th), ethereal jaunt (7th), control undead (8th), foresight (9th). This ability replaces the spirit magic spells gained from the shaman's spirit."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spirit Warden ~ Rebuke Spirits", at_level: 2, description: Some("The spirit warden gains the ability to channel positive energy as a cleric of her level. Regardless of her alignment, she can only use this ability to harm undead creatures. The spirit warden can use this ability %1 times per day.|3+CHA"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spirit Warden ~ Laugh at Death", at_level: 10, description: Some("The spirit warden's familiarity with the dead has filled her with contempt for death itself. She gains a +4 insight bonus on saving throws against death effects and to avoid or remove negative levels."), benefit: None },
            ],
        },
        // Shaman Archetype ~ Unsworn Shaman -- acg_abilities_class.lst:3590
        ArchetypeSwapEntry {
            key: "Shaman Archetype ~ Unsworn Shaman",
            subject: "Shaman",
            archetype_name: "Unsworn Shaman",
            description: Some("An unsworn shaman never binds herself to one specific spirit, always making new deals as she deems necessary for the circumstances that she finds herself in. While this weakens the powers she can access from any one spirit, it gives her access to a broader overall range of abilities."),
            source_page: Some("p.112"),
            prerequisites: Some(&["PRECLASS:1,Shaman=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Shaman Archetype ~ Unsworn Shaman],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ShamanSpiritMagic,TYPE.ShamanSpiritAnimal,TYPE.ShamanWanderingSpirit,TYPE.ShamanWanderingHex,TYPE.ShamanSpirit,TYPE.ShamanHex]"]),
            replaces: Some(&["ShamanSpiritMagic", "ShamanSpiritAnimal", "ShamanWanderingSpirit", "ShamanWanderingHex", "ShamanSpirit", "ShamanHex"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Unsworn Shaman ~ Minor Spirit", at_level: 1, description: Some("The unsworn shaman also forms a temporary bond with a minor spirit each day, granting her access to a shaman or witch hex of her choosing, but not a major hex or a grand hex. She must make this selection each day when she prepares her spells for the day. Until she changes the minor spirit, she continues to have access to the shaman or witch hex. At 2nd level, she can instead select a hex from one of her wandering spirits selected for that day. If she selects a shaman or witch hex, she treats her shaman level as her witch level, and uses her Wisdom in place of her Intelligence for the purpose of that hex. She can make temporary bonds with two minor spirits (thus gaining two hexes) at 4th level, and with one additional minor spirit (and hex) every 4 levels thereafter. This ability replaces spirit and alters hex."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Unsworn Shaman ~ Spirit Animal", at_level: 2, description: Some("An unsworn shaman's spirit animal gains the spirit animal bonus from one of her wandering spirits (see below). This bonus can be changed each day when the shaman prepares spells, but it must correspond to one of the shaman's wandering spirits. This ability alters spirit animal."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Unsworn Shaman ~ Wandering Spirit", at_level: 2, description: Some("The unsworn shaman gains access to the wandering spirit Class Feature.Shaman Class Feature. At 10th level, she gains the abilities listed in the greater version of her wandering spirit. At 18th level, she gains the abilities listed in the true version of her wandering spirit. Additionally, at 6th level, she also gains a second wandering spirit, gaining the abilities listed in the greater version of that spirit at 14th level, and the abilities listed in the true version at 20th level. This ability alters wandering spirit and replaces wandering hex."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Unsworn Shaman ~ Spirit Magic", at_level: 2, description: Some("The unsworn shaman gains this ability at 2nd level rather than at 1st. This ability alters spirit magic."), benefit: None },
            ],
        },
        // Shaman Archetype ~ Visionary -- acg_abilities_class.lst:3591
        ArchetypeSwapEntry {
            key: "Shaman Archetype ~ Visionary",
            subject: "Shaman",
            archetype_name: "Visionary",
            description: Some("The visionary is a master at divination, drawing upon her intimate relationship with the spirit world to ferret out all manner of secrets and insights about the world around her and beyond."),
            source_page: Some("p.112"),
            prerequisites: Some(&["PRECLASS:1,Shaman=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Shaman Archetype ~ Visionary],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ShamanWanderingSpirit,TYPE.ShamanWanderingHex6,TYPE.ShamanWanderingSpiritGreater,TYPE.ShamanWanderingSpiritTrue]"]),
            replaces: Some(&["ShamanWanderingSpirit", "ShamanWanderingHex6", "ShamanWanderingSpiritGreater", "ShamanWanderingSpiritTrue"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Visionary ~ Bonus Feat", at_level: 4, description: Some("The visionary gains Diviner's Delving as a bonus feat, even if she does not meet the prerequisites."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Visionary ~ Discern Magical Expertise", at_level: 4, description: Some("The visionary can determine what type of spellcasting expertise a creature possesses by studying the creature for 2 rounds with detect magic or any of the following spells: detect chaos, detect evil, detect good, or detect law (if the creature belongs to the alignment). This ability tells the visionary what bloodlines, domains, hexes, schools, or mysteries (if any) the creature possesses. A successful Will saving throw negates this effect (DC %1). A creature affected by this ability cannot be affected by it again for 24 hours. This ability functions through magical sensors as with clairaudience/clairvoyance and scrying.|10+floor(ShamanLVL/2)+WIS"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Visionary ~ Vision Spirit Magic", at_level: 4, description: Some("[NOT IMPLEMENTED] The visionary adds the following spells to the list of spells she can cast using spirit magic: see alignment (1st), see invisibility (2nd), clairaudience/ clairvoyance (3rd), detect scrying (4th), prying eyes (5th), legend lore (6th), vision (7th), moment of prescience (8th), foresight (9th). This ability replaces the spirit magic spells gained from the shaman's wandering spirit."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Visionary ~ Improved Divination", at_level: 6, description: Some("The visionary becomes more adept at divination magic. When she casts the augury spell, her chance for an accurate answer is automatically the maximum of 90%%. Likewise, when she casts divination, she has the maximum 90%% chance of an accurate answer. Finally, the visionary can prepare scrying as a 4th-level spell, and it requires only 1 minute to cast. The visionary also has a 10%% chance per caster level to cast the spells listed in the scrying spell description, instead of 5%% per caster level (to a maximum of 100%%). The visionary must still prepare these spells to receive these benefits."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Visionary ~ Wandering Spirit", at_level: 12, description: Some("The visionary forms a temporary bond with another spirit (other than the one she selected using her spirit class feature). This is identical to the 4th-level shaman class feature. This adds the wandering spirit magic spells to the list of spells she can cast using spirit magic, along with vision spirit magic and her original spirit. At 20th level, she gains the abilities listed in the greater version of her wandering spirit. This ability replaces the greater version of wandering spirit gained at 12th level and true version of wandering spirit gained at 20th level."), benefit: None },
            ],
        },
        // Shaman Archetype ~ Witch Doctor -- acg_abilities_class.lst:3592
        ArchetypeSwapEntry {
            key: "Shaman Archetype ~ Witch Doctor",
            subject: "Shaman",
            archetype_name: "Witch Doctor",
            description: Some("The witch doctor is a healer who specializes in afflictions of the soul. Often misunderstood, she protects her tribe with healing powers, powerful defensive magic, and her own divine \"witchcraft.\""),
            source_page: Some("p.113"),
            prerequisites: Some(&["!PREALIGN:LE,NE,CE", "PRECLASS:1,Shaman=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Shaman Archetype ~ Witch Doctor],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ShamanHex4,TYPE.ShamanHex12,TYPE.ShamanHex8,TYPE.ShamanHex10]"]),
            replaces: Some(&["ShamanHex4", "ShamanHex12", "ShamanHex8", "ShamanHex10"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Witch Doctor ~ Alignment", at_level: 1, description: Some("A witch doctor cannot be of evil alignment."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Witch Doctor ~ Channel Energy", at_level: 4, description: Some("The witch doctor can draw transcendental energies to her location, flooding it with positive energy as the cleric class feature. The witch doctor uses her shaman level - 3 as her effective cleric level, and can channel energy %1 times per day. This is a separate pool of channel energy that does not stack with the life spirit's channel spirit ability.|3+CHA"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Witch Doctor ~ Counter Curse", at_level: 8, description: Some("The witch doctor can choose to lose any prepared spirit magic spell that is 3rd level or higher in order to spontaneously cast dispel magic or remove curse. This ability can only target a spell effect that is on an ally (including herself ). If she forfeits a spirit magic spell higher than 3rd level, she gains a +2 sacred bonus on her caster level check to dispel the spell or to remove the curse for every spell level higher than 3rd that she sacrifices."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Witch Doctor ~ Countering Hex", at_level: 10, description: Some("The witch doctor can use her hex magic to counterspell as a readied action as dispel magic instead. She must succeed at a dispel check (1d20 + her shaman level) with a DC equal to 11 + the spell's caster level. If countering hex succeeds, the spell fizzles away and is lost. Failure means the spell is not countered. In either case, the witch doctor cannot attempt to use this hex against any of that caster's spells again for 24 hours. The witch doctor cannot use countering hex on an ongoing effect, a magic item, or a hex."), benefit: None },
            ],
        },
        // Skald Archetype ~ Fated Champion -- acg_abilities_class.lst:3675
        ArchetypeSwapEntry {
            key: "Skald Archetype ~ Fated Champion",
            subject: "Skald",
            archetype_name: "Fated Champion",
            description: Some("Many cultures see fate as a limit that is both stifling and unwanted, regardless of the destiny that lurks in the days and years ahead. Among cultures where skalds are the keepers of lore and wisdom, there are those who learn to read the winds of fate and take up the mantle of the fated champion, knowing and embracing their destined paths with strength borne of conviction."),
            source_page: Some("p.114"),
            prerequisites: Some(&["PRECLASS:1,Skald=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Skald Archetype ~ Fated Champion],[!PREABILITY:1,CATEGORY=Archetype,TYPE.SkaldWellVersed,TYPE.SkaldDirgeOfDoom,TYPE.SkaldMasterSkald,TYPE.SkaldSpellKenning]"]),
            replaces: Some(&["SkaldWellVersed", "SkaldSpellKenning", "SkaldDirgeOfDoom", "SkaldMasterSkald"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Fated Champion ~ Watcher of the Weave", at_level: 2, description: Some("A fated champion learns to see the tapestry of events moments before it is woven. He gains a +%1 insight bonus on initiative checks.|floor(SkaldLVL/2)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fated Champion ~ Far Seer", at_level: 5, description: Some("A fated champion learns a method of spell kenning that is especially suited for divining and understanding the future, but not for directly harming opponents. For the purpose of determining what level spell slot he expends when using spell kenning, the skald treats the spell level of spells that predict the future (such as augury and divination) as one spell level lower (minimum spell level 1st), and treats the spell level of spells that deal damage as one spell level higher. This alters the spell kenning class feature."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fated Champion ~ Shield of Foresight", at_level: 10, description: Some("A fated champion's certainty of his own fate grants him the ability to stare down fear and remain unbowed. When using raging song, he is immune to fear effects, and allies affected by his raging song gain a +5 bonus on saves attempted against fear effects."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fated Champion ~ Not This Day", at_level: 20, description: Some("The fated champion gains the ability to reweave the strands of fate. As an immediate action, he can expend 10 rounds of raging song to either reroll a saving throw or force an opponent to reroll an attack roll. The decision to use this ability must be made before the results of the initial roll are revealed, and the champion or his opponent must take the result of the reroll."), benefit: None },
            ],
        },
        // Skald Archetype ~ Herald of the Horn -- acg_abilities_class.lst:3676
        ArchetypeSwapEntry {
            key: "Skald Archetype ~ Herald of the Horn",
            subject: "Skald",
            archetype_name: "Herald of the Horn",
            description: Some("Even the loudest voice can often times be drowned out by the din of battle. Whether with the polished metal trumpet of a standing army or the crude curved animal horn of savage raiders, a herald of the horn sounds his raging song with thunderous blasts, which can bolster allies or shatter castle walls."),
            source_page: Some("p.115"),
            prerequisites: Some(&["PRECLASS:1,Skald=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Skald Archetype ~ Herald of the Horn],[!PREABILITY:1,CATEGORY=Archetype,TYPE.SkaldLoreMaster,TYPE.SkaldScribeScroll,TYPE.SkaldSpellKenning5,TYPE.SkaldSpellKenning11,TYPE.SkaldSpellKenning17]"]),
            replaces: Some(&["SkaldLoreMaster", "SkaldScribeScroll", "SkaldSpellKenning", "SkaldSpellKenning", "SkaldSpellKenning"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Herald of the Horn ~ Arcane Bond", at_level: 1, description: Some("A herald of the horn forms a powerful bond with a horn (musical instrument). This functions like an arcane bloodline sorcerer's arcane bond with an object. Like a weapon, wand, or staff, the horn must be held in one hand when the herald of the horn casts skald spells."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Herald of the Horn ~ Rousing Retort", at_level: 5, description: Some("A herald of the horn can use raging song to free allies from enchantment effects and fear. When beginning a raging song, he can expend 4 rounds of that ability to grant all allies within 60 feet a new saving throw against an ongoing enchantment or fear effect. The allies gain a +2 bonus on this new saving throw."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Herald of the Horn ~ Horn Call", at_level: 7, description: Some("A herald's horn enhances his sonic spells. If a skald spell with the sonic descriptor is cast using the horn, its DC increases by %1.|1+(SkaldLVL>=13)+(SkaldLVL>=19)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Herald of the Horn ~ Crumbling Blast", at_level: 11, description: Some("A herald of the horn can use his horn to create a devastating shock wave of energy. Once per day, he can sound a note on the horn that functions like a horn of blasting|PREVARLT:SkaldLVL,17"), benefit: None },
            ],
        },
        // Skald Archetype ~ Spell Warrior -- acg_abilities_class.lst:3677
        ArchetypeSwapEntry {
            key: "Skald Archetype ~ Spell Warrior",
            subject: "Skald",
            archetype_name: "Spell Warrior",
            description: Some("The spell warrior uses his arcane knowledge rather than his rage to turn the tide of battle in favor of himself and his allies. With a clash of bracers and a sonorous chant, the Spell Warrior's song reaches out to touch the weapons of his allies, lending them arcane power. While his song does not inspire rage, he can use the power of his music to enchant the weapons of his allies and counter the spells of his foes."),
            source_page: Some("p.116"),
            prerequisites: Some(&["PRECLASS:1,Skald=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Skald Archetype ~ Spell Warrior],[!PREABILITY:1,CATEGORY=Archetype,TYPE.SkaldScribeScroll,TYPE.SkaldInspiredRage,TYPE.SkaldDirgeOfDoom,TYPE.SkaldSpellKenning,TYPE.SkaldMasterSkald]"]),
            replaces: Some(&["SkaldScribeScroll", "SkaldInspiredRage", "SkaldDirgeOfDoom", "SkaldSpellKenning", "SkaldMasterSkald"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Spell Warrior ~ Improved Counterspell", at_level: 1, description: Some("The spell warrior receives the Improved Counterspell feat as a bonus feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spell Warrior ~ Enhance Weapons", at_level: 1, description: Some("The spell warrior can use his raging song to grant a +%1 enhancement bonus to the weapons (including ammunition) of allies within 60 feet. The maximum bonus gained is based upon the number of weapons affected: +5 to one weapon, +4 to two weapons, +3 to three weapons, or +2 to four or more weapons. Fifty pieces of ammunition count as one weapon for this purpose. The wielder of a weapon enhanced by this raging song counts as if he were under the effect of an inspired rage raging song for all purposes involving the skald's rage powers. These bonuses can also be used to add any of the following weapon special abilities to the weapons enhanced by this ability: dancing, defending, distance, flaming, frost, ghost touch, keen, mighty cleaving, returning, shock, seeking, or speed. Adding these weapon special abilities consumes an amount of bonus equal to the special ability's cost (see Table 15-9: Melee Weapon Special Abilities on page 469 of the Core Rulebook). These enhancement bonuses and special abilities overlap with any enhancements or special abilities the weapon already has, though duplicate special abilities do not stack. If an affected weapon is not magical, at least a +1 enhancement bonus must be added before any other special abilities can be. The bonus and special abilities granted by this raging song are determined when the song begins, and cannot be changed until the raging song ends and another is begun. These bonuses apply to only one end of a double weapon.|1+floor(SkaldLVL/5)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spell Warrior ~ Greater Counterspell", at_level: 5, description: Some("The spell warrior gains increased versatility when attempting to counteract enemy spellcasting."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spell Warrior ~ Song of Arcane Manipulation", at_level: 10, description: Some("A spell warrior can sacrifice his own rage magic to counter an opponent's spell. When using raging song, he can counterspell as an immediate action without interrupting his raging song. However, in addition to expending a spell slot (or spell slots) to attempt to counter the opponent's spell, the skald must expend 1 round of raging song per spell level of the opponent's spell (for example, if attempting to counterspell a 3rd-level spell, the skald must expend one of his own 3rd-level spell slots and 3 rounds of raging song)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spell Warrior ~ Spell Tamper", at_level: 20, description: Some("When a spell warrior successfully counterspells an opponent's spell, the opponent suffers a backlash of magical energy and takes 1d6 points of damage per spell level of the countered spell. If the opponent succeeds at a Will saving throw (DC %1), the damage is reduced by half. This damage is magical and is not subject to damage reduction or energy resistance.|10+floor(SkaldLVL/2)+CHA"), benefit: None },
            ],
        },
        // Skald Archetype ~ Totemic Skald -- acg_abilities_class.lst:3678
        ArchetypeSwapEntry {
            key: "Skald Archetype ~ Totemic Skald",
            subject: "Skald",
            archetype_name: "Totemic Skald",
            description: Some("The totemic skald forms a close connection to an animal totem. Through the power of this mystical ally, the skald can change shapes, assuming its form as his own. Additionally, the totem animal grants the skald a measure of its spiritual power that he can share with his allies."),
            source_page: Some("p.117"),
            prerequisites: Some(&["PRECLASS:1,Skald=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Skald Archetype ~ Totemic Skald],[!PREABILITY:1,CATEGORY=Archetype,TYPE.SkaldRagePower3,TYPE.SkaldUncannyDodge,TYPE.SkaldImprovedUncannyDodge,TYPE.SkaldSpellKenning]"]),
            replaces: Some(&["SkaldRagePowers", "SkaldUncannyDodge", "SkaldImprovedUncannyDodge", "SkaldSpellKenning"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Totemic Skald ~ Totem", at_level: 3, description: Some("The totemic skald chooses one animal from the hunter's animal focus list (see page 27). Once selected, this choice cannot be changed. This animal becomes the skald's personal totem animal and influences his later abilities."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Totemic Skald ~ Song of the Beast", at_level: 3, description: Some("The totemic skald grants the animal focus abilities of his totem animal (as the hunter's animal focus ability) to all allies affected by his raging song. He treats his skald level as his hunter level for determining the abilities of the animal focus (such as the improvements gained at 8th and 15th level). This ability replaces the rage power gained at 3rd level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Totemic Skald ~ Totem Empathy", at_level: 4, description: Some("The totemic skald can improve the attitude of animals of the same type as his totem. This ability functions just like a Diplomacy check to improve the attitude of a person. The skald rolls 1d20 and adds his skald level and his Charisma modifier to determine the totem empathy check result. The typical domestic animal has a starting attitude of indifferent, while wild animals are usually unfriendly. In addition, the totemic skald can cast charm animal as a spell-like ability. This only functions against animals that are of the same type as his totem animal (subject to GM discretion)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Totemic Skald ~ Wild Shape", at_level: 5, description: Some("A totemic skald gains the ability to wild shape into the form of a Small or Medium version of his totem animal, as the druid class feature. His effective druid level for this ability is equal to his skald level - 1. He can use this ability %1 times per day. This doesn't allow the skald to assume other forms, such as elementals, plants, or other kinds of animals. When in animal form, the skald is treated as able to speak normally for the purpose of using raging song, but not for using other abilities that require speech (such as spellcasting). The skald uses his class level as his druid level for the purpose of qualifying for feats that affect wild shape (such as Wild Speech).|min(3,floor((SkaldLVL+1)/6))"), benefit: None },
            ],
        },
        // Sorcerer Archetype ~ Eldritch Scrapper -- acg_abilities_class.lst:3794
        ArchetypeSwapEntry {
            key: "Sorcerer Archetype ~ Eldritch Scrapper",
            subject: "Sorcerer",
            archetype_name: "Eldritch Scrapper",
            description: Some("An eldritch scrapper is usually spoiling for a fight, looking to prove that she's just as tough as a martial character. A veteran of many brawls against opponents who were suspicious of her manifesting magic, an eldritch scrapper has a thick skin and a fighting style that blends weapons with spells."),
            source_page: Some("p.122"),
            prerequisites: Some(&["PRECLASS:1,Sorcerer=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Sorcerer Archetype ~ Eldritch Scrapper],[!PREABILITY:1,CATEGORY=Archetype,TYPE.SorcererBloodlinePowerLvl1.SorcererBloodlinePowerLvl9.SorcererBloodlinePowerLvl15]"]),
            replaces: Some(&["SorcererBloodlinePowerLvl1", "SorcererBloodlinePowerLvl9", "SorcererBloodlinePowerLvl15"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Eldritch Scrapper ~ Martial Flexibility", at_level: 1, description: Some("At 1st level, an eldritch scrapper gains the brawler's martial flexibility class feature, using her sorcerer level as her brawler level for the purposes of uses per day. The scrapper treats Arcane Strike and Combat Casting as combat feats for the purpose of this ability. At 9th level, an eldritch scrapper can use this ability to gain the benefit of two combat feats at the same time. She can select one feat as a move action or two feats as a standard action. She can use one of these feats to meet a prerequisite of the second feat; doing so means she cannot replace the feat that is currently fulfilling another feat's prerequisites without also replacing all feats that require it. Each individual feat selected counts toward her daily uses of this ability. At 15th level, an eldritch scrapper can use this ability to gain the benefit of three combat feats at the same time. She can select one feat as a swift action, two feats as a move action, or three feats as a standard action. She can use one of the feats to meet a prerequisite of the second and third feats, and use the second feat to meet a prerequisite of the third feat. Each individual feat selected counts toward her daily uses of this ability. This ability replaces the sorcerer's bloodline powers gained 1st, 9th, and 15th levels."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Eldritch Scrapper ~ Bloodline Weapons", at_level: 1, description: Some("[NOT IMPLEMENTED] If an eldritch scrapper's 1st-level bloodline power would normally grant her natural attacks (such as bite or claws), at 3rd level she can select that 1st-level bloodline power in place of her 3rd-level bloodline power."), benefit: None },
            ],
        },
        // Sorcerer Archetype ~ Mongrel Mage -- acg_abilities_class.lst:3795
        ArchetypeSwapEntry {
            key: "Sorcerer Archetype ~ Mongrel Mage",
            subject: "Sorcerer",
            archetype_name: "Mongrel Mage",
            description: Some("A mongrel mage is a sorcerer whose bloodline is so weak, or mixed with so many others, that her power isn't clearly associated with any bloodline source. A mongrel mage's bloodline powers can change on a daily basis, but always fall short of those of a full-blooded sorcerer's powers. A mongrel mage is a dabbler in all bloodlines but a master of none, and she is looked down upon by true sorcerers."),
            source_page: Some("p.122"),
            prerequisites: Some(&["PRECLASS:1,Sorcerer=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Sorcerer Archetype ~ Mongrel Mage],[!PREABILITY:1,CATEGORY=Archetype,TYPE.SorcererBloodlineFeats,TYPE.CF_SorcererBloodline,TYPE.CF_SorcererBloodlinePowers,TYPE.CF_SorcererBloodlineSpells]"]),
            replaces: Some(&["SorcererBloodlineFeats", "CF_SorcererBloodline", "CF_SorcererBloodlinePowers", "CF_SorcererBloodlineSpells"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Mongrel Mage ~ Mongrel Reservoir", at_level: 1, description: Some("At 1st level, a mongrel mage has an innate pool of magical energy, known as her mongrel reservoir, that she can draw upon to activate her weakened bloodline. Her mongrel reservoir can hold %1 points of magical energy. Each day when refreshing her spell slots, her mongrel reservoir is restored to full. The mongrel reservoir can never hold more points that the total mentioned above; points gained in excess of this maximum are lost. Each day when she refreshes her spell slots, the mongrel mage selects one sorcerer bloodline. She must select an ordinary bloodline with this ability, not one altered by the wildblooded archetype or any other archetype. She gains this bloodline's 1st-level bloodline power for that day, using it as if she were only a 1st-level sorcerer. (If this ability is used to gain an arcane bond and the bonded item is selected, she can use the item only to cast a 1st-level spell known, as she counts as only a 1st-level sorcerer.) A mongrel mage can have only one bloodline selected at a time. As a swift action, she can expend 1 point from her mongrel reservoir to activate that bloodline, allowing her to use its 1st-level bloodline powers as well as its bloodline arcana at her full sorcerer level, including using a bonded item from an arcane bond. This lasts for %2 rounds. At 3rd level, when activating her selected bloodline, a mongrel mage can instead spend 2 points from her mongrel reservoir to allow her to use the bloodline's 1st- and 3rd-level powers as well as its bloodline arcana at her full sorcerer level for %2 rounds. At 7th level, when a mongrel mage is activating her selected bloodline, the mongrel mage can instead spend 3 points from her mongrel reservoir, allowing her to use the bloodline's 1st-, 3rd-, and 7th-level powers as well as its bloodline arcana at her full sorcerer level %2 of rounds. At 20th level, when activating her selected bloodline, a mongrel mage can instead spend 5 points from her mongrel reservoir, allowing her to use all of the bloodline's powers as well as its bloodline arcana at her full sorcerer level for %2 rounds.|3+SorcererLVL|max(1,CHA)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mongrel Mage ~ Bloodline Spells", at_level: 7, description: Some("Each day when she selects her bloodline, a mongrel mage adds the 1st-, 2nd-, and 3rd-level spells from her selected bloodline to her current list of spells known.|PREVARLT:SorcererLVL,13"), benefit: None },
            ],
        },
        // Summoner Archetype ~ Naturalist -- acg_abilities_class.lst:3809
        ArchetypeSwapEntry {
            key: "Summoner Archetype ~ Naturalist",
            subject: "Summoner",
            archetype_name: "Naturalist",
            description: Some("A naturalist is a summoner who is in tune with the natural world, using his magic like a lens to focus various animal aspects onto his eidolon. More akin to a hunter than to other arcane spellcasters, a naturalist instinctively masters the power of such creatures as the bear, wolf, mouse, or tiger to make his exotic eidolon the perfect living tool for battle or stealth, and he eventually discovers how to apply these transformations to himself as well."),
            source_page: Some("p.123"),
            prerequisites: Some(&["PRECLASS:1,Summoner=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Summoner Archetype ~ Naturalist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.SummonerSummonMonsterI,TYPE.SummonerShieldAlly,TYPE.SummonerGreaterShieldAlly,TYPE.SummonerAspect,TYPE.SummonerLifeBond,TYPE.SummonerGreaterAspect]"]),
            replaces: Some(&["SummonerSummonMonsterI", "SummonerShieldAlly", "SummonerGreaterShieldAlly", "SummonerAspect", "SummonerLifeBond", "SummonerGreaterAspect"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Naturalist ~ Nature's Call", at_level: 1, description: Some("You can cast summon nature's ally %1 as a spell-like ability %2 times per day as a standard action and the creatures remain for %3 minutes (instead of %3 rounds).  You can only use this ability to summon creatures of the animal, magical beast, or vermin type. Drawing upon this ability uses up the same power you use to call your eidolon. As a result, you can only use this ability when your eidolon is not summoned.|NaturalistSummonNaturesAllyLVL|NaturalistSummonNaturesAllyTimes|NaturalistSummonNaturesAllyDuration"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Naturalist ~ Animal Focus", at_level: 4, description: Some("As a swift action a naturalist can enhance his eidolon with the aspect of an animal. Each time he uses this ability, he can select a hunter's animal aspect and apply it to his eidolon. His hunter level for this ability is %1. He does not gain the ability to add an animal aspect to himself. This effect lasts until the eidolon is dismissed or sent back to its home plane. When an animal aspect is applied to an eidolon, its form is altered by superficial physical changes appropriate to that aspect. For example, using aspect of the bat might give the eidolon larger ears and eyes and perhaps membranes of skin connecting its limbs to its body, while using aspect of the stag might give the eidolon antlers and hoof like nails on its feat. None of these changes interfere with any of the eidolon's natural or magical abilities.|SummonerLVL-2"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Naturalist ~ Second Animal Focus", at_level: 10, description: Some("Whenever a naturalist uses animal focus, he may apply two different animal aspects to his eidolon. The eidolon's form gains superficial physical charges appropriate to the chosen aspect."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Naturalist ~ Shared Focus", at_level: 14, description: Some("The naturalist begins to take on some of the feral nature of his eidolon. Whenever the naturalist uses animal focus to grant an aspect to his eidolon, he also gains the effects of the chosen aspect. The naturalist keeps this aspect until his eidolon is dismissed or sent back to its home plane. The naturalist gains the effects of only one aspect, even if the eidolon gains more than one (naturalist's choice)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Naturalist ~ Third Animal Focus", at_level: 18, description: Some("Whenever a naturalist uses his animal focus ability, he can apply three different animal aspects to his eidolon (one of which lasts until he decides to change it)."), benefit: None },
            ],
        },
        // Swashbuckler Archetype ~ Daring Infiltrator -- acg_abilities_class.lst:3840
        ArchetypeSwapEntry {
            key: "Swashbuckler Archetype ~ Daring Infiltrator",
            subject: "Swashbuckler",
            archetype_name: "Daring Infiltrator",
            description: Some("Not known for their flashy entrances or for standing out in a crowd, a daring infiltrator uses stealth, disguise, and ruthless guile to pursue her goals. Some of these swashbucklers work to undermine evil organizations, while others are master thieves or mysterious assassins."),
            source_page: Some("p.124"),
            prerequisites: Some(&["PRECLASS:1,Swashbuckler=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Swashbuckler Archetype ~ Daring Infiltrator],[!PREABILITY:1,CATEGORY=Archetype,TYPE.SwashbucklerClassSkills,TYPE.SwashbucklerBonusFeats,TYPE.SwashbucklerCharmedLife,TYPE.SwashbucklerSwashbucklerInitiativeDeed,TYPE.SwashbucklerMenacingSwordplayDeed,TYPE.SwashbucklerBleedingWoundDeed]"]),
            replaces: Some(&["SwashbucklerClassSkills", "SwashbucklerBonusFeats", "SwashbucklerCharmedLife", "SwashbucklerSwashbucklerInitiativeDeed", "SwashbucklerMenacingSwordplayDeed", "SwashbucklerBleedingWoundDeed"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Daring Infiltrator ~ Class Skills", at_level: 1, description: Some("A daring infiltrator gains Disguise and Stealth as class skills, but does not gain Diplomacy, Perform, and Profession as class skills. This alters the swashbuckler's class skill list."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Daring Infiltrator ~ Bonus Feats", at_level: 1, description: Some("In addition to combat feats, a daring infiltrator's bonus feats at 4th, 8th, 12th, 16th and 20th level can come from the following list: Alertness, Antagonize, Cosmopolitan, Deceitful, Deft Hands, Disarming Threat, Persuasive, Prodigy, and Skill Focus."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Daring Infiltrator ~ Quick-Tongued", at_level: 2, description: Some("A daring infiltrator gains a +%1 bonus on Bluff checks.|floor((SwashbucklerLVL+2)/2)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Daring Infiltrator ~ Deeds", at_level: 3, description: Some("A daring infiltrator gains the following deeds, each of which replaces an existing deed. Clandestine Expertise, Silence Is Golden, and Authoritative Bluff."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deed ~ Clandestine Expertise", at_level: 3, description: Some("A daring infiltrator with at least 1 panache point gains a +2 bonus on Disguise and Stealth checks. Also, as long as she has at least 1 panache point, when she successfully aids another with a Disguise or Stealth check, she grants the subject a +4 bonus on the skill check instead of the normal +2. This deed replaces swashbuckler initiative."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deed ~ Silence Is Golden", at_level: 3, description: Some("When a daring infiltrator with at least 1 panache point succeeds at a trip or grapple combat maneuver check, the target is rendered mute for 1 round. For every 5 by which the result of the combat maneuver check exceeds the opponent's CMD, the target remains mute for an additional round. A mute creature cannot speak, use language-dependent effects or verbal components, or use command words. This deed replaces menacing swordplay."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deed ~ Authoritative Bluff", at_level: 11, description: Some("A daring infiltrator can spend 1 panache point to reroll a Bluff check after the roll is made but before the results are revealed. She must take the result of the second roll, even if it is lower. Additionally, a daring infiltrator with at least 1 panache point gains a +5 bonus on Bluff checks to pretend to be someone's superior (socially or in the military). If she succeeds at the check, the target obeys any reasonable orders she gives as it would those of an actual superior in the situation. This deed replaces bleeding wound."), benefit: None },
            ],
        },
        // Swashbuckler Archetype ~ Flying Blade -- acg_abilities_class.lst:3841
        ArchetypeSwapEntry {
            key: "Swashbuckler Archetype ~ Flying Blade",
            subject: "Swashbuckler",
            archetype_name: "Flying Blade",
            description: Some("While most swashbucklers prefer their battles up close, others prefer dealing death from a distance."),
            source_page: Some("p.124"),
            prerequisites: Some(&["PRECLASS:1,Swashbuckler=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Swashbuckler Archetype ~ Flying Blade],[!PREABILITY:1,CATEGORY=Archetype,TYPE.SwashbucklerPanache,TYPE.SwashbucklerDodgingPanacheDeed,TYPE.SwashbucklerMenacingSwordplayDeed,TYPE.SwashbucklerTargetedStrikeDeed,TYPE.SwashbucklerBleedingWoundDeed,TYPE.SwashbucklerPerfectThrustDeed,TYPE.SwashbucklerSwashbucklerWeaponTraining,TYPE.SwashbucklerSwashbucklerWeaponMastery]"]),
            replaces: Some(&["SwashbucklerPanache", "SwashbucklerDodgingPanacheDeed", "SwashbucklerMenacingSwordplayDeed", "SwashbucklerTargetedStrikeDeed", "SwashbucklerBleedingWoundDeed", "SwashbucklerPerfectThrustDeed", "SwashbucklerWeaponTraining", "SwashbucklerSwashbucklerWeaponMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Flying Blade ~ Panache", at_level: 1, description: Some("Unlike other swashbucklers, a flying blade regains panache only when she confirms a critical hit or makes a killing blow with a dagger or starknife. This ability alters panache."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Flying Blade ~ Deeds", at_level: 1, description: Some("A flying blade gains the following deeds, each of which replaces an existing deed. Subtle Throw, Disrupting Counter, Precise Throw, Targeted Throw, Bleeding Wound, and Perfect Throw."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Flying Blade ~ Flying Blade Training", at_level: 5, description: Some("A flying blade gains a +%1 bonus on attack and damage rolls when using daggers or starknives in combat. When a flying blade wields a dagger or starknife, she gains the benefit of the Improved Critical feat with those weapons. Additionally, a flying blade increases the range increment of a thrown dagger or starknife by %2 feet. The increase of range increment stacks with that of precise throw.|FlyingBladeTrainingBonus|FlyingBladeRangeBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Flying Blade ~ Flying Blade Mastery", at_level: 20, description: Some("When an attack that a flying blade makes with a dagger or starknife threatens a critical hit, that critical hit is automatically confirmed. Furthermore, the critical modifiers of daggers and starknives increase by 1 (x2 becomes x3, and so on)."), benefit: None },
            ],
        },
        // Swashbuckler Archetype ~ Inspired Blade -- acg_abilities_class.lst:3842
        ArchetypeSwapEntry {
            key: "Swashbuckler Archetype ~ Inspired Blade",
            subject: "Swashbuckler",
            archetype_name: "Inspired Blade",
            description: Some("An inspired blade is both a force of personality and a sage of swordplay dedicated to the perfection of combat with the rapier. They use the science and geometry with swordplay to beautiful and deadly effect."),
            source_page: Some("p.125"),
            prerequisites: Some(&["PRECLASS:1,Swashbuckler=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Swashbuckler Archetype ~ Inspired Blade],[!PREABILITY:1,CATEGORY=Archetype,TYPE.SwashbucklerBleedingWound,TYPE.SwashbucklerPanache,TYPE.SwashbucklerSwashbucklerFinesse,TYPE.SwashbucklerDeeds,TYPE.SwashbucklerSwashbucklerWeaponTraining,TYPE.SwashbucklerSwashbucklerWeaponMastery]"]),
            replaces: Some(&["SwashbucklerBleedingWoundDeed", "SwashbucklerPanache", "SwashbucklerFinesse", "SwashbucklerWeaponTraining", "SwashbucklerWeaponMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Inspired Blade ~ Inspired Panache", at_level: 1, description: Some("Each day, an inspired blade gains %1 panache points. Unlike other swashbucklers, an inspired blade gains no panache from a killing blow. She gains panache only from scoring a critical hit with a rapier. This ability alters the panache class feature.|PanachePoints"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Inspired Blade ~ Inspired Finesse", at_level: 1, description: Some("An inspired blade gains the benefits of Weapon Finesse with the rapier (this ability counts as having the Weapon Finesse feat for the purpose of meeting feat prerequisites) and gains Weapon Focus (rapier) as a bonus feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Inspired Blade ~ Rapier Training", at_level: 5, description: Some("An inspired blade gains a +%1 bonus on attack rolls and a +%2 bonus on damage rolls with rapiers. While wielding a rapier, she gains the benefit of the Improved Critical feat.|InspiredRapierBonus|InspiredRapierBonus+1"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deed ~ Inspired Strike", at_level: 11, description: Some("An inspired blade can spend 1 panache point when making an attack with a rapier to gain a +%1 insight bonus on that attack roll. When an inspired blade hits with an attack augmented by inspired strike, she can spend 1 additional panache point to make the hit a critical threat, though if she does so, she does not regain panache if she confirms that critical threat. The cost of this deed cannot be reduced by abilities such as Signature Deed.|max(INT,1)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Inspired Blade ~ Rapier Weapon Mastery", at_level: 20, description: Some("When an inspired blade threatens a critical hit with a rapier, that critical hit is automatically confirmed. Furthermore, the critical threat range increases by 1 (this increase to the critical threat range stacks with the increase from rapier training, to a total threat range of 14-20), and the critical modifier of the weapon increases by 1 (x2 becomes x3, for example). This ability replaces swashbuckler weapon mastery."), benefit: None },
            ],
        },
        // Swashbuckler Archetype ~ Mouser -- acg_abilities_class.lst:3843
        ArchetypeSwapEntry {
            key: "Swashbuckler Archetype ~ Mouser",
            subject: "Swashbuckler",
            archetype_name: "Mouser",
            description: Some("In the hands of a trained warrior, a well-sharpened blade is deadly regardless of size. A mouser moves in close, using her size and skill as an advantage."),
            source_page: Some("p.125"),
            prerequisites: Some(&["PRECLASS:1,Swashbuckler=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Swashbuckler Archetype ~ Mouser],[!PREABILITY:1,CATEGORY=Archetype,TYPE.SwashbucklerBleedingWoundDeed,TYPE.SwashbucklerTargetedStrikeDeed,TYPE.SwashbucklerOpportuneParryDeed,TYPE.SwashbucklerRiposteDeed,TYPE.SwashbucklerMenacingSwordplayDeed]"]),
            replaces: Some(&["SwashbucklerBleedingWoundDeed", "SwashbucklerTargetedStrikeDeed", "SwashbucklerOpportuneParryDeed", "SwashbucklerRiposteDeed", "SwashbucklerMenacingSwordplayDeed"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Mouser ~ Deeds", at_level: 1, description: Some("The mouser gains the following deeds, each of which replaces an existing deed. Underfoot Assault, Quick Steal, Hamstring, and Cat's Charge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mouser ~ Underfoot Assault", at_level: 1, description: Some("If a foe whose size is larger than the mouser's is adjacent to her and misses her with a melee attack, the mouser can as an immediate action spend 1 panache point to move 5 feet into an area of the attacker's space. This movement does not count against the mouser's movement the next round, and it doesn't provoke attacks of opportunity. While the mouser is within a foe's space, she is considered to occupy her square within that foe's space. While the mouser is within her foe's space, the foe takes a -4 penalty on all attack rolls and combat maneuver checks not made against the mouser, and all of the mouser's allies that are adjacent to both the foe and the mouser are considered to be flanking the foe. The mouser is considered to be flanking the foe whose space she is within if she is adjacent to an ally who is also adjacent to the foe. The mouser can move within her foe's space and leave the foe's space unhindered and without provoking attacks of opportunity, but if the foe attempts to move to a position where the mouser is no longer in its space, the movement provokes an attack of opportunity from the mouser. This deed replaces opportune parry and riposte."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mouser ~ Quick Steal", at_level: 3, description: Some("As a swift action the mouser can spend 1 panache point when she hits a for larger than her size with a light or one-handed piercing melee weapon to attempt a steal combat maneuver check against the creature she hit. Using this deed does not provoke an attack of opportunity. This deed replaces menacing swordplay."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mouser ~ Hamstring", at_level: 7, description: Some("As long as a mouser has at least 1 panache point when she hits a foe whose size is larger than her own with a light or one-handed piercing melee weapon, she can as a swift action attempt a dirty trick combat maneuver check. Instead of the normal conditions that can be applied with dirty trick, this deed can only stagger the target if the check is successful. This deed replaces targeted strike."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mouser ~ Cat's Charge", at_level: 11, description: Some("As long as a mouser has at least 1 panache point, when she charges a foe whose size is larger than her own, the mouser can end her charge in any space she can reach, not just the closest space. All other requirements of the charge must still be satisfied. This deed replaces bleeding wound."), benefit: None },
            ],
        },
        // Swashbuckler Archetype ~ Musketeer -- acg_abilities_class.lst:3844
        ArchetypeSwapEntry {
            key: "Swashbuckler Archetype ~ Musketeer",
            subject: "Swashbuckler",
            archetype_name: "Musketeer",
            description: Some("A number of organizations and kingdoms search for warriors who are brave (or foolish) enough to wield firearms on the battlefield. The daring nature of swashbucklers often makes them ideal candidates for such service."),
            source_page: Some("p.126"),
            prerequisites: Some(&["PRECLASS:1,Swashbuckler=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Swashbuckler Archetype ~ Musketeer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.SwashbucklerFinesse,TYPE.DeedDodgingPanache]"]),
            replaces: Some(&["SwashbucklerFinesse", "SwashbucklerWeaponProficiencies", "DeedDodgingPanache"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Musketeer ~ Weapon Proficiency", at_level: 1, description: Some("A musketeer gains proficiency with all simple weapons and martial weapons, as well as onehanded and two-handed firearms. This ability replaces the swashbuckler's weapon proficiencies."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Musketeer ~ Musketeer Instruction", at_level: 1, description: Some("A musketeer gains the benefits of the Weapon Finesse feature with the rapier (this counts as having the Weapon Finesse feat for purposes of meeting feat prerequisites), as well as both Rapid Reload (musket) and Gunsmithing feats as bonus feats."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Musketeer ~ Quick Clear", at_level: 1, description: Some("As a standard action the musketeer can spend 1 panache point to remove the broken condition from a single firearm he is currently wielding, as long as the firearm gained that condition through a misfire. This deed replaces dodging panache."), benefit: None },
            ],
        },
        // Swashbuckler Archetype ~ Mysterious Avenger -- acg_abilities_class.lst:3845
        ArchetypeSwapEntry {
            key: "Swashbuckler Archetype ~ Mysterious Avenger",
            subject: "Swashbuckler",
            archetype_name: "Mysterious Avenger",
            description: Some("While some swashbucklers fight for queen and country, and others for coin, glory, or just the enhancement of their own reputations, the mysterious avenger fights directly for a cause. Instead of gaining personal glory for her heroic deeds, she keeps her identity hidden in order to fight for those who cannot fight for themselves."),
            source_page: Some("p.126"),
            prerequisites: Some(&["PREALIGN:LG,NG,CG", "PRECLASS:1,Swashbuckler=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Swashbuckler Archetype ~ Mysterious Avenger],[!PREABILITY:1,CATEGORY=Archetype,TYPE.SwashbucklerProficiency,TYPE.SwashbucklerBonusFeat4,TYPE.SwashbucklerNimble,TYPE.SwashbucklerWeaponTraining,TYPE.CF_SwachbucklerBonusFeat]"]),
            replaces: Some(&["SwashbucklerClassSkills", "SwashbucklerProficiency", "SwashbucklerFinesse", "SwashbucklerBonusFeat4", "SwashbucklerNimble", "SwashbucklerWeaponTraining", "CF_SwachbucklerBonusFeat"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Mysterious Avenger ~ Alignment", at_level: 1, description: Some("A mysterious avenger must be of a good alignment, and must be dedicated to the protection of the good and the powerless under her care. If she ceases to be good or betrays the trust of those she was sworn to protect, she loses her secret identity and greater charmed life class features. She can regain them if she atones for her violations by some means that the GM sees fit (possibly by way of the atonement spell if the mysterious avenger is especially religious)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mysterious Avenger ~ Class Skills", at_level: 1, description: Some("A mysterious avenger adds Disguise to her list of class skills. This alters the swashbuckler's class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mysterious Avenger ~ Weapon and Armor Proficiency", at_level: 1, description: Some("A mysterious avenger loses her proficiency with bucklers, but gains proficiency in the whip exotic weapon. This alters the swashbuckler's weapon and armor proficiency."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mysterious Avenger ~ Avenger Finesse", at_level: 1, description: Some("A mysterious avenger gains all of the benefits of the swashbuckler's finesse class feature, and gains the ability to use a whip in place of a light or one-handed piercing melee weapon for all swashbuckler class features and deeds. This ability alters swashbuckler finesse."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mysterious Avenger ~ Secret Identity", at_level: 3, description: Some("A mysterious avenger's force of personality and dedication to her cause give her the ability to keep her true identity secret, even from magical prying. She gains a +4 bonus on Disguise checks in a single disguise of her choice, typically her avenger persona. Once this disguise has been chosen, it can't be changed. She also gains a +4 bonus on saving throws against divination effect. At 11th level, she becomes immune to all scrying effects and other magical effects used in attempts to uncover her secret identity."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mysterious Avenger ~ Greater Charmed Life", at_level: 4, description: Some("The mysterious avenger gains three extra uses of charmed life. As an immediate action, she can expend one use of charmed life to gain a +%1 bonus to her AC. She must choose to do this before the attack roll is made against her.|max(1,CHA)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mysterious Avenger ~ Avenger's Target", at_level: 5, description: Some("A mysterious avenger gains a +%1 bonus on attack and damage rolls with light or one-handed piercing melee weapons and whips. Furthermore, a mysterious avenger can study an opponent she can see as a move action. The mysterious avenger then gains a +%1 bonus on Bluff, Knowledge, Perception, Sense Motive, and Survival checks against that opponent, a +%1 bonus on weapon attack and damage rolls against it, and a +%1 bonus to DCs of any deeds used against that opponent. A mysterious avenger can maintain these bonuses against %1 opponents at a time; these bonuses remain in effect until either the opponent is dead or the mysterious avenger studies a new target. A mysterious avenger can discard this connection to a studied target as a free action, allowing her to study another target in its place. At 10th level, a mysterious avenger can study an opponent as a move or swift action.|AvengerTargetBonus"), benefit: None },
            ],
        },
        // Swashbuckler Archetype ~ Picaroon -- acg_abilities_class.lst:3846
        ArchetypeSwapEntry {
            key: "Swashbuckler Archetype ~ Picaroon",
            subject: "Swashbuckler",
            archetype_name: "Picaroon",
            description: Some("While some swashbucklers take pride in their ability to wear down an opponent with great skill at arms and clever positioning, there are those who use firearms to get in close and hit hard."),
            source_page: Some("p.127"),
            prerequisites: Some(&["PRECLASS:1,Swashbuckler=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Swashbuckler Archetype ~ Picaroon],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DeedOpportuneParry,TYPE.DeedRiposte,TYPE.DeedKipUp,TYPE.DeedSuperiorFeint,TYPE.DeedBleedingWound,TYPE.SwashbucklerFinesse]"]),
            replaces: Some(&["SwashbucklerWeaponProficiency", "SwashbucklerPanache", "DeedOpportuneParry", "DeedRiposte", "DeedKipUp", "DeedSuperiorFeint", "DeedBleedingWound", "SwashbucklerFinesse"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Picaroon ~ Weapon Proficiency", at_level: 1, description: Some("A picaroon gains proficiency with all simple weapons and martial weapons, as well as one-handed firearms."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Picaroon ~ Panache", at_level: 1, description: Some("Unlike other swashbucklers, a picaroon regains panache when she confirms a critical hit or makes a killing blow with a light or one-handed piercing melee weapon or a one-handed firearm. This ability alters panache."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Picaroon ~ Deeds", at_level: 1, description: Some("The picaroon gains the following deeds, each of which replaces an existing deed. Melee Shooter, Quick Clear, Gun Feint, and Lightning Reload."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Picaroon ~ Melee Shooter", at_level: 1, description: Some("As a swift action when wielding both a light or one-handed piercing melee weapon and a one-handed firearm, the picaroon can spend 1 panache point to avoid provoking attacks of opportunity with the first ranged attack made by the one-handed firearm during her turn. This deed replaces opportune parry and riposte."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Picaroon ~ Quick Clear", at_level: 3, description: Some("As a standard action the picaroon can spend 1 panache point to remove the broken condition from a single one-handed firearm she is currently wielding, as long as the firearm gained that condition through a misfire. This deed replaces kip-up."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Picaroon ~ Gun Feint", at_level: 7, description: Some("A picaroon can use the ferocious reputation of firearms to her advantage. A picaroon with at least 1 panache point can feint instead of attacking with her firearm as part of a full attack. She can spend 1 panache point to gain a +5 bonus on this check. This deed replaces superior feint."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Picaroon ~ Lightning Reload", at_level: 11, description: Some("Once per round the picaroon can spend 1 panache point to reload a single barrel of a one-handed firearm as a swift action. If she has the Rapid Reload feat or is using an alchemical cartridge, she can instead reload a single barrel of the weapon as a free action each round. Using this deed doesn't provoke attacks of opportunity. This deed replaces bleeding wound."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Picaroon ~ Two-Weapon Finesse", at_level: 1, description: Some("A picaroon gains the benefits of the Weapon Finesse feat with light or onehanded piercing melee weapons. She also gains the effects of the Two-Weapon Fighting feat as long as she is wielding a light or one-handed piercing melee weapon in one hand and one-handed firearm in the other hand. This ability counts as having both the Weapon Finesse and Two-Weapon Fighting feats for the purposes of meeting feat requirements."), benefit: None },
            ],
        },
        // Warpriest Archetype ~ Champion of the Faith -- acg_abilities_class.lst:3930
        ArchetypeSwapEntry {
            key: "Warpriest Archetype ~ Champion of the Faith",
            subject: "Warpriest",
            archetype_name: "Champion of the Faith",
            description: Some("Champions of the faith are crusaders who use the power of their divine patron to annihilate the faith's enemies."),
            source_page: Some("p.128"),
            prerequisites: Some(&["PRECLASS:1,Warpriest=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Warpriest Archetype ~ Champion of the Faith],[!PREABILITY:1,CATEGORY=Archetype,TYPE.WarpriestSacredWeapon,TYPE.WarpriestBonusFeat3,TYPE.WarpriestChannelEnergy,TYPE.CF_WarpriestBonusFeat]"]),
            replaces: Some(&["WarpriestSacredWeapon", "WarpriestBonusFeat3", "WarpriestChannelEnergy", "CF_WarpriestBonusFeat"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Champion of the Faith ~ Chosen Alignment", at_level: 1, description: Some("A champion of the faith must select one of the following as his chosen alignment: chaos, evil, good, or law. This choice must be one of the alignments shared by the champion of the faith and his deity. Champions of the faith who are neutral with no other alignment components (or whose deity is) can choose any of the above alignments for this purpose. Additionally, a champion of the faith must select the blessing corresponding to his chosen alignment, even if it's not on his deity's list of domains. His chosen alignment's opposite is referred to as his opposed alignment. Good and evil oppose one another, just as law and chaos oppose one another."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Champion of the Faith ~ Sacred Weapon", at_level: 1, description: Some("Sacred weapons (including his deities favored weapon and all weapons with Weapon Focus) can do base %1d%2 damage instead of the weapons normal base damage.|WarpriestSacredWeaponBaseDice|WarpriestSacredWeaponBaseDiceSize"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Champion of the Faith ~ Detect Alignment", at_level: 3, description: Some("A champion of the faith can detect his opposed alignment. As a move action, the champion of the faith can focus on a single item or creature within 60 feet and determine whether it"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Champion of the Faith ~ Smite", at_level: 4, description: Some("A champion of the faith can focus his powers against his chosen foes. As a swift action, the champion of the faith chooses one target within sight to smite. If this target is"), benefit: None },
            ],
        },
        // Warpriest Archetype ~ Cult Leader -- acg_abilities_class.lst:3931
        ArchetypeSwapEntry {
            key: "Warpriest Archetype ~ Cult Leader",
            subject: "Warpriest",
            archetype_name: "Cult Leader",
            description: Some("Referred to as fanatics, lunatics, or obsessives, cultists see themselves as genuine devotees of their deity. And the hierarchs of those devotees, the cult leaders, are the most fanatical of them all. Cult leaders are known for turning reasonable hearts toward corrupted teachings and striking at those that get in the way of their agenda."),
            source_page: Some("p.128"),
            prerequisites: Some(&["PRECLASS:1,Warpriest=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Warpriest Archetype ~ Cult Leader],[!PREABILITY:1,CATEGORY=Archetype,TYPE.WarpriestClassSkills,TYPE.WarpriestProficiency,TYPE.WarpriestChannelEnergy,TYPE.WarpriestFocusWeapon,TYPE.WarpriestBonusFeat3,TYPE.WarpriestBonusFeat9,TYPE.WarpriestBonusFeat12,TYPE.WarpriestBonusFeat15,TYPE.CF_WarpriestBonusFeat]"]),
            replaces: Some(&["WarpriestClassSkills", "WarpriestProficiency", "WarpriestChannelEnergy", "WarpriestFocusWeapon", "WarpriestBonusFeat3", "WarpriestBonusFeat9", "WarpriestBonusFeat12", "WarpriestBonusFeat15", "CF_WarpriestBonusFeat"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Cult Leader ~ Class Skills", at_level: 1, description: Some("The cult leader's class skills are Acrobatics (Dex), Bluff (Cha), Climb (Str), Craft (Int), Diplomacy (Cha), Disguise (Cha), Escape Artist (Dex), Heal (Wis), Intimidate (Cha), Knowledge (local) (Int), Knowledge (religion) (Int), Perception (Wis), Profession (Wis), Sense Motive (Wis), Sleight of Hand (Dex), Spellcraft (Int), and Stealth (Dex). These skills replace the warpriest's class skills. Skill Ranks per Level: 4 + Int modifier."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cult Leader ~ Weapon and Armor Proficiency", at_level: 1, description: Some("Cult leaders are proficient with all simple weapons, plus the hand crossbow, rapier, sap, shortbow, and short sword, as well as the favored weapon of their deity. They are proficient with light armor and light shields. The cult leader does not gain Weapon Focus as a bonus feat as a warpriest normally would. This replaces the warpriest's weapon and armor proficiencies."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cult Leader ~ Well-Hidden", at_level: 1, description: Some("A cult leader gains a +2 bonus on Disguise and Stealth checks."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cult Leader ~ Sneak Attack", at_level: 3, description: Some("A cult leader gains the sneak attack ability, as the rogue class feature. If he already has sneak attack from another class, the extra damage from the classes that grant sneak attack stack for the purpose of determining the sneak attack's extra damage dice. This extra damage is %1d6.|SneakAttackDice"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cult Leader ~ Enthrall", at_level: 4, description: Some("A cult leader can cast enthrall. Using this ability consumes two uses of his fervor ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cult Leader ~ Hide in Plain Sight", at_level: 12, description: Some("A cult leader can use the Stealth skill even while being observed. As long as he is within 10 feet of an area of dim light, a cult leader can hide himself from view in the open without anything to actually hide behind. He cannot, however, hide in his own shadow."), benefit: None },
            ],
        },
        // Warpriest Archetype ~ Disenchanter -- acg_abilities_class.lst:3932
        ArchetypeSwapEntry {
            key: "Warpriest Archetype ~ Disenchanter",
            subject: "Warpriest",
            archetype_name: "Disenchanter",
            description: Some("While many warpriests focus on threats to the body, the disenchanter focuses on dangers to the mind and the soul. Using the power of his patron, the disenchanter seeks to keep the powers of magic in check."),
            source_page: Some("p.129"),
            prerequisites: Some(&["PRECLASS:1,Warpriest=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Warpriest Archetype ~ Disenchanter],[!PREABILITY:1,CATEGORY=Archetype,TYPE.WarpriestBonusFeat6,TYPE.WarpriestChannelEnergy,TYPE.CF_WarpriestBonusFeat,TYPE.WarpriestBonusFeat]"]),
            replaces: Some(&["WarpriestBonusFeat6", "WarpriestChannelEnergy", "CF_WarpriestBonusFeat", "WarpriestBonusFeat"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Disenchanter ~ Bonus Feats", at_level: 1, description: Some("Whenever a disenchanter gains a bonus feat, he must choose from a special list. He need not meet the prerequisites for these feats. This ability alters bonus feats."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Disenchanter ~ Mystic Interference", at_level: 4, description: Some("A disenchanter can channel a burst of pure abjuration magic to grant protection to himself and all allies with 30 feet. Affected creatures receive a +%1 bonus on saving throws against spells and spell-like abilities for %2 rounds. This is a sacred bonus if the warpriest is good-aligned or able to spontaneously cast cure spells, and a profane bonus if the warpriest is evil-aligned or able to spontaneously cast inflict spells. Using this ability consumes two uses of his fervor ability.|min(5,floor(WarpriestLVL/4))|WarpriestLVL"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Disenchanter ~ Banish Enchantment", at_level: 6, description: Some("A disenchanter learns to focus his mystic interference. As a standard action, he can consume two uses of his fervor ability to target a single creature within 30 feet with a targeted dispel magic."), benefit: None },
            ],
        },
        // Warpriest Archetype ~ Divine Commander -- acg_abilities_class.lst:3933
        ArchetypeSwapEntry {
            key: "Warpriest Archetype ~ Divine Commander",
            subject: "Warpriest",
            archetype_name: "Divine Commander",
            description: Some("Some warpriests are called to lead great armies and face legions of foes. These divine commanders live for war and fight for glory. Their hearts quicken at battle cries, and they charge forth with their deity's symbol held high. These leaders of armies do so to promote the agenda of their faith, and lead armies of devoted followers willing to give their lives for the cause. (Several of the divine commander's abilities reference and interact with the mass combat rules in Pathfinder RPG Ultimate Campaign.)"),
            source_page: Some("p.129"),
            prerequisites: Some(&["PRECLASS:1,Warpriest=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Warpriest Archetype ~ Divine Commander],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_WarpriestBlessings,TYPE.WarpriestBlessings,TYPE.WarpriestBonusFeat3,TYPE.WarpriestBonusFeat6,TYPE.WarpriestBonusFeat12,TYPE.WarpriestBonusFeat15]"]),
            replaces: Some(&["CF_WarpriestBlessings", "WarpriestBlessings", "WarpriestBonusFeat3", "WarpriestBonusFeat6", "WarpriestBonusFeat12", "WarpriestBonusFeat15"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Divine Commander ~ Mount", at_level: 1, description: Some("A divine commander gains the service of a loyal and trusty steed to carry her into battle. This mount functions as a druid's animal companion, using the divine commander's level as her effective druid level. The creature must be one that she is capable of riding and must be suitable as a mount. A Medium divine commander can select a camel or a horse. A Small divine commander can select a pony or wolf, but can also select a boar or a dog if she is at least 4th level. (The GM might approve other animals as suitable mounts.) A divine commander does not take an armor check penalty on Ride checks while riding this mount. The mount is always considered combat trained, and begins play with Light Armor Proficiency as a bonus feat. A divine commander's mount does not gain the share spells special ability. Should a divine commander's mount die, she can find another mount to serve her after 1 week of mourning. This new mount does not gain the link, evasion, devotion, or improved evasion special abilities until the next time the divine commander gains a level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Divine Commander ~ Battle Tactician", at_level: 3, description: Some("A divine commander gains a teamwork feat as a bonus feat. She must meet the prerequisites for this feat. As a standard action, the divine commander can grant this feat to all allies within 30 feet who can see and hear her. Allies retain the use of this bonus feat for %1 rounds. Allies do not need to meet the prerequisites of this bonus feat. The divine commander can use this ability|3+floor((WarpriestLVL-1)/2)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Divine Commander ~ Blessed Mount", at_level: 6, description: Some("A divine commander's mount becomes a creature blessed by his deity. ainst two types of energy damage of the divine commander's choice. If a divine commander's mount dies and the divine commander finds another mount, the new mount becomes a blessed mount the next time the divine commander gains a level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Divine Commander ~ Greater Battle Tactician", at_level: 12, description: Some("The divine commander gains an additional teamwork feat as a bonus feat. She must meet the prerequisites for this feat. The divine commander can grant this feat to her allies using the battle tactician ability. Additionally, using the battle tactician ability is now a swift action."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Divine Commander ~ Bless Army", at_level: 15, description: Some("A divine commander can raise her holy symbol high and give a blessing to the army she is leading. The divine commander's army gains a +1 bonus to the army's OM and DV (Ultimate Campaign 235). This is a sacred bonus if the warpriest is good-aligned or able to spontaneously cast cure spells, and it is a profane bonus if the warpriest is evil-aligned or able to spontaneously cast inflict spells. Using this ability requires the divine commander to expend two uses of her fervor ability. This bonus lasts for 1 battle, and it must be performed during the tactics phase of the battle. If this is performed when a mass combat is not imminent, the fervor is spent without granting any bonus."), benefit: None },
            ],
        },
        // Warpriest Archetype ~ Forgepriest -- acg_abilities_class.lst:3934
        ArchetypeSwapEntry {
            key: "Warpriest Archetype ~ Forgepriest",
            subject: "Warpriest",
            archetype_name: "Forgepriest",
            description: Some("Armorers of exquisite skill, forgepriests take inspiration from their deity to produce the most perfect weapons and armor they can, the better to equip the armies of the faithful."),
            source_page: Some("p.130"),
            prerequisites: Some(&["PRECLASS:1,Warpriest=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Warpriest Archetype ~ Forgepriest],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_WarpriestBlessings,TYPE.WarpriestBlessings,TYPE.WarpriestBonusFeat,TYPE.WarpriestBonusFeat3,TYPE.WarpriestBonusFeat6,TYPE.WarpriestChannelEnergy]"]),
            replaces: Some(&["CF_WarpriestBlessings", "WarpriestBlessings", "WarpriestBonusFeat", "WarpriestBonusFeat3", "WarpriestBonusFeat6", "WarpriestChannelEnergy"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Forgepriest ~ Blessings", at_level: 1, description: Some("A forgepriest selects only one blessing. This alters the blessings class feature."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Forgepriest ~ Smith's Spells", at_level: 1, description: Some("[NOT IMPLEMENTED] A forgepriest adds the following spells to his spell list: 1st-jury rig, shield; 2nd-heat metal, shatter; 3rd-keen edge, quench, versatile weapon; 4th-wreath of blades; 5th-fabricate, major creation; 6th-mage's sword."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Forgepriest ~ Forge Mastery", at_level: 2, description: Some("The forgepriest adds a +%1 bonus equal to half his level to all Craft checks to make metal items, armor, and weapons.|TL/2"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Forgepriest ~ Bonus Feats", at_level: 9, description: Some("A forgepriest can select item creation feats in addition to combat feats when he gains a bonus feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Forgepriest ~ Craft Magic Arms and Armor", at_level: 3, description: Some("A forgepriest gains Craft Magic Arms and Armor as a bonus feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Forgepriest ~ Creator's Bond", at_level: 4, description: Some("When a forgepriest uses his sacred weapon ability with an item he created personally, he can expend two uses of his fervor ability to increase the bonus granted by 1. Once the forgepriest has the sacred armor ability, he can also use this ability in conjunction with that."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Forgepriest ~ Heat of the Forge", at_level: 6, description: Some("A forgepriest gains fire resistance %1.|ForgepriestFireResistance"), benefit: None },
            ],
        },
        // Warpriest Archetype ~ Sacred Fist -- acg_abilities_class.lst:3935
        ArchetypeSwapEntry {
            key: "Warpriest Archetype ~ Sacred Fist",
            subject: "Warpriest",
            archetype_name: "Sacred Fist",
            description: Some("Unlike many warpriests, sacred fists leave behind armor and shield and instead rely on their fists and whatever protection their deity bestows on them."),
            source_page: Some("p.130"),
            prerequisites: Some(&["PRECLASS:1,Warpriest=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Warpriest Archetype ~ Sacred Fist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.WarpriestClassSkills,TYPE.WarpriestProficiency,TYPE.WarpriestSacredWeapon,TYPE.WarpriestFocusWeapon,TYPE.WarpriestBonusFeat3,TYPE.WarpriestBonusFeat6,TYPE.WarpriestBonusFeat9,TYPE.WarpriestBonusFeat12,TYPE.WarpriestBonusFeat18,TYPE.WarpriestSacredArmor]"]),
            replaces: Some(&["WarpriestClassSkills", "WarpriestProficiency", "WarpriestSacredWeapon", "WarpriestFocusWeapon", "WarpriestBonusFeat3", "WarpriestBonusFeat6", "WarpriestBonusFeat9", "WarpriestBonusFeat12", "WarpriestBonusFeat18", "WarpriestSacredArmor"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Sacred Fist ~ Class Skills", at_level: 1, description: Some("The sacred fist's class skills are Acrobatics (Dex), Climb (Str), Craft (Int), Diplomacy (Cha), Escape Artist (Dex), Heal (Wis), Intimidate (Cha), Knowledge (history) (Int), Knowledge (religion) (Int), Perception (Wis), Profession (Wis), Sense Motive (Wis), Ride (Dex), Sense Motive (Wis), Spellcraft (Int), Stealth (Dex), and Swim (Str). These skills replace the warpriest's class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sacred Fist ~ Weapon and Armor Proficiency", at_level: 1, description: Some("Sacred fists are proficient with the club, crossbow (light or heavy), dagger, handaxe, javelin, kama, nunchaku, quarterstaff, sai, shortspear, short sword, shuriken, siangham, sling, and spear. Sacred fists are not proficient with any armor or shields. When wearing armor, using shield, or carrying a medium or heavy load, a sacred fist loses his AC bonus and flurry of blows."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sacred Fist ~ AC Bonus", at_level: 1, description: Some("A deity protects her sacred fist as long as he is unarmored and unencumbered. A sacred fist adds his Wisdom modifier (minimum 0) to his AC and his CMD. In addition, a sacred fist gains a +1 dodge bonus to AC and CMD at 4th level. This bonus increases by 1 for every 4 levels thereafter (to a maximum of +5 at 20th level). These bonuses to AC apply even against touch attacks or when the sacred fist is flat-footed. He loses these bonuses when he is immobilized or helpless, when he wears any armor, when he carries a shield, or when he carries a medium or heavy load. This counts as monk ability of the same name, and the sacred fist's warpriest level stack with monk levels for determining the benefit."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sacred Fist ~ Flurry of Blows", at_level: 1, description: Some("A sacred fist can make a flurry of blows attack as a full-attack action. This ability works like the monk ability of the same name, except the sacred fist's attack bonus from warpriest levels does not count as his warpriest level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sacred Fist ~ Unarmed Strike", at_level: 1, description: Some("A sacred fist gains Improved Unarmed Strike as a bonus feat. He uses his warpriest levels as monk levels for determining the amount of damage dealt with an unarmed strike."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sacred Fist ~ Blessed Fortitude", at_level: 3, description: Some("A sacred fist can avoid even magical and unusual attacks with help from his deity. If he succeeds at a Fortitude saving throw against an attack that has a reduced effect on a successful save, he instead avoids the effect entirely. A helpless sacred fist does not gain the benefit of the blessed fortitude ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sacred Fist ~ Bonus Style Feat", at_level: 6, description: Some("The sacred fist gains a style feat as a bonus feat. The sacred fist must meet the style feat's prerequisites. He uses his warpriest levels as monk levels for the purposes of meeting the feat's prerequisites. At 12th and 18th levels, a sacred fist gains either another style feat or a feat that requires a style feat as a prerequisite."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sacred Fist ~ Ki Pool", at_level: 7, description: Some("The sacred fist gains a ki pool. This functions as the monk class feature, using the sacred fist's level - 3 as his monk level when determining the number of points in his pool and bonuses granted to his unarmed strike. Additionally, the sacred fist can as a swift action spend 1 point from his ki pool to grant himself a +1 insight bonus to his AC for 1 minute. (This is in addition the normal ki ability to gain a dodge bonus to AC.) This insight bonus increases by 1 for every 3 levels above 7th (to a maximum of +5 at 19th level)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sacred Fist ~ Miraculous Fortitude", at_level: 9, description: Some("The sacred fist's blessed fortitude ability improves. He still takes no damage or negative effect when he succeeds at a Fortitude save, but now when he fails a Fortitude saving throw against a spell or effect that deals damage (including ability damage and drain), he takes only half the amount of damage. A helpless sacred fist does not gain the benefit of miraculous fortitude."), benefit: None },
            ],
        },
        // Witch Archetype ~ Hex Channeler -- acg_abilities_class.lst:4016
        ArchetypeSwapEntry {
            key: "Witch Archetype ~ Hex Channeler",
            subject: "Witch",
            archetype_name: "Hex Channeler",
            description: Some("A hex channeler is a witch who devotes herself to either life-healing the wounded and destroying the undead-or death, slaying the living and aiding undead."),
            source_page: Some("p.132"),
            prerequisites: Some(&["PRECLASS:1,Witch=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Witch Archetype ~ Hex Channeler],[!PREABILITY:1,CATEGORY=Archetype,TYPE.WitchHex2,TYPE.CF_WitchHex]"]),
            replaces: Some(&["WitchHex2", "CF_WitchHex"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Hex Channeler ~ Channel Energy", at_level: 2, description: Some("At 2nd level, a hex channeler can call upon her patron to release a wave of energy from herself or her familiar. A good witch channels positive energy (like a good cleric), and an evil witch channels negative energy (like an evil cleric). A witch who is neither good nor evil must choose whether she channels positive or negative energy; once this choice is made, it cannot be reversed. Channeling energy causes a burst that affects all creatures of one type (either undead or living) in a 30-foot radius centered on the witch. The witch can channel energy a number of times per day equal to 3 + her Charisma modifier (minimum 1). This otherwise functions as a cleric using channel energy, except the witch does not require a holy symbol to use this ability. The hex channeler uses her witch level as her cleric level for all other effects dependent upon channel energy (except increasing the amount of damage healed or dealt). The hex channeler can choose whether or not to include herself or her familiar in this effect. This burst heals or deals 1d6 points of damage. Every time the hex channeler is able to learn a new hex (including major or grand hexes, but not hexes gained through the Extra Hex feat), she can instead increase her channel energy amount by 1d6. This ability replaces the hex gained at 2nd level."), benefit: None },
            ],
        },
        // Witch Archetype ~ Mountain Witch -- acg_abilities_class.lst:4017
        ArchetypeSwapEntry {
            key: "Witch Archetype ~ Mountain Witch",
            subject: "Witch",
            archetype_name: "Mountain Witch",
            description: Some("Mountains can be sanctuaries for witches hunted by society. Here they form bonds with the spirits of the lofty reaches."),
            source_page: Some("p.132"),
            prerequisites: Some(&["PRECLASS:1,Witch=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Witch Archetype ~ Mountain Witch],[!PREABILITY:1,CATEGORY=Archetype,TYPE.WitchHex2,TYPE.CF_WitchHex]"]),
            replaces: Some(&["WitchHex2", "CF_WitchHex"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Mountain Witch ~ Spells", at_level: 1, description: Some("A mountain witch replaces some of her patron spells with the following: 2nd-magic stone, 4th-stone callAPG, 6th- meld into stone, 8th-stoneskin, 10th-wall of stone, 12th-stone tell, 14th-statue, 16th-repel metal or stone, 18th-clashing rocksAPG."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mountain Witch ~ Stone Spirit Hex", at_level: 1, description: Some("A mountain witch can select hexes from the shaman's stone spirit (see page 44) as if they were witch hexes. She uses her witch level as her shaman level to determine the effect of the hex, and Intelligence instead of Wisdom to determine its DC. This ability alters hex."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mountain Witch ~ Mountain Beast Empathy", at_level: 2, description: Some("At 2nd level, a mountain witch can influence the attitude of mountain-dwelling animals, as the druid's wild empathy class feature but only on animals native to mountainous environments. The mountain witch uses her witch level as her druid level for this ability. This ability replaces the hex gained at 2nd level."), benefit: None },
            ],
        },        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_has_87_records() {
        assert_eq!(archetype_swap_tables().len(), 87);
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

    /// The finding this table exists to check: does UPsi's own
    /// TYPE/ABILITY disagreement generalize? Confirmed: 34% here vs
    /// UPsi's corrected 13% -- same direction, book-dependent magnitude.
    #[test]
    fn the_type_and_ability_lists_genuinely_disagree() {
        let total_replaces: usize =
            archetype_swap_tables().iter().map(|e| e.replaces.map_or(0, |r| r.len())).sum();
        let total_grants: usize = archetype_swap_tables().iter().map(|e| e.grants.len()).sum();
        assert_eq!(total_replaces, 378, "total TYPE: replaced-slot count across all 87 records");
        assert_eq!(total_grants, 337, "total ABILITY: granted-feature count across all 87 records");
        assert_ne!(total_replaces, total_grants);

        let equal_count_records = archetype_swap_tables()
            .iter()
            .filter(|e| e.replaces.map_or(0, |r| r.len()) == e.grants.len())
            .count();
        assert_eq!(equal_count_records, 30, "of 87 (34%) -- corrected figure, see this module's own doc comment");
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
        assert_eq!(resolved, 333, "333 of 337 grants carry real DESC:/BENEFIT: text -- see this module's own doc comment for the 4 that did not");
    }
}
