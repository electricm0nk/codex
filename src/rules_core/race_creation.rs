//! The character-creation chassis a race must present before a player may
//! choose it.
//!
//! # Why this module exists here and not in the desktop crate
//!
//! This predicate was written inside `apps/desktop/src-tauri/src/
//! character_hub.rs` as a private helper of `build_race_creation_roster`, the
//! function behind the `list_race_creation_roster` Tauri command. That is
//! still its only *product* consumer, and its behaviour is unchanged by the
//! move.
//!
//! What the move buys is a **second, independent** consumer that could not
//! reach it before: `src/bin/v06_work_inventory.rs`, which lives in this
//! crate and cannot depend on the desktop crate. Until this move, the
//! inventory's `race`-kind verdict answered "is this race modelled?" by
//! testing membership in [`RaceId::ALL`](crate::rules_core::rules_tables::
//! crb::race_tables::RaceId) — the original seven-variant CRB enum — while
//! the product had long since moved to the corpus-driven
//! [`RaceCorpus`](crate::rules_core::race_resolver::RaceCorpus). The gap is
//! recorded three times over in `docs/release/SD-31-corpus-closure-grind/
//! artifacts/OPEN-ISSUES.md` (rows 170, 207, 226), each naming this exact
//! remedy: point the classifier at the mechanism the product really uses.
//!
//! **The predicate is shared, never re-implemented.** Re-deriving "would this
//! race be offered?" inside the inventory binary would be an instrument
//! asserting the product's behaviour rather than observing it — the failure
//! mode `probe_race_trait_corpus`'s own doc comment in that binary exists to
//! prevent. One function, two callers.
//!
//! # What passing this predicate means
//!
//! A race passes only if the loaded corpus states, readably, **all** of:
//!
//! - a creature size ([`ResolvedRace::size`]) — never defaulted to Medium;
//! - a base land speed ([`ResolvedRace::walk_speed_ft`]);
//! - senses that parse, or none at all (a race declaring no `VISION:` token
//!   honestly has normal vision; an *unrecognized* token is an error);
//! - **a real ability-score magnitude**: either a fixed `BONUS:STAT` set or a
//!   floating "+N to one ability score" pool. A race stating neither is
//!   refused.
//!
//! That last clause is what makes this a magnitude observation rather than a
//! load observation. The values it returns are consumed downstream by
//! `applyRacialAbilityAdjustments` (`apps/desktop/src/characterHub/
//! composeCreateCharacterRequest.ts`), which bakes them into the ability
//! scores submitted at character creation — so a race that passes here has a
//! number that changes the player's calculated sheet, and a race that fails
//! is withheld from the roster and named in its diagnostics.

use std::collections::BTreeMap;

use crate::rules_core::race_resolver::{ResolvedRace, ResolvedTrait};
use crate::rules_core::size::SizeCategory;

/// PCGen's `BONUS:STAT` ability codes, mapped to the ability names the wire
/// DTOs (`AbilityScoresDto` / `characterHubModel.ABILITY_KEYS`) use.
const STAT_CODE_TO_ABILITY: &[(&str, &str)] = &[
    ("STR", "strength"),
    ("DEX", "dexterity"),
    ("CON", "constitution"),
    ("INT", "intelligence"),
    ("WIS", "wisdom"),
    ("CHA", "charisma"),
];

/// The `TYPE:` token PCGen tags a race's ability-modifier row with.
const RACIAL_ABILITY_SCORES_TYPE: &str = "Racial Ability Scores";

