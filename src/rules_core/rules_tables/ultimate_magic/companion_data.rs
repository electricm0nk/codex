//! ultimate_magic companion tables, transcribed verbatim from the book's own
//! PCGen `.lst` rows.
//!
//! GENERATED FILE -- do not hand-edit. Regenerate with
//! `python3 scripts/transcribe_companion_tables.py ultimate_magic`, whose unit set is
//! `docs/work-inventory.json`'s own units for this book rather than a raw
//! line count over the `.lst`.
//!
//! Sources, with the file AND line each record was read from carried per row:
//!   * `um_races_companion.lst` -- 10 companion creature rows
//!   * `um_abilities_companion.lst` -- 22 companion ability rows
//!
//! NOT transcribed -- ability rows no creature row of this book owns, so
//! nothing could ever reach them on screen. Dropped rather than emitted
//! unreachable (`decisions.md §50`, adopted from the monster lane; §56.1).
//! These rows keep their `not-ingested` status in
//! `docs/work-inventory.json`, which is where the shortfall is counted; they
//! are NOT a `reach_gate` `OPEN_FINDINGS` entry, because that list is keyed by
//! FAMILY and this book's `companions` family does reach a player:
//!   * `Black Blade Arcane Pool`
//!   * `Black Blade Strike`
//!   * `Black Blade ~ Alertness`
//!   * `Black Blade ~ Ego`
//!   * `Black Blade ~ Energy Attunement`
//!   * `Black Blade ~ Enhancement Bonus`
//!   * `Black Blade ~ Life Drinker`
//!   * `Black Blade ~ Senses`
//!   * `Black Blade ~ Spell Defense`
//!   * `Black Blade ~ Telepathy`
//!   * `Black Blade ~ Teleport Blade`
//!   * `Black Blade ~ Transfer Arcana`
//!   * `Black Blade ~ Unbreakable`
//!   * `Companion Stat ~ Mindless to 1 INT`
//!   * `Evolution ~ BM Acid Splash 1`
//!   * `Evolution ~ BM Acid Splash 3`
//!   * `Evolution ~ BM Dancing Lights 1`
//!   * `Evolution ~ BM Dancing Lights 3`
//!   * `Evolution ~ BM Daze 1`
//!   * `Evolution ~ BM Daze 3`
//!   * `Evolution ~ BM Detect Magic 1`
//!   * `Evolution ~ BM Detect Magic 3`
//!   * `Evolution ~ BM Flare 1`
//!   * `Evolution ~ BM Flare 3`
//!   * `Evolution ~ BM Ghost Sound 1`
//!   * `Evolution ~ BM Ghost Sound 3`
//!   * `Evolution ~ BM Light 1`
//!   * `Evolution ~ BM Light 3`
//!   * `Evolution ~ BM Mage Hand 1`
//!   * `Evolution ~ BM Mage Hand 3`
//!   * `Evolution ~ BM Ray of Frost 1`
//!   * `Evolution ~ BM Ray of Frost 3`
//!   * `Evolution ~ BM Stabilize 1`
//!   * `Evolution ~ BM Stabilize 3`
//!   * `Evolution ~ BM Touch of Fatigue 1`
//!   * `Evolution ~ BM Touch of Fatigue 3`
//!   * `Evolution ~ Basic Magic`
//!   * `Evolution ~ Channel Resistance`
//!   * `Evolution ~ Dimension Door`
//!   * `Evolution ~ Head`
//!   * `Evolution ~ Hooved Feet`
//!   * `Evolution ~ Hooved Hands`
//!   * `Evolution ~ Improved Channel Resistance`
//!   * `Evolution ~ Incorporeal Form`
//!   * `Evolution ~ Keen Scent`
//!   * `Evolution ~ Lifesense`
//!   * `Evolution ~ Low-Light`
//!   * `Evolution ~ Magic Upgrade`
//!   * `Evolution ~ Major Acid Arrow 1`
//!   * `Evolution ~ Major Acid Arrow 3`
//!   * `Evolution ~ Major Cure Moderate Wounds 1`
//!   * `Evolution ~ Major Cure Moderate Wounds 3`
//!   * `Evolution ~ Major Darkness 1`
//!   * `Evolution ~ Major Darkness 3`
//!   * `Evolution ~ Major Daze Monster 1`
//!   * `Evolution ~ Major Daze Monster 3`
//!   * `Evolution ~ Major Glide 1`
//!   * `Evolution ~ Major Glide 3`
//!   * `Evolution ~ Major Invisibility 1`
//!   * `Evolution ~ Major Invisibility 3`
//!   * `Evolution ~ Major Lesser Restoration 1`
//!   * `Evolution ~ Major Lesser Restoration 3`
//!   * `Evolution ~ Major Levitate 1`
//!   * `Evolution ~ Major Levitate 3`
//!   * `Evolution ~ Major Magic`
//!   * `Evolution ~ Major Minor Image 1`
//!   * `Evolution ~ Major Minor Image 3`
//!   * `Evolution ~ Major Scorching Ray 1`
//!   * `Evolution ~ Major Scorching Ray 3`
//!   * `Evolution ~ Major See Invisibility 1`
//!   * `Evolution ~ Major See Invisibility 3`
//!   * `Evolution ~ Major Spider Climb 1`
//!   * `Evolution ~ Major Spider Climb 3`
//!   * `Evolution ~ Minor Burning Hands 1`
//!   * `Evolution ~ Minor Burning Hands 3`
//!   * `Evolution ~ Minor Comprehend Languages 1`
//!   * `Evolution ~ Minor Comprehend Languages 3`
//!   * `Evolution ~ Minor Cure Light Wounds 1`
//!   * `Evolution ~ Minor Cure Light Wounds 3`
//!   * `Evolution ~ Minor Detect Chaos 1`
//!   * `Evolution ~ Minor Detect Chaos 3`
//!   * `Evolution ~ Minor Detect Evil 1`
//!   * `Evolution ~ Minor Detect Evil 3`
//!   * `Evolution ~ Minor Detect Good 1`
//!   * `Evolution ~ Minor Detect Good 3`
//!   * `Evolution ~ Minor Detect Law 1`
//!   * `Evolution ~ Minor Detect Law 3`
//!   * `Evolution ~ Minor Magic`
//!   * `Evolution ~ Minor Magic Missile 1`
//!   * `Evolution ~ Minor Magic Missile 3`
//!   * `Evolution ~ Minor Obscuring Mist 1`
//!   * `Evolution ~ Minor Obscuring Mist 3`
//!   * `Evolution ~ Minor Silent Image 1`
//!   * `Evolution ~ Minor Silent Image 3`
//!   * `Evolution ~ Minor Vanish 1`
//!   * `Evolution ~ Minor Vanish 3`
//!   * `Evolution ~ Minor Ventriloquism 1`
//!   * `Evolution ~ Minor Ventriloquism 3`
//!   * `Evolution ~ No Breath`
//!   * `Evolution ~ See in Darkness`
//!   * `Evolution ~ Ultimate Arcane Sight 1`
//!   * `Evolution ~ Ultimate Create Food and Water 1`
//!   * `Evolution ~ Ultimate Cure Serious Wounds 1`
//!   * `Evolution ~ Ultimate Daylight 1`
//!   * `Evolution ~ Ultimate Fireball 1`
//!   * `Evolution ~ Ultimate Fly 1`
//!   * `Evolution ~ Ultimate Gaseous Form 1`
//!   * `Evolution ~ Ultimate Lightning Bolt 1`
//!   * `Evolution ~ Ultimate Magic`
//!   * `Evolution ~ Ultimate Major Image 1`
//!   * `Evolution ~ Ultimate Stinking Cloud 1`
//!   * `Evolution ~ Ultimate Tongues 1`
//!   * `Evolution ~ Ultimate Water Breathing 1`
//!   * `Evolution ~ Undead Appearance`
//!   * `Evolution ~ Unnatural Aura`
//!   * `Giant Spider Vermin Companion ~ Poison`
//!   * `Temp Evolution ~ Basic Magic`
//!   * `Temp Evolution ~ Channel Resistance`
//!   * `Temp Evolution ~ Dimension Door`
//!   * `Temp Evolution ~ Head`
//!   * `Temp Evolution ~ Hooved Feet`
//!   * `Temp Evolution ~ Hooved Hands`
//!   * `Temp Evolution ~ Improved Channel Resistance`
//!   * `Temp Evolution ~ Incorporeal Form`
//!   * `Temp Evolution ~ Keen Scent`
//!   * `Temp Evolution ~ Lifesense`
//!   * `Temp Evolution ~ Low-Light`
//!   * `Temp Evolution ~ Magic Upgrade`
//!   * `Temp Evolution ~ Major Magic`
//!   * `Temp Evolution ~ Minor Magic`
//!   * `Temp Evolution ~ No Breath`
//!   * `Temp Evolution ~ See in Darkness`
//!   * `Temp Evolution ~ Ultimate Magic`
//!   * `Temp Evolution ~ Undead Appearance`
//!   * `Temp Evolution ~ Unnatural Aura`
//!
//! NOT transcribed -- `*_classes_companion.lst` CLASS rows (`decisions.md
//! §65.1`). A PCGen monster class is the hit-dice progression a creature
//! row's `MONSTERCLASS:` token names -- it states no `SIZE:`, no `MOVE:` and
//! no natural attacks, so every field this chassis models transcribes empty.
//! Modelling it is a new record type (a level progression table), not a wider
//! predicate on this one. Left honestly `not-ingested`; the creature rows that
//! name them ship, and carry the token verbatim:
//!   * `1`
//!   * `Black Blade`
//!   * `Vermin Companion`

