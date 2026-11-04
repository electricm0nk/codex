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

use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::character_custom::{load_character_custom_at_root, CharacterCustomDto};
use codex::rules_core::character_input::CharacterInput;
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
    /// `None` only on a row of a character saved before there was a ledger.
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

/// The rows of a character saved before there was a ledger, whose levels nothing recorded.
fn rows_of_a_character_with_no_ledger(before: &[Held]) -> CharacterLedgerDto {
    CharacterLedgerDto {
        entries: before
            .iter()
            .map(|held| LedgerEntryDto { kind: held.kind, id: held.id.clone(), target: held.target.clone(), taken_at_level: None, assigned_on_upgrade: true })
            .collect(),
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
/// records changed. A character with no ledger gets one only when something it records changes.
fn next_ledger(root: &Path, before: &[Held], after: &[Held], level: u8) -> Result<Option<CharacterLedgerDto>, String> {
    let stored = load_character_ledger_at_root(root)?;
    if stored.is_none() && before == after {
        return Ok(None);
    }
    let next = in_step(&stored.clone().unwrap_or_else(|| rows_of_a_character_with_no_ledger(before)), after, level);
    Ok((stored.as_ref() != Some(&next)).then_some(next))
}

/// What the ledger becomes when a change takes the character from `before` to `after`. Read before
/// the character is saved, so a ledger that cannot be read refuses the change instead of following it.
pub(crate) fn ledger_after_change_at_root(root: &Path, before: &CharacterInput, after: &CharacterInput) -> Result<Option<CharacterLedgerDto>, String> {
    let custom = load_character_custom_at_root(root)?;
    next_ledger(root, &held_by(before, &custom), &held_by(after, &custom), character_level(after))
}

/// [`ledger_after_change_at_root`] for a new character: everything it holds was taken at its level.
pub(crate) fn ledger_of_a_new_character(root: &Path, input: &CharacterInput) -> Result<Option<CharacterLedgerDto>, String> {
    let custom = load_character_custom_at_root(root)?;
    next_ledger(root, &[], &held_by(input, &custom), character_level(input))
}

/// What the ledger becomes when the Custom tab's grants go from `previous` to `next`.
pub(crate) fn ledger_after_custom_change_at_root(root: &Path, previous: &CharacterCustomDto, next: &CharacterCustomDto) -> Result<Option<CharacterLedgerDto>, String> {
    let input = SavedCharacterStore::load(root).map_err(|err| err.message)?.character_input;
    next_ledger(root, &held_by(&input, previous), &held_by(&input, next), character_level(&input))
}

pub(crate) fn write_character_ledger_at_root(root: &Path, ledger: &CharacterLedgerDto) -> Result<(), String> {
    let path = root.join(LEDGER_FILE_NAME);
    let json = serde_json::to_string_pretty(ledger).map_err(|err| format!("failed to serialize the ledger: {err}"))?;
    std::fs::write(&path, json).map_err(|err| format!("{}: {err}", path.display()))
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
