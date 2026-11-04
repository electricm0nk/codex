//! The level ledger: at what character level each feat, spell, skill rank and ability increase was
//! taken, saved as `ledger.json` beside the character (the `custom.json` precedent). The engine never
//! reads it and no number depends on it, so `selected_feats` and the saved character stay as they are.
//!
//! The ledger follows what the character holds rather than what a command says it did. Every
//! Pathfinder change passes `mutate_saved_character_at_root`, and the one other way a selection
//! changes is an ability grant on the Custom tab; each of those states what is held before and after
//! ([`ledger_after_change_at_root`], [`ledger_after_custom_change_at_root`]). A held thing with no row
//! gets one at the level the character is now, and a row nothing holds goes. So a new command cannot
//! forget to stamp, and a level-up followed by its feat stamps the new level.
//!
//! An ability increase is a positive `ability:<name>` grant, one row a point: the Pathfinder sheet
//! has no other writer of a score after creation, and a penalty is not an increase.
//!
//! A character saved before there was a ledger gets one on its first load, or at its first change
//! if that comes sooner ([`rows_of_a_character_with_no_ledger`]): each thing it holds goes in the
//! earliest slot of its kind, in saved order, marked as worked out afterwards. What no slot fits has
//! no level. The rules numbers are the engine's, read off the character as it stood at each level.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::character_custom::{load_character_custom_at_root, CharacterCustomDto};
use crate::character_hub::{feat_budget_for, load_character_hit_points_at_root, sheet_lines_for, SheetLineDto};
use crate::feat_catalog::build_feat_catalog;
use codex::rules_core::character_input::{CharacterClassLevel, CharacterInput};
use codex::rules_core::class_seeds::canonical_seeds_for;
use codex::rules_core::feat_budget::feat_slot_levels;
use codex::rules_core::pilot_compute::{compute_pilot_base_chassis, spells_known_table, PilotBaseChassisComputation};
use codex::rules_core::rules_catalog::class_spell_levels::class_spell_level;
use codex::rules_core::{feat_effects, feat_identity};
use codex::saved_character::local_store::SavedCharacterStore;

const LEDGER_FILE_NAME: &str = "ledger.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LedgerKind {
    Feat,
    Spell,
    SkillRank,
    AbilityIncrease,
}

/// One choice. `id` is the id the character holds it by. `target` is a chooser feat's target, a
/// spell's source class, or the id of the grant behind an ability increase. One skill rank is one row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerEntryDto {
    pub kind: LedgerKind,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// `None` only on a row of a character saved before there was a ledger that no slot fits.
    pub taken_at_level: Option<u8>,
    /// The level was worked out afterwards, not recorded when the choice was made.
    #[serde(default)]
    pub assigned_on_upgrade: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterLedgerDto {
    #[serde(default)]
    pub entries: Vec<LedgerEntryDto>,
}

/// A thing the character holds that the ledger has a row for.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Held {
    kind: LedgerKind,
    id: String,
    target: Option<String>,
}

fn held_by(input: &CharacterInput, custom: &CharacterCustomDto) -> Vec<Held> {
    let chosen = &input.chosen;
    let targets = feat_effects::chosen_feat_targets(&chosen.selected_feats, &chosen.selected_choices);
    let mut held = Vec::new();
    for (index, feat) in chosen.selected_feats.iter().enumerate() {
        // Targets are recorded per feat, not per copy: the nth copy has the nth target, if any.
        let copy = chosen.selected_feats[..index].iter().filter(|earlier| feat_identity::same(earlier, feat)).count();
        let target = targets.iter().find(|entry| feat_identity::same(&entry.feat_id, feat)).and_then(|entry| entry.targets.get(copy));
        held.push(Held { kind: LedgerKind::Feat, id: feat.clone(), target: target.cloned() });
    }
    // A spell known and prepared is two selections and one spell learned.
    for spell in &chosen.spells_selected {
        let row = Held { kind: LedgerKind::Spell, id: spell.spell_id.clone(), target: Some(spell.source_class_id.clone()) };
        if !held.iter().any(|other| same_thing(other, &row) && same_target(other, &row)) {
            held.push(row);
        }
    }
    for skill in &chosen.skill_allocations {
        held.extend((0..skill.ranks).map(|_| Held { kind: LedgerKind::SkillRank, id: skill.skill_id.clone(), target: None }));
    }
    for grant in custom.grants.iter().filter(|grant| grant.target.starts_with("ability:")) {
        held.extend((0..grant.value.max(0)).map(|_| Held { kind: LedgerKind::AbilityIncrease, id: grant.target.clone(), target: Some(grant.id.clone()) }));
    }
    held
}

