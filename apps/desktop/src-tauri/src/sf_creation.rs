//! Starfinder 1e character creation (SD-37 E6.2, `docs/release/SD-37-starfinder-1e/epic-breakdown.md`
//! Epic E6): race -> theme -> class -> point buy, read from the converted Starfinder package
//! (`data/starfinder-1e/sheet_rules`) and saved as the `CharacterInput` [`crate::sf_adapter`] reads.
//!
//! # What the flow offers
//!
//! | Step | Offered | Source |
//! |---|---|---|
//! | race | every `race` record with a racial Hit Points row (the row the chassis needs: `sf_chassis` refuses a race without one, so a drone frame is not offered) | the race record; its name is its `Race`-category ability's label |
//! | theme | every `theme`-pool record tagged `Theme Selection` | the theme record |
//! | class | every `class` record, with its key-ability options | `sf_chassis::key_ability_options` |
//! | choices | every open pick the held set (race, theme, class at 1st level) carries | below |
//! | point buy | the budget, the creation cap and the computed scores | `sf_abilities` |
//!
//! A pick is one of three shapes, all read from the held set, never from a per-race table:
//!
//! - **offer**: a held rule whose own `offers` is a `Rules` set the engine counts as open
//!   (`offer_open`); the options are the records `offer_selects` names. Saved as a rule choice
//!   `(chooser, option)`, which the engine holds (`held_set`'s choice step).
//! - **pool**: a held rule targeting `Pool(<p>)` (a `BONUS:ABILITYPOOL`) with no offer of its own,
//!   where `<p>` is the slug of one record (`2_racial_stat_bonus`): that record is held (saved as
//!   a pick) and its own value choice is asked once per pool point.
//! - **value**: a held rule whose own `offers` picks a skill (`Skills`) or, for a `Chosen` target,
//!   an ability score (`FreeText`, the oracle's `CHOOSE:PCSTAT`). Saved as `(rule, value)`.
//!
//! Feat picks (the `feat` pool, offers from a feat pool) and the other offer kinds (languages,
//! spellcaster classes, ...) are listed as "chosen on the sheet", not asked here: no sheet total
//! this flow computes reads them.
//!
//! # The scores
//!
//! The point buy is `sf_abilities::compute` over the held set (race, theme and chosen racial
//! adjustments folded by the engine). The save stores the FINAL scores, the shape
//! `sf_adapter`'s module table reads. A build the engine refuses (over budget, a score above
//! 18 at creation, a race without a racial Hit Points row) is a named, blocking diagnostic and
//! nothing is saved.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState, SelectedChoice,
};
use codex::rules_core::pilot_compute::sf_abilities::{self, SfAbilityBuild, MAX_SCORE_AT_CREATION, POINT_BUY_BUDGET};
use codex::rules_core::pilot_compute::sf_chassis::{self, SfChassisRefusal};
use codex::rules_core::pilot_compute::sf_defense::{self, SfHeld};
use codex::rules_core::sheet_rule::{
    evaluate_applies, evaluate_expr, offer_open, offer_selects, split_rule_id, Ability, BonusTarget, EvalContext, OptionSet,
    SheetRule, SheetRulePackage, SheetValue,
};
use codex::saved_character::local_store::SavedCharacterStore;
use codex::saved_character::{SavedCharacterEnvelope, SavedCharacterRevisionKind, CURRENT_SAVED_CHARACTER_SCHEMA_VERSION};

use crate::character_hub::{CharacterSummaryDto, DiagnosticDto, ExplanationDto};
use crate::sf_adapter::{self, STARFINDER_RULE_SYSTEM_ID};

pub const REFUSED_NO_RACE: &str = "sf_creation.race_not_chosen";
pub const REFUSED_NO_THEME: &str = "sf_creation.theme_not_chosen";
pub const REFUSED_NO_CLASS: &str = "sf_creation.class_not_chosen";
pub const REFUSED_UNKNOWN_RECORD: &str = "sf_creation.record_not_offered";
pub const REFUSED_CHOICE_UNFILLED: &str = "sf_creation.choice_unfilled";
pub const REFUSED_CHOICE_NOT_OFFERED: &str = "sf_creation.choice_not_offered";
pub const REFUSED_NO_NAME: &str = "sf_creation.name_missing";
/// An optional pick left open: listed, never blocking.
pub const NOT_CHOSEN_YET: &str = "sf_creation.not_chosen_yet";

const SRD_POINT_BUY: &str = "SRD Buying Ability Scores (https://www.aonsrd.com/Rules.aspx?ID=42)";
const ABILITIES: [(Ability, &str, &str); 6] = [
    (Ability::Str, "STR", "Strength"),
    (Ability::Dex, "DEX", "Dexterity"),
    (Ability::Con, "CON", "Constitution"),
    (Ability::Int, "INT", "Intelligence"),
    (Ability::Wis, "WIS", "Wisdom"),
    (Ability::Cha, "CHA", "Charisma"),
];
/// Pools whose picks are feats: chosen on the sheet, not in this flow.
const FEAT_POOLS: [&str; 2] = ["feat", "combat_feat"];

/// Points spent on each ability, Str Dex Con Int Wis Cha.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointBuyDto {
    pub strength: i64,
    pub dexterity: i64,
    pub constitution: i64,
    pub intelligence: i64,
    pub wisdom: i64,
    pub charisma: i64,
}

impl PointBuyDto {
    fn as_array(&self) -> [i64; 6] {
        [self.strength, self.dexterity, self.constitution, self.intelligence, self.wisdom, self.charisma]
    }
}

/// One answered pick: the choice it answers and the option chosen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfPickDto {
    pub slot_id: String,
    pub option_id: String,
}

/// The creation form's state: what the player has chosen so far.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfCreationRequest {
    #[serde(default)]
    pub character_id: String,
    #[serde(default)]
    pub display_label: String,
    #[serde(default)]
    pub saved_at: String,
    #[serde(default)]
    pub race_id: Option<String>,
    #[serde(default)]
    pub theme_id: Option<String>,
    #[serde(default)]
    pub class_id: Option<String>,
    /// `STR` .. `CHA`, for a class whose key ability is a choice.
    #[serde(default)]
    pub key_ability: Option<String>,
    #[serde(default)]
    pub point_buy: PointBuyDto,
    #[serde(default)]
    pub picks: Vec<SfPickDto>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfOptionDto {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfClassOptionDto {
    pub id: String,
    pub label: String,
    /// `STR` .. `CHA`: one entry for a class with a fixed key ability, two for a choice.
    pub key_ability_options: Vec<SfOptionDto>,
}

/// The point-buy rules, from the engine's own constants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointBuyRulesDto {
    pub budget: i64,
    pub max_score_at_creation: i64,
    pub source: String,
}

