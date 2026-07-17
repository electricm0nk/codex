//! SD-20 spellbook engine — Epic 2 (`scope-draft.md` §1.2,
//! `technical-design.md` §2.1).
//!
//! Fills the gap SD-19's corpus-aware compute seam explicitly left open:
//! `pilot_compute_corpus::compute_pilot_with_corpus` proves spell
//! *reachability* only (a selected spell resolves against the corpus and
//! lands in `CorpusDerivedSection.school_coverage[<school>].spells`), and
//! its own doc comment says so ("the seam still computes no spell slot
//! math, no spells-known/prepared posture, and no spell save DCs ...
//! those remain a future SD-N's scope, not a gap in this row's own
//! reachability claim" — see `support_state_matrix.rs`'s
//! `school.abjuration.spell_reachability` row). This module is that
//! future SD-N: it computes spell *effects*, prepared/known posture,
//! spell save DCs, and bonus slots from high ability, per school.
//!
//! Landed one school per cycle (`scope-draft.md` §1.2 Step 2 order:
//! abjuration, then conjuration, divination, enchantment, evocation,
//! illusion, necromancy, transmutation, universal). Abjuration
//! (`spellbook::abjuration`), Conjuration (`spellbook::conjuration`),
//! Divination (`spellbook::divination`), Enchantment
//! (`spellbook::enchantment`), Evocation (`spellbook::evocation`), and
//! Illusion (`spellbook::illusion`) are landed as of this cycle;
//! `compute_spellbook_coverage` dispatches by school and produces a real
//! `SpellEffect` only for the schools whose per-school contribution
//! module has landed. Unlanded schools' selections still resolve
//! (existence + school are checked) but contribute no effect yet — a
//! future cycle's per-school file adds that school's contribution
//! without touching this dispatch's shape.
//!
//! Reads spell level and effect text from the canonical CRB spell-list
//! table store (`rules_tables::crb::spell_list::SPELL_LIST`, SD-19's
//! foundation slice) via `TableCellRef`-style lookups — never hand-rolled
//! or re-derived. Spell *identity* (does this spell exist, what corpus
//! record does it resolve to) still goes through SD-19's own
//! `spell_resolver::spell_id_resolve`, exactly as `compute_pilot_with_corpus`
//! already does; this module does not re-implement corpus resolution.
//!
//! `technical-design.md` §2.1's illustrative seam signature takes a third
//! `rules_tables: &RulesTables` parameter; no `RulesTables` type exists in
//! this repo (the doc's own convention — see `pilot_compute_corpus.rs`'s
//! doc comment on `PilotReceipt` for precedent: illustrative doctrine
//! types are adapted to the real codebase shape, not re-derived as
//! parallel types). This module reads the table store directly
//! (`rules_tables::crb::spell_list::SPELL_LIST`), matching how
//! `spell_resolver.rs` and `equipment_resolver.rs` already read it.

pub mod abjuration;
pub mod conjuration;
pub mod divination;
pub mod enchantment;
pub mod evocation;
pub mod illusion;

use std::collections::BTreeMap;

use crate::rules_core::character_input::{AbilityScores, AcquisitionMode, CharacterInput};
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::rules_tables::crb::spell_list::Pf1SchoolId;
use crate::rules_core::source_content::SourcePackageContent;
use crate::rules_core::spell_resolver::spell_id_resolve;

/// A caster's school of specialization. Reuses the existing strict-school
/// enum (`rules_tables::crb::spell_list::Pf1SchoolId`, SD-19's foundation
/// slice) rather than re-deriving a duplicate school taxonomy — an Arcane
/// School specialization IS one of the nine strict schools.
pub type ArcaneSchool = Pf1SchoolId;

/// One resolved spell's effect: the school-tagged content read from the
/// canonical table store (level, effect text) plus its provenance.
#[derive(Debug, Clone, PartialEq)]
pub struct SpellEffect {
    pub spell_id: String,
    pub school: Pf1SchoolId,
    pub level: u8,
    pub effect_text: String,
    pub table_cell: TableCellRef,
}

/// A spell the character has prepared today in a specific slot.
#[derive(Debug, Clone, PartialEq)]
pub struct PreparedSpell {
    pub effect: SpellEffect,
    pub source_class_id: String,
}

/// A spell a spontaneous (or granted-access) caster knows.
#[derive(Debug, Clone, PartialEq)]
pub struct KnownSpell {
    pub effect: SpellEffect,
    pub source_class_id: String,
}