/// One race's complete, readable character-creation chassis.
///
/// Constructed only by [`race_creation_chassis`], which refuses rather than
/// defaults any field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RaceCreationChassis {
    /// The corpus's own display-cased race key, e.g. `"Half-Elf"`.
    pub race_key: String,
    /// The corpus book directory this race's chassis record was loaded from.
    pub book_id: String,
    /// The race's real creature size — [`ResolvedRace::size`], i.e. the
    /// `~ Size` trait's `TEMPLATE:SIZE_<code>` over the chassis'
    /// `FACT:BaseSize`. Never the chassis token alone: Aasimar and Tiefling
    /// carry `FACT:BaseSize|S` and are Medium creatures.
    pub size: SizeCategory,
    /// The race's senses as the Character Sheet prints them, e.g.
    /// `Darkvision 60 ft.`, `Low-light vision`, or `Normal`.
    pub vision: String,
    /// Base land speed in feet — [`ResolvedRace::walk_speed_ft`]. Not the
    /// chassis row's `MOVE:Walk` alone: Goblin's and Hobgoblin's chassis rows
    /// say `MOVE:Walk,0` and their `~ Speed` traits override it to 30.
    pub base_speed_ft: i32,
    /// Fixed racial ability modifiers. Only non-zero entries appear.
    pub ability_adjustments: BTreeMap<String, i16>,
    /// Points the player distributes freely — PF1's "+2 to one ability
    /// score" races. `0` for a race with no such pool.
    pub floating_bonus_points: u8,
    /// **The `race_trait` record this chassis' ability magnitude was read
    /// from**, by its corpus [`ResolvedTrait::key`] (e.g.
    /// `"Drow ~ Ability Scores"`).
    ///
    /// Not `Option`: [`race_creation_chassis`] refuses a race stating no
    /// ability magnitude at all, so an `Ok` chassis always has a source row,
    /// and the type says so rather than leaving a caller to unwrap a `None`
    /// that cannot happen.
    ///
    /// # Why a consumer reports what it read (`SD31-W15-RACETRAIT-001`)
    ///
    /// `src/bin/v06_work_inventory.rs` needs to answer, per `race_trait`
    /// record, "did a real magnitude consumer read **this record's own**
    /// numbers?" — the question wave 12 demoted 251 units for getting wrong
    /// by answering a coarser one ("does this record's *race* have a seam
    /// somewhere?"). It could re-implement [`racial_ability_scores_trait`]'s
    /// selection rule to guess the answer; every time this program has done
    /// that, the re-implementation and the product drifted. So the consumer
    /// states it instead, and the inventory observes rather than asserts.
    pub ability_adjustments_source_trait_key: String,
}

/// The race's ability-modifier trait, if it declares one.
fn racial_ability_scores_trait(race: &ResolvedRace) -> Option<&ResolvedTrait> {
    race.traits
        .iter()
        .find(|resolved| resolved.type_tokens.iter().any(|t| t == RACIAL_ABILITY_SCORES_TYPE))
}

/// The fixed ability modifiers a `Racial Ability Scores` row declares.
///
/// Reads `BONUS:STAT|<codes>|<magnitude>` chains only. `<codes>` is
/// comma-separated and frequently names more than one ability — Goblin's
/// `BONUS:STAT|STR,CHA|-2` grants **both** — so every code in the list is
/// credited. An unrecognized code is reported rather than dropped.
fn fixed_ability_adjustments(
    ability_trait: &ResolvedTrait,
) -> Result<BTreeMap<String, i16>, String> {
    let mut out: BTreeMap<String, i16> = BTreeMap::new();
    for chain in &ability_trait.raw_bonus_chains {
        if chain.qualifiers.first().map(String::as_str) != Some("STAT") {
            continue;
        }
        let (Some(codes), Some(raw_magnitude)) = (chain.qualifiers.get(1), chain.qualifiers.get(2))
        else {
            return Err(format!(
                "{}: a BONUS:STAT chain is missing its codes or magnitude",
                ability_trait.key
            ));
        };
        let magnitude: i16 = raw_magnitude.parse().map_err(|_| {
            format!(
                "{}: BONUS:STAT magnitude {raw_magnitude:?} is not an integer",
                ability_trait.key
            )
        })?;
        for code in codes.split(',') {
            let code = code.trim();
            let ability = STAT_CODE_TO_ABILITY
                .iter()
                .find(|(stat, _)| *stat == code)
                .map(|(_, ability)| *ability)
                .ok_or_else(|| {
                    format!("{}: unknown BONUS:STAT ability code {code:?}", ability_trait.key)
                })?;
            *out.entry(ability.to_owned()).or_insert(0) += magnitude;
        }
    }
    out.retain(|_, delta| *delta != 0);
    Ok(out)
}

