//! The shared blocks (SD-39 E8.2, `technical-design.md §1`, operator ruling R2): one builder per
//! block type -- Identity, Abilities, the square block (Hit Points, Initiative, Encumbrance, Conditions), Armor Class, Saving Throws, Attack,
//! Speed (the Combat block's parts, SD-40 `decisions.md §8`), Skills, Weapons, Armor, Inventory,
//! Feats, ClassFeatures, Spells (list or spellbook), Custom, Pets, Actions, Math, BioNotes. A layout
//! file picks which blocks a system prints, in which order.
//!
//! Every block reads what the screen reads: the `LoadSavedCharacterResponse`, the owned copies
//! (`list_held_equipment_at_root`, not `equipment_selections`, which also holds the loadout's
//! `item:shield` and the Power Attack toggle; `decisions.md §25`), the Custom tab's records and the
//! bio. A number is printed as the engine computed it, never worked out here (NR-2, `decisions.md
//! §19`): a value the screen derives in TypeScript and the backend does not hold prints as `—`.

mod bio_notes;
mod core;
mod extras;
mod feats;
mod gear;
mod math;
mod skills;
mod spells;

pub use bio_notes::bio_notes;
pub use core::{abilities, armor_class, attack, character_details, languages, name_campaign, portrait, saves, speed, square_block};
pub use extras::{custom, pets};
pub use feats::{actions, class_features, feats};
pub(crate) use feats::line_row;
pub use gear::{armor, inventory, weapons};
pub use math::math;
pub use skills::skills;
pub use spells::{spellbook, spells};

use crate::character_custom::CharacterCustomDto;
use crate::character_hub::{CharacterBioDto, CharacterMoneyDto, ExplanationDto, HeldEquipmentCopyDto, LoadSavedCharacterResponse};
use crate::character_ledger::{AdmittedKind, OverrideFlag};
use crate::print::model::{Derivation, SheetImage};
use crate::print::PrintRequest;

/// What a sheet is printed from.
pub struct SheetSources<'a> {
    pub loaded: &'a LoadSavedCharacterResponse,
    /// Every copy the character owns, worn, carried or left behind (E7.5, R21).
    pub held: Vec<HeldEquipmentCopyDto>,
    pub custom: CharacterCustomDto,
    pub bio: CharacterBioDto,
    /// The Gear tab's money (`load_character_money`): Pathfinder only, `None` on the other sheets.
    pub money: Option<CharacterMoneyDto>,
    /// The character's saved portrait (`portrait.png`, as `load_character_portrait` reads it):
    /// Pathfinder only, `None` when there is none or on the other sheets.
    pub portrait: Option<SheetImage>,
    pub request: &'a PrintRequest,
}

impl SheetSources<'_> {
    /// The engine's `explanations` row with this id.
    pub(crate) fn explanation(&self, id: &str) -> Option<&ExplanationDto> {
        self.loaded.explanations.iter().find(|row| row.id == id)
    }

    /// The derivation of a number the row `id` states: that row, or none recorded.
    pub(crate) fn derivation_of(&self, id: &str) -> Vec<Derivation> {
        self.explanation(id).map(derivation).into_iter().collect()
    }

    /// The override tags on a record of `kind` held by `id`, in the words the sheet's tag prints.
    pub(crate) fn override_tags(&self, kind: AdmittedKind, id: &str) -> Vec<String> {
        self.loaded
            .admitted_under_override
            .iter()
            .filter(|row| row.kind == kind && row.id == id)
            .map(|row| override_tag(row.override_flag))
            .collect()
    }
}

/// One explanations row as a derivation: its id and value, then the engine's words verbatim.
pub(crate) fn derivation(row: &ExplanationDto) -> Derivation {
    Derivation { source: format!("{} = {}", row.id, row.value), detail: row.detail.clone() }
}

/// The tag the sheet shows on what an override let in (`OverrideTag.tsx`'s `overrideTagText`,
/// over `customModel.ts`'s `OVERRIDE_TOGGLES` labels).
pub(crate) fn override_tag(flag: OverrideFlag) -> String {
    let label = match flag {
        OverrideFlag::FeatsWithoutPrerequisites => "Feats without prerequisites",
        OverrideFlag::WeaponsWithoutProficiency => "Weapons without proficiency",
        OverrideFlag::SpellsOutsideClassList => "Spells outside the class list or level",
        OverrideFlag::ArmorWithoutProficiency => "Armor and shields without proficiency",
        OverrideFlag::IgnoreSlotLimits => "Ignore body slot limits",
        OverrideFlag::IgnoreFeatBudget => "Ignore the feat budget",
        OverrideFlag::IgnoreAlignmentRestrictions => "Ignore alignment restrictions",
        OverrideFlag::IgnoreEncumbrance => "Ignore encumbrance",
    };
    format!("Override: {label}")
}

/// `knowledge_arcana` reads `Knowledge arcana`; the first letter raised, underscores spaced.
pub(crate) fn words_of(slug: &str) -> String {
    let spaced = slug.replace('_', " ");
    let mut chars = spaced.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

/// A weight in pounds as the engine holds it: `29 lb`, `2.5 lb`. Formatting only.
pub(crate) fn pounds(value: f64) -> String {
    format!("{} lb", trim_number(value))
}

/// `29.0` reads `29`, `2.50` reads `2.5`. Formatting only.
pub(crate) fn trim_number(value: f64) -> String {
    let text = format!("{value:.2}");
    text.trim_end_matches('0').trim_end_matches('.').to_owned()
}

#[cfg(test)]
mod print_pf_blocks;