/// Per-character spellbook engine output (`technical-design.md` §2.1).
/// Independent of `contract::PilotReceipt` — Epic 1 owns
/// `src/rules_core/contract.rs` and wiring `SpellbookCoverage` into
/// `PilotReceipt` is that epic's (or Epic 8 integration closure's) future
/// extension, not this module's file-touch scope.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpellbookCoverage {
    pub spells_prepared: Vec<PreparedSpell>,
    pub spells_known: Vec<KnownSpell>,
    pub slots_total: BTreeMap<u8, u8>,
    pub slots_used: BTreeMap<u8, u8>,
    /// The saving-throw DC for the highest-level spell of that class
    /// present in `spells_prepared`/`spells_known` this call
    /// (`10 + spell level + casting-ability modifier`, the same formula
    /// already grounded per-class in `pilot_compute.rs`'s bard/paladin/
    /// ranger/sorcerer spell-save-DC explanations). A per-spell DC (which
    /// varies by that spell's own level) is always recoverable from
    /// `effect.level` on the individual `PreparedSpell`/`KnownSpell`
    /// record; this field is a convenience summary, not the sole source
    /// of truth for a specific cast.
    pub spell_save_dc: BTreeMap<String, u8>,
    /// Bonus spell slots per spell level from a high casting-ability
    /// modifier (PF1 Core Rulebook Table 1-3: a caster with ability
    /// modifier M gains one bonus slot of level L (1-9) when M >= L, plus
    /// one more for every full 4 points M exceeds L). Only levels that
    /// grant at least one bonus slot are present.
    pub bonus_slots_from_ability: BTreeMap<u8, u8>,
    pub school_specialization: Option<ArcaneSchool>,
}

/// Which ability score a casting class's spell save DCs and bonus slots
/// key off. Mirrors the DC formula already grounded per-class in
/// `pilot_compute.rs` (bard/sorcerer -> Charisma, cleric/druid/ranger ->
/// Wisdom, wizard -> Intelligence, paladin -> Charisma).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CastingAbility {
    Intelligence,
    Wisdom,
    Charisma,
}

/// Maps a class id to its casting ability. Accepts both the bare-name
/// convention (`"wizard"`, used by SD-19's own seam tests) and the
/// `"class:"`-prefixed convention (`"class:wizard"`, used by the GE-06
/// deterministic fixture and Epic 1's contract tests) — this module does
/// not own either convention, so it normalizes rather than picking a
/// side. Returns `None` for a non-casting or unrecognized class id (no
/// spell save DC / bonus slots are computed for that class's selections).
fn casting_ability_for_class(class_id: &str) -> Option<CastingAbility> {
    let bare = class_id.strip_prefix("class:").unwrap_or(class_id);
    match bare {
        "wizard" => Some(CastingAbility::Intelligence),
        "cleric" | "druid" | "ranger" => Some(CastingAbility::Wisdom),
        "sorcerer" | "bard" | "paladin" => Some(CastingAbility::Charisma),
        _ => None,
    }
}

/// The PF1 ability modifier formula (`floor(score / 2) - 5`), matching
/// `pilot_compute.rs`'s private `ability_modifier` fn bit-for-bit. Not
/// importable across modules (it is private there), so this is the same
/// well-documented, zero-interpretation-risk PF1 arithmetic rule
/// restated, not a new or diverging rule.
fn ability_modifier_for(scores: &AbilityScores, ability: CastingAbility) -> i16 {
    let score = match ability {
        CastingAbility::Intelligence => scores.intelligence,
        CastingAbility::Wisdom => scores.wisdom,
        CastingAbility::Charisma => scores.charisma,
    };
    score.div_euclid(2) - 5
}