/// One pick the flow asks for.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfCreationSlotDto {
    /// The choice id the pick is saved under (the choosing rule's id).
    pub slot_id: String,
    /// What is being chosen, from the choosing record's label.
    pub label: String,
    /// `option` (a record), `skill` or `ability`.
    pub kind: String,
    /// How many options this slot takes.
    pub count: usize,
    /// Whether one option may be taken more than once.
    pub repeatable: bool,
    pub options: Vec<SfOptionDto>,
    /// The option the package marks as taking nothing (tagged `No Archetype`, `No Variant
    /// Ability`), preselected by the form; `None` when the slot has no such option.
    pub default_option_id: Option<String>,
    /// Whether the creation scores depend on it (an ability-score pick, or options that adjust
    /// an ability score): only these block creation while open. Every other pick may stay open,
    /// as a PCGen ability pool may, and is listed as not chosen yet.
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfScoreTermDto {
    pub label: String,
    pub value: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfAbilityScoreDto {
    /// `STR` .. `CHA`.
    pub ability: String,
    pub label: String,
    /// The score at creation (base, race, theme, point buy).
    pub score: i64,
    pub modifier: i64,
    pub terms: Vec<SfScoreTermDto>,
}

/// The form's next state: the lists, the picks still open and the computed scores.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfCreationPreviewDto {
    pub races: Vec<SfOptionDto>,
    pub themes: Vec<SfOptionDto>,
    pub classes: Vec<SfClassOptionDto>,
    pub point_buy_rules: PointBuyRulesDto,
    pub slots: Vec<SfCreationSlotDto>,
    /// Picks the held set carries that this flow does not ask (feats, languages, ...).
    pub chosen_on_the_sheet: Vec<String>,
    /// Empty until a race is chosen; the engine's scores otherwise.
    pub ability_scores: Vec<SfAbilityScoreDto>,
    pub points_spent: i64,
    pub points_unspent: i64,
    /// Everything that keeps the character from being created (`claimBlocking`), and the
    /// optional picks still open (not blocking), by stable id.
    pub problems: Vec<DiagnosticDto>,
}

/// The created character, or why it was not created (nothing saved).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum SfCreateResponse {
    Saved {
        summary: Box<CharacterSummaryDto>,
        /// The `sf.*` explanation rows of the saved character (every sheet total).
        explanations: Vec<ExplanationDto>,
    },
    Blocked { diagnostics: Vec<DiagnosticDto> },
}

fn problem(id: &str, message: impl Into<String>) -> DiagnosticDto {
    DiagnosticDto { id: id.to_owned(), message: message.into(), claim_blocking: true }
}

fn refusal_problem(refusal: &SfChassisRefusal) -> DiagnosticDto {
    problem(refusal.id, refusal.message.clone())
}

fn ability_code(a: Ability) -> &'static str {
    ABILITIES.iter().find(|(x, _, _)| *x == a).map_or("", |(_, code, _)| code)
}

fn ability_option(a: Ability) -> SfOptionDto {
    let (_, code, name) = ABILITIES.iter().find(|(x, _, _)| *x == a).copied().unwrap_or((a, "", ""));
    SfOptionDto { id: code.to_owned(), label: name.to_owned() }
}

fn parse_ability(code: &str) -> Option<Ability> {
    ABILITIES.iter().find(|(_, c, name)| c.eq_ignore_ascii_case(code.trim()) || name.eq_ignore_ascii_case(code.trim())).map(|(a, _, _)| *a)
}

fn principals_of_kind<'a>(package: &'a SheetRulePackage, kind: &'a str) -> impl Iterator<Item = &'a SheetRule> + 'a {
    package.rules_of_kind(kind).filter(|r| !r.id.contains('#'))
}

fn siblings<'a>(package: &'a SheetRulePackage, id: &str) -> Vec<&'a SheetRule> {
    let prefix = format!("{id}#");
    package.rules.range(prefix.clone()..).take_while(|(k, _)| k.starts_with(&prefix)).map(|(_, r)| r).collect()
}

/// The race records the chassis can compute: each carries a racial Hit Points row.
pub fn race_options(package: &SheetRulePackage) -> Vec<SfOptionDto> {
    let mut out: Vec<SfOptionDto> = principals_of_kind(package, "race")
        .filter(|race| {
            std::iter::once(*race).chain(siblings(package, &race.id)).any(|r| r.target == Some(BonusTarget::Hp))
        })
        .map(|race| {
            // The `Race`-category ability the race grants carries the bare race name; the race
            // principal's own label names its first ability row (`Ysoki (Str)`).
            let name = package
                .granted_from(&race.id)
                .iter()
                .filter_map(|id| package.rule(id))
                .find(|r| r.pool == "race" && !r.id.contains('#'))
                .map_or_else(|| race.label.clone(), |r| r.label.clone());
            SfOptionDto { id: race.id.clone(), label: name }
        })
        .collect();
    out.sort_by(|a, b| a.label.cmp(&b.label).then(a.id.cmp(&b.id)));
    out
}

/// The theme records: `theme`-pool principals tagged `Theme Selection`.
pub fn theme_options(package: &SheetRulePackage) -> Vec<SfOptionDto> {
    let mut out: Vec<SfOptionDto> = package
        .rules
        .values()
        .filter(|r| !r.id.contains('#') && r.pool == "theme" && r.tags.iter().any(|t| t == "Theme Selection"))
        .map(|r| SfOptionDto { id: r.id.clone(), label: r.label.clone() })
        .collect();
    out.sort_by(|a, b| a.label.cmp(&b.label).then(a.id.cmp(&b.id)));
    out
}

/// The class records and each one's key-ability options.
pub fn class_options(package: &SheetRulePackage) -> Vec<SfClassOptionDto> {
    let mut out: Vec<SfClassOptionDto> = principals_of_kind(package, "class")
        .filter_map(|class| {
            let keys = sf_chassis::key_ability_options(class).ok()?;
            Some(SfClassOptionDto {
                id: class.id.clone(),
                label: class.label.clone(),
                key_ability_options: keys.into_iter().map(ability_option).collect(),
            })
        })
        .collect();
    out.sort_by(|a, b| a.label.cmp(&b.label).then(a.id.cmp(&b.id)));
    out
}