fn same_thing(left: &Held, right: &Held) -> bool {
    left.kind == right.kind
        && match left.kind {
            LedgerKind::Feat => feat_identity::same(&left.id, &right.id),
            _ => left.id.eq_ignore_ascii_case(&right.id),
        }
}

fn same_target(left: &Held, right: &Held) -> bool {
    match (&left.target, &right.target) {
        (Some(left), Some(right)) => left.eq_ignore_ascii_case(right),
        (None, None) => true,
        _ => false,
    }
}

fn character_level(input: &CharacterInput) -> u8 {
    input.chosen.class_levels.iter().fold(0, |total: u8, class| total.saturating_add(class.level))
}

/// The character as it stood at `level`, its classes taken in saved order. Its spells are each class's
/// canonical ones: a spell above that level's reach would make the engine state no casting rows at all.
fn input_at_level(input: &CharacterInput, level: u8) -> CharacterInput {
    let mut left = level;
    let mut at = input.clone();
    at.chosen.class_levels = input
        .chosen
        .class_levels
        .iter()
        .filter_map(|class| {
            let taken = class.level.min(left);
            left -= taken;
            (taken > 0).then(|| CharacterClassLevel { level: taken, ..class.clone() })
        })
        .collect();
    at.chosen.spells_selected = at
        .chosen
        .class_levels
        .iter()
        .flat_map(|class| canonical_seeds_for(class.class_id.strip_prefix("class:").unwrap_or(&class.class_id), class.level).1)
        .collect();
    at
}

fn explanation_value(base: &PilotBaseChassisComputation, id: &str) -> Option<i64> {
    base.explanations.iter().find(|row| row.id == id).map(|row| i64::from(row.value))
}

/// The class's skill points at this level plus a human's extra rank a level, which is no class's term.
fn skill_points(base: &PilotBaseChassisComputation) -> i64 {
    explanation_value(base, "class_chassis.skill_points").or_else(|| explanation_value(base, "multiclass.skill_points")).unwrap_or(0)
}

/// How many ranks each level of the character can place.
fn ranks_each_level(bases: &[PilotBaseChassisComputation]) -> Vec<i64> {
    let mut earlier = 0;
    bases
        .iter()
        .map(|base| {
            let points = skill_points(base);
            let gained = (points - earlier).max(0) + explanation_value(base, "race.human.trait_bundle.extra_skill_ranks").unwrap_or(0);
            earlier = points;
            gained
        })
        .collect()
}

/// The spell level a selection is learned at. An Oracle draws on the Cleric's list.
fn spell_level_for_class(class_id: &str, spell_id: &str) -> Option<u8> {
    class_spell_level(class_id, spell_id).or_else(|| if class_id == "class:oracle" { class_spell_level("class:cleric", spell_id) } else { None })
}

/// `<head>.spell_level_<n>[_and_higher]` or the alchemist's `extract_level_<n>`.
fn row_spell_level(id: &str) -> Option<(&str, u8)> {
    let (head, tail) = id.rsplit_once('.')?;
    let level = tail.strip_prefix("spell_level_").or_else(|| tail.strip_prefix("extract_level_"))?;
    Some((head, level.strip_suffix("_and_higher").unwrap_or(level).parse().ok()?))
}

/// What one character level lets each class learn. Keys are a class id and a spell level.
#[derive(Default, Clone)]
struct SpellsAtLevel {
    can_cast: BTreeSet<(String, u8)>,
    known_limit: BTreeMap<(String, u8), i64>,
    book_limit: BTreeMap<(String, u8), i64>,
}

/// The spell slots of a character's levels and what has gone in them so far.
struct SpellSlots {
    levels: Vec<SpellsAtLevel>,
    /// (class id, spell level, index into `levels`).
    placed: Vec<(String, u8, usize)>,
}

