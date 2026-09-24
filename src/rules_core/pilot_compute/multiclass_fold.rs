//! SD-36 Epic F3b (`docs/release/SD-36-consolidation/epic-f-class-completion.md` §5):
//! the multiclass gate and fold for EVERY class with a chassis -- one mechanical rule,
//! no class named.
//!
//! # Gate ([`multiclass_member`], [`is_supported_multiclass_mix`](super::is_supported_multiclass_mix))
//!
//! A class level joins a mix when
//! - it is a non-prestige class whose ISOLATED single-class input (the same input with
//!   only this class level) passes [`has_supported_class_chassis`](super::has_supported_class_chassis),
//!   or a prestige class whose converted record has a chassis row at that level; AND
//! - each of its three save progressions has a source the fold can sum: the CRB class
//!   table (`good_saves_for`, the pre-F3b reading for the 11 tabled classes), or else the
//!   converted chassis record's [`ClassChassis::save_shape`] is `Good` or `Poor`.
//!   `Degraded`, `Unrecognized` or no record at all is a NAMED claim-blocking diagnostic
//!   ([`SAVE_SHAPE_DEGRADED`], [`SAVE_SHAPE_UNRECOGNIZED`], [`SAVE_SHAPE_UNKNOWN`]) --
//!   never folded in as a poor save.
//!
//! A mix also needs at least one non-prestige class (a prestige class cannot be a first
//! class; F2b's `prestige_class.requires_base_class_levels`).
//!
//! # Fold
//!
//! - BAB: sum of each class's own BAB (the isolated single-class chassis; a prestige
//!   class's converted row).
//! - Saves: each class's EXACT (untruncated) save value summed, floored once
//!   (`compute_multiclass_base_chassis`, unchanged rule, now in exact rationals).
//! - HP ([`explain_multiclass_fold`]): per class `ClassChassis::hit_points`, the maximized
//!   die only for the first-listed class's first level (character level 1), + Con each
//!   level; any class with no hit die => [`HIT_POINTS_UNKNOWN`], no total printed.
//! - Skill points: per class `ClassChassis::skill_points`; a class whose record states no
//!   skill ranks per level => [`SKILL_POINTS_UNKNOWN`], no total printed. Since SD-36 F3b2
//!   every chassis-bearing class record states its ranks (converted from `STARTSKILLPTS`).
//! - Class-feature lines: each class's isolated single-class run, explanation ids
//!   re-scoped `multiclass.<class>.<original id>` (skipped when the mix already printed
//!   the same id itself; character-level totals -- a level-1 hit-point line, the chassis BAB/save rows
//!   -- are not copied, the fold computes those once). The isolated run's claim-blocking
//!   CLASS-LINE diagnostics (`class_feature.*`, `class_spell.*`, `class_chassis.<class>.*`)
//!   carry over re-scoped the same way, so a class feature that cannot compute alone does
//!   not compute in a mix either. Character-level pillar diagnostics (combat, defense,
//!   skills, the chassis gate, a prestige class's "alone" rule and entry gate) are not
//!   carried: the mix computes and answers those itself.
//! - Prestige entry requirements print met/unmet ([`PRESTIGE_ENTRY_GATE_MET`] /
//!   [`PRESTIGE_ENTRY_GATE_UNMET`]), never claim-blocking.
//! - Class skills and weapon proficiencies were already unions over `class_levels`
//!   (`selected_skill_*_is_class_skill`, `skill_allocation::class_skill_set`, the
//!   proficiency union); nothing to change here.

use std::collections::BTreeMap;
use std::sync::OnceLock;

#[allow(unused_imports)]
pub(crate) use super::*;
use super::class_chassis_sheet_rules::{
    self, ChassisUnknown, ClassChassis, SaveProgression, HIT_POINTS_UNKNOWN, SKILL_POINTS_UNKNOWN,
};
use crate::rules_core::sheet_rule::Rat;