fn point_buy_rules() -> PointBuyRulesDto {
    PointBuyRulesDto { budget: POINT_BUY_BUDGET, max_score_at_creation: MAX_SCORE_AT_CREATION, source: SRD_POINT_BUY.to_owned() }
}

/// The `CharacterInput` the request describes at 1st level, with `auto_picks` held and the
/// given scores (the point buy before race and theme while the scores are being computed).
fn compose_input(request: &SfCreationRequest, auto_picks: &[String], answered: &[SfPickDto], scores: [i64; 6]) -> CharacterInput {
    let mut selected_feats: Vec<String> = request.theme_id.iter().cloned().collect();
    for pick in auto_picks {
        if !selected_feats.contains(pick) {
            selected_feats.push(pick.clone());
        }
    }
    let mut selected_choices: Vec<SelectedChoice> = Vec::new();
    if let (Some(class), Some(key)) = (&request.class_id, &request.key_ability) {
        selected_choices.push(SelectedChoice { choice_set_id: class.clone(), selection_id: key.clone() });
    }
    selected_choices.extend(
        request
            .picks
            .iter()
            .chain(answered)
            .map(|p| SelectedChoice { choice_set_id: p.slot_id.clone(), selection_id: p.option_id.clone() }),
    );
    let s = scores.map(|v| i16::try_from(v).unwrap_or(i16::MAX));
    CharacterInput {
        case_id: None,
        source_package_id: STARFINDER_RULE_SYSTEM_ID.to_owned(),
        chosen: ChosenCharacterState {
            race_id: request.race_id.clone().unwrap_or_default(),
            class_levels: request
                .class_id
                .iter()
                .map(|class_id| CharacterClassLevel { class_id: class_id.clone(), level: 1 })
                .collect(),
            ability_scores: AbilityScores {
                strength: s[0],
                dexterity: s[1],
                constitution: s[2],
                intelligence: s[3],
                wisdom: s[4],
                charisma: s[5],
            },
            selected_feats,
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices,
            selected_traits: Vec::new(),
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

fn ctx_of(sf: &SfHeld, id: &str) -> EvalContext {
    let principal = id.split('#').next().unwrap_or(id);
    let entry = sf.held.rules.get(id).or_else(|| sf.held.rules.get(principal)).cloned().unwrap_or_default();
    EvalContext { holder_class: entry.holder_class, spell_level: entry.spell_level.unwrap_or(0), item_tags: Vec::new() }
}

fn count_of(package: &SheetRulePackage, sf: &SfHeld, rule: &SheetRule, expr: &codex::rules_core::sheet_rule::Expr) -> i64 {
    let n = evaluate_expr(expr, &sf.held, package, &sf.facts, ctx_of(sf, &rule.id));
    if n.den == 0 { 0 } else { n.num / n.den }
}

/// The skills an option set names (every base skill when it names none, or `all`): the
/// package's `Base`-tagged skill records, not their `Display` twins.
fn skill_options(package: &SheetRulePackage, listed: &[String]) -> Vec<SfOptionDto> {
    let every = listed.is_empty() || listed.iter().any(|s| s == "all");
    let mut seen = BTreeSet::new();
    let mut out: Vec<SfOptionDto> = principals_of_kind(package, "skill")
        .filter(|r| r.tags.iter().any(|t| t == "Base"))
        .filter_map(|r| {
            let slug = split_rule_id(&r.id).2.to_string();
            (every || listed.contains(&slug)).then(|| (slug, r.label.clone()))
        })
        .filter(|(slug, _)| seen.insert(slug.clone()))
        .map(|(id, label)| SfOptionDto { id, label })
        .collect();
    out.sort_by(|a, b| a.label.cmp(&b.label));
    out
}

/// Every rule the held set holds, each once: the held records and their `#` siblings.
fn held_rules<'a>(package: &'a SheetRulePackage, sf: &SfHeld) -> Vec<&'a SheetRule> {
    let mut rules: Vec<&SheetRule> = Vec::new();
    let mut seen = BTreeSet::new();
    for id in sf.held.rules.keys().filter(|id| !sf.held.removed.contains(*id)) {
        let Some(rule) = package.rule(id) else { continue };
        for r in std::iter::once(rule).chain(siblings(package, id)) {
            // The held set holds whole records; a sibling with a gate of its own (a level- or
            // archetype-gated pool row) counts only when its gate includes this character.
            let holds = !r.id.contains('#')
                || evaluate_applies(&r.applies, &sf.held, package, &sf.facts, ctx_of(sf, &r.id)).includes();
            if holds && !sf.held.removed.contains(&r.id) && seen.insert(r.id.clone()) {
                rules.push(r);
            }
        }
    }
    rules
}

/// What the held set asks for, and the pool records it holds on the player's behalf.
struct Discovery {
    slots: Vec<SfCreationSlotDto>,
    chosen_on_the_sheet: Vec<String>,
    auto_picks: Vec<String>,
    /// The class's key-ability pick as the package states it (`Class ~ Soldier` offers
    /// `Strength` / `Dexterity`): the choosing template and each option's ability. The form's
    /// key-ability field answers it, so it is not asked twice.
    key_ability_slots: Vec<(String, Vec<(Ability, String)>)>,
}

fn discover(package: &SheetRulePackage, sf: &SfHeld, pool_counts_from: &BTreeMap<String, i64>) -> Discovery {
    let mut slots: Vec<SfCreationSlotDto> = Vec::new();
    let mut later: Vec<String> = Vec::new();
    let mut pools: BTreeMap<String, (i64, Vec<String>)> = BTreeMap::new();
    let mut auto_picks: Vec<String> = Vec::new();
    let mut key_ability_slots: Vec<(String, Vec<(Ability, String)>)> = Vec::new();

    for rule in held_rules(package, sf) {
        let own_offer = rule.offers.as_ref().filter(|o| o.id == rule.id);
        if let (Some(BonusTarget::Pool(pool)), SheetValue::Number(value), None) = (&rule.target, &rule.value, own_offer) {
            let entry = pools.entry(pool.clone()).or_default();
            entry.0 += count_of(package, sf, rule, value);
            entry.1.push(rule.label.clone());
        }
        let Some(offer) = own_offer else { continue };
        let count = count_of(package, sf, rule, &offer.count);
        // A record held for a pool (`2_racial_stat_bonus`) answers its own choice once per pool point.
        let count = pool_counts_from.get(&rule.id).map_or(count, |n| count * n);
        if count <= 0 {
            continue;
        }
        match &offer.from {
            OptionSet::Rules { pool, .. } if FEAT_POOLS.contains(&pool.as_str()) => {
                later.push(format!("{} ({count} {pool} pick{})", rule.label, if count == 1 { "" } else { "s" }));
            }
            OptionSet::Rules { .. } => {
                if !offer_open(package, &sf.held, &sf.facts, rule) {
                    continue;
                }
                let members: Vec<&SheetRule> = package.rules.values().filter(|r| offer_selects(offer, r)).collect();
                if members.is_empty() {
                    later.push(format!("{} (no option in the package)", rule.label));
                    continue;
                }
                let abilities: Option<Vec<(Ability, String)>> =
                    members.iter().map(|r| parse_ability(&r.label).map(|a| (a, r.id.clone()))).collect();
                if let (Some(abilities), "template") = (abilities, split_rule_id(&rule.id).1) {
                    key_ability_slots.push((rule.id.clone(), abilities));
                    continue;
                }
                let default_option_id =
                    members.iter().find(|r| r.tags.iter().any(|t| t.starts_with("No "))).map(|r| r.id.clone());
                let required = members.iter().any(|m| {
                    std::iter::once(*m).chain(siblings(package, &m.id)).any(|r| matches!(r.target, Some(BonusTarget::Ability(_))))
                });
                let mut options: Vec<SfOptionDto> =
                    members.iter().map(|r| SfOptionDto { id: r.id.clone(), label: r.label.clone() }).collect();
                options.sort_by(|a, b| a.label.cmp(&b.label).then(a.id.cmp(&b.id)));
                slots.push(SfCreationSlotDto {
                    slot_id: rule.id.clone(),
                    label: rule.label.clone(),
                    kind: "option".into(),
                    count: count as usize,
                    repeatable: false,
                    options,
                    default_option_id,
                    required,
                });
            }
            OptionSet::Skills(listed) if !listed.iter().any(|s| s == "crossclass") => {
                slots.push(SfCreationSlotDto {
                    slot_id: rule.id.clone(),
                    label: rule.label.clone(),
                    kind: "skill".into(),
                    count: count as usize,
                    repeatable: false,
                    options: skill_options(package, listed),
                    default_option_id: None,
                    required: false,
                });
            }
            OptionSet::FreeText if rule.target == Some(BonusTarget::Chosen(rule.id.clone())) => {
                slots.push(SfCreationSlotDto {
                    slot_id: rule.id.clone(),
                    label: rule.label.clone(),
                    kind: "ability".into(),
                    count: count as usize,
                    repeatable: rule.repeatable,
                    options: ABILITIES.iter().map(|(a, _, _)| ability_option(*a)).collect(),
                    default_option_id: None,
                    required: true,
                });
            }
            OptionSet::Languages(_) => later.push(format!("{} (languages)", rule.label)),
            _ => later.push(rule.label.clone()),
        }
    }

    for (pool, (n, from)) in pools {
        if n <= 0 {
            continue;
        }
        if FEAT_POOLS.contains(&pool.as_str()) {
            later.push(format!("{} ({n} feat{})", from.join(", "), if n == 1 { "" } else { "s" }));
            continue;
        }
        // The pool's one record: an ability-category pick named by the pool itself.
        let record = package.find("ability", &pool).and_then(|id| package.rule(id)).filter(|r| r.offers.is_some());
        match record {
            Some(r) => auto_picks.push(r.id.clone()),
            None => later.push(format!("{} ({n} {} pick{})", from.join(", "), pool.replace('_', " "), if n == 1 { "" } else { "s" })),
        }
    }
    later.sort();
    later.dedup();
    Discovery { slots, chosen_on_the_sheet: later, auto_picks, key_ability_slots }
}

/// The pool counts for each record a pool holds (the record's id -> pool points).
fn pool_counts(package: &SheetRulePackage, sf: &SfHeld) -> BTreeMap<String, i64> {
    let mut out: BTreeMap<String, i64> = BTreeMap::new();
    for r in held_rules(package, sf) {
        if r.offers.as_ref().is_some_and(|o| o.id == r.id) {
            continue;
        }
        if let (Some(BonusTarget::Pool(pool)), SheetValue::Number(value)) = (&r.target, &r.value) {
            if let Some(record) = package.find("ability", pool) {
                *out.entry(record.clone()).or_default() += count_of(package, sf, r, value);
            }
        }
    }
    out
}

/// The key ability the request settles: the class's only one, or the player's choice.
fn chosen_key_ability(package: &SheetRulePackage, request: &SfCreationRequest) -> Option<Ability> {
    let class = package.rule(request.class_id.as_deref()?)?;
    match sf_chassis::key_ability_options(class).ok()?.as_slice() {
        [only] => Some(*only),
        options => request.key_ability.as_deref().and_then(parse_ability).filter(|a| options.contains(a)),
    }
}

/// The held set of the request at 1st level, with every pool record held and the package's
/// key-ability pick answered, iterated until both settle.
fn settle(package: &SheetRulePackage, request: &SfCreationRequest) -> Result<(SfHeld, Discovery, CharacterInput), SfChassisRefusal> {
    let points = request.point_buy.as_array();
    let key = chosen_key_ability(package, request);
    let mut auto: Vec<String> = Vec::new();
    let mut answered: Vec<SfPickDto> = Vec::new();
    let mut last = None;
    for _ in 0..5 {
        let input = compose_input(request, &auto, &answered, points.map(|p| 10 + p));
        let (build, _) = sf_adapter::build_from_input(package, &input)?;
        let sf = sf_defense::held(package, &build)?;
        let counts = pool_counts(package, &sf);
        let found = discover(package, &sf, &counts);
        let mut changed = false;
        for p in &found.auto_picks {
            if !auto.contains(p) {
                auto.push(p.clone());
                changed = true;
            }
        }
        for (slot_id, options) in &found.key_ability_slots {
            if let Some((_, option)) = options.iter().find(|(a, _)| Some(*a) == key) {
                let pick = SfPickDto { slot_id: slot_id.clone(), option_id: option.clone() };
                if !answered.contains(&pick) {
                    answered.push(pick);
                    changed = true;
                }
            }
        }
        if !changed {
            return Ok((sf, found, input));
        }
        last = Some((sf, found, input));
    }
    Ok(last.expect("the loop ran"))
}

/// The answered picks checked against the slots: every slot full, every pick offered.
fn pick_problems(request: &SfCreationRequest, slots: &[SfCreationSlotDto]) -> Vec<DiagnosticDto> {
    let mut out = Vec::new();
    for slot in slots {
        let chosen: Vec<&str> =
            request.picks.iter().filter(|p| p.slot_id == slot.slot_id).map(|p| p.option_id.as_str()).collect();
        if chosen.len() > slot.count {
            out.push(problem(
                REFUSED_CHOICE_NOT_OFFERED,
                format!("{}: {} chosen, {} offered", slot.label, chosen.len(), slot.count),
            ));
        } else if chosen.len() < slot.count {
            out.push(DiagnosticDto {
                id: if slot.required { REFUSED_CHOICE_UNFILLED } else { NOT_CHOSEN_YET }.to_owned(),
                message: format!("{}: choose {} ({} chosen)", slot.label, slot.count, chosen.len()),
                claim_blocking: slot.required,
            });
        }
        let distinct: BTreeSet<&str> = chosen.iter().copied().collect();
        if !slot.repeatable && distinct.len() != chosen.len() {
            out.push(problem(REFUSED_CHOICE_NOT_OFFERED, format!("{}: the same option chosen twice", slot.label)));
        }
        for option in &chosen {
            if !slot.options.iter().any(|o| o.id == *option) {
                out.push(problem(REFUSED_CHOICE_NOT_OFFERED, format!("{}: {option} is not one of its options", slot.label)));
            }
        }
    }
    for pick in &request.picks {
        if !slots.iter().any(|s| s.slot_id == pick.slot_id) {
            out.push(problem(REFUSED_CHOICE_NOT_OFFERED, format!("{}: no such choice for this character", pick.slot_id)));
        }
    }
    out
}

/// The creation form's next state for `request` (`preview_starfinder_character`).
pub fn preview(package: &SheetRulePackage, request: &SfCreationRequest) -> (SfCreationPreviewDto, Option<CharacterInput>) {
    let races = race_options(package);
    let themes = theme_options(package);
    let classes = class_options(package);
    let points = request.point_buy.as_array();
    let points_spent: i64 = points.iter().sum();
    let mut out = SfCreationPreviewDto {
        races,
        themes,
        classes,
        point_buy_rules: point_buy_rules(),
        slots: Vec::new(),
        chosen_on_the_sheet: Vec::new(),
        ability_scores: Vec::new(),
        points_spent,
        points_unspent: POINT_BUY_BUDGET - points_spent,
        problems: Vec::new(),
    };
    for (field, value, list, id) in [
        ("race", &request.race_id, &out.races, REFUSED_NO_RACE),
        ("theme", &request.theme_id, &out.themes, REFUSED_NO_THEME),
    ] {
        match value {
            None => out.problems.push(problem(id, format!("choose a {field}"))),
            Some(v) if !list.iter().any(|o| &o.id == v) => {
                out.problems.push(problem(REFUSED_UNKNOWN_RECORD, format!("{v}: not a Starfinder {field} this flow offers")))
            }
            Some(_) => {}
        }
    }
    match &request.class_id {
        None => out.problems.push(problem(REFUSED_NO_CLASS, "choose a class")),
        Some(c) => match out.classes.iter().find(|o| &o.id == c) {
            None => out.problems.push(problem(REFUSED_UNKNOWN_RECORD, format!("{c}: not a Starfinder class this flow offers"))),
            Some(class) if class.key_ability_options.len() > 1 => {
                let ok = request
                    .key_ability
                    .as_deref()
                    .and_then(parse_ability)
                    .is_some_and(|a| class.key_ability_options.iter().any(|o| o.id == ability_code(a)));
                if !ok {
                    out.problems.push(problem(
                        sf_chassis::REFUSED_KEY_ABILITY,
                        format!("{}: choose the key ability ({})", class.label, class.key_ability_options.iter().map(|o| o.label.as_str()).collect::<Vec<_>>().join(" or ")),
                    ));
                }
            }
            Some(_) => {}
        },
    }
    if !out.problems.iter().all(|p| p.id != REFUSED_NO_RACE && p.id != REFUSED_UNKNOWN_RECORD) {
        return (out, None);
    }
    let (sf, found, input) = match settle(package, request) {
        Ok(settled) => settled,
        Err(refusal) => {
            out.problems.push(refusal_problem(&refusal));
            return (out, None);
        }
    };
    out.problems.extend(pick_problems(request, &found.slots));
    out.slots = found.slots;
    out.chosen_on_the_sheet = found.chosen_on_the_sheet;
    let _ = sf;
    let (build, _) = match sf_adapter::build_from_input(package, &input) {
        Ok(b) => b,
        Err(refusal) => {
            out.problems.push(refusal_problem(&refusal));
            return (out, None);
        }
    };
    let abilities = SfAbilityBuild { point_buy: points, increases: BTreeMap::new() };
    match sf_abilities::compute(package, &build, &abilities) {
        Ok(scores) => {
            out.ability_scores = ABILITIES
                .iter()
                .enumerate()
                .map(|(i, (_, code, name))| SfAbilityScoreDto {
                    ability: (*code).to_owned(),
                    label: (*name).to_owned(),
                    score: scores.scores[i].total,
                    modifier: sf_chassis::ability_modifier(scores.scores[i].total),
                    terms: scores.scores[i].terms.iter().map(|t| SfScoreTermDto { label: t.label.clone(), value: t.value }).collect(),
                })
                .collect();
            out.points_unspent = scores.points_unspent;
            let finals = scores.finals();
            let mut input = input;
            let s = finals.map(|v| i16::try_from(v).unwrap_or(i16::MAX));
            input.chosen.ability_scores =
                AbilityScores { strength: s[0], dexterity: s[1], constitution: s[2], intelligence: s[3], wisdom: s[4], charisma: s[5] };
            (out, Some(input))
        }
        Err(refusal) => {
            out.problems.push(refusal_problem(&refusal));
            (out, None)
        }
    }
}

/// Creates the character the request describes at `root`: recomputed with every Starfinder
/// total and saved, or refused with every problem and nothing saved.
pub fn create_at_root(
    package: &SheetRulePackage,
    root: &Path,
    request: &SfCreationRequest,
    app_version: String,
) -> Result<SfCreateResponse, String> {
    let (preview, input) = preview(package, request);
    let mut diagnostics: Vec<DiagnosticDto> = preview.problems.into_iter().filter(|p| p.claim_blocking).collect();
    if request.display_label.trim().is_empty() {
        diagnostics.push(problem(REFUSED_NO_NAME, "enter the character's name"));
    }
    let input = match input {
        Some(input) if diagnostics.is_empty() => input,
        _ => return Ok(SfCreateResponse::Blocked { diagnostics }),
    };
    let sheet = match sf_adapter::compute_sheet(package, &input) {
        Ok(sheet) => sheet,
        Err(refusal) => return Ok(SfCreateResponse::Blocked { diagnostics: vec![refusal_problem(&refusal)] }),
    };
    let envelope = SavedCharacterEnvelope {
        character_id: request.character_id.clone(),
        revision_id: format!("{}.rev.1", request.character_id),
        revision_kind: SavedCharacterRevisionKind::Authoritative,
        saved_at: request.saved_at.clone(),
        schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
        app_or_runtime_version: app_version,
        content_or_rules_provenance: STARFINDER_RULE_SYSTEM_ID.to_owned(),
        game_system: STARFINDER_RULE_SYSTEM_ID.to_owned(),
        latest_authoritative_revision_ref: format!("{}.rev.1", request.character_id),
        display_label: request.display_label.trim().to_owned(),
        character_input: input,
    };
    SavedCharacterStore::save(&envelope, root).map_err(|err| err.message)?;
    Ok(SfCreateResponse::Saved {
        summary: Box::new(sf_adapter::summary_dto(&envelope)),
        explanations: sheet
            .explanations()
            .into_iter()
            .map(|e| ExplanationDto { id: e.id, value: e.value, detail: e.detail })
            .collect(),
    })
}

fn package_or_error() -> Result<&'static SheetRulePackage, String> {
    sf_adapter::package().map_err(|r| format!("{}: {}", r.id, r.message))
}