impl SpellSlots {
    fn of(at_levels: &[CharacterInput], bases: &[PilotBaseChassisComputation]) -> Self {
        let mut levels: Vec<SpellsAtLevel> = Vec::new();
        for (input, base) in at_levels.iter().zip(bases) {
            // Access stays once opened, and a known table that stops (a Bard's ends at 10) stays at its last row.
            let mut now = levels.last().cloned().unwrap_or_default();
            for class in &input.chosen.class_levels {
                let row_part = format!(".{}.", class.class_id.strip_prefix("class:").unwrap_or(&class.class_id));
                now.book_limit.retain(|(id, _), _| *id != class.class_id);
                for (head, spell_level, value) in base.explanations.iter().filter(|row| row.id.contains(&row_part)).filter_map(|row| row_spell_level(&row.id).map(|(head, level)| (head, level, i64::from(row.value)))) {
                    if (head.ends_with("spells_per_day") || head.ends_with("extracts_per_day")) && value > 0 {
                        now.can_cast.insert((class.class_id.clone(), spell_level));
                    }
                    if head.ends_with("book_size") {
                        now.book_limit.insert((class.class_id.clone(), spell_level), value);
                    }
                }
                let table = spells_known_table(&class.class_id, class.level).unwrap_or_default();
                for (spell_level, known) in table.iter().enumerate().filter_map(|(level, known)| Some((u8::try_from(level).ok()?, i64::from((*known)?)))) {
                    now.known_limit.insert((class.class_id.clone(), spell_level), known);
                    if known > 0 {
                        now.can_cast.insert((class.class_id.clone(), spell_level));
                    }
                }
            }
            levels.push(now);
        }
        SpellSlots { levels, placed: Vec::new() }
    }

    fn placed_by(&self, class_id: &str, level: usize, counts: impl Fn(u8) -> bool) -> i64 {
        self.placed.iter().filter(|(id, spell_level, at)| id == class_id && *at <= level && counts(*spell_level)).count() as i64
    }

    /// Whether one more spell of `spell_level` learned at `from` stays inside every later limit. A
    /// spellbook's row for `n` bounds level `n` and higher; its row for 0 bounds cantrips alone.
    fn fits(&self, class_id: &str, spell_level: u8, from: usize) -> bool {
        (from..self.levels.len()).all(|level| {
            let limits = &self.levels[level];
            let in_known = limits.known_limit.get(&(class_id.to_owned(), spell_level)).is_none_or(|limit| self.placed_by(class_id, level, |other| other == spell_level) < *limit);
            let bound_by_book = |bound: u8| {
                limits.book_limit.get(&(class_id.to_owned(), bound)).is_none_or(|limit| self.placed_by(class_id, level, |other| if bound == 0 { other == 0 } else { other >= bound }) < *limit)
            };
            in_known && if spell_level == 0 { bound_by_book(0) } else { (1..=spell_level).all(bound_by_book) }
        })
    }

    /// The earliest level with a slot for this spell. A spell its class has no list level for, and
    /// one its class never reaches, has no level.
    fn place(&mut self, class_id: &str, spell_id: &str) -> Option<u8> {
        let spell_level = spell_level_for_class(class_id, spell_id)?;
        let key = (class_id.to_owned(), spell_level);
        let at = (0..self.levels.len()).find(|level| self.levels[*level].can_cast.contains(&key) && self.fits(class_id, spell_level, *level))?;
        self.placed.push((class_id.to_owned(), spell_level, at));
        u8::try_from(at + 1).ok()
    }
}

