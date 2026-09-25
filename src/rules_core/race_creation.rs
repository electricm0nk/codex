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
pub const RACIAL_ABILITY_SCORES_TYPE: &str = "Racial Ability Scores";

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
/// Reads the ability adjustments the converter transcribed off the row
/// ([`bonus_chain_reader::AbilityAdjustment`]). The code list is
/// comma-separated and frequently names more than one ability — Goblin's
/// `STR,CHA` at `-2` grants **both** — so every code in the list is credited.
/// An unrecognized code is reported rather than dropped.
fn fixed_ability_adjustments(
    ability_trait: &ResolvedTrait,
) -> Result<BTreeMap<String, i16>, String> {
    let mut out: BTreeMap<String, i16> = BTreeMap::new();
    for adjustment in &ability_trait.declared_bonuses.ability_adjustments {
        let (Some(codes), Some(raw_magnitude)) =
            (adjustment.codes.as_ref(), adjustment.magnitude.as_ref())
        else {
            return Err(format!(
                "{}: a declared ability adjustment is missing its codes or magnitude",
                ability_trait.key
            ));
        };
        let magnitude: i16 = raw_magnitude.parse().map_err(|_| {
            format!(
                "{}: ability-adjustment magnitude {raw_magnitude:?} is not an integer",
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
                    format!("{}: unknown ability code {code:?}", ability_trait.key)
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
/// The source splits the fact across two places: the *number of picks* is
/// machine-readable, and the converter transcribes it as
/// [`DeclaredBonuses::ability_pool_picks`]; the *magnitude per pick* appears
/// only in the row's own display name. That is stated here rather than hidden,
/// and the name is matched strictly — a row that does not have the shape
/// yields an error naming it, never a guessed magnitude.
fn floating_ability_bonus_points(ability_trait: &ResolvedTrait) -> Result<u8, String> {
    let picks: u8 = ability_trait.declared_bonuses.ability_pool_picks;
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
        // Both of PCGen's ways of stating more than one sense on one row --
        // separate keyed fields (Svirfneblin's `Darkvision (120)` +
        // `Low-Light Vision`) and one field with a `|`-joined tail (Dhampir's
        // `Darkvision (60)|Low-Light Vision`, SD-32 card-11 T2b lane,
        // 2026-08-23) -- are already flattened to one segment list by
        // `ResolvedTrait::declared_vision`, so this loop reads the same facts
        // it always did without knowing how the row spelled them.
        for segment in &resolved.declared_vision {
            let value = segment.as_str();
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
                return Err(format!("{}: unrecognized declared sense {value:?}", resolved.key));
            };
            if !readings.contains(&reading) {
                readings.push(reading);
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
    use crate::rules_core::race_resolver::{SizeSource, SpeedSource};

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









}