/// The Starfinder creation form's next state: the race, theme and class lists, the picks the
/// chosen race, theme and class ask for, and the engine's point-buy scores.
#[tauri::command]
pub fn preview_starfinder_character(request: SfCreationRequest) -> Result<SfCreationPreviewDto, String> {
    Ok(preview(package_or_error()?, &request).0)
}

/// Creates and saves a Starfinder 1e character at 1st level.
#[tauri::command]
pub fn create_starfinder_character(app: tauri::AppHandle, request: SfCreationRequest) -> Result<SfCreateResponse, String> {
    let root = crate::character_hub::resolve_character_root(&app, &request.character_id)?;
    create_at_root(package_or_error()?, &root, &request, app.package_info().version.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    use crate::rule_system_adapter::RuleSystemAdapter;
    use crate::sf_adapter::StarfinderAdapter;

    const SEED_BUILDS: &str = "docs/release/SD-37-starfinder-1e/artifacts/epic_0/seed-builds.md";

    fn package() -> &'static SheetRulePackage {
        sf_adapter::package().expect("the Starfinder package loads")
    }

    fn tempdir(label: &str) -> PathBuf {
        let unique = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("time").as_nanos();
        let path = std::env::temp_dir().join(format!("codex-sf-creation-{label}-{}-{unique}", std::process::id()));
        std::fs::create_dir_all(&path).expect("temp dir");
        path
    }

    fn pick(slot: &str, option: &str) -> SfPickDto {
        SfPickDto { slot_id: slot.to_owned(), option_id: option.to_owned() }
    }

    fn points([strength, dexterity, constitution, intelligence, wisdom, charisma]: [i64; 6]) -> PointBuyDto {
        PointBuyDto { strength, dexterity, constitution, intelligence, wisdom, charisma }
    }

    fn request(race: &str, theme: &str, class: &str, key: Option<&str>, spent: [i64; 6], picks: Vec<SfPickDto>) -> SfCreationRequest {
        SfCreationRequest {
            character_id: format!("sf-create-{}", split_rule_id(class).2),
            display_label: format!("{} {}", split_rule_id(race).2, split_rule_id(class).2),
            saved_at: "2026-10-05T00:00:00Z".into(),
            race_id: Some(race.into()),
            theme_id: Some(theme.into()),
            class_id: Some(class.into()),
            key_ability: key.map(str::to_owned),
            point_buy: points(spent),
            picks,
        }
    }

    /// Each seed's `Points spent` row and its creation-score row (`At 1st level`, or `**Final**`
    /// for a seed with no 5th-level increase) from `seed-builds.md`: bytes the engine does not read.
    fn seed_rows(section: &str) -> ([i64; 6], [i64; 6]) {
        let text = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(SEED_BUILDS)).expect("seed-builds.md");
        let body: String = text.split("\n## ").find(|s| s.starts_with(section)).expect("seed section").to_owned();
        let row = |prefix: &str| -> Option<[i64; 6]> {
            let line = body.lines().find(|l| l.starts_with(prefix))?;
            let cells: Vec<i64> = line
                .split('|')
                .skip(2)
                .take(6)
                .map(|c| c.trim().trim_matches('*').trim_start_matches('+').replace('\u{2212}', "-"))
                .map(|c| if c.is_empty() { 0 } else { c.parse().expect("number") })
                .collect();
            Some(cells.try_into().expect("six cells"))
        };
        let spent = row("| Points spent").expect("points row");
        let at_creation = row("| At 1st level").or_else(|| row("| **Final**")).expect("score row");
        (spent, at_creation)
    }

    fn scores(preview: &SfCreationPreviewDto) -> Vec<i64> {
        preview.ability_scores.iter().map(|s| s.score).collect()
    }

    fn slot<'a>(preview: &'a SfCreationPreviewDto, label_part: &str) -> &'a SfCreationSlotDto {
        preview
            .slots
            .iter()
            .find(|s| s.label.contains(label_part))
            .unwrap_or_else(|| panic!("no slot {label_part:?} in {:?}", preview.slots.iter().map(|s| &s.label).collect::<Vec<_>>()))
    }

    /// The four seeds as creation requests (`seed-builds.md` §1-§4: race, theme, class, key
    /// ability and the racial/theme picks), each with its own `Points spent` row.
    fn seed_requests() -> Vec<(&'static str, SfCreationRequest, [i64; 6])> {
        let (soldier_spent, soldier_scores) = seed_rows("1. SF-Soldier-3");
        let (mystic_spent, mystic_scores) = seed_rows("2. SF-Mystic-5");
        let (tech_spent, tech_scores) = seed_rows("3. SF-Technomancer-5");
        let (envoy_spent, envoy_scores) = seed_rows("4. SF-Envoy-3");
        vec![
            (
                "SF-Soldier-3",
                request("core:race:human", "core:ability:mercenary", "core:class:soldier", Some("STR"), soldier_spent, vec![
                    pick("core:ability:2_racial_stat_bonus", "STR"),
                    pick("core:ability:human", "core:ability:human_no_variant_ability"),
                    pick("core:ability:soldier", "core:ability:soldier_no_archetype"),
                    pick("core:ability:soldier_class_feature_primary_fighting_style", "core:ability:primary_fighting_style_sharpshoot"),
                ]),
                soldier_scores,
            ),
            (
                "SF-Mystic-5",
                request("core:race:lashunta", "core:ability:priest", "core:class:mystic", None, mystic_spent, vec![
                    pick("core:ability:lashunta", "core:ability:lashunta_no_variant_ability"),
                    pick("core:ability:mystic", "core:ability:mystic_no_archetype"),
                    pick("core:ability:mystic_class_feature_connection", "core:ability:empath"),
                    pick("core:ability:lashunta_default_dimorphic", "core:ability:lashunta_subrace_damaya"),
                    pick("core:ability:2_racial_bonus_to_skill", "diplomacy"),
                    pick("core:ability:2_racial_bonus_to_skill", "medicine"),
                ]),
                mystic_scores,
            ),
            (
                "SF-Technomancer-5",
                request("core:race:android", "core:ability:scholar", "core:class:technomancer", None, tech_spent, vec![
                    pick("core:ability:android", "core:ability:android_no_no_variant_ability"),
                    pick("core:ability:scholar_theme_benefit_theme_knowledge", "core:pool_option:scholar_theme_chosen_skill_physical_science"),
                    pick("core:pool_option:scholar_theme_chosen_skill_physical_science", "core:ability:physical_science_specialty_physics"),
                ]),
                tech_scores,
            ),
            (
                "SF-Envoy-3",
                request("core:race:ysoki", "core:ability:icon", "core:class:envoy", None, envoy_spent, vec![
                    pick("core:ability:ysoki", "core:ability:ysoki_no_variant_ability"),
                    pick("core:ability:envoy_class_feature_envoy_improvisation", "core:ability:envoy_improvisation_inspiring_boost"),
                ]),
                envoy_scores,
            ),
        ]
    }

    /// The lists come from the package: the four seed races (and no drone frame, which has no
    /// racial Hit Points), the four seed themes, the Core classes with their key abilities, and
    /// the engine's point-buy budget and creation cap.
    #[test]
    fn the_creation_lists_come_from_the_starfinder_package() {
        let (preview, _) = preview(package(), &SfCreationRequest::default());
        let race_ids: Vec<&str> = preview.races.iter().map(|r| r.id.as_str()).collect();
        for race in ["core:race:human", "core:race:lashunta", "core:race:android", "core:race:ysoki"] {
            assert!(race_ids.contains(&race), "{race} in {race_ids:?}");
        }
        assert!(!race_ids.contains(&"core:race:drone"), "a drone frame is not a playable race");
        let ysoki = preview.races.iter().find(|r| r.id == "core:race:ysoki").unwrap();
        assert_eq!(ysoki.label, "Ysoki");
        let theme_ids: Vec<&str> = preview.themes.iter().map(|r| r.id.as_str()).collect();
        for theme in ["core:ability:mercenary", "core:ability:priest", "core:ability:scholar", "core:ability:icon", "core:ability:themeless"] {
            assert!(theme_ids.contains(&theme), "{theme} in {theme_ids:?}");
        }
        let soldier = preview.classes.iter().find(|c| c.id == "core:class:soldier").expect("soldier");
        assert_eq!(soldier.key_ability_options.iter().map(|o| o.id.as_str()).collect::<Vec<_>>(), ["STR", "DEX"]);
        let envoy = preview.classes.iter().find(|c| c.id == "core:class:envoy").expect("envoy");
        assert_eq!(envoy.key_ability_options.iter().map(|o| o.id.as_str()).collect::<Vec<_>>(), ["CHA"]);
        assert_eq!(preview.point_buy_rules.budget, 10);
        assert_eq!(preview.point_buy_rules.max_score_at_creation, 18);
        assert!(preview.ability_scores.is_empty(), "no scores before a race is chosen");
        let ids: Vec<&str> = preview.problems.iter().map(|p| p.id.as_str()).collect();
        assert_eq!(ids, [REFUSED_NO_RACE, REFUSED_NO_THEME, REFUSED_NO_CLASS]);
    }

    /// A human asks for its `+2 Racial Stat Bonus` (any one ability: the oracle's
    /// `Human Race Selection ~ Default` `BONUS:ABILITYPOOL|+2 Racial Stat Bonus|1`), and the
    /// chosen ability moves that score by 2; a half-elf asks for the same pick.
    #[test]
    fn a_human_asks_for_its_racial_stat_bonus_and_the_score_moves() {
        for race in ["core:race:human", "core:race:half_elf"] {
            let r = request(race, "core:ability:mercenary", "core:class:envoy", None, [0; 6], Vec::new());
            let (open, _) = preview(package(), &r);
            let bonus = slot(&open, "+2 Racial Stat Bonus");
            assert_eq!((bonus.kind.as_str(), bonus.count), ("ability", 1), "{race}");
            assert_eq!(bonus.options.len(), 6, "{race}");
            assert!(open.problems.iter().any(|p| p.id == REFUSED_CHOICE_UNFILLED), "{race}: an open pick blocks creation");
            let mut chosen = r.clone();
            chosen.picks.push(pick(&bonus.slot_id, "CON"));
            let (after, _) = preview(package(), &chosen);
            let con = |p: &SfCreationPreviewDto| p.ability_scores[2].score;
            assert_eq!(con(&after), con(&open) + 2, "{race}: the +2 lands on Con");
        }
    }

    /// The four seeds created through the flow: each creation score equals the seed's
    /// `seed-builds.md` row (24 of 24), and the saved character loads through
    /// `StarfinderAdapter` with no blocking diagnostic and its `sf.*` rows.
    #[test]
    fn the_seed_builds_created_through_the_flow_have_their_creation_scores() {
        let characters = tempdir("seeds");
        let mut checked = 0;
        for (seed, mut r, want) in seed_requests() {
            let (open, _) = preview(package(), &r);
            assert!(open.problems.iter().all(|p| !p.claim_blocking), "{seed}: {:?} (slots {:?})", open.problems, open.slots);
            assert_eq!(scores(&open), want.to_vec(), "{seed}");
            checked += 6;
            r.character_id = seed.to_ascii_lowercase();
            let root = characters.join(&r.character_id);
            let created = create_at_root(package(), &root, &r, "test".into()).expect("creates");
            let SfCreateResponse::Saved { summary, explanations } = created else { panic!("{seed}: {created:?}") };
            assert_eq!(summary.game_system, STARFINDER_RULE_SYSTEM_ID);
            assert!(explanations.iter().any(|e| e.id == "sf.stamina"), "{seed}");
            let loaded = StarfinderAdapter.load_saved_character(&root).expect("loads");
            assert!(loaded.diagnostics.iter().all(|d| !d.claim_blocking), "{seed}: {:?}", loaded.diagnostics);
            let a = &loaded.ability_scores;
            assert_eq!(
                [a.strength, a.dexterity, a.constitution, a.intelligence, a.wisdom, a.charisma].map(i64::from).to_vec(),
                want.to_vec(),
                "{seed}: saved final scores"
            );
        }
        assert_eq!(checked, 24);
        std::fs::remove_dir_all(&characters).ok();
    }

    /// A refused build saves nothing: over the 10-point budget, a score above 18 at creation,
    /// an open pick, a missing name.
    #[test]
    fn a_refused_build_is_blocked_and_nothing_is_saved() {
        let characters = tempdir("refused");
        let (_, soldier, _) = seed_requests().remove(0);
        let mut over = soldier.clone();
        over.point_buy.charisma += 1;
        let mut cap = soldier.clone();
        cap.point_buy = points([8, 0, 0, 0, 0, 0]);
        let mut open = soldier.clone();
        open.picks.retain(|p| p.slot_id != "core:ability:2_racial_stat_bonus");
        let mut unnamed = soldier.clone();
        unnamed.display_label = "  ".into();
        for (r, want) in [
            (over, sf_abilities::REFUSED_POINT_BUY_OVER_BUDGET),
            (cap, sf_abilities::REFUSED_SCORE_OVER_18_AT_CREATION),
            (open, REFUSED_CHOICE_UNFILLED),
            (unnamed, REFUSED_NO_NAME),
        ] {
            let root = characters.join(want.replace('.', "-"));
            let created = create_at_root(package(), &root, &r, "test".into()).expect("answers");
            let SfCreateResponse::Blocked { diagnostics } = created else { panic!("{want}: saved {created:?}") };
            assert!(diagnostics.iter().any(|d| d.id == want && d.claim_blocking), "{want}: {diagnostics:?}");
            assert!(!root.join("character.json").exists() && SavedCharacterStore::load(&root).is_err(), "{want}: nothing saved");
        }
        std::fs::remove_dir_all(&characters).ok();
    }

    /// The soldier's key ability is asked once: the class's `Str or Dex` choice. The package's
    /// own key-ability pick (`Class ~ Soldier` offers `Strength` / `Dexterity`) is answered by it,
    /// not asked again, and is saved with the character; with no choice the build is refused.
    #[test]
    fn the_key_ability_is_asked_once_and_answers_the_packages_own_pick() {
        let (_, soldier, _) = seed_requests().remove(0);
        let (open, input) = preview(package(), &soldier);
        assert!(!open.slots.iter().any(|s| s.slot_id == "core:template:class_soldier"), "{:?}", open.slots);
        let input = input.expect("computes");
        let has = |set: &str, sel: &str| input.chosen.selected_choices.iter().any(|c| c.choice_set_id == set && c.selection_id == sel);
        assert!(has("core:class:soldier", "STR"), "the chassis reads the class-id choice");
        assert!(has("core:template:class_soldier", "core:pool_option:soldier_key_ability_strength"), "the package pick is answered");
        let mut none = soldier.clone();
        none.key_ability = None;
        let (refused, _) = preview(package(), &none);
        assert!(refused.problems.iter().any(|p| p.id == sf_chassis::REFUSED_KEY_ABILITY && p.claim_blocking), "{:?}", refused.problems);
        let mut illegal = soldier.clone();
        illegal.key_ability = Some("WIS".into());
        let (refused, _) = preview(package(), &illegal);
        assert!(refused.problems.iter().any(|p| p.id == sf_chassis::REFUSED_KEY_ABILITY), "{:?}", refused.problems);
    }

    /// A lashunta asks for its subrace (damaya or korasha, the converted options) and its two
    /// Student skills; the subrace's adjustments come from the chosen record.
    #[test]
    fn a_lashunta_asks_for_its_subrace_and_student_skills() {
        let r = request("core:race:lashunta", "core:ability:priest", "core:class:mystic", None, [0; 6], Vec::new());
        let (open, _) = preview(package(), &r);
        let subrace = slot(&open, "DIMORPHIC");
        let ids: Vec<&str> = subrace.options.iter().map(|o| o.id.as_str()).collect();
        assert!(ids.contains(&"core:ability:lashunta_subrace_damaya") && ids.contains(&"core:ability:lashunta_subrace_korasha"), "{ids:?}");
        let student = slot(&open, "+2 Racial Bonus to Skill");
        assert_eq!((student.kind.as_str(), student.count), ("skill", 2));
        let mut korasha = r.clone();
        korasha.picks.push(pick(&subrace.slot_id, "core:ability:lashunta_subrace_korasha"));
        let mut damaya = r.clone();
        damaya.picks.push(pick(&subrace.slot_id, "core:ability:lashunta_subrace_damaya"));
        let (k, _) = preview(package(), &korasha);
        let (d, _) = preview(package(), &damaya);
        assert_ne!(scores(&k), scores(&d), "the subrace's own rows move the scores");
    }
}