/// The rows of a character saved before there was a ledger, by slot order (R8): feats through the
/// level, racial and class pools from where each is granted, spells through what each class level
/// lets the class know, ranks through each level's budget and no faster than the character's level
/// (CRB p.87), ability increases at every 4th level. The level was worked out, so every row says so.
fn rows_of_a_character_with_no_ledger(input: &CharacterInput, custom: &CharacterCustomDto) -> Result<CharacterLedgerDto, String> {
    let level = character_level(input);
    let at_levels: Vec<CharacterInput> = (1..=level).map(|at| input_at_level(input, at)).collect();
    let bases: Vec<PilotBaseChassisComputation> = at_levels.iter().map(compute_pilot_base_chassis).collect();
    let budgets = at_levels.iter().zip(&bases).map(|(at, base)| feat_budget_for(at, base)).collect::<Result<Vec<_>, _>>()?;

    let mut feats = feat_slot_levels(&input.chosen.selected_feats, &budgets).into_iter();
    let mut spells = SpellSlots::of(&at_levels, &bases);
    let mut rank_room = ranks_each_level(&bases);
    let mut ranks_of_skill: BTreeMap<String, usize> = BTreeMap::new();
    let mut increases = 0u32;
    let entries = held_by(input, custom)
        .into_iter()
        .map(|held| {
            let taken_at_level = match held.kind {
                LedgerKind::Feat => feats.next().flatten(),
                LedgerKind::Spell => held.target.as_deref().and_then(|class| spells.place(class, &held.id)),
                LedgerKind::SkillRank => {
                    let rank = ranks_of_skill.entry(held.id.to_ascii_lowercase()).or_default();
                    *rank += 1;
                    let at = (*rank - 1..rank_room.len()).find(|level| rank_room[*level] > 0);
                    at.and_then(|at| {
                        rank_room[at] -= 1;
                        u8::try_from(at + 1).ok()
                    })
                }
                LedgerKind::AbilityIncrease => {
                    increases += 1;
                    u8::try_from(increases * 4).ok().filter(|at| *at <= level)
                }
            };
            LedgerEntryDto { kind: held.kind, id: held.id, target: held.target, taken_at_level, assigned_on_upgrade: true }
        })
        .collect();
    Ok(CharacterLedgerDto { entries })
}

/// Builds the ledger of a character that has none. Nothing on the sheet reads it, so a failure does
/// not stop the sheet: it writes nothing, says so on stderr, and the next load tries again.
pub(crate) fn build_character_ledger_on_first_load(root: &Path, input: &CharacterInput) {
    if root.join(LEDGER_FILE_NAME).exists() {
        return;
    }
    let built = load_character_custom_at_root(root)
        .and_then(|custom| rows_of_a_character_with_no_ledger(input, &custom))
        .and_then(|ledger| write_character_ledger_at_root(root, &ledger));
    if let Err(reason) = built {
        eprintln!("{}: the level ledger was not built, and the next load tries again: {reason}", root.display());
    }
}

/// `ledger` brought in step with `held`. A row keeps its level for as long as what it records is held;
/// of two rows for one thing the later goes first. A feat whose target changed is still that feat.
fn in_step(ledger: &CharacterLedgerDto, held: &[Held], level: u8) -> CharacterLedgerDto {
    let row_as_held = |row: &LedgerEntryDto| Held { kind: row.kind, id: row.id.clone(), target: row.target.clone() };
    let mut row_holds: Vec<Option<usize>> = vec![None; ledger.entries.len()];
    let mut claimed = vec![false; held.len()];
    let mut claim = |matches: &dyn Fn(&Held, &Held) -> bool| {
        for (held_index, thing) in held.iter().enumerate() {
            if claimed[held_index] {
                continue;
            }
            let free = ledger.entries.iter().enumerate().find(|(row_index, row)| row_holds[*row_index].is_none() && matches(&row_as_held(row), thing));
            if let Some((row_index, _)) = free {
                row_holds[row_index] = Some(held_index);
                claimed[held_index] = true;
            }
        }
    };
    claim(&|row, thing| same_thing(row, thing) && same_target(row, thing));
    claim(&|row, thing| row.kind == LedgerKind::Feat && same_thing(row, thing));

    let kept = ledger.entries.iter().zip(&row_holds).filter_map(|(row, holds)| {
        holds.map(|held_index| LedgerEntryDto { target: held[held_index].target.clone(), ..row.clone() })
    });
    let stamped = held.iter().zip(&claimed).filter(|(_, claimed)| !**claimed).map(|(thing, _)| LedgerEntryDto {
        kind: thing.kind,
        id: thing.id.clone(),
        target: thing.target.clone(),
        taken_at_level: Some(level),
        assigned_on_upgrade: false,
    });
    CharacterLedgerDto { entries: kept.chain(stamped).collect() }
}

