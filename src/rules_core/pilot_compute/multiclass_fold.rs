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
/// A single-class character's class skill-point total.
pub(crate) const CLASS_CHASSIS_SKILL_POINTS: &str = "class_chassis.skill_points";

const SAVE_NAMES: [&str; 3] = ["Fortitude", "Reflex", "Will"];

/// The input with `class_level` as its only class -- the isolated single-class run the
/// gate and the fold both read.
pub(crate) fn isolated_input(input: &CharacterInput, class_level: &CharacterClassLevel) -> CharacterInput {
    let mut isolated = input.clone();
    isolated.chosen.class_levels = vec![class_level.clone()];
    isolated
}

/// Every chassis-bearing converted class record in every book, keyed by slug, one answer per
/// slug by [`resolve_printings`] (SD-36 F3 polish P3: never "the alphabetically-first book wins").
fn all_book_records() -> &'static BTreeMap<String, ClassChassis> {
    static TABLE: OnceLock<BTreeMap<String, ClassChassis>> = OnceLock::new();
    TABLE.get_or_init(|| {
        printings_by_slug()
            .into_iter()
            .filter_map(|(slug, printings)| resolve_printings(printings).map(|chassis| (slug, chassis)))
            .collect()
    })
}

/// Every conventional chassis-bearing class record in every book, grouped by slug (each group in
/// book-directory order).
pub(crate) fn printings_by_slug() -> BTreeMap<String, Vec<ClassChassis>> {
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
    let mut out: BTreeMap<String, Vec<ClassChassis>> = BTreeMap::new();
    for book in &books {
        for ((_, slug), chassis) in class_chassis_sheet_rules::records(&[book.as_str()]) {
            if chassis.is_conventional() {
                out.entry(slug).or_default().push(chassis);
            }
        }
    }
    out
}

