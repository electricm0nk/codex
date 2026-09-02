//! v0.8 E-1 -- `rate_encounter`: the DM Toolkit's encounter-difficulty
//! rating, a bridge over `codex::rules_core::encounters::Encounter::new`.
//! Party members are levels; monsters are either a real catalog monster
//! (resolved to its corpus `CR:` through `monster_catalog`) or a bare CR.
//!
//! **Two engine limits this response discloses per encounter, because a
//! quietly wrong "Medium" gets a party killed:**
//!
//! 1. The engine's XP table is verified for CR 1-10 only. Below CR 1 it
//!    rounds the CR and then floors it to 1 (`encounters.rs`,
//!    `xp_for_cr`: "CR below 1 is floored to CR 1 -- this module doesn't
//!    yet ground the rulebook's fractional-CR sub-table"), so a CR 1/4
//!    creature is rated as a full CR 1 creature. Above CR 10 it extrapolates
//!    by the table's own doubling-every-2-CR pattern, "not independently
//!    re-verified against the rulebook above CR 10". Every monster in the
//!    response carries `crAsRated` (the CR the engine actually used, read
//!    back from the engine by rating that monster alone) and, when that
//!    differs from the catalog CR or lies outside 1-10, an
//!    `outsideVerifiedRange` flag with the engine's own reason.
//! 2. `Difficulty` is four tiers collapsed from the rulebook's five
//!    (Easy/Average/Challenging/Hard/Epic -> Easy/Medium/Hard/Deadly; the
//!    engine's `Hard` covers both APL+1 "Challenging" and APL+2 "Hard").
//!    The response carries `elMinusApl` -- the engine's own two numbers
//!    subtracted -- and a `difficultyScale` note so the UI can say so. It
//!    does not name the rulebook tier: that mapping lives in the engine's
//!    doc comment, not in an exposed function, and a second copy here would
//!    be the rules-in-the-bridge shape this sprint refuses.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use codex::rules_core::encounters::{CharacterSnapshot, Difficulty, Encounter, MonsterRef};

use codex::saved_character::local_store::SavedCharacterStore;

use crate::character_hub::{character_level, resolve_character_root};
use crate::monster_catalog::{build_monster_catalog, MonsterCatalogEntryDto};