/// The ledger to write after a change, or `None` when there is nothing to write: nothing the ledger
/// records changed. A character with no ledger gets one only when something it records changes, and
/// `legacy` is then what it already held.
fn next_ledger(root: &Path, legacy: impl FnOnce() -> Result<CharacterLedgerDto, String>, before: &[Held], after: &[Held], level: u8) -> Result<Option<CharacterLedgerDto>, String> {
    let stored = load_character_ledger_at_root(root)?;
    if stored.is_none() && before == after {
        return Ok(None);
    }
    let from = match &stored {
        Some(stored) => stored.clone(),
        None => legacy()?,
    };
    let next = in_step(&from, after, level);
    Ok((stored.as_ref() != Some(&next)).then_some(next))
}

/// What the ledger becomes when a change takes the character from `before` to `after`. Read before
/// the character is saved, so a ledger that cannot be read refuses the change instead of following it.
pub(crate) fn ledger_after_change_at_root(root: &Path, before: &CharacterInput, after: &CharacterInput) -> Result<Option<CharacterLedgerDto>, String> {
    let custom = load_character_custom_at_root(root)?;
    next_ledger(root, || rows_of_a_character_with_no_ledger(before, &custom), &held_by(before, &custom), &held_by(after, &custom), character_level(after))
}

/// [`ledger_after_change_at_root`] for a new character: everything it holds was taken at its level.
pub(crate) fn ledger_of_a_new_character(root: &Path, input: &CharacterInput) -> Result<Option<CharacterLedgerDto>, String> {
    let custom = load_character_custom_at_root(root)?;
    next_ledger(root, || Ok(CharacterLedgerDto::default()), &[], &held_by(input, &custom), character_level(input))
}

/// What the ledger becomes when the Custom tab's grants go from `previous` to `next`.
pub(crate) fn ledger_after_custom_change_at_root(root: &Path, previous: &CharacterCustomDto, next: &CharacterCustomDto) -> Result<Option<CharacterLedgerDto>, String> {
    let input = SavedCharacterStore::load(root).map_err(|err| err.message)?.character_input;
    next_ledger(root, || rows_of_a_character_with_no_ledger(&input, previous), &held_by(&input, previous), &held_by(&input, next), character_level(&input))
}

pub(crate) fn write_character_ledger_at_root(root: &Path, ledger: &CharacterLedgerDto) -> Result<(), String> {
    let path = root.join(LEDGER_FILE_NAME);
    let json = serde_json::to_string_pretty(ledger).map_err(|err| format!("failed to serialize the ledger: {err}"))?;
    std::fs::write(&path, json).map_err(|err| format!("{}: {err}", path.display()))
}

/// A feat as the drawer prints it. `target` is a chooser feat's target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressionFeatDto {
    pub id: String,
    pub name: String,
    pub target: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressionSpellDto {
    /// The spell's id is the catalog's key, which is what the sheet's Spells tab prints.
    pub id: String,
    pub class_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressionRanksDto {
    pub skill_id: String,
    pub ranks: u32,
}

/// `ability` is the name in the grant's target: `strength` for `ability:strength`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressionAbilityIncreaseDto {
    pub ability: String,
    pub points: u32,
}

/// What a set of ledger rows took, grouped for reading.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressionChoicesDto {
    pub feats: Vec<ProgressionFeatDto>,
    pub spells: Vec<ProgressionSpellDto>,
    pub ranks: Vec<ProgressionRanksDto>,
    pub ability_increases: Vec<ProgressionAbilityIncreaseDto>,
}

/// A class feature line of the sheet that first appears at this level, or (`raised`) whose printed
/// value changes here. `value` is what the sheet prints for it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressionClassFeatureDto {
    pub id: String,
    pub label: String,
    pub value: String,
    pub raised: bool,
}