/// A class level this gate cannot place in a mix (no chassis at that level).
pub(crate) const MULTICLASS_CLASS_UNSUPPORTED: &str = "multiclass.class_unsupported";
/// A save whose converted record shows the words-not-`Expr` symptom.
pub(crate) const SAVE_SHAPE_DEGRADED: &str = "multiclass.save_shape.degraded";
/// A save whose `Expr` is neither PF1 good nor poor progression.
pub(crate) const SAVE_SHAPE_UNRECOGNIZED: &str = "multiclass.save_shape.unrecognized";
/// A class with no class table and no converted chassis record to read saves from.
pub(crate) const SAVE_SHAPE_UNKNOWN: &str = "multiclass.save_shape.unknown";
/// A prestige class in a mix whose entry requirements the character meets.
pub(crate) const PRESTIGE_ENTRY_GATE_MET: &str = "multiclass.prestige_entry_gate.met";
/// A prestige class in a mix whose entry requirements the character does not meet.
pub(crate) const PRESTIGE_ENTRY_GATE_UNMET: &str = "multiclass.prestige_entry_gate.unmet";
/// The character's hit-point total across every class.
pub(crate) const MULTICLASS_HIT_POINTS: &str = "multiclass.hit_points";
/// The character's class skill-point total across every class.
pub(crate) const MULTICLASS_SKILL_POINTS: &str = "multiclass.skill_points";

const SAVE_NAMES: [&str; 3] = ["Fortitude", "Reflex", "Will"];

/// The input with `class_level` as its only class -- the isolated single-class run the
/// gate and the fold both read.
pub(crate) fn isolated_input(input: &CharacterInput, class_level: &CharacterClassLevel) -> CharacterInput {
    let mut isolated = input.clone();
    isolated.chosen.class_levels = vec![class_level.clone()];
    isolated
}

/// Every chassis-bearing converted class record in every book, keyed by slug (first
/// book in directory order wins; the three slugs two books share are all prestige
/// classes [`generic_class_chassis::record`] answers first).
fn all_book_records() -> &'static BTreeMap<String, ClassChassis> {
    static TABLE: OnceLock<BTreeMap<String, ClassChassis>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let root = crate::support::paths::repo_root().join("data/sheet_rules");
        let mut books: Vec<String> = std::fs::read_dir(&root)
            .map(|entries| {
                entries
                    .flatten()
                    .filter(|e| e.path().join("class").is_dir())
                    .filter_map(|e| e.file_name().to_str().map(str::to_owned))
                    .collect()
            })
            .unwrap_or_default();
        books.sort();
        let mut out = BTreeMap::new();
        for book in &books {
            for ((_, slug), chassis) in class_chassis_sheet_rules::records(&[book.as_str()]) {
                if chassis.is_conventional() {
                    out.entry(slug).or_insert(chassis);
                }
            }
        }
        out
    })
}

/// The converted chassis record for `class_id` (`"class:<slug>"`): the generic
/// class-family population first (its book precedence), then any other book.
pub(crate) fn chassis_record(class_id: &str) -> Option<&'static ClassChassis> {
    generic_class_chassis::record(class_id)
        .or_else(|| all_book_records().get(class_id.strip_prefix("class:")?))
}

fn blocking(id: &str, message: String) -> ComputationDiagnostic {
    ComputationDiagnostic { id: id.to_owned(), message, claim_blocking: true }
}

/// PF1's base-class fractional save value at `level` (`level/2 + 2` good, `level/3`
/// poor), exact.
fn table_save_value(level: u8, good: bool) -> Rat {
    let level = i64::from(level);
    if good { Rat { num: level + 4, den: 2 } } else { Rat { num: level, den: 3 } }
}

/// One class level's place in a mix: whether it is a prestige class, and its three
/// exact save values -- or the named reason it cannot join.
pub(crate) struct MulticlassMember {
    pub(crate) prestige: bool,
    pub(crate) saves: [Rat; 3],
}