/// The spellbook engine seam (`technical-design.md` §2.1). Resolves every
/// `input.chosen.spells_selected` entry against the corpus, dispatches to
/// the landed per-school contribution function for that spell's school,
/// classifies the result into `spells_prepared` / `spells_known` per its
/// `acquisition_mode`, and computes `spell_save_dc` /
/// `bonus_slots_from_ability` from the selecting class's casting ability.
///
/// `school_specialization` is always `None` in this cycle — Epic 2's
/// scope is per-school spell coverage/effects, not the Wizard
/// specialization-choice mechanic (already partially grounded elsewhere,
/// see `pilot_compute.rs`'s `class_chassis.wizard.specialization_choice`
/// explanation, which stays this repo's authority for that choice).
pub fn compute_spellbook_coverage(
    input: &CharacterInput,
    corpus: &SourcePackageContent,
) -> SpellbookCoverage {
    let mut coverage = SpellbookCoverage::default();

    for selection in &input.chosen.spells_selected {
        let Some((record, _table_cell)) =
            spell_id_resolve(&selection.spell_id, RuleSetId::Crb, corpus)
        else {
            continue;
        };
        let Some(school) = record
            .school
            .as_deref()
            .and_then(Pf1SchoolId::from_corpus_str)
        else {
            continue;
        };

        // Per-school contribution functions land one per cycle (Step 2).
        // Abjuration, Conjuration, Divination, Enchantment, Evocation, and
        // Illusion are wired as of this cycle.
        let landed_effect = match school {
            Pf1SchoolId::Abjuration => abjuration::resolve_abjuration_spell_effect(
                &selection.spell_id,
            )
            .map(|effect| SpellEffect {
                spell_id: effect.spell_id,
                school,
                level: effect.level,
                effect_text: effect.effect_text,
                table_cell: effect.table_cell,
            }),
            Pf1SchoolId::Conjuration => conjuration::resolve_conjuration_spell_effect(
                &selection.spell_id,
            )
            .map(|effect| SpellEffect {
                spell_id: effect.spell_id,
                school,
                level: effect.level,
                effect_text: effect.effect_text,
                table_cell: effect.table_cell,
            }),
            Pf1SchoolId::Divination => divination::resolve_divination_spell_effect(
                &selection.spell_id,
            )
            .map(|effect| SpellEffect {
                spell_id: effect.spell_id,
                school,
                level: effect.level,
                effect_text: effect.effect_text,
                table_cell: effect.table_cell,
            }),
            Pf1SchoolId::Enchantment => enchantment::resolve_enchantment_spell_effect(
                &selection.spell_id,
            )
            .map(|effect| SpellEffect {
                spell_id: effect.spell_id,
                school,
                level: effect.level,
                effect_text: effect.effect_text,
                table_cell: effect.table_cell,
            }),
            Pf1SchoolId::Evocation => evocation::resolve_evocation_spell_effect(
                &selection.spell_id,
            )
            .map(|effect| SpellEffect {
                spell_id: effect.spell_id,
                school,
                level: effect.level,
                effect_text: effect.effect_text,
                table_cell: effect.table_cell,
            }),
            Pf1SchoolId::Illusion => illusion::resolve_illusion_spell_effect(
                &selection.spell_id,
            )
            .map(|effect| SpellEffect {
                spell_id: effect.spell_id,
                school,
                level: effect.level,
                effect_text: effect.effect_text,
                table_cell: effect.table_cell,
            }),
            _ => None,
        };
        let Some(spell_effect) = landed_effect else {
            continue;
        };

        match selection.acquisition_mode {
            AcquisitionMode::Prepared => {
                coverage.spells_prepared.push(PreparedSpell {
                    effect: spell_effect.clone(),
                    source_class_id: selection.source_class_id.clone(),
                });
            }
            AcquisitionMode::Known | AcquisitionMode::Granted => {
                coverage.spells_known.push(KnownSpell {
                    effect: spell_effect.clone(),
                    source_class_id: selection.source_class_id.clone(),
                });
            }
        }

        if let Some(ability) = casting_ability_for_class(&selection.source_class_id) {
            let modifier = ability_modifier_for(&input.chosen.ability_scores, ability);
            let dc = (10i16 + i16::from(spell_effect.level) + modifier).max(0) as u8;
            coverage
                .spell_save_dc
                .entry(selection.source_class_id.clone())
                .and_modify(|existing| *existing = (*existing).max(dc))
                .or_insert(dc);

            for level in 1..=9u8 {
                if modifier >= i16::from(level) {
                    let bonus = ((modifier - i16::from(level)) / 4 + 1) as u8;
                    coverage
                        .bonus_slots_from_ability
                        .entry(level)
                        .and_modify(|existing| *existing = (*existing).max(bonus))
                        .or_insert(bonus);
                }
            }
        }
    }

    coverage
        .spells_known
        .sort_by(|a, b| a.effect.spell_id.cmp(&b.effect.spell_id));
    coverage
        .spells_prepared
        .sort_by(|a, b| a.effect.spell_id.cmp(&b.effect.spell_id));

    coverage
}