/// Slots this level opened that nothing fills. A choice fills the earliest free slot, the migration's
/// rule, so what is left over is the latest. Spells count only classes with a known table.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressionUnspentDto {
    pub feats: i64,
    pub spells: i64,
    pub ranks: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressionLevelDto {
    pub character_level: u8,
    pub class_id: String,
    pub class_level: u8,
    /// The hit die result saved for this class level, if one was (`hit_points.json`).
    pub hit_die_result: Option<u8>,
    #[serde(flatten)]
    pub taken: ProgressionChoicesDto,
    pub class_features: Vec<ProgressionClassFeatureDto>,
    /// A Pathfinder character gains an ability increase at this level.
    pub ability_increase_due: bool,
    pub unspent: ProgressionUnspentDto,
    /// Any row at this level had its level worked out afterwards.
    pub assigned_on_upgrade: bool,
}

/// The character level by level, and what holds no level the character has: an over-budget row of a
/// legacy save, or a row stamped above the character's level.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressionLedgerDto {
    pub levels: Vec<ProgressionLevelDto>,
    pub without_level: ProgressionChoicesDto,
}

fn add_choice(into: &mut ProgressionChoicesDto, row: &LedgerEntryDto, feat_names: &[(String, String)]) {
    match row.kind {
        LedgerKind::Feat => {
            let name = feat_names.iter().find(|(key, _)| feat_identity::same(key, &row.id)).map_or_else(|| row.id.clone(), |(_, name)| name.clone());
            into.feats.push(ProgressionFeatDto { id: row.id.clone(), name, target: row.target.clone() });
        }
        LedgerKind::Spell => into.spells.push(ProgressionSpellDto { id: row.id.clone(), class_id: row.target.clone() }),
        LedgerKind::SkillRank => match into.ranks.iter_mut().find(|skill| skill.skill_id.eq_ignore_ascii_case(&row.id)) {
            Some(skill) => skill.ranks += 1,
            None => into.ranks.push(ProgressionRanksDto { skill_id: row.id.clone(), ranks: 1 }),
        },
        LedgerKind::AbilityIncrease => {
            let ability = row.id.strip_prefix("ability:").unwrap_or(&row.id);
            match into.ability_increases.iter_mut().find(|increase| increase.ability == ability) {
                Some(increase) => increase.points += 1,
                None => into.ability_increases.push(ProgressionAbilityIncreaseDto { ability: ability.to_owned(), points: 1 }),
            }
        }
    }
}

/// The class and class level behind each character level, classes in saved order.
fn class_of_each_level(input: &CharacterInput) -> Vec<(String, u8)> {
    input.chosen.class_levels.iter().flat_map(|class| (1..=class.level).map(|at| (class.class_id.clone(), at))).collect()
}

/// A line's words without the count its rule offers: `Armor Training (armor training choice picks)`.
fn feature_label(label: &str) -> String {
    label.split_once(" (").filter(|(_, rest)| rest.ends_with(" picks)")).map_or(label, |(name, _)| name).to_owned()
}

/// The class features the sheet's "Rules and features" prints, as they arrive level by level: one
/// row for a rule when it is first held, then a row for each of its lines whose value moves. These
/// lines and not the engine's `class_feature.*` explanations, because a packaged app's corpus holds
/// no class feature records, so most of those explanations exist only in a checkout.
fn class_features_by_level(lines_by_level: &[Vec<SheetLineDto>]) -> Vec<Vec<ProgressionClassFeatureDto>> {
    let rule_of = |id: &str| id.split_once('#').map_or(id, |(rule, _)| rule).to_owned();
    let mut rules: BTreeSet<String> = BTreeSet::new();
    let mut printed: BTreeMap<String, String> = BTreeMap::new();
    lines_by_level
        .iter()
        .map(|lines| {
            let features: Vec<&SheetLineDto> = lines.iter().filter(|line| line.kind == "class_feature").collect();
            let mut gained = Vec::new();
            for line in &features {
                let rule = rule_of(&line.id);
                if !rules.contains(&rule) {
                    // A rule's own line names it; only a rule with none is named by its first.
                    let named = features.iter().find(|other| other.id == rule).unwrap_or(line);
                    gained.push(ProgressionClassFeatureDto { id: rule.clone(), label: feature_label(&named.label), value: named.value.clone(), raised: false });
                    rules.insert(rule);
                } else if printed.get(&line.id).is_some_and(|earlier| *earlier != line.value) {
                    gained.push(ProgressionClassFeatureDto { id: line.id.clone(), label: feature_label(&line.label), value: line.value.clone(), raised: true });
                }
            }
            for line in features {
                printed.insert(line.id.clone(), line.value.clone());
            }
            gained
        })
        .collect()
}