pub(crate) fn multiclass_member(
    input: &CharacterInput,
    class_level: &CharacterClassLevel,
) -> Result<MulticlassMember, ComputationDiagnostic> {
    let class_id = class_level.class_id.as_str();
    let level = class_level.level;
    let prestige = generic_class_chassis::is_prestige(class_id);
    let has_chassis = if prestige {
        generic_class_chassis::resolve(class_id, level).is_some()
    } else {
        has_supported_class_chassis(&isolated_input(input, class_level))
    };
    if !has_chassis {
        return Err(blocking(
            MULTICLASS_CLASS_UNSUPPORTED,
            format!(
                "{class_id} {level}: no class chassis at this level (the class alone does not \
                 pass the chassis gate{}), so the mix cannot fold it",
                if prestige { "; its converted record has no row at this level" } else { "" }
            ),
        ));
    }
    if let Some((fort, reflex, will)) = table_class_id(class_id).and_then(good_saves_for) {
        return Ok(MulticlassMember {
            prestige,
            saves: [
                table_save_value(level, fort),
                table_save_value(level, reflex),
                table_save_value(level, will),
            ],
        });
    }
    let Some(record) = chassis_record(class_id) else {
        return Err(blocking(
            SAVE_SHAPE_UNKNOWN,
            format!(
                "{class_id} {level}: no class table and no converted chassis record states this \
                 class's save progressions, so its saves cannot be folded into a mix (never \
                 assumed poor)"
            ),
        ));
    };
    let mut saves = [Rat::ZERO; 3];
    for (index, save) in saves.iter_mut().enumerate() {
        let name = SAVE_NAMES[index];
        match record.save_shape(index) {
            Some(SaveProgression::Good | SaveProgression::Poor) => {
                *save = record.save_value_exact(index, level).ok_or_else(|| {
                    blocking(
                        MULTICLASS_CLASS_UNSUPPORTED,
                        format!("{class_id} {level}: {name} has no value at this level"),
                    )
                })?;
            }
            Some(SaveProgression::Degraded) => {
                return Err(blocking(
                    SAVE_SHAPE_DEGRADED,
                    format!(
                        "{class_id} {level}: the converted {name} save degraded to words \
                         ({}:class:{}), so it cannot be folded into a mix (never assumed poor)",
                        record.book, record.slug
                    ),
                ));
            }
            Some(SaveProgression::Unrecognized) | None => {
                return Err(blocking(
                    SAVE_SHAPE_UNRECOGNIZED,
                    format!(
                        "{class_id} {level}: the converted {name} save progression \
                         ({}:class:{}) is neither PF1's good nor poor class-level save, so it \
                         cannot be folded into a mix (never assumed poor)",
                        record.book, record.slug
                    ),
                ));
            }
        }
    }
    Ok(MulticlassMember { prestige, saves })
}

/// Why `input` (a length-2+ mix) is not a supported mix, as named claim-blocking
/// diagnostics: each class that cannot join, or F2b's game rule when every class is a
/// prestige class. Empty for a supported mix.
pub(crate) fn multiclass_mix_rejections(input: &CharacterInput) -> Vec<ComputationDiagnostic> {
    let mut out = Vec::new();
    let mut any_base = false;
    for class_level in &input.chosen.class_levels {
        match multiclass_member(input, class_level) {
            Ok(member) => any_base |= !member.prestige,
            Err(diagnostic) => {
                any_base |= !generic_class_chassis::is_prestige(&class_level.class_id);
                out.push(diagnostic);
            }
        }
    }
    if !any_base {
        out.push(blocking(
            PRESTIGE_REQUIRES_BASE_CLASS_LEVELS_DIAGNOSTIC_ID,
            PRESTIGE_REQUIRES_BASE_CLASS_LEVELS_MESSAGE.to_owned(),
        ));
    }
    out
}

/// A mix member's base attack bonus: the isolated single-class chassis, or a prestige
/// class's converted row.
pub(crate) fn member_base_attack_bonus(
    input: &CharacterInput,
    class_level: &CharacterClassLevel,
    ability_modifiers: &AbilityModifiers,
) -> Option<i16> {
    if generic_class_chassis::is_prestige(&class_level.class_id) {
        return generic_class_chassis::resolve(&class_level.class_id, class_level.level)
            .map(|row| row.base_attack_bonus);
    }
    let isolated = isolated_input(input, class_level);
    let (bab, _) = compute_class_chassis(&isolated, ability_modifiers, &mut Vec::new(), &mut Vec::new())?;
    Some(bab)
}