/// The freely-distributed "+2 to one ability score" points a
/// `Racial Ability Scores` row grants.
///
/// PCGen splits the fact across two places: the *number of picks* is
/// machine-readable (`BONUS:ABILITYPOOL|Ability Bonus|1`) but the *magnitude
/// per pick* appears only in the row's own display name. That is stated here
/// rather than hidden, and the name is matched strictly — a row that does not
/// have the shape yields an error naming it, never a guessed magnitude.
fn floating_ability_bonus_points(ability_trait: &ResolvedTrait) -> Result<u8, String> {
    let picks: u8 = ability_trait
        .raw_bonus_chains
        .iter()
        .filter(|chain| {
            chain.qualifiers.first().map(String::as_str) == Some("ABILITYPOOL")
                && chain.qualifiers.get(1).map(String::as_str) == Some("Ability Bonus")
        })
        .map(|chain| chain.qualifiers.get(2).and_then(|n| n.parse::<u8>().ok()).unwrap_or(0))
        .sum();
    if picks == 0 {
        return Ok(0);
    }
    let magnitude = ability_trait
        .name
        .strip_prefix('+')
        .and_then(|rest| rest.strip_suffix(" to One Ability Score"))
        .and_then(|n| n.parse::<u8>().ok())
        .ok_or_else(|| {
            format!(
                "{}: an ability-pool row must state its magnitude in its own name, got {:?}",
                ability_trait.key, ability_trait.name
            )
        })?;
    Ok(picks * magnitude)
}

/// The race's senses, rendered the way the Character Sheet's Details panel
/// prints them, from the `VISION:` tokens on its resolved traits.
///
/// A race with no `VISION:` token honestly has normal vision. An
/// unrecognized token yields an error naming it rather than being silently
/// skipped — a dropped sense is a rules fact the player would never learn was
/// missing.
fn vision_reading(race: &ResolvedRace) -> Result<String, String> {
    let mut readings: Vec<String> = Vec::new();
    for resolved in &race.traits {
        for token in resolved.raw_tokens.iter().filter(|t| t.key == "VISION") {
            // PCGen states more than one sense on a single `VISION:` row two
            // different ways: as separate `VISION:`-keyed fields on the same
            // row (Svirfneblin's `VISION:Darkvision (120) VISION:Low-Light
            // Vision`, two distinct `RawToken`s this loop already visits
            // separately) or as one field with a `|`-joined tail (Dhampir's
            // `VISION:Darkvision (60)|Low-Light Vision`, SD-32 card-11 T2b
            // lane, 2026-08-23). Both are the same fact stated two ways, so
            // both are split into segments here rather than only the first
            // shape being read.
            for segment in token.value.split('|') {
                let value = segment.trim();
                let reading = if let Some(range) =
                    value.strip_prefix("Darkvision (").and_then(|rest| rest.strip_suffix(')'))
                {
                    range
                        .parse::<u16>()
                        .map(|feet| format!("Darkvision {feet} ft."))
                        .map_err(|_| format!("{}: unreadable Darkvision range {value:?}", resolved.key))?
                } else if value == "Low-Light Vision" {
                    "Low-light vision".to_owned()
                } else {
                    return Err(format!("{}: unrecognized VISION token {value:?}", resolved.key));
                };
                if !readings.contains(&reading) {
                    readings.push(reading);
                }
            }
        }
    }
    Ok(if readings.is_empty() { "Normal".to_owned() } else { readings.join(", ") })
}

