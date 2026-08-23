//! bestiary_6 monster + monster-ability tables, transcribed verbatim
//! from the book's own PCGen `.lst` rows.
//!
//! GENERATED FILE -- do not hand-edit. Regenerate with
//! `python3 scripts/transcribe_monster_tables.py bestiary_6`, whose unit set is
//! `docs/work-inventory.json`'s own units for this book rather than a raw
//! line count over the `.lst` (which counts `.MOD`/`.COPY` overlays the
//! inventory correctly excludes).
//!
//! Sources, with the line each record was read from carried per row:
//!   * `b6_abilities_race.lst` -- 13 monster-ability rows
//!   * `ce_abilities_race.lst` -- 3 monster-ability rows
//!
//! 16 further ability row(s) in this book are ORPHANS -- no monster
//! row here claims them, so they SHIP with `owners: &[]` rather than being
//! dropped (`decisions.md §20`: an un-ingested row's shape cannot be measured,
//! and Gate 1's DoD needs every unit's shape measured). `list_monster_catalog`
//! only ever walks a monster's OWN `ability_keys`, so an owner-less record here
//! reaches no screen -- reachability is NOT claimed for these, and each key is
//! pinned as a named, provable non-reach in `reach_gate.rs::
//! UNREACHED_RECORD_FINDINGS`, never silently assumed reachable:
//!   * `b6_abilities_race.lst:5`
//!   * `b6_abilities_race.lst:6`
//!   * `b6_abilities_race.lst:9`
//!   * `b6_abilities_race.lst:12`
//!   * `b6_abilities_race.lst:15`
//!   * `b6_abilities_race.lst:18`
//!   * `b6_abilities_race.lst:21`
//!   * `b6_abilities_race.lst:22`
//!   * `b6_abilities_race.lst:25`
//!   * `b6_abilities_race.lst:26`
//!   * `b6_abilities_race.lst:29`
//!   * `b6_abilities_race.lst:32`
//!   * `b6_abilities_race.lst:35`
//!   * `ce_abilities_race.lst:2446`
//!   * `ce_abilities_race.lst:2447`
//!   * `ce_abilities_race.lst:2448`

use crate::rules_core::rules_tables::monster_chassis::{MonsterAbilityDelivery, MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock, NaturalAttack, Speed, StatAdjustment};

/// Every bestiary_6 monster stat block (0 rows).
pub(super) static MONSTERS: &[MonsterStatBlock] = &[
];