/// `true` for an isolated-run explanation that is a character-level total the fold
/// computes once (the chassis BAB/save rows, a class's own level-1 hit-point total, which holds only
/// when that class is the character's first level), not a class line.
fn is_character_level_total(id: &str) -> bool {
    id == "class_chassis.base_attack_bonus"
        || id.starts_with("class_chassis.base_save.")
        || id.ends_with(".level_1_hit_points")
}

/// `true` for an isolated-run explanation that is this class's own line (a class
/// feature, spell, pool or per-class chassis line), as opposed to a race, ability,
/// feat, combat or defense line the mix computes itself.
fn is_class_line(id: &str) -> bool {
    (id.starts_with("class_feature.") || id.starts_with("class_spell.") || id.starts_with("class_chassis."))
        && !is_character_level_total(id)
}

/// The fold's character-level totals and per-class lines for a supported mix (see the
/// module doc). Called once by `compute_pilot_base_chassis`, after every in-mix
/// explanation is pushed.
pub(crate) fn explain_multiclass_fold(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let class_levels = &input.chosen.class_levels;

    // Hit points and skill points: per-class terms, summed only when every class has one.
    let mut hp_terms: Vec<(String, i16)> = Vec::new();
    let mut sp_terms: Vec<(String, i16)> = Vec::new();
    let mut hp_unknown = false;
    let mut sp_unknown = false;
    for (index, class_level) in class_levels.iter().enumerate() {
        let class_id = &class_level.class_id;
        let record = chassis_record(class_id);
        let no_record = |id: &'static str, what: &str| ChassisUnknown {
            id,
            message: format!(
                "{class_id}: no converted class record states this class's {what}, so its \
                 {what} are Unknown"
            ),
        };
        let hp = record
            .ok_or_else(|| no_record(HIT_POINTS_UNKNOWN, "hit points"))
            .and_then(|r| r.hit_points(class_level.level, index == 0, ability_modifiers.constitution));
        let sp = record
            .ok_or_else(|| no_record(SKILL_POINTS_UNKNOWN, "skill points"))
            .and_then(|r| r.skill_points(class_level.level, ability_modifiers.intelligence));
        for (result, terms, unknown) in [(hp, &mut hp_terms, &mut hp_unknown), (sp, &mut sp_terms, &mut sp_unknown)] {
            match result {
                Ok(value) => terms.push((format!("{class_id} {}: {value}", class_level.level), value)),
                Err(unknown_total) => {
                    *unknown = true;
                    diagnostics.push(ComputationDiagnostic {
                        id: unknown_total.id.to_owned(),
                        message: unknown_total.message,
                        claim_blocking: false,
                    });
                }
            }
        }
    }
    for (id, what, terms, unknown, rule) in [
        (
            MULTICLASS_HIT_POINTS,
            "hit points",
            &hp_terms,
            hp_unknown,
            "maximized hit die at character level 1 (the first-listed class), the non-rolling \
             average (die/2 + 1) at every other level, + Constitution modifier per level (at \
             least 1)",
        ),
        (
            MULTICLASS_SKILL_POINTS,
            "skill points",
            &sp_terms,
            sp_unknown,
            "each class's skill ranks per level + Intelligence modifier (at least 1), times its \
             levels",
        ),
    ] {
        if unknown {
            continue;
        }
        let total: i16 = terms.iter().map(|(_, v)| v).sum();
        let parts: Vec<&str> = terms.iter().map(|(t, _)| t.as_str()).collect();
        explanations.push(ComputationExplanation {
            id: id.to_owned(),
            value: total,
            detail: format!("Multiclass {what} {total}: {rule} ({})", parts.join("; ")),
        });
    }

    // Each class's own lines, verbatim from its isolated single-class run.
    for class_level in class_levels {
        let slug = class_level.class_id.strip_prefix("class:").unwrap_or(&class_level.class_id);
        let isolated = compute_pilot_base_chassis(&isolated_input(input, class_level));
        for explanation in isolated.explanations {
            if !is_class_line(&explanation.id) || explanations.iter().any(|e| e.id == explanation.id) {
                continue;
            }
            explanations.push(ComputationExplanation {
                id: format!("multiclass.{slug}.{}", explanation.id),
                ..explanation
            });
        }
        for diagnostic in isolated.diagnostics {
            // Only the class's own blocking lines: the character-level pillars
            // (combat, defense, skills, the chassis gate) are computed by the mix
            // itself, and a prestige class alone is blocked on those by rule. A
            // line the mix already raised under its own id (a check that runs for
            // a class alone or mixed) is not raised twice.
            if !diagnostic.claim_blocking
                || !is_class_line(&diagnostic.id)
                || diagnostic.id == "class_chassis.unsupported"
                || diagnostic.id.starts_with("class_chassis.prestige_entry_gate.")
                || diagnostics.iter().any(|d| d.id == diagnostic.id)
            {
                continue;
            }
            diagnostics.push(ComputationDiagnostic {
                id: format!("multiclass.{slug}.{}", diagnostic.id),
                ..diagnostic
            });
        }
        if generic_class_chassis::is_prestige(&class_level.class_id)
            && let Some(gate) =
                prestige_class_entry_gate::evaluate_prestige_class_entry(&class_level.class_id, input)
        {
            diagnostics.push(ComputationDiagnostic {
                id: if gate.qualifies { PRESTIGE_ENTRY_GATE_MET } else { PRESTIGE_ENTRY_GATE_UNMET }
                    .to_owned(),
                message: if gate.qualifies {
                    format!(
                        "{} ({}): entry requirements met ({} clause(s) satisfied, {} unmodelled)",
                        gate.display_name,
                        class_level.class_id,
                        gate.met.len(),
                        gate.unmodelled.len()
                    )
                } else {
                    format!(
                        "{} ({}): entry requirements NOT met -- {} (printed, not enforced: the \
                         sheet prints the class the player chose)",
                        gate.display_name,
                        class_level.class_id,
                        gate.unmet.join("; ")
                    )
                },
                claim_blocking: false,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_prestige_save_folds_its_own_table_form_exactly() {
        // Arcane Archer good Fortitude `(level+1)/2`: exactly 2 at 3rd (the prestige
        // table's +2), not the base-class form's 3.5.
        let record = chassis_record("class:arcane_archer").expect("CRB record");
        assert_eq!(record.save_shape(0), Some(SaveProgression::Good));
        assert_eq!(record.save_value_exact(0, 3), Some(Rat { num: 2, den: 1 }));
        // Loremaster poor Fortitude `(level+1)/3` at 2nd: exactly 1.
        let record = chassis_record("class:loremaster").expect("CRB record");
        assert_eq!(record.save_value_exact(0, 2), Some(Rat { num: 1, den: 1 }));
        // Magus good Will `level/2 + 2` at 1st: 5/2, untruncated.
        let record = chassis_record("class:magus").expect("UM record");
        assert_eq!(record.save_value_exact(2, 1), Some(Rat { num: 5, den: 2 }));
    }

    #[test]
    fn a_class_with_no_table_and_no_record_is_named_unknown() {
        // Every Pathfinder Unchained class carries no converted chassis rows (F3a) and no
        // CRB table: its saves cannot be folded, and it says so.
        let fixture = crate::rules_core::class_census::load_sweep_fixture().expect("fixture");
        let level = CharacterClassLevel { class_id: "class:unchained_rogue".to_owned(), level: 3 };
        assert!(chassis_record(&level.class_id).is_none());
        match multiclass_member(&fixture, &level) {
            Err(d) => assert_eq!(d.id, SAVE_SHAPE_UNKNOWN, "{d:?}"),
            Ok(_) => panic!("unchained_rogue must not fold without a save source"),
        }
    }
}