/// Which levels' slots stay open when `unspent` of those `opened` have no choice in them: the latest,
/// since a choice takes the earliest free slot. Adds each to `into`.
fn add_unspent(into: &mut [i64], opened: &[i64], mut unspent: i64) {
    for (level, room) in opened.iter().enumerate().rev() {
        let here = unspent.min(*room).max(0);
        into[level] += here;
        unspent -= here;
    }
}

/// How much each level adds to a running total.
fn gained_each_level(totals: impl Iterator<Item = i64>) -> Vec<i64> {
    let mut earlier = 0;
    totals
        .map(|total| {
            let gained = total - earlier;
            earlier = total;
            gained
        })
        .collect()
}

/// What the drawer lists. Reads only: a character with no ledger file is shown the rows its next
/// load will write. An unreadable file is an error, not an empty history.
pub(crate) fn list_progression_ledger_at_root(root: &Path) -> Result<ProgressionLedgerDto, String> {
    let input = SavedCharacterStore::load(root).map_err(|err| err.message)?.character_input;
    let ledger = match load_character_ledger_at_root(root)? {
        Some(ledger) => ledger,
        None => rows_of_a_character_with_no_ledger(&input, &load_character_custom_at_root(root)?)?,
    };
    let level = character_level(&input);
    let at_levels: Vec<CharacterInput> = (1..=level).map(|at| input_at_level(&input, at)).collect();
    let bases: Vec<PilotBaseChassisComputation> = at_levels.iter().map(compute_pilot_base_chassis).collect();
    let budgets = at_levels.iter().zip(&bases).map(|(at, base)| feat_budget_for(at, base)).collect::<Result<Vec<_>, _>>()?;

    let feat_names: Vec<(String, String)> = build_feat_catalog().entries.into_iter().map(|entry| (entry.key, entry.name)).collect();
    let mut result = ProgressionLedgerDto::default();
    let mut taken = vec![ProgressionChoicesDto::default(); at_levels.len()];
    let mut assigned = vec![false; at_levels.len()];
    let mut spells_placed: BTreeMap<(String, u8), i64> = BTreeMap::new();
    let mut ranks_placed = 0;
    for row in &ledger.entries {
        let Some(at) = row.taken_at_level.filter(|at| (1..=level).contains(at)).map(|at| usize::from(at - 1)) else {
            add_choice(&mut result.without_level, row, &feat_names);
            continue;
        };
        add_choice(&mut taken[at], row, &feat_names);
        assigned[at] |= row.assigned_on_upgrade;
        match (row.kind, row.target.as_deref()) {
            (LedgerKind::SkillRank, _) => ranks_placed += 1,
            (LedgerKind::Spell, Some(class)) => {
                if let Some(spell_level) = spell_level_for_class(class, &row.id) {
                    *spells_placed.entry((class.to_owned(), spell_level)).or_default() += 1;
                }
            }
            _ => {}
        }
    }

    let mut unspent_feats = vec![0; at_levels.len()];
    // Across pools: the budget charges a feat to a pool, not a level, so which pool is open says nothing of when.
    let opened_feats = gained_each_level(budgets.iter().map(|budget| budget.pools().iter().map(|pool| pool.granted).sum()));
    let room_left = budgets.last().map_or(0, |budget| budget.pools().iter().map(|pool| pool.remaining().max(0)).sum());
    add_unspent(&mut unspent_feats, &opened_feats, room_left);
    let mut unspent_ranks = vec![0; at_levels.len()];
    let rooms = ranks_each_level(&bases);
    add_unspent(&mut unspent_ranks, &rooms, rooms.iter().sum::<i64>() - ranks_placed);
    let mut unspent_spells = vec![0; at_levels.len()];
    let slots = SpellSlots::of(&at_levels, &bases);
    if let Some(last) = slots.levels.last() {
        for (key, limit) in &last.known_limit {
            let opened = gained_each_level(slots.levels.iter().map(|at| at.known_limit.get(key).copied().unwrap_or(0)));
            add_unspent(&mut unspent_spells, &opened, limit - spells_placed.get(key).copied().unwrap_or(0));
        }
    }

    let saved_hit_dice = load_character_hit_points_at_root(root)?.levels;
    let lines_by_level: Vec<Vec<SheetLineDto>> = at_levels.iter().zip(&bases).map(|(at, base)| sheet_lines_for(at, base).0).collect();
    let mut features = class_features_by_level(&lines_by_level).into_iter();
    result.levels = class_of_each_level(&input)
        .into_iter()
        .zip(taken)
        .enumerate()
        .map(|(at, ((class_id, class_level), taken))| ProgressionLevelDto {
            character_level: u8::try_from(at + 1).unwrap_or(u8::MAX),
            hit_die_result: saved_hit_dice.iter().filter(|saved| saved.class_id == class_id).nth(usize::from(class_level) - 1).map(|saved| saved.value),
            class_id,
            class_level,
            taken,
            class_features: features.next().unwrap_or_default(),
            ability_increase_due: (at + 1) % 4 == 0,
            unspent: ProgressionUnspentDto { feats: unspent_feats[at], spells: unspent_spells[at], ranks: unspent_ranks[at] },
            assigned_on_upgrade: assigned[at],
        })
        .collect();
    Ok(result)
}