use crate::rules_core::rules_tables::companion_chassis::{CompanionAbilityDelivery, CompanionAbilityFacet, CompanionAbilityRecord, CompanionDescriptionVariant, CompanionRecord, NaturalAttack, Speed, StatAdjustment};

/// Every ultimate_magic companion creature (10 rows).
pub(super) static COMPANIONS: &[CompanionRecord] = &[
    CompanionRecord {
        key: "Companion (Ant (Giant))",
        name: "Companion (Ant (Giant))",
        size: Some("S"),
        speeds: &[Speed { mode: "Walk", feet: 30 }, Speed { mode: "Climb", feet: 20 }],
        reach_feet: Some(5),
        race_type: Some("Companion"),
        race_subtype: None,
        monster_class: Some("Companion:2"),
        type_segments: &[],
        natural_attacks: &[NaturalAttack { name: "Bite", damage_dice: Some("1d4") }],
        stat_adjustments: &[StatAdjustment { ability: "DEX", amount: 2 }, StatAdjustment { ability: "CON", amount: 4 }, StatAdjustment { ability: "INT", amount: -10 }, StatAdjustment { ability: "WIS", amount: 2 }],
        natural_armor: Some(2),
        source_page: Some("p.36"),
        ability_keys: &["Companion Advancement ~ Ant (Giant)", "Giant Ant Companion ~ Poison"],
        external_ability_refs: &["Grab"],
        source_file: "um_races_companion.lst",
        source_line: 9,
    },
    CompanionRecord {
        key: "Companion (Beetle (Giant))",
        name: "Companion (Beetle (Giant))",
        size: Some("S"),
        speeds: &[Speed { mode: "Walk", feet: 20 }, Speed { mode: "Fly", feet: 20 }],
        reach_feet: Some(5),
        race_type: Some("Companion"),
        race_subtype: None,
        monster_class: Some("Companion:2"),
        type_segments: &[],
        natural_attacks: &[NaturalAttack { name: "Bite", damage_dice: Some("1d6") }],
        stat_adjustments: &[StatAdjustment { ability: "STR", amount: 2 }, StatAdjustment { ability: "DEX", amount: 2 }, StatAdjustment { ability: "CON", amount: 2 }, StatAdjustment { ability: "INT", amount: -10 }, StatAdjustment { ability: "CHA", amount: -6 }],
        natural_armor: Some(6),
        source_page: Some("p.36"),
        ability_keys: &["Companion Advancement ~ Beetle (Giant)"],
        external_ability_refs: &["Flight Maneuverability"],
        source_file: "um_races_companion.lst",
        source_line: 10,
    },
    CompanionRecord {
        key: "Companion (Centipede (Giant))",
        name: "Companion (Centipede (Giant))",
        size: Some("S"),
        speeds: &[Speed { mode: "Walk", feet: 20 }, Speed { mode: "Climb", feet: 20 }],
        reach_feet: Some(5),
        race_type: Some("Companion"),
        race_subtype: None,
        monster_class: Some("Companion:2"),
        type_segments: &[],
        natural_attacks: &[NaturalAttack { name: "Bite", damage_dice: Some("1d4") }],
        stat_adjustments: &[StatAdjustment { ability: "STR", amount: -2 }, StatAdjustment { ability: "DEX", amount: 6 }, StatAdjustment { ability: "INT", amount: -10 }, StatAdjustment { ability: "CHA", amount: -8 }],
        natural_armor: Some(2),
        source_page: Some("p.36"),
        ability_keys: &["Giant Centipede Vermin Companion ~ Poison", "Companion Advancement ~ Centipede (Giant)"],
        external_ability_refs: &["Can't Be Tripped"],
        source_file: "um_races_companion.lst",
        source_line: 11,
    },
    CompanionRecord {
        key: "Companion (Crab (Giant))",
        name: "Companion (Crab (Giant))",
        size: Some("S"),
        speeds: &[Speed { mode: "Walk", feet: 30 }, Speed { mode: "Swim", feet: 20 }],
        reach_feet: Some(5),
        race_type: Some("Companion"),
        race_subtype: Some("Aquatic"),
        monster_class: Some("Companion:2"),
        type_segments: &[],
        natural_attacks: &[NaturalAttack { name: "Claw", damage_dice: Some("1d3") }],
        stat_adjustments: &[StatAdjustment { ability: "STR", amount: 2 }, StatAdjustment { ability: "CON", amount: 2 }, StatAdjustment { ability: "DEX", amount: 4 }, StatAdjustment { ability: "INT", amount: -10 }, StatAdjustment { ability: "CHA", amount: -6 }],
        natural_armor: Some(5),
        source_page: Some("p.36"),
        ability_keys: &["Giant Crab Companion ~ Water Dependency", "Companion Advancement ~ Crab (Giant)"],
        external_ability_refs: &["Constrict", "Grab"],
        source_file: "um_races_companion.lst",
        source_line: 12,
    },
    CompanionRecord {
        key: "Companion (Leech (Giant))",
        name: "Companion (Leech (Giant))",
        size: Some("S"),
        speeds: &[Speed { mode: "Walk", feet: 5 }, Speed { mode: "Swim", feet: 20 }],
        reach_feet: Some(5),
        race_type: Some("Companion"),
        race_subtype: None,
        monster_class: Some("Companion:2"),
        type_segments: &[],
        natural_attacks: &[NaturalAttack { name: "Bite", damage_dice: Some("1d4") }, NaturalAttack { name: "Giant Leech Blindsense", damage_dice: None }],
        stat_adjustments: &[StatAdjustment { ability: "STR", amount: -2 }, StatAdjustment { ability: "CON", amount: 2 }, StatAdjustment { ability: "DEX", amount: 4 }, StatAdjustment { ability: "INT", amount: -10 }, StatAdjustment { ability: "CHA", amount: -9 }],
        natural_armor: Some(0),
        source_page: Some("p.36"),
        ability_keys: &["Companion ~ Susceptible to Salt", "Giant Leech Companion ~ Blood Drain", "Companion Advancement ~ Leech (Giant)"],
        external_ability_refs: &["Amphibious", "Universal Monster Rule ~ Attach", "Can't Be Tripped"],
        source_file: "um_races_companion.lst",
        source_line: 13,
    },
    CompanionRecord {
        key: "Companion (Mantis (Giant))",
        name: "Companion (Mantis (Giant))",
        size: Some("M"),
        speeds: &[Speed { mode: "Walk", feet: 30 }, Speed { mode: "Climb", feet: 30 }, Speed { mode: "Fly", feet: 40 }],
        reach_feet: Some(5),
        race_type: Some("Companion"),
        race_subtype: None,
        monster_class: Some("Companion:2"),
        type_segments: &[],
        natural_attacks: &[NaturalAttack { name: "Claw", damage_dice: Some("1d4") }],
        stat_adjustments: &[StatAdjustment { ability: "DEX", amount: 4 }, StatAdjustment { ability: "INT", amount: -10 }, StatAdjustment { ability: "WIS", amount: 2 }, StatAdjustment { ability: "CHA", amount: -4 }],
        natural_armor: Some(3),
        source_page: Some("p.37"),
        ability_keys: &["Giant Mantis Companion ~ Lunge", "Companion Advancement ~ Mantis (Giant)", "Giant Mantis Companion ~ Mandibles", "Giant Mantis Companion ~ Sudden Strike"],
        external_ability_refs: &["Flight Maneuverability", "Grab"],
        source_file: "um_races_companion.lst",
        source_line: 14,
    },
    CompanionRecord {
        key: "Companion (Scorpion (Giant))",
        name: "Companion (Scorpion (Giant))",
        size: Some("M"),
        speeds: &[Speed { mode: "Walk", feet: 40 }],
        reach_feet: Some(5),
        race_type: Some("Companion"),
        race_subtype: None,
        monster_class: Some("Companion:2"),
        type_segments: &[],
        natural_attacks: &[NaturalAttack { name: "Claw", damage_dice: Some("1d4") }, NaturalAttack { name: "Sting", damage_dice: Some("1d4") }],
        stat_adjustments: &[StatAdjustment { ability: "DEX", amount: 2 }, StatAdjustment { ability: "CON", amount: 2 }, StatAdjustment { ability: "INT", amount: -10 }, StatAdjustment { ability: "CHA", amount: -8 }],
        natural_armor: Some(1),
        source_page: Some("p.37"),
        ability_keys: &["Giant Scorpion Companion ~ Poison", "Companion Advancement ~ Scorpion (Giant)"],
        external_ability_refs: &["Grab"],
        source_file: "um_races_companion.lst",
        source_line: 15,
    },
    CompanionRecord {
        key: "Companion (Slug (Giant))",
        name: "Companion (Slug (Giant))",
        size: Some("M"),
        speeds: &[Speed { mode: "Walk", feet: 20 }],
        reach_feet: Some(5),
        race_type: Some("Companion"),
        race_subtype: None,
        monster_class: Some("Companion:2"),
        type_segments: &[],
        natural_attacks: &[NaturalAttack { name: "Tongue", damage_dice: Some("1d4") }, NaturalAttack { name: "Giant Slug Blindsense", damage_dice: None }],
        stat_adjustments: &[StatAdjustment { ability: "STR", amount: 2 }, StatAdjustment { ability: "CON", amount: 2 }, StatAdjustment { ability: "DEX", amount: -2 }, StatAdjustment { ability: "INT", amount: -10 }, StatAdjustment { ability: "CHA", amount: -9 }],
        natural_armor: Some(4),
        source_page: Some("p.37"),
        ability_keys: &["Companion ~ Susceptible to Salt", "Giant Slug Companion ~ Acid", "Giant Slug Companion ~ Spit Acid", "Companion Advancement ~ Slug (Giant)"],
        external_ability_refs: &["Can't Be Tripped"],
        source_file: "um_races_companion.lst",
        source_line: 16,
    },
    CompanionRecord {
        key: "Companion (Spider (Giant))",
        name: "Companion (Spider (Giant))",
        size: Some("S"),
        speeds: &[Speed { mode: "Walk", feet: 30 }, Speed { mode: "Climb", feet: 30 }],
        reach_feet: Some(5),
        race_type: Some("Companion"),
        race_subtype: None,
        monster_class: Some("Companion:2"),
        type_segments: &[],
        natural_attacks: &[NaturalAttack { name: "Bite", damage_dice: Some("1d4") }],
        stat_adjustments: &[StatAdjustment { ability: "STR", amount: -4 }, StatAdjustment { ability: "DEX", amount: 6 }, StatAdjustment { ability: "INT", amount: -10 }, StatAdjustment { ability: "CHA", amount: -8 }],
        natural_armor: Some(0),
        source_page: Some("p.37"),
        ability_keys: &["Companion Advancement ~ Spider (Giant)"],
        external_ability_refs: &["Giant Spider Companion ~ Poison"],
        source_file: "um_races_companion.lst",
        source_line: 17,
    },
    CompanionRecord {
        key: "Companion (Wasp (Giant))",
        name: "Companion (Wasp (Giant))",
        size: Some("M"),
        speeds: &[Speed { mode: "Walk", feet: 20 }, Speed { mode: "Fly", feet: 60 }],
        reach_feet: Some(5),
        race_type: Some("Companion"),
        race_subtype: None,
        monster_class: Some("Companion:2"),
        type_segments: &[],
        natural_attacks: &[NaturalAttack { name: "Sting", damage_dice: Some("1d6") }],
        stat_adjustments: &[StatAdjustment { ability: "DEX", amount: 4 }, StatAdjustment { ability: "INT", amount: -10 }, StatAdjustment { ability: "WIS", amount: 2 }, StatAdjustment { ability: "CHA", amount: -6 }],
        natural_armor: Some(2),
        source_page: Some("p.37"),
        ability_keys: &["Giant Wasp Companion ~ Poison", "Companion Advancement ~ Wasp (Giant)"],
        external_ability_refs: &["Flight Maneuverability"],
        source_file: "um_races_companion.lst",
        source_line: 18,
    },
];