/// The CR range `encounters.rs`'s `VERIFIED_XP_TABLE` covers (inclusive).
const VERIFIED_CR_MIN: i32 = 1;
const VERIFIED_CR_MAX: i32 = 10;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncounterMonsterInput {
    /// A real monster: `MonsterCatalogEntryDto::key` (unique across the
    /// catalog, e.g. `"beastiary1:monster:wolf"`). Resolved to its corpus
    /// `CR:`; an unknown key is an error, never a default CR.
    #[serde(default)]
    pub catalog_key: Option<String>,
    /// A bare Challenge Rating, for a monster not in the catalog. Ignored
    /// when `catalog_key` is given.
    #[serde(default)]
    pub challenge_rating: Option<f32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartyMemberInput {
    /// A saved character: its level is read from the on-disk build
    /// (engine-side), never passed in by the UI. An id that does not
    /// resolve fails the whole rating -- silently dropping a member would
    /// shrink the party and inflate the difficulty.
    #[serde(default)]
    pub character_id: Option<String>,
    /// A hypothetical member by bare level, so a DM can rate a party
    /// without saving characters first. Ignored when `character_id` is
    /// given.
    #[serde(default)]
    pub level: Option<u8>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateEncounterRequest {
    /// Must not be empty.
    pub party: Vec<PartyMemberInput>,
    pub monsters: Vec<EncounterMonsterInput>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RatedPartyMemberDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_id: Option<String>,
    /// The level the engine rated this member at -- resolved from the
    /// saved build for a `characterId`, or the bare level given.
    pub level: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RatedMonsterDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The CR the caller supplied or the catalog holds, verbatim.
    pub challenge_rating: f32,
    /// The whole-number CR the engine actually rated this monster as --
    /// read back from the engine by rating the monster on its own (a
    /// single monster's Encounter Level IS its rated CR), not recomputed
    /// here. Differs from `challenge_rating` for every fractional CR.
    pub cr_as_rated: i32,
    /// `true` when `challenge_rating` lies outside the engine's verified
    /// CR 1-10 table, or the engine rated it as a different CR.
    pub outside_verified_range: bool,
    /// The engine's own stated limitation for this monster, for the DM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DifficultyScaleDto {
    pub tiers: u8,
    pub collapsed_from: u8,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RateEncounterResponse {
    /// `"easy" | "medium" | "hard" | "deadly"` -- the engine's four tiers.
    pub difficulty: String,
    pub average_party_level: i32,
    pub encounter_level: i32,
    /// `encounter_level - average_party_level`, the engine's own two
    /// numbers: -1 or less is Easy, 0 Medium, +1 or +2 Hard, +3 or more
    /// Deadly. Lets the UI show where inside `hard` an encounter sits.
    pub el_minus_apl: i32,
    pub party: Vec<RatedPartyMemberDto>,
    pub monsters: Vec<RatedMonsterDto>,
    pub any_outside_verified_range: bool,
    /// `(1, 10)` -- the inclusive CR range the engine's XP table is
    /// verified for.
    pub verified_cr_range: (i32, i32),
    pub difficulty_scale: DifficultyScaleDto,
    /// Encounter-level disclosures the DM should read before trusting the
    /// number.
    pub caveats: Vec<String>,
}

fn catalog_index() -> &'static BTreeMap<String, MonsterCatalogEntryDto> {
    static INDEX: OnceLock<BTreeMap<String, MonsterCatalogEntryDto>> = OnceLock::new();
    INDEX.get_or_init(|| {
        build_monster_catalog()
            .entries
            .into_iter()
            .map(|entry| (entry.key.clone(), entry))
            .collect()
    })
}

fn difficulty_name(difficulty: Difficulty) -> &'static str {
    match difficulty {
        Difficulty::Easy => "easy",
        Difficulty::Medium => "medium",
        Difficulty::Hard => "hard",
        Difficulty::Deadly => "deadly",
    }
}

/// Resolves one input to `(key, name, CR)`.
fn resolve_monster(input: &EncounterMonsterInput) -> Result<(Option<String>, Option<String>, f32), String> {
    if let Some(key) = &input.catalog_key {
        let entry = catalog_index()
            .get(key)
            .ok_or_else(|| format!("'{key}' is not a monster in the catalog; refusing to rate it at a guessed CR"))?;
        return Ok((Some(entry.key.clone()), Some(entry.name.clone()), entry.challenge_rating));
    }
    match input.challenge_rating {
        Some(cr) if cr.is_finite() && cr >= 0.0 => Ok((None, None, cr)),
        Some(cr) => Err(format!("'{cr}' is not a usable challenge rating")),
        None => Err("a monster needs either a catalogKey or a challengeRating".to_owned()),
    }
}

/// The engine's own limitation statement for a CR it cannot rate from its
/// verified table, or `None` when the CR is inside it and rated as itself.
fn range_reason(challenge_rating: f32, cr_as_rated: i32) -> Option<String> {
    if challenge_rating < VERIFIED_CR_MIN as f32 {
        return Some(format!(
            "The engine's XP table has no fractional-CR entries; it floors a CR {challenge_rating} \
             creature to CR {cr_as_rated}, so this monster counts as a full CR {cr_as_rated} \
             creature (400 XP) in the encounter level. The rulebook's sub-CR-1 table would \
             count it for less."
        ));
    }
    if challenge_rating > VERIFIED_CR_MAX as f32 {
        return Some(format!(
            "CR {challenge_rating} is above the engine's verified XP table (CR 1-10); its XP is \
             extrapolated by continuing the table's doubling-every-2-CR pattern and is not \
             independently re-verified against the rulebook above CR 10."
        ));
    }
    if (cr_as_rated as f32 - challenge_rating).abs() > f32::EPSILON {
        return Some(format!(
            "The engine rounds CR {challenge_rating} to CR {cr_as_rated} before rating; only \
             whole-number CRs are in its verified table."
        ));
    }
    None
}

/// A saved character's level, read from its on-disk build through
/// `character_hub::character_level` -- the one statement of "character
/// level = sum of class levels" in this crate (the same identity the
/// engine's private `skill_allocation::character_level` documents). A
/// Fighter 3 / Wizard 1 is level 4.
pub(crate) fn saved_character_level_at_root(root: &std::path::Path) -> Result<u8, String> {
    let envelope = SavedCharacterStore::load(root).map_err(|err| err.message)?;
    Ok(character_level(&envelope.character_input))
}

/// The rating itself, split from the command so it is unit-testable:
/// `resolve_level` maps a `characterId` to its level (the command wires
/// the real saved-character store; tests wire a temp root).
pub(crate) fn rate_encounter_request(
    request: &RateEncounterRequest,
    resolve_level: &dyn Fn(&str) -> Result<u8, String>,
) -> Result<RateEncounterResponse, String> {
    if request.party.is_empty() {
        return Err("an encounter needs at least one party member; with none, the average party \
                    level is 0 and every encounter would rate Deadly"
            .to_owned());
    }
    let mut rated_party = Vec::with_capacity(request.party.len());
    for member in &request.party {
        let level = match (&member.character_id, member.level) {
            (Some(character_id), _) => resolve_level(character_id)
                .map_err(|err| format!("party member '{character_id}' could not be rated: {err}"))?,
            (None, Some(level)) => level,
            (None, None) => {
                return Err("a party member needs either a characterId or a level".to_owned())
            }
        };
        rated_party.push(RatedPartyMemberDto { character_id: member.character_id.clone(), level });
    }
    let party: Vec<CharacterSnapshot> =
        rated_party.iter().map(|member| CharacterSnapshot::new(member.level)).collect();

    let mut monsters = Vec::with_capacity(request.monsters.len());
    let mut refs = Vec::with_capacity(request.monsters.len());
    for input in &request.monsters {
        let (catalog_key, name, challenge_rating) = resolve_monster(input)?;
        let monster = MonsterRef::new(challenge_rating);
        // A lone monster's Encounter Level is exactly the CR the engine
        // rated it as -- read it back rather than re-deriving the engine's
        // rounding/flooring here.
        let cr_as_rated = Encounter::new(&party, &[monster]).encounter_level;
        let reason = range_reason(challenge_rating, cr_as_rated);
        monsters.push(RatedMonsterDto {
            catalog_key,
            name,
            challenge_rating,
            cr_as_rated,
            outside_verified_range: reason.is_some(),
            reason,
        });
        refs.push(monster);
    }

    let result = Encounter::new(&party, &refs);
    let mut caveats = Vec::new();
    if refs.is_empty() {
        caveats.push(
            "With no monsters the engine reports Easy by rule (no threat), not by computation."
                .to_owned(),
        );
    }
    let any_outside_verified_range = monsters.iter().any(|m| m.outside_verified_range);
    if any_outside_verified_range {
        caveats.push(
            "At least one monster's CR is outside the engine's verified CR 1-10 table; see each \
             monster's reason before trusting this rating."
                .to_owned(),
        );
    }

    Ok(RateEncounterResponse {
        difficulty: difficulty_name(result.difficulty).to_owned(),
        average_party_level: result.average_party_level,
        encounter_level: result.encounter_level,
        el_minus_apl: result.encounter_level - result.average_party_level,
        party: rated_party,
        monsters,
        any_outside_verified_range,
        verified_cr_range: (VERIFIED_CR_MIN, VERIFIED_CR_MAX),
        difficulty_scale: DifficultyScaleDto {
            tiers: 4,
            collapsed_from: 5,
            note: "The engine rates on four tiers collapsed from the rulebook's five: Easy (EL at \
                   most APL-1), Medium (= rulebook Average, EL = APL), Hard (covers BOTH rulebook \
                   Challenging at APL+1 and rulebook Hard at APL+2), Deadly (= rulebook Epic, EL \
                   at least APL+3). Use elMinusApl to tell the two Hard cases apart."
                .to_owned(),
        },
        caveats,
    })
}

/// Rates an encounter for the DM Toolkit. See the module doc for the two
/// engine limits every response discloses.
#[tauri::command]
pub fn rate_encounter(
    app: tauri::AppHandle,
    request: RateEncounterRequest,
) -> Result<RateEncounterResponse, String> {
    rate_encounter_request(&request, &|character_id| {
        let root = resolve_character_root(&app, character_id)?;
        saved_character_level_at_root(&root)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn levels(n: usize, level: u8) -> Vec<PartyMemberInput> {
        vec![PartyMemberInput { character_id: None, level: Some(level) }; n]
    }
    fn no_saved_characters(id: &str) -> Result<u8, String> {
        Err(format!("test resolver has no character '{id}'"))
    }
    fn cr(challenge_rating: f32) -> EncounterMonsterInput {
        EncounterMonsterInput { catalog_key: None, challenge_rating: Some(challenge_rating) }
    }
    fn key(key: &str) -> EncounterMonsterInput {
        EncounterMonsterInput { catalog_key: Some(key.to_owned()), challenge_rating: None }
    }
    fn rate(party: Vec<PartyMemberInput>, monsters: Vec<EncounterMonsterInput>) -> RateEncounterResponse {
        rate_encounter_request(&RateEncounterRequest { party, monsters }, &no_saved_characters)
            .expect("should rate")
    }

    /// The engine module's own canonical case: 4 level-3 PCs vs one CR 2
    /// monster is Easy (APL 3, EL 2). Nothing here is out of range.
    #[test]
    fn rates_a_verified_range_encounter_from_bare_crs() {
        let r = rate(levels(4, 3), vec![cr(2.0)]);
        assert_eq!(r.difficulty, "easy");
        assert_eq!(r.average_party_level, 3);
        assert_eq!(r.encounter_level, 2);
        assert_eq!(r.el_minus_apl, -1);
        assert_eq!(r.monsters.len(), 1);
        assert_eq!(r.monsters[0].cr_as_rated, 2);
        assert!(!r.monsters[0].outside_verified_range);
        assert!(r.monsters[0].reason.is_none());
    }

    /// A real catalog monster resolves to its corpus CR and carries its name.
    #[test]
    fn resolves_a_real_catalog_monster_to_its_cr() {
        let r = rate(levels(4, 1), vec![key("beastiary1:monster:wolf")]);
        let wolf = &r.monsters[0];
        assert_eq!(wolf.name.as_deref(), Some("Wolf"));
        assert_eq!(wolf.challenge_rating, 1.0);
        assert_eq!(wolf.cr_as_rated, 1);
        assert_eq!(r.difficulty, "medium");
    }

    #[test]
    fn an_unknown_catalog_key_is_an_error_never_a_default_cr() {
        let result = rate_encounter_request(
            &RateEncounterRequest { party: levels(4, 1), monsters: vec![key("beastiary1:monster:no_such_thing")] },
            &no_saved_characters,
        );
        let err = result.expect_err("an unknown monster must not be rated");
        assert!(err.contains("no_such_thing"), "{err}");
    }

    /// The uncomfortable one, pinned so it stays visible: the engine rates a
    /// CR 1/4 creature as CR 1. Four CR 1/4 creatures against four level-1
    /// PCs therefore come out DEADLY (4 x 400 XP = CR 5 vs APL 1), where the
    /// rulebook's fractional sub-table would make them a Medium fight. The
    /// response flags every such monster with the engine's own reason.
    #[test]
    fn a_fractional_cr_is_flagged_and_its_engine_rating_is_disclosed() {
        let r = rate(levels(4, 1), vec![cr(0.25), cr(0.25), cr(0.25), cr(0.25)]);
        assert_eq!(r.difficulty, "deadly", "this is what the engine computes today");
        assert_eq!(r.encounter_level, 5);
        for m in &r.monsters {
            assert_eq!(m.challenge_rating, 0.25);
            assert_eq!(m.cr_as_rated, 1, "the engine floors it to CR 1");
            assert!(m.outside_verified_range);
            let reason = m.reason.as_deref().expect("a reason");
            assert!(reason.contains("CR 1"), "{reason}");
        }
        assert!(r.any_outside_verified_range);
    }

    #[test]
    fn a_real_fractional_cr_catalog_monster_is_flagged_the_same_way() {
        let r = rate(levels(4, 1), vec![key("bestiary_2:monster:badger")]);
        let badger = &r.monsters[0];
        assert_eq!(badger.challenge_rating, 0.5);
        assert_eq!(badger.cr_as_rated, 1);
        assert!(badger.outside_verified_range);
    }

    /// Above CR 10 the engine extrapolates; the DM is told so.
    #[test]
    fn a_cr_above_10_is_flagged_as_extrapolated() {
        let r = rate(levels(4, 20), vec![key("beastiary:monster:tarrasque")]);
        let t = &r.monsters[0];
        assert_eq!(t.challenge_rating, 25.0);
        assert_eq!(t.cr_as_rated, 25);
        assert!(t.outside_verified_range);
        assert!(t.reason.as_deref().unwrap_or("").contains("extrapolat"));
        assert_eq!(r.encounter_level, 25);
        assert_eq!(r.difficulty, "deadly");
        assert!(r.any_outside_verified_range);
    }

    /// Four tiers from five: the engine's `Hard` spans both APL+1 and
    /// APL+2, and the response says so.
    #[test]
    fn the_response_discloses_the_four_of_five_tier_collapse() {
        let plus_one = rate(levels(4, 3), vec![cr(4.0)]);
        let plus_two = rate(levels(4, 3), vec![cr(5.0)]);
        assert_eq!((plus_one.difficulty.as_str(), plus_one.el_minus_apl), ("hard", 1));
        assert_eq!((plus_two.difficulty.as_str(), plus_two.el_minus_apl), ("hard", 2));
        assert_eq!(plus_one.difficulty_scale.tiers, 4);
        assert_eq!(plus_one.difficulty_scale.collapsed_from, 5);
        assert!(plus_one.difficulty_scale.note.contains("Challenging"));
        assert_eq!(plus_one.verified_cr_range, (1, 10));
    }

    #[test]
    fn an_empty_party_is_refused() {
        let result = rate_encounter_request(
            &RateEncounterRequest { party: vec![], monsters: vec![cr(1.0)] },
            &no_saved_characters,
        );
        assert!(result.is_err(), "APL 0 would rate everything Deadly; refuse instead");
    }

    #[test]
    fn a_monster_with_neither_key_nor_cr_is_refused() {
        let result = rate_encounter_request(
            &RateEncounterRequest {
                party: levels(4, 1),
                monsters: vec![EncounterMonsterInput { catalog_key: None, challenge_rating: None }],
            },
            &no_saved_characters,
        );
        assert!(result.is_err());
    }

    // ----- party members from saved characters (lead ruling, option a) -----

    fn saved_character_root(label: &str) -> std::path::PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        std::env::temp_dir().join(format!("codex-encounter-{label}-{unique}"))
    }

    fn create_fighter(root: &std::path::Path, level: u8) {
        use crate::character_hub::{create_character_at_root, AbilityScoresDto, CreateCharacterRequest};
        let request = CreateCharacterRequest {
            character_id: "party-member".to_owned(),
            display_label: "Party member".to_owned(),
            race_id: "race:human".to_owned(),
            class_id: "class:fighter".to_owned(),
            level,
            ability_scores: AbilityScoresDto {
                strength: 16,
                dexterity: 14,
                constitution: 14,
                intelligence: 10,
                wisdom: 12,
                charisma: 8,
            },
            ability_bonus_target: "strength".to_owned(),
            selected_alternate_trait_keys: Vec::new(),
            companion_species: None,
            selected_traits: Vec::new(),
            trait_skill_choices: Vec::new(),
            additional_choices: Vec::new(),
            saved_at: "2026-09-02T00:00:00Z".to_owned(),
        };
        let response = create_character_at_root(root, &request, "test".to_owned()).expect("create");
        assert!(matches!(response, crate::character_hub::CreateCharacterResponse::Saved { .. }), "{response:?}");
    }

    /// A saved character's level is read from its on-disk build by the
    /// resolver the command wires; the response echoes the resolved level.
    #[test]
    fn a_saved_characters_level_is_resolved_from_its_build() {
        let root = saved_character_root("single-class");
        create_fighter(&root, 3);

        let response = rate_encounter_request(
            &RateEncounterRequest {
                party: vec![
                    PartyMemberInput { character_id: Some("party-member".to_owned()), level: None },
                    PartyMemberInput { character_id: None, level: Some(3) },
                ],
                monsters: vec![cr(2.0)],
            },
            &|id| if id == "party-member" { saved_character_level_at_root(&root) } else { Err(format!("no '{id}'")) },
        )
        .expect("should rate");

        assert_eq!(response.party[0].character_id.as_deref(), Some("party-member"));
        assert_eq!(response.party[0].level, 3);
        assert_eq!(response.party[1].level, 3);
        assert_eq!(response.average_party_level, 3);
        assert_eq!(response.difficulty, "easy");
        std::fs::remove_dir_all(&root).ok();
    }

    /// Multiclass: character level is the SUM of class levels (Fighter 3 /
    /// Wizard 1 = level 4), the identity `skill_allocation::character_level`
    /// documents -- not the highest single class.
    #[test]
    fn a_multiclass_characters_level_is_the_sum_of_its_class_levels() {
        use crate::character_hub::level_up_character_at_root;
        let root = saved_character_root("multiclass");
        create_fighter(&root, 3);
        let dipped = level_up_character_at_root(&root, "class:wizard", Vec::new(), None, "2026-09-02T00:01:00Z")
            .expect("level up");
        assert!(matches!(dipped, crate::character_hub::CreateCharacterResponse::Saved { .. }), "{dipped:?}");

        assert_eq!(saved_character_level_at_root(&root).expect("level"), 4);
        std::fs::remove_dir_all(&root).ok();
    }

    /// An id that does not resolve is an error naming it -- never a
    /// silently smaller party, which would inflate the difficulty.
    #[test]
    fn an_unresolvable_character_id_fails_the_whole_rating() {
        let result = rate_encounter_request(
            &RateEncounterRequest {
                party: vec![
                    PartyMemberInput { character_id: Some("ghost".to_owned()), level: None },
                    PartyMemberInput { character_id: None, level: Some(3) },
                ],
                monsters: vec![cr(2.0)],
            },
            &no_saved_characters,
        );
        let err = result.expect_err("must not drop the member");
        assert!(err.contains("ghost"), "{err}");
    }

    #[test]
    fn a_party_member_with_neither_id_nor_level_is_refused() {
        let result = rate_encounter_request(
            &RateEncounterRequest {
                party: vec![PartyMemberInput { character_id: None, level: None }],
                monsters: vec![cr(2.0)],
            },
            &no_saved_characters,
        );
        assert!(result.is_err());
    }

    /// The engine's own business rule for no monsters (Easy) passes
    /// through, with a caveat saying it is a rule, not a computation.
    #[test]
    fn no_monsters_is_easy_with_a_caveat() {
        let r = rate(levels(4, 3), vec![]);
        assert_eq!(r.difficulty, "easy");
        assert!(r.caveats.iter().any(|c| c.contains("no monsters")));
    }
}