/// Every bestiary_6 monster-ability record (16 rows).
pub(super) static MONSTER_ABILITIES: &[MonsterAbilityRecord] = &[
    MonsterAbilityRecord {
        key: "Coral Capuchin ~ Cursed Bite",
        name: "Cursed Bite",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Supernatural),
        traits: &["CoralCapuchinRacialTrait"],
        description: Some("A coral capuchin's bite bestows some of the creature's benefits and weaknesses upon the victim unless the victim succeeds at a DC %1 Constitution check. The curse delivered by this bite persists for 1d6 hours, and cannot affect the same creature more than once in a 24-hour period. Affected creatures can hold their breath for double the normal amount of time, but begin drying out when exposed to air. Victims take 1d6 points of damage for every 10 minutes they are out of water, though spending a full-round action to bathe the victim in any sort of water halts this damage."),
        description_variables: &["10+HD/2+CON"],
        source_page: Some("p.66"),
        owners: &[],
        source_file: "b6_abilities_race.lst",
        source_line: 5,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Coral Capuchin ~ Moisture Dependency",
        name: "Moisture Dependency",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &["CoralCapuchinRacialTrait"],
        description: Some("A coral capuchin can breathe both air and water and survive indefinitely on land, but the creature must regularly be either submerged in water or thoroughly wetted down, or else it dries out. A coral capuchin can remain out of water for %1 hours before it suffers any negative effects. After this time, the creature takes 1d6 points of damage for every hour it remains dry. Bathing the creature in water of any sort resets this time frame."),
        description_variables: &["CONSCORE"],
        source_page: Some("p.66"),
        owners: &[],
        source_file: "b6_abilities_race.lst",
        source_line: 6,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Deinotherium ~ Sweep",
        name: "Sweep",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &["DeinotheriumRacialTrait"],
        description: Some("A deinotherium can sweep a target with its tusks and knock the victim to the ground. As part of a charge, a deinotherium can move up to twice its base speed in a straight line and make a gore attack at any point during its movement. If this attack is successful, the target is knocked prone and the deinotherium can deal damage with its trample attack before continuing its movement."),
        description_variables: &[],
        source_page: Some("p.184"),
        owners: &[],
        source_file: "b6_abilities_race.lst",
        source_line: 9,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Devil Monkey ~ Puncture Armor",
        name: "Puncture Armor",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &["DevilMonkeyRacialTrait"],
        description: Some("A devil monkey's fangs are devastating against armor and apparel. When a devil monkey hits with its bite, the creature bitten must succeed at a DC %1 Reflex save or the damage dealt by the bite is also dealt to any armor worn by the creature. If the target isn't wearing armor and fails this save, there's a 50%% chance the bite damage is applied to a magic item worn in the body, chest, head, or shoulders slot (determine which item is bitten randomly among all potential targets)."),
        description_variables: &["10+HD/2+CON"],
        source_page: Some("p.93"),
        owners: &[],
        source_file: "b6_abilities_race.lst",
        source_line: 12,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Dunkleosteus ~ Gulp",
        name: "Gulp",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &["DunkleosteusRacialTrait"],
        description: Some("A dunkleosteus can open its giant mouth in a fraction of a second, creating a vortex that draws a target in. When underwater, a dunkleosteus gains an additional 5 feet of reach with its bite attack and a +2 bonus on combat maneuver checks when attempting to grapple."),
        description_variables: &[],
        source_page: Some("p.129"),
        owners: &[],
        source_file: "b6_abilities_race.lst",
        source_line: 15,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Elasmotherium ~ Impaling Horn",
        name: "Impaling Horn",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &["ElasmotheriumRacialTrait"],
        description: Some("An elasmotherium's immense horn can fully impale a creature that is two or more size categories smaller than the elasmotherium (a Medium or smaller creature for the typical elasmotherium). This ability effectively adds the grab universal monster ability to the elasmotherium's gore attack against such a creature, but if the elasmotherium successfully \"grabs\" a foe in this manner by impaling it on the horn, the elasmotherium does not gain the grappled condition. An elasmotherium's options while grappling a foe in this way are limited-it can either move while grappling the foe, or it can whip its head and attempt to fling the foe as if using the Awesome Blow feat (the damage dealt by this is equal to its gore damage). An elasmotherium can impale only one creature at a time with its horn, but can continue to attack normally with its gore."),
        description_variables: &[],
        source_page: Some("p.185"),
        owners: &[],
        source_file: "b6_abilities_race.lst",
        source_line: 18,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Kentrosaurus ~ Defensive Spikes",
        name: "Defensive Spikes",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &["KentrosaurusRacialTrait"],
        description: Some("At the end of its turn, a kentrosaurus can crouch down and splay its back and shoulder spikes. It cannot make attacks of opportunity when using its defensive spikes in this way, but until its next turn any creature that attacks the kentrosaurus with light or one-handed melee weapons, natural weapons, or unarmed strikes takes 1d6 points of piercing damage unless it succeeds at a DC %1 Reflex saving throw."),
        description_variables: &["10+HD/2+DEX"],
        source_page: Some("p.95"),
        owners: &[],
        source_file: "b6_abilities_race.lst",
        source_line: 21,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Kentrosaurus ~ Impaling Strike",
        name: "Impaling Strike",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &["KentrosaurusRacialTrait"],
        description: Some("A kentrosaurus's tail is a primary attack that deals piercing damage and has a x3 critical damage multiplier."),
        description_variables: &[],
        source_page: Some("p.95"),
        owners: &[],
        source_file: "b6_abilities_race.lst",
        source_line: 22,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Mockingfey ~ Mock",
        name: "Mock",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Supernatural),
        traits: &["MockingfeyRacialTrait"],
        description: Some("As a standard action, a mockingfey can take on the appearance of any creature it can see. This functions like veil, but affects only the mockingfey. The mockingfey does not change size, and if the creature it's mimicking is larger than itself, the mockingfey simply appears to be a miniature version of it. Anyone interacting with this effect can attempt a DC %1 Will save to see through the ruse. A mockingfey can maintain a guise indefinitely, but can't change to a form other than its own without a visual reference-once a given disguise has ended, the fey must see the subject again to resume that form."),
        description_variables: &["10+HD/2+CHA"],
        source_page: Some("p.189"),
        owners: &[],
        source_file: "b6_abilities_race.lst",
        source_line: 25,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Mockingfey ~ SLA",
        name: "Spell-Like Abilities",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::SpellLike),
        traits: &["MockingfeyRacialTrait"],
        description: None,
        description_variables: &[],
        source_page: None,
        owners: &[],
        source_file: "b6_abilities_race.lst",
        source_line: 26,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Mokele-Mbembe ~ Whip Tail",
        name: "Whip Tail",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &["MokeleMbembeRacialTrait"],
        description: Some("When not submerged, a mokele-mbembe can crack its tail like a whip as a standard action, creating a sonic boom in a 5-foot burst up to 20 feet away. Any creature in the burst's area must succeed at a DC %1 Fortitude save or be stunned for 1 round. Mokele-mbembes are immune to this sonic effect."),
        description_variables: &["10+HD/2+CON"],
        source_page: Some("p.190"),
        owners: &[],
        source_file: "b6_abilities_race.lst",
        source_line: 29,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Quetzalcoatlus ~ Razor-Sharp Beak",
        name: "Razor-Sharp Beak",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &["QuetzalcoatlusRacialTrait"],
        description: Some("A quetzalcoatlus's bite attack has a critical multiplier of x3."),
        description_variables: &[],
        source_page: Some("p.95"),
        owners: &[],
        source_file: "b6_abilities_race.lst",
        source_line: 32,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Giant Raven ~ Scavenger",
        name: "Scavenger",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &["GiantRavenRacialTrait"],
        description: Some("Giant ravens are hardy birds that often feed on carrion or even undead flesh. As a direct result of this unusual dietary habit, they gain a +4 bonus on saves to resist ingested diseases."),
        description_variables: &[],
        source_page: Some("p.240"),
        owners: &[],
        source_file: "b6_abilities_race.lst",
        source_line: 35,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Universal Monster Rule ~ Ferocity",
        name: "Ferocity",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &["Universal Monster Rule", "Defensive"],
        description: Some("The creature remains conscious and can continue fighting even if its hit point total is below 0. The creature is still staggered and loses 1 hit point each round. A creature with ferocity still dies when its hit point total reaches a negative amount equal to its Constitution score."),
        description_variables: &[],
        source_page: Some("p.293"),
        owners: &[],
        source_file: "ce_abilities_race.lst",
        source_line: 2446,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Universal Monster Rule ~ Negative Energy Affinity",
        name: "Negative Energy Affinity",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &["Universal Monster Rule", "Defensive"],
        description: Some("The creature is alive but reacts to positive and negative energy as if it were undead--positive energy harms it, negative energy heals it."),
        description_variables: &[],
        source_page: Some("p.296"),
        owners: &[],
        source_file: "ce_abilities_race.lst",
        source_line: 2447,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Universal Monster Rule ~ Scent",
        name: "Scent",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &["Universal Monster Rule", "Sense"],
        description: Some("This special sense allows a creature to detect approaching enemies, sniff out hidden foes, and track by sense of smell. Creatures with the scent ability can identify familiar odors just as humans do familiar sights.&nl;The creature can detect opponents within 30 feet by sense of smell. If the opponent is upwind, the range increases to 60 feet; if downwind, it drops to 15 feet. Strong scents, such as smoke or rotting garbage, can be detected at twice the ranges noted above. Overpowering scents, such as skunk musk or troglodyte stench, can be detected at triple the normal range.&nl;When a creature detects a scent, the exact location of the source is not revealed--only its presence somewhere within range. The creature can take a move action to note the direction of the scent. When the creature is within 5 feet of the source, it pinpoints the source's location.&nl;A creature with the scent ability can follow tracks by smell, attempting a Wisdom or Survival check to find or follow a track. The typical DC for a fresh trail is 10 (no matter what kind of surface holds the scent). This DC increases or decreases depending on how strong the quarry's odor is, the number of creatures, and the age of the trail. For each hour that the trail is cold, the DC increases by 2. Creatures tracking by scent ignore the effects of surface conditions and poor visibility. The ability otherwise follows the rules for the Survival skill."),
        description_variables: &[],
        source_page: Some("p.298"),
        owners: &[],
        source_file: "ce_abilities_race.lst",
        source_line: 2448,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
];