/// One slug's chassis from its printings. The supersession ruling (`decisions.md` §12: the newest
/// printing of ONE object wins) needs publication order (`.pcc` `SOURCEDATE:`) and a
/// field-by-field proof that the printings are one object; only the converter's resolver
/// (`codex-ingest` `sheet_rule::reprint`) holds both, and the runtime package carries neither.
/// Applied to today's three shared class slugs, that resolver proves one (Hellknight: identical
/// class rows, Adventurer's Guide 2017-06 over Inner Sea World Guide 2011-03) and leaves two
/// unordered (Cyphermage and Red Mantis Assassin rows differ in non-`SOURCE` tokens and state no
/// `DESC:`), `f3p-receipt.md` §P3. So a slug several books state is answered only when every
/// printing states the SAME chassis ([`ClassChassis::same_chassis`]): then no printing can change
/// a number, and the first in directory order is kept for its citation. Printings that disagree
/// answer nothing (the class's saves and hit points are then named Unknown by the fold), never a
/// book chosen by its directory name.
pub(crate) fn resolve_printings(printings: Vec<ClassChassis>) -> Option<ClassChassis> {
    let mut printings = printings.into_iter();
    let first = printings.next()?;
    printings.all(|other| first.same_chassis(&other)).then_some(first)
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

/// SD-36 F3 polish P1: the class lines whose value reads the CHARACTER's base attack bonus with
/// this class's own levels substituted per the class's rule (the line's value is that substituted
/// BAB plus a constant). ONE rule for every such line: in a mix, the base attack bonus the OTHER
/// classes give adds to it (the fold's total BAB minus this class's own). Monk's Flurry of Blows
/// (CRB p.57): "the monk's base attack bonus from monk levels is equal to her monk level", and
/// BAB from other classes adds. Classification of every class line that speaks of a base attack
/// bonus: `tests/sd36_f3_polish.rs::p1_scan_every_class_line_that_speaks_of_a_base_attack_bonus_is_classified`.
pub(crate) const LINES_READING_CHARACTER_BAB: &[&str] = &["class_chassis.monk.flurry_of_blows_attack_bonus"];

/// `explanation` (a [`LINES_READING_CHARACTER_BAB`] line from a class's isolated run) with the
/// other classes' base attack bonus `others` added.
fn with_other_classes_bab(explanation: ComputationExplanation, others: i16) -> ComputationExplanation {
    let value = explanation.value + others;
    ComputationExplanation {
        detail: format!(
            "{}. In this mix the base attack bonus from the character's other classes ({others:+}) \
             adds (the class's own levels stand in for its own base attack bonus only): {} {others:+} \
             = {value}",
            explanation.detail, explanation.value
        ),
        value,
        ..explanation
    }
}

/// `true` for an isolated-run explanation that is a character-level total the fold
/// computes once (the chassis BAB/save rows, a class's own level-1 hit-point total, which holds only
/// when that class is the character's first level), not a class line.
fn is_character_level_total(id: &str) -> bool {
    id == "class_chassis.base_attack_bonus"
        || id == CLASS_CHASSIS_SKILL_POINTS
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

/// One class's class skill-point term: `levels x max(1, ranks + Int)` (CRB p.30), the ranks read
/// from its chassis record, else from its converted class principal (or the base class a
/// class-selection class is taken on, [`class_chassis_sheet_rules::skill_ranks_per_level_from_package`]).
/// `Ok((total, ranks, source))`; a class neither states is [`SKILL_POINTS_UNKNOWN`], named.
pub(crate) fn class_skill_points(
    class_id: &str,
    levels: u8,
    intelligence_modifier: i16,
) -> Result<(i16, u8, String), ChassisUnknown> {
    let slug = class_id.strip_prefix("class:").unwrap_or(class_id);
    let (ranks, source) = chassis_record(class_id)
        .and_then(|r| r.skill_ranks_per_level.map(|ranks| (ranks, format!("{}:class:{}", r.book, r.slug))))
        .or_else(|| class_chassis_sheet_rules::skill_ranks_per_level_from_package(slug))
        .ok_or_else(|| ChassisUnknown {
            id: SKILL_POINTS_UNKNOWN,
            message: format!(
                "{class_id}: no converted class record states this class's skill ranks per level, \
                 so its skill points are Unknown"
            ),
        })?;
    Ok((i16::from(levels) * (i16::from(ranks) + intelligence_modifier).max(1), ranks, source))
}

/// A single-class character's class skill points (SD-36 F3b3): the same per-class term the
/// fold sums ([`ClassChassis::skill_points`]: skill ranks per level + Intelligence modifier, at
/// least 1, times the class's levels), so a class alone and the same class in a mix print one
/// number. Racial and favored-class extras are not the class's: the single-class path adds none
/// to any skill-point total (Human's extra rank is a printed recognition line,
/// `race.human.trait_bundle.extra_skill_ranks`), so neither does this line. A class whose record
/// states no skill ranks per level is [`SKILL_POINTS_UNKNOWN`], named, never 0.
pub(crate) fn explain_single_class_skill_points(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let [class_level] = input.chosen.class_levels.as_slice() else { return };
    let class_id = &class_level.class_id;
    match class_skill_points(class_id, class_level.level, ability_modifiers.intelligence) {
        Ok((total, ranks, source)) => explanations.push(ComputationExplanation {
            id: CLASS_CHASSIS_SKILL_POINTS.to_owned(),
            value: total,
            detail: format!(
                "{class_id} {} skill points {total}: {ranks} skill ranks per level + Intelligence \
                 modifier ({:+}), at least 1, times {} level(s) ({source}); class term only -- \
                 racial and favored-class ranks are not the class's",
                class_level.level, ability_modifiers.intelligence, class_level.level
            ),
        }),
        Err(unknown) => diagnostics.push(ComputationDiagnostic {
            id: unknown.id.to_owned(),
            message: unknown.message,
            claim_blocking: false,
        }),
    }
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
        // SD-36 F3 polish P4: the fold reads a class's hit die off its converted CHASSIS record
        // (a class record whose base attack bonus and saves converted); a class without one may
        // still state a hit die on its principal (the CRB Monk's `Hit die`), so the Unknown names
        // the missing chassis, not a missing hit die.
        let no_chassis = || ChassisUnknown {
            id: HIT_POINTS_UNKNOWN,
            message: format!(
                "{class_id}: no converted class chassis record (a class record whose base attack \
                 bonus and save progressions converted, the record the fold reads a hit die from), \
                 so its hit points are Unknown"
            ),
        };
        let hp = record
            .ok_or_else(no_chassis)
            .and_then(|r| r.hit_points(class_level.level, index == 0, ability_modifiers.constitution));
        let sp = class_skill_points(class_id, class_level.level, ability_modifiers.intelligence)
            .map(|(total, ..)| total);
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

    // The character's BAB, for a class line that reads it (`LINES_READING_CHARACTER_BAB`).
    let member_babs: Vec<Option<i16>> = class_levels
        .iter()
        .map(|class_level| member_base_attack_bonus(input, class_level, ability_modifiers))
        .collect();
    let total_bab: Option<i16> = member_babs.iter().copied().sum();

    // Each class's own lines, verbatim from its isolated single-class run -- except a line that
    // reads the character's base attack bonus, which gets the other classes' BAB added.
    for (class_level, own_bab) in class_levels.iter().zip(member_babs) {
        let slug = class_level.class_id.strip_prefix("class:").unwrap_or(&class_level.class_id);
        let isolated = compute_pilot_base_chassis(&isolated_input(input, class_level));
        for explanation in isolated.explanations {
            if !is_class_line(&explanation.id) || explanations.iter().any(|e| e.id == explanation.id) {
                continue;
            }
            let explanation = match (LINES_READING_CHARACTER_BAB.contains(&explanation.id.as_str()), total_bab, own_bab) {
                (true, Some(total), Some(own)) => with_other_classes_bab(explanation, total - own),
                _ => explanation,
            };
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

    /// SD-36 F3b3 (3): the four prestige classes whose carrier mix stays Blocked on
    /// `multiclass.save_shape.unrecognized`. Each slot below is the converted `Expr`, a faithful
    /// conversion of the oracle's own formula; the formula is not a PF1 save progression, so the
    /// classifier is right and nothing is guessed. PCGen's parser divides before it adds, so
    /// `classlevel+1/3` is `level + 1/3`, whose value at 10th level is 10 -- more than any PF1
    /// save table gives a 10-level class (prestige good +5, CRB p.374+). Mammoth Rider's
    /// `(classlevel+2)/2` gives +6 at 10th and +2 at 2nd, where the prestige good table gives
    /// +5 and +1, and its own row declares `ClassSaveGood_Fortitude` (the oracle contradicting its
    /// formula). The oracle lines: isg_classes.lst:25 (Exalted, Inner Sea Gods p.200),
    /// isg_classes.lst:48 (Sentinel, p.202), isc_classes.lst:29 (Ulfen Guard, Inner Sea Combat
    /// p.34), ag_classes.lst:260 (Mammoth Rider, Adventurer's Guide p.128).
    #[test]
    fn the_four_unrecognized_prestige_saves_are_oracle_formula_defects_not_a_missed_shape() {
        // (class, [Fort, Ref, Will] unrecognized?, value at 10th of each unrecognized slot)
        let cases: [(&str, [bool; 3], [i16; 3]); 4] = [
            ("class:exalted", [true, true, true], [10, 10, 10]),
            ("class:sentinel", [true, true, true], [10, 10, 10]),
            ("class:ulfen_guard", [true, true, true], [11, 10, 11]),
            ("class:mammoth_rider", [true, false, false], [6, 3, 3]),
        ];
        for (class_id, unrecognized, at_ten) in cases {
            let record = chassis_record(class_id).unwrap_or_else(|| panic!("{class_id} record"));
            let row = record.row_at(10).unwrap_or_else(|| panic!("{class_id} row 10"));
            assert_eq!([row.fort_save, row.ref_save, row.will_save], at_ten, "{class_id} at 10th");
            for (index, flagged) in unrecognized.into_iter().enumerate() {
                let shape = record.save_shape(index);
                if flagged {
                    assert_eq!(shape, Some(SaveProgression::Unrecognized), "{class_id} {}", SAVE_NAMES[index]);
                    // Above PF1's prestige good save at 10th (+5): not a progression the
                    // classifier missed.
                    assert!([row.fort_save, row.ref_save, row.will_save][index] > 5, "{class_id}");
                } else {
                    assert_eq!(shape, Some(SaveProgression::Poor), "{class_id} {}", SAVE_NAMES[index]);
                }
            }
            let fixture = crate::rules_core::class_census::load_sweep_fixture().expect("fixture");
            let level = CharacterClassLevel { class_id: class_id.to_owned(), level: 3 };
            match multiclass_member(&fixture, &level) {
                Err(d) => assert_eq!(d.id, SAVE_SHAPE_UNRECOGNIZED, "{d:?}"),
                Ok(_) => panic!("{class_id} must not fold an unrecognized save"),
            }
        }
    }

    #[test]
    fn two_more_prestige_saves_the_f3c_carriers_reach_are_the_same_oracle_formula_defect() {
        // SD-36 F3c: the carrier rule now names a carrier for Pure Legion Enforcer and Evangelist,
        // so their mixes reach the save gate for the first time. Both oracle `BONUS:SAVE` lines
        // are the same defect as F3b3 §3, converted faithfully:
        // - Pure Legion Enforcer (isc_classes.lst, Inner Sea Combat): `classlevel()+3/2`,
        //   `classlevel()+1/3`, `classlevel()+3/2` -- level + 3/2 (Ulfen Guard's shape), +11 at 10th.
        // - Evangelist (isg_classes.lst, Inner Sea Gods): Reflex `classlevel()/3+1` under its own
        //   `ClassSaveGood_Reflex` declaration; +4 at 10th, which is none of PF1's four save forms
        //   at 10th (base good 7, base poor 3, prestige good 5, prestige poor 3).
        let pf1_forms_at_ten = [7i16, 3, 5];
        let cases: [(&str, [bool; 3], [i16; 3]); 2] = [
            ("class:pure_legion_enforcer", [true, true, true], [11, 10, 11]),
            ("class:evangelist", [false, true, false], [3, 4, 3]),
        ];
        for (class_id, unrecognized, at_ten) in cases {
            let record = chassis_record(class_id).unwrap_or_else(|| panic!("{class_id} record"));
            let row = record.row_at(10).unwrap_or_else(|| panic!("{class_id} row 10"));
            let values = [row.fort_save, row.ref_save, row.will_save];
            assert_eq!(values, at_ten, "{class_id} at 10th");
            for (index, flagged) in unrecognized.into_iter().enumerate() {
                let shape = record.save_shape(index);
                if flagged {
                    assert_eq!(shape, Some(SaveProgression::Unrecognized), "{class_id} {}", SAVE_NAMES[index]);
                    assert!(!pf1_forms_at_ten.contains(&values[index]), "{class_id} {}", SAVE_NAMES[index]);
                } else {
                    assert_eq!(shape, Some(SaveProgression::Poor), "{class_id} {}", SAVE_NAMES[index]);
                }
            }
            let fixture = crate::rules_core::class_census::load_sweep_fixture().expect("fixture");
            let level = CharacterClassLevel { class_id: class_id.to_owned(), level: 3 };
            match multiclass_member(&fixture, &level) {
                Err(d) => assert_eq!(d.id, SAVE_SHAPE_UNRECOGNIZED, "{d:?}"),
                Ok(_) => panic!("{class_id} must not fold an unrecognized save"),
            }
        }
    }

    /// SD-36 F3 polish P3, the pin: every class slug more than one book states (denominator:
    /// every conventional chassis-bearing class record in every book), named, and each one's
    /// printings state the same chassis -- so neither this table nor
    /// `generic_class_chassis`'s book-precedence table can print a number that depends on which
    /// printing answered. A printing that differs fails here, naming the slug.
    #[test]
    fn every_slug_two_books_state_has_one_chassis_across_its_printings() {
        let printings = printings_by_slug();
        let records: usize = printings.values().map(Vec::len).sum();
        let shared: Vec<(&str, Vec<&str>)> = printings
            .iter()
            .filter(|(_, p)| p.len() > 1)
            .map(|(slug, p)| (slug.as_str(), p.iter().map(|c| c.book.as_str()).collect()))
            .collect();
        println!("{records} chassis records over {} slugs; shared by 2+ books: {shared:?}", printings.len());
        assert_eq!(
            shared,
            vec![
                ("cyphermage", vec!["adventurers_guide", "inner_sea_magic"]),
                ("hellknight", vec!["adventurers_guide", "inner_sea_world_guide"]),
                ("red_mantis_assassin", vec!["adventurers_guide", "inner_sea_world_guide"]),
            ],
            "the shared-slug population moved"
        );
        for (slug, group) in printings.iter().filter(|(_, p)| p.len() > 1) {
            for other in &group[1..] {
                assert!(
                    group[0].same_chassis(other),
                    "{slug}: {} and {} state different chassis -- the fold would print a number \
                     that depends on which printing answered",
                    group[0].book,
                    other.book
                );
            }
            assert!(chassis_record(&format!("class:{slug}")).is_some(), "{slug}");
        }
    }

    /// P3, the rule: printings that agree answer; printings that disagree answer nothing (never
    /// the first book by directory name).
    #[test]
    fn printings_that_disagree_answer_nothing() {
        let warrior = class_chassis_sheet_rules::record("core_rulebook", "warrior").expect("warrior").clone();
        let commoner = class_chassis_sheet_rules::record("core_rulebook", "commoner").expect("commoner").clone();
        assert!(!warrior.same_chassis(&commoner));
        assert!(resolve_printings(vec![warrior.clone(), commoner.clone()]).is_none());
        assert!(resolve_printings(vec![commoner.clone(), warrior.clone()]).is_none());
        assert_eq!(resolve_printings(vec![warrior.clone(), warrior.clone()]).map(|c| c.slug), Some("warrior".to_owned()));
        assert!(resolve_printings(Vec::new()).is_none());
    }

    #[test]
    fn a_class_whose_record_states_no_skill_ranks_is_named_unknown() {
        // Eidolon: `STARTSKILLPTS:EidolonSkillPoints` (apg_classes.lst:211), a variable its
        // closure does not define -- no row (`_defects/skill-ranks-unresolved.json`), never 0.
        let err = class_skill_points("class:eidolon", 3, 0).expect_err("eidolon states no ranks");
        assert_eq!(err.id, SKILL_POINTS_UNKNOWN);
        assert!(err.message.contains("class:eidolon"), "{}", err.message);
    }

    #[test]
    fn skill_ranks_come_from_the_record_or_the_base_class_it_is_taken_on() {
        // Monk's CRB principal is not chassis-bearing (degraded BAB, F3a) but states 4
        // (cr_classes.lst STARTSKILLPTS:4, CRB p.56): 3 levels, Int +1 = 15.
        assert_eq!(class_skill_points("class:monk", 3, 1).map(|(t, r, _)| (t, r)).ok(), Some((15, 4)));
        // Unchained Rogue is taken on Rogue (8, CRB p.67; Pathfinder Unchained p.20 keeps 8):
        // 2 levels, Int -1 = 14.
        let (total, ranks, source) = class_skill_points("class:unchained_rogue", 2, -1).expect("base class row");
        assert_eq!((total, ranks), (14, 8), "{source}");
        assert!(source.ends_with(":class:rogue"), "{source}");
        // Int low enough: at least 1 per level (CRB p.30).
        assert_eq!(class_skill_points("class:fighter", 4, -3).map(|(t, ..)| t).ok(), Some(4));
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