/// Every ultimate_magic companion ability record (22 rows).
pub(super) static COMPANION_ABILITIES: &[CompanionAbilityRecord] = &[
    CompanionAbilityRecord {
        key: "Companion ~ Susceptible to Salt",
        name: "Susceptible to Salt",
        facet: Some(CompanionAbilityFacet::SpecialQuality),
        delivery: Some(CompanionAbilityDelivery::Extraordinary),
        type_segments: &["SpecialQuality", "Extraordinary", "Weakness"],
        description: Some("A handful of salt burns as if it were a flask of acid, causing 1d6 points of damage per use."),
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: Some("p.187"),
        owners: &["Companion (Leech (Giant))", "Companion (Slug (Giant))"],
        source_file: "um_abilities_companion.lst",
        source_line: 21,
    },
    CompanionAbilityRecord {
        key: "Giant Ant Companion ~ Poison",
        name: "Poison",
        facet: Some(CompanionAbilityFacet::SpecialAttack),
        delivery: Some(CompanionAbilityDelivery::Extraordinary),
        type_segments: &["SpecialAttack", "Extraordinary"],
        description: Some("Frequency 1 round [4], effect 1 Str damage, cure 1 save, DC %1"),
        description_variables: &["GiantAntCompanionPoisonDC"],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: Some("p.36"),
        owners: &["Companion (Ant (Giant))"],
        source_file: "um_abilities_companion.lst",
        source_line: 22,
    },
    CompanionAbilityRecord {
        key: "Giant Centipede Vermin Companion ~ Poison",
        name: "Poison",
        facet: Some(CompanionAbilityFacet::SpecialAttack),
        delivery: Some(CompanionAbilityDelivery::Extraordinary),
        type_segments: &["SpecialAttack", "Extraordinary"],
        description: Some("Frequency 1 round [6], effect 1 Dex damage, cure 1 save, DC %1"),
        description_variables: &["GiantCentipedeCompanionPoisonDC"],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: Some("p.36"),
        owners: &["Companion (Centipede (Giant))"],
        source_file: "um_abilities_companion.lst",
        source_line: 25,
    },
    CompanionAbilityRecord {
        key: "Giant Crab Companion ~ Water Dependency",
        name: "Water Dependency",
        facet: Some(CompanionAbilityFacet::SpecialQuality),
        delivery: Some(CompanionAbilityDelivery::Extraordinary),
        type_segments: &["SpecialQuality", "Extraordinary", "Weakness"],
        description: Some("Survive out of water for %1 hours, after which it begins to suffocate as if it were drowning."),
        description_variables: &["CON"],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: Some("p.36"),
        owners: &["Companion (Crab (Giant))"],
        source_file: "um_abilities_companion.lst",
        source_line: 26,
    },
    CompanionAbilityRecord {
        key: "Giant Leech Companion ~ Blood Drain",
        name: "Blood Drain",
        facet: Some(CompanionAbilityFacet::SpecialAttack),
        delivery: Some(CompanionAbilityDelivery::Extraordinary),
        type_segments: &["SpecialAttack", "Extraordinary"],
        description: Some("A giant leech drains blood at the end of each turn it is attached"),
        description_variables: &[],
        description_variants: &[CompanionDescriptionVariant { text: "A giant leech drains blood at the end of each turn it is attached", variables: &[], conditions: &[] }, CompanionDescriptionVariant { text: " inflicting 1 point of Strength damage.", variables: &[], conditions: &["!PREABILITY:1,CATEGORY=Special Ability,Companion Advancement (Leech (Giant))"] }, CompanionDescriptionVariant { text: " inflicting 1 point of Strength  and Constitution damage.", variables: &[], conditions: &["PREABILITY:1,CATEGORY=Special Ability,Companion Advancement (Leech (Giant))"] }],
        stat_adjustments: &[],
        source_page: Some("p.36"),
        owners: &["Companion (Leech (Giant))"],
        source_file: "um_abilities_companion.lst",
        source_line: 28,
    },
    CompanionAbilityRecord {
        key: "Giant Mantis Companion ~ Lunge",
        name: "Lunge",
        facet: Some(CompanionAbilityFacet::SpecialAttack),
        delivery: Some(CompanionAbilityDelivery::Extraordinary),
        type_segments: &["SpecialAttack", "Extraordinary"],
        description: Some("A giant mantis's limbs are capable of reaching much farther than normal for a creature of its size. As a full-attack action, it can make a single attack with its claws at double its normal reach. When a giant mantis attacks with a claw in this manner, it gains a +4 bonus on its attack roll. A giant mantis cannot make attacks of opportunity with its lunge."),
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: Some("p.200"),
        owners: &["Companion (Mantis (Giant))"],
        source_file: "um_abilities_companion.lst",
        source_line: 30,
    },
    CompanionAbilityRecord {
        key: "Giant Mantis Companion ~ Mandibles",
        name: "Mandibles",
        facet: Some(CompanionAbilityFacet::SpecialAttack),
        delivery: Some(CompanionAbilityDelivery::Extraordinary),
        type_segments: &["SpecialAttack", "Extraordinary"],
        description: Some("1d6 secondary attack against grabbed target"),
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: Some("p.37"),
        owners: &["Companion (Mantis (Giant))"],
        source_file: "um_abilities_companion.lst",
        source_line: 31,
    },
    CompanionAbilityRecord {
        key: "Giant Mantis Companion ~ Sudden Strike",
        name: "Sudden Strike",
        facet: Some(CompanionAbilityFacet::SpecialAttack),
        delivery: Some(CompanionAbilityDelivery::Extraordinary),
        type_segments: &["SpecialAttack", "Extraordinary"],
        description: Some("May take a full attack in the surprise round"),
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: Some("p.37"),
        owners: &["Companion (Mantis (Giant))"],
        source_file: "um_abilities_companion.lst",
        source_line: 32,
    },
    CompanionAbilityRecord {
        key: "Giant Scorpion Companion ~ Poison",
        name: "Poison",
        facet: Some(CompanionAbilityFacet::SpecialAttack),
        delivery: Some(CompanionAbilityDelivery::Extraordinary),
        type_segments: &["SpecialAttack", "Extraordinary"],
        description: Some("Frequency 1 round [6]"),
        description_variables: &[],
        description_variants: &[CompanionDescriptionVariant { text: "Frequency 1 round [6]", variables: &[], conditions: &[] }, CompanionDescriptionVariant { text: " effect 1 Str damage, cure 1 save, DC %1", variables: &["GiantScorpionCompanionPoisonDC"], conditions: &["!PREABILITY:1,CATEGORY=Special Ability,Companion Advancement (Scorpion (Giant))"] }, CompanionDescriptionVariant { text: " effect 1d2 Str damage, cure 1 save, DC %1", variables: &["GiantScorpionCompanionPoisonDC"], conditions: &["PREABILITY:1,CATEGORY=Special Ability,Companion Advancement (Scorpion (Giant))"] }],
        stat_adjustments: &[],
        source_page: Some("p.37"),
        owners: &["Companion (Scorpion (Giant))"],
        source_file: "um_abilities_companion.lst",
        source_line: 33,
    },
    CompanionAbilityRecord {
        key: "Giant Slug Companion ~ Acid",
        name: "Acid",
        facet: Some(CompanionAbilityFacet::SpecialAttack),
        delivery: Some(CompanionAbilityDelivery::Extraordinary),
        type_segments: &["SpecialAttack", "Extraordinary"],
        description: None,
        description_variables: &[],
        description_variants: &[CompanionDescriptionVariant { text: "A giant slug tongue attack does an extra 1 point of acid damage.", variables: &[], conditions: &["!PREABILITY:1,CATEGORY=Special Ability,Companion Advancement (Slug (Giant))"] }, CompanionDescriptionVariant { text: "A giant slug tongue attack does an extra 1d2 points of acid damage.", variables: &[], conditions: &["PREABILITY:1,CATEGORY=Special Ability,Companion Advancement (Slug (Giant))"] }],
        stat_adjustments: &[],
        source_page: Some("p.37"),
        owners: &["Companion (Slug (Giant))"],
        source_file: "um_abilities_companion.lst",
        source_line: 35,
    },
    CompanionAbilityRecord {
        key: "Giant Slug Companion ~ Spit Acid",
        name: "Spit Acid",
        facet: Some(CompanionAbilityFacet::SpecialAttack),
        delivery: Some(CompanionAbilityDelivery::Extraordinary),
        type_segments: &["SpecialAttack", "Extraordinary"],
        description: Some("A giant slug can spit acid at an opponent within 30 feet (no range increment). With a successful ranged touch attack, the target takes 1d%1 points of acid damage (no save)."),
        description_variables: &["SpitAcidDamageDieSize"],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: Some("p.37"),
        owners: &["Companion (Slug (Giant))"],
        source_file: "um_abilities_companion.lst",
        source_line: 36,
    },
    CompanionAbilityRecord {
        key: "Giant Wasp Companion ~ Poison",
        name: "Poison",
        facet: Some(CompanionAbilityFacet::SpecialAttack),
        delivery: Some(CompanionAbilityDelivery::Extraordinary),
        type_segments: &["SpecialAttack", "Extraordinary"],
        description: Some("Frequency 1 round [6], effect 1 Dex damage, cure 1 save, DC %1"),
        description_variables: &["GiantWaspCompanionPoisonDC"],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: Some("p.37"),
        owners: &["Companion (Wasp (Giant))"],
        source_file: "um_abilities_companion.lst",
        source_line: 40,
    },
    CompanionAbilityRecord {
        key: "Companion Advancement ~ Ant (Giant)",
        name: "Companion Advancement (Ant (Giant))",
        facet: Some(CompanionAbilityFacet::CompanionAdvancement),
        delivery: None,
        type_segments: &["CompanionAdvancement"],
        description: None,
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: None,
        owners: &["Companion (Ant (Giant))"],
        source_file: "um_abilities_companion.lst",
        source_line: 44,
    },
    CompanionAbilityRecord {
        key: "Companion Advancement ~ Beetle (Giant)",
        name: "Companion Advancement (Beetle (Giant))",
        facet: Some(CompanionAbilityFacet::CompanionAdvancement),
        delivery: None,
        type_segments: &["CompanionAdvancement"],
        description: None,
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: None,
        owners: &["Companion (Beetle (Giant))"],
        source_file: "um_abilities_companion.lst",
        source_line: 45,
    },
    CompanionAbilityRecord {
        key: "Companion Advancement ~ Centipede (Giant)",
        name: "Companion Advancement (Centipede (Giant))",
        facet: Some(CompanionAbilityFacet::CompanionAdvancement),
        delivery: None,
        type_segments: &["CompanionAdvancement"],
        description: None,
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: None,
        owners: &["Companion (Centipede (Giant))"],
        source_file: "um_abilities_companion.lst",
        source_line: 46,
    },
    CompanionAbilityRecord {
        key: "Companion Advancement ~ Crab (Giant)",
        name: "Companion Advancement (Crab (Giant))",
        facet: Some(CompanionAbilityFacet::CompanionAdvancement),
        delivery: None,
        type_segments: &["CompanionAdvancement"],
        description: None,
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[StatAdjustment { ability: "STR", amount: -2 }],
        source_page: None,
        owners: &["Companion (Crab (Giant))"],
        source_file: "um_abilities_companion.lst",
        source_line: 48,
    },
    CompanionAbilityRecord {
        key: "Companion Advancement ~ Leech (Giant)",
        name: "Companion Advancement (Leech (Giant))",
        facet: Some(CompanionAbilityFacet::CompanionAdvancement),
        delivery: None,
        type_segments: &["CompanionAdvancement"],
        description: None,
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[StatAdjustment { ability: "STR", amount: -2 }],
        source_page: None,
        owners: &["Companion (Leech (Giant))"],
        source_file: "um_abilities_companion.lst",
        source_line: 50,
    },
    CompanionAbilityRecord {
        key: "Companion Advancement ~ Mantis (Giant)",
        name: "Companion Advancement (Mantis (Giant))",
        facet: Some(CompanionAbilityFacet::CompanionAdvancement),
        delivery: None,
        type_segments: &["CompanionAdvancement"],
        description: None,
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: None,
        owners: &["Companion (Mantis (Giant))"],
        source_file: "um_abilities_companion.lst",
        source_line: 51,
    },
    CompanionAbilityRecord {
        key: "Companion Advancement ~ Scorpion (Giant)",
        name: "Companion Advancement (Scorpion (Giant))",
        facet: Some(CompanionAbilityFacet::CompanionAdvancement),
        delivery: None,
        type_segments: &["CompanionAdvancement"],
        description: None,
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: None,
        owners: &["Companion (Scorpion (Giant))"],
        source_file: "um_abilities_companion.lst",
        source_line: 52,
    },
    CompanionAbilityRecord {
        key: "Companion Advancement ~ Slug (Giant)",
        name: "Companion Advancement (Slug (Giant))",
        facet: Some(CompanionAbilityFacet::CompanionAdvancement),
        delivery: None,
        type_segments: &["CompanionAdvancement"],
        description: None,
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[StatAdjustment { ability: "STR", amount: -6 }, StatAdjustment { ability: "CON", amount: -2 }],
        source_page: None,
        owners: &["Companion (Slug (Giant))"],
        source_file: "um_abilities_companion.lst",
        source_line: 54,
    },
    CompanionAbilityRecord {
        key: "Companion Advancement ~ Spider (Giant)",
        name: "Companion Advancement (Spider (Giant))",
        facet: Some(CompanionAbilityFacet::CompanionAdvancement),
        delivery: None,
        type_segments: &["CompanionAdvancement"],
        description: None,
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: None,
        owners: &["Companion (Spider (Giant))"],
        source_file: "um_abilities_companion.lst",
        source_line: 55,
    },
    CompanionAbilityRecord {
        key: "Companion Advancement ~ Wasp (Giant)",
        name: "Companion Advancement (Wasp (Giant))",
        facet: Some(CompanionAbilityFacet::CompanionAdvancement),
        delivery: None,
        type_segments: &["CompanionAdvancement"],
        description: None,
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: None,
        owners: &["Companion (Wasp (Giant))"],
        source_file: "um_abilities_companion.lst",
        source_line: 56,
    },
];