/// Builds one race's creation chassis, or the reason it cannot be offered.
///
/// The `Err` string is the *diagnostic a player-facing surface prints*, not a
/// panic message: a race whose chassis cannot be read completely is withheld
/// and named, so one gap costs that race and not the rest of the roster.
pub fn race_creation_chassis(race: &ResolvedRace) -> Result<RaceCreationChassis, String> {
    let size = race
        .size
        .ok_or_else(|| format!("{}: declares no readable creature size", race.race_key))?;
    let base_speed_ft = race
        .walk_speed_ft
        .ok_or_else(|| format!("{}: declares no readable base land speed", race.race_key))?;
    let vision = vision_reading(race)?;
    let ability_scores_row = racial_ability_scores_trait(race);
    let (ability_adjustments, floating_bonus_points) = match ability_scores_row {
        Some(ability_trait) => {
            (fixed_ability_adjustments(ability_trait)?, floating_ability_bonus_points(ability_trait)?)
        }
        None => (BTreeMap::new(), 0),
    };
    if ability_adjustments.is_empty() && floating_bonus_points == 0 {
        return Err(format!(
            "{}: states neither a fixed ability modifier nor a floating ability pool",
            race.race_key
        ));
    }

    Ok(RaceCreationChassis {
        race_key: race.race_key.clone(),
        book_id: race.book_id.clone(),
        size,
        vision,
        base_speed_ft,
        ability_adjustments,
        floating_bonus_points,
        // Unreachable `None`: the refusal above already returned for every
        // race whose ability-scores row is absent or states no magnitude, so
        // an `Ok` chassis always has one. Named rather than defaulted, for
        // the same reason every other field here is.
        ability_adjustments_source_trait_key: ability_scores_row
            .map(|row| row.key.clone())
            .ok_or_else(|| {
                format!("{}: an ability magnitude was read from no row", race.race_key)
            })?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::race_resolver::{SizeSource, SpeedSource, TraitRole};

    fn bare_race(race_key: &str) -> ResolvedRace {
        ResolvedRace {
            race_key: race_key.to_owned(),
            name: race_key.to_owned(),
            book_id: "test_book".to_owned(),
            size: Some(SizeCategory::Medium),
            chassis_size: Some(SizeCategory::Medium),
            size_source: SizeSource::Chassis,
            race_type: Some("Humanoid".to_owned()),
            chassis_walk_speed_ft: Some(30),
            walk_speed_ft: Some(30),
            speed_source: SpeedSource::Chassis,
            traits: Vec::new(),
            fired_flags: Vec::new(),
            suppressions: Vec::new(),
            unmatched_selections: Vec::new(),
            inert_flags: Vec::new(),
        }
    }

    fn ability_trait(name: &str, chains: &[(&str, &str, &str)]) -> ResolvedTrait {
        use crate::rules_core::shape_b_v1::RawBonusChain;
        ResolvedTrait {
            key: format!("Test ~ {name}"),
            name: name.to_owned(),
            book_id: "test_book".to_owned(),
            role: TraitRole::Default,
            type_tokens: vec![RACIAL_ABILITY_SCORES_TYPE.to_owned()],
            description: None,
            source_page: None,
            raw_tokens: Vec::new(),
            raw_bonus_chains: chains
                .iter()
                .map(|(a, b, c)| RawBonusChain {
                    qualifiers: vec![(*a).to_owned(), (*b).to_owned(), (*c).to_owned()],
                })
                .collect(),
        }
    }

    /// The clause that makes this a magnitude predicate rather than a load
    /// predicate: a race the corpus loaded, with a real size and speed, is
    /// still REFUSED when it states no ability magnitude at all.
    #[test]
    fn a_race_with_no_ability_magnitude_is_refused() {
        let race = bare_race("Nomag");
        let err = race_creation_chassis(&race).unwrap_err();
        assert!(
            err.contains("states neither a fixed ability modifier nor a floating ability pool"),
            "unexpected refusal reason: {err}"
        );
    }

    /// A size that cannot be read is a refusal, never a defaulted Medium.
    #[test]
    fn a_race_with_no_readable_size_is_refused() {
        let mut race = bare_race("Sizeless");
        race.traits.push(ability_trait("Sizeless Ability Scores", &[("STAT", "STR", "2")]));
        race.size = None;
        let err = race_creation_chassis(&race).unwrap_err();
        assert!(err.contains("declares no readable creature size"), "unexpected reason: {err}");
    }

    /// A speed that cannot be read is a refusal too.
    #[test]
    fn a_race_with_no_readable_speed_is_refused() {
        let mut race = bare_race("Speedless");
        race.traits.push(ability_trait("Speedless Ability Scores", &[("STAT", "DEX", "2")]));
        race.walk_speed_ft = None;
        let err = race_creation_chassis(&race).unwrap_err();
        assert!(err.contains("declares no readable base land speed"), "unexpected reason: {err}");
    }

    /// A comma-separated `BONUS:STAT` code list credits every code — the
    /// Goblin `BONUS:STAT|STR,CHA|-2` shape.
    #[test]
    fn a_multi_code_bonus_stat_chain_credits_every_code() {
        let mut race = bare_race("Multi");
        race.traits.push(ability_trait("Multi Ability Scores", &[("STAT", "STR,CHA", "-2")]));
        let chassis = race_creation_chassis(&race).expect("a real magnitude is stated");
        assert_eq!(chassis.ability_adjustments.get("strength"), Some(&-2));
        assert_eq!(chassis.ability_adjustments.get("charisma"), Some(&-2));
        assert_eq!(chassis.floating_bonus_points, 0);
    }

    /// A floating pool passes on its own, with no fixed modifier — the Human
    /// shape — and its magnitude comes from the row's own name.
    #[test]
    fn a_floating_ability_pool_passes_on_its_own() {
        let mut race = bare_race("Floater");
        race.traits.push(ability_trait(
            "+2 to One Ability Score",
            &[("ABILITYPOOL", "Ability Bonus", "1")],
        ));
        let chassis = race_creation_chassis(&race).expect("a floating pool is a real magnitude");
        assert!(chassis.ability_adjustments.is_empty());
        assert_eq!(chassis.floating_bonus_points, 2);
    }

    /// An ability-pool row that does not state its magnitude in its own name
    /// is an error, never a guessed magnitude.
    #[test]
    fn an_ability_pool_row_without_a_stated_magnitude_is_an_error() {
        let mut race = bare_race("Vague");
        race.traits
            .push(ability_trait("Some Bonus", &[("ABILITYPOOL", "Ability Bonus", "1")]));
        let err = race_creation_chassis(&race).unwrap_err();
        assert!(err.contains("must state its magnitude in its own name"), "unexpected: {err}");
    }

    /// A trait carrying no `Racial Ability Scores` type token — the shape
    /// every ordinary racial trait has.
    fn plain_trait(name: &str) -> ResolvedTrait {
        let mut plain = ability_trait(name, &[("STAT", "STR", "9")]);
        plain.type_tokens = vec!["Special Quality".to_owned()];
        plain
    }

    /// **The chassis NAMES the record its ability magnitude was read from.**
    ///
    /// `SD31-W15-RACETRAIT-001`. Without this, a second consumer asking "which
    /// `race_trait` record did the character-creation path actually read?" has
    /// to re-implement [`racial_ability_scores_trait`]'s selection rule and
    /// [`fixed_ability_adjustments`]' parsing — an instrument *asserting* this
    /// module's behaviour instead of *observing* it, which is exactly the
    /// failure this module's own header comment exists to prevent. The
    /// consumer reports what it read; nobody guesses.
    #[test]
    fn the_chassis_names_the_trait_record_its_ability_magnitude_came_from() {
        let mut race = bare_race("Named");
        race.traits.push(ability_trait("Named Ability Scores", &[("STAT", "CON,WIS", "2")]));
        let chassis = race_creation_chassis(&race).expect("a real magnitude is stated");
        assert_eq!(chassis.ability_adjustments_source_trait_key, "Test ~ Named Ability Scores");
    }

    /// …and it names the row that really supplied the numbers, not merely the
    /// first trait the race applies. A positional answer would credit whichever
    /// record happened to sort first, which is the "credit resting on a
    /// DIFFERENT record" shape wave 12 demoted 251 units for.
    #[test]
    fn the_named_source_row_is_the_ability_scores_row_not_merely_the_first_trait() {
        let mut race = bare_race("Two");
        race.traits.push(plain_trait("Decoy"));
        race.traits.push(ability_trait("Two Ability Scores", &[("STAT", "STR", "2")]));
        let chassis = race_creation_chassis(&race).expect("a real magnitude is stated");
        assert_eq!(chassis.ability_adjustments_source_trait_key, "Test ~ Two Ability Scores");
        assert_eq!(chassis.ability_adjustments.get("strength"), Some(&2));
    }

    /// Modifiers that cancel to zero do not count as a magnitude: a race
    /// whose only `BONUS:STAT` chains sum to nothing is refused exactly as a
    /// race with no chain at all is.
    #[test]
    fn ability_adjustments_that_cancel_to_zero_do_not_count_as_a_magnitude() {
        let mut race = bare_race("Cancel");
        race.traits.push(ability_trait(
            "Cancel Ability Scores",
            &[("STAT", "STR", "2"), ("STAT", "STR", "-2")],
        ));
        let err = race_creation_chassis(&race).unwrap_err();
        assert!(
            err.contains("states neither a fixed ability modifier nor a floating ability pool"),
            "unexpected refusal reason: {err}"
        );
    }
}