/// `None` when the character has no ledger file.
pub(crate) fn load_character_ledger_at_root(root: &Path) -> Result<Option<CharacterLedgerDto>, String> {
    let path = root.join(LEDGER_FILE_NAME);
    if !path.exists() {
        return Ok(None);
    }
    let contents = std::fs::read_to_string(&path).map_err(|err| format!("{}: {err}", path.display()))?;
    serde_json::from_str(&contents).map(Some).map_err(|err| format!("{}: invalid ledger JSON: {err}", path.display()))
}

/// Copies the ledger beside one character to beside another; no file, nothing to copy.
pub(crate) fn copy_character_ledger(from: &Path, to: &Path) -> Result<(), String> {
    let source = from.join(LEDGER_FILE_NAME);
    if !source.exists() {
        return Ok(());
    }
    std::fs::copy(&source, to.join(LEDGER_FILE_NAME)).map(|_| ()).map_err(|err| format!("{}: {err}", source.display()))
}

/// Writes the ledger an imported file states, held to what the same file says the character holds:
/// a row nothing holds goes, and a thing with no row is stamped at the character's level.
pub(crate) fn restore_character_ledger_at_root(root: &Path, ledger: &CharacterLedgerDto, input: &CharacterInput) -> Result<(), String> {
    let custom = load_character_custom_at_root(root)?;
    write_character_ledger_at_root(root, &in_step(ledger, &held_by(input, &custom), character_level(input)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex::rules_core::class_census::load_sweep_fixture;
    use codex::rules_core::class_seeds::input_for;
    use codex::rules_core::rules_catalog::class_spell_levels::classes_with_spell_lists;

    /// A class the slots could not read would give every one of its spells no level, in silence.
    /// Every class with a spell list states what it can cast, by a row or by its known table. Levels
    /// 10 and 20: a Bard's rows and table stop at 10, and a Paladin has no spells before 4.
    #[test]
    fn ledger_migration_every_class_with_a_spell_list_can_place_a_spell() {
        let fixture = load_sweep_fixture().expect("census fixture");
        let classes: Vec<&str> = classes_with_spell_lists().into_iter().chain(["class:oracle"]).collect();
        assert_eq!(classes.len(), 18, "the 17 lists and the Oracle, on the Cleric's: {classes:?}");
        for class_id in classes {
            let slug = class_id.strip_prefix("class:").unwrap_or(class_id);
            let at_levels = [input_for(&fixture, slug, 10), input_for(&fixture, slug, 20)];
            let bases: Vec<_> = at_levels.iter().map(compute_pilot_base_chassis).collect();
            let slots = SpellSlots::of(&at_levels, &bases);
            let can_cast = |spell_level: u8| slots.levels[1].can_cast.contains(&(class_id.to_owned(), spell_level));
            assert!(can_cast(1), "{class_id} states no 1st-level casting at 20: {:?}", slots.levels[1].can_cast);
        }
    }
}
