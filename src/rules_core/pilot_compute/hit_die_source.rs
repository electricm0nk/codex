//! SD-36 Epic F6b: ONE rule for a class's hit die -- the source that COMPUTES the class's hit
//! points is the source whose die the sheet prints, and the creation roster offers a class iff
//! that source exists.
//!
//! # The rule
//!
//! [`hit_die_source`] asks, in order:
//!
//! 1. **The bespoke class module** ([`crate::rules_core::durability::bespoke_hit_die`]): the
//!    Core Rulebook, APG, ACG and Pathfinder Unchained class tables that `compute_max_hp` already
//!    computes a single-class character's hit points from. The CRB Monk is here with d8 (CRB
//!    p.56), which is why the sheet never prints the converted Monk record's `Hit die d10` -- that
//!    row is the FS-23 oracle defect (`cr_classes.lst:147` says `HD:10`), not the book.
//! 2. **The converted class record** ([`class_chassis_sheet_rules::hit_die_from_package`]): the
//!    class principal's `StatBlock "Hit die"` row, or -- for a class-selection class whose own
//!    principal states no die and grants `TakenOnClass(<base>)` (Pathfinder Unchained's four) --
//!    the base class line's row, which is the line such a character holds.
//!
//! `None` from both is the only way a class has no hit points: never a fabricated die.
//!
//! [`class_hit_points`] is the per-class term every hit-point fold reads (the multiclass fold,
//! the desktop roster's `hitPointsDie`), and [`print_hit_die_lines`] is the one place a class
//! line's `Hit die:` prose is written, from the same source.

use crate::rules_core::pilot_compute::class_chassis_sheet_rules::{self, ChassisUnknown, HIT_POINTS_UNKNOWN};
use crate::rules_core::sheet_rule::SheetLine;

/// Where a class's hit die comes from, and the die.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HitDieSource {
    /// Die size: `d8` -> `8`.
    pub die: u8,
    /// The source, cited: a bespoke module path, or the converted rule id read.
    pub source: String,
}

/// The hit die the engine computes `class_id`'s (`"class:<slug>"`) hit points from -- see the
/// module doc for the order.
pub fn hit_die_source(class_id: &str) -> Option<HitDieSource> {
    if let Some((die, module)) = crate::rules_core::durability::bespoke_hit_die(class_id) {
        return Some(HitDieSource { die, source: format!("{module} (bespoke class module)") });
    }
    let slug = class_id.strip_prefix("class:")?;
    let (die, rule_id) = class_chassis_sheet_rules::hit_die_from_package(slug)?;
    Some(HitDieSource { die, source: rule_id })
}

/// The label a class line's hit-die prose carries (`ProseFamily::StatBlock("Hit die")` renders
/// `Hit die: d8`).
const HIT_DIE_PROSE_LABEL: &str = "Hit die: ";

/// Writes every class line's `Hit die:` prose from [`hit_die_source`] -- the one place a sheet
/// prints a class's hit die. A class line (`kind == "class"`, a principal id with no `#`)
/// whose class has a source prints `Hit die: d<source die>` in place of whatever `Hit die:` line
/// the converted record's prose rendered (the CRB Monk's rendered `Hit die: d10` is the FS-23
/// oracle defect; the source computing its hit points is the CRB table's d8), or gains one
/// when its record states none (a class-selection class taken on its base class line). A class
/// with no source prints no `Hit die:` line: its hit points are Unknown, and a printed die would
/// be one the engine does not compute from.
pub fn print_hit_die_lines(lines: &mut [SheetLine]) {
    for line in lines.iter_mut().filter(|l| l.kind == "class" && !l.id.contains('#')) {
        let Some((_, slug)) = line.id.rsplit_once(":class:") else { continue };
        let source = hit_die_source(&format!("class:{slug}"));
        let mut rows: Vec<String> = line.prose.split('\n').filter(|row| !row.is_empty()).map(str::to_owned).collect();
        // The converted row's place, when it rendered one; otherwise the end of the stat block.
        let at = rows.iter().position(|row| row.starts_with(HIT_DIE_PROSE_LABEL));
        rows.retain(|row| !row.starts_with(HIT_DIE_PROSE_LABEL));
        if let Some(source) = source {
            rows.insert(at.unwrap_or(rows.len()), format!("{HIT_DIE_PROSE_LABEL}d{}", source.die));
        }
        line.prose = rows.join("\n");
    }
}

/// PF1 hit points for `levels` levels of a class with a `die`-sided hit die: the full die for the
/// level that is the character's 1st (`includes_first_character_level`), the non-rolling average
/// ([`crate::rules_core::durability::average_hit_die_value`]) for every other, each level plus
/// `constitution_modifier` and floored at 1.
pub fn hit_points_from_die(die: u8, levels: u8, includes_first_character_level: bool, constitution_modifier: i16) -> i16 {
    let mut total = 0_i16;
    for level in 1..=levels {
        let die_value = if level == 1 && includes_first_character_level {
            i16::from(die)
        } else {
            crate::rules_core::durability::average_hit_die_value(die)
        };
        total += (die_value + constitution_modifier).max(1);
    }
    total
}

/// One class's hit-point term: [`hit_points_from_die`] over [`hit_die_source`], or
/// [`HIT_POINTS_UNKNOWN`] naming the class when no source states a die.
pub fn class_hit_points(
    class_id: &str,
    levels: u8,
    includes_first_character_level: bool,
    constitution_modifier: i16,
) -> Result<(i16, HitDieSource), ChassisUnknown> {
    let source = hit_die_source(class_id).ok_or_else(|| ChassisUnknown {
        id: HIT_POINTS_UNKNOWN,
        message: format!(
            "{class_id}: no bespoke class module and no converted class record (nor the base class \
             line it is taken on) states a hit die, so its hit points are Unknown"
        ),
    })?;
    Ok((hit_points_from_die(source.die, levels, includes_first_character_level, constitution_modifier), source))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::class_census::load_sweep_fixture;

    /// The census fixture's Constitution modifier (`constitution:14`, Human: no racial Con).
    fn fixture_con_modifier() -> i16 {
        let fixture = load_sweep_fixture().expect("census fixture");
        let con = fixture.chosen.ability_scores.constitution;
        crate::rules_core::pilot_compute::ability_modifier(con)
    }

    /// The five roster classes F4c left at HP Unknown, each at level 5 on the census fixture
    /// (Con 14, +2), hand-worked with the non-rolling rule: level 1 the full die, levels 2-5 the
    /// average (die/2 + 1), + Con every level.
    ///
    /// - Monk: d8 (CRB p.56, "Hit Die: d8"): 8+2 + 4 x (5+2) = **38**.
    /// - Unchained Barbarian: d12 (Pathfinder Unchained class entry, "Hit Die: d12"): 12+2 + 4 x (7+2) = **50**.
    /// - Unchained Monk: d10 (Pathfinder Unchained class entry, "Hit Die: d10"): 10+2 + 4 x (6+2) = **44**.
    /// - Unchained Rogue: d8 (Pathfinder Unchained class entry, "Hit Die: d8"): 8+2 + 4 x (5+2) = **38**.
    /// - Unchained Summoner: d8 (Pathfinder Unchained class entry, "Hit Die: d8"): 8+2 + 4 x (5+2) = **38**.
    #[test]
    fn the_five_hp_unknown_classes_now_print_hp() {
        let con = fixture_con_modifier();
        assert_eq!(con, 2, "census fixture Con 14");
        let mut failures = Vec::new();
        for (class_id, die, hp) in [
            ("class:monk", 8, 38),
            ("class:unchained_barbarian", 12, 50),
            ("class:unchained_monk", 10, 44),
            ("class:unchained_rogue", 8, 38),
            ("class:unchained_summoner", 8, 38),
        ] {
            match class_hit_points(class_id, 5, true, con) {
                Ok((got, source)) if got == hp && source.die == die => {}
                Ok((got, source)) => failures.push(format!("{class_id} 5: expected d{die} {hp} HP, got d{} {got} ({})", source.die, source.source)),
                Err(unknown) => failures.push(format!("{class_id} 5: expected d{die} {hp} HP, got Unknown ({})", unknown.message)),
            }
        }
        assert!(failures.is_empty(), "{} of 5 still HP Unknown or wrong:\n{}", failures.len(), failures.join("\n"));
    }

    /// The bespoke tier and the converted tier disagree on exactly one census class: the CRB
    /// Monk (FS-23: the pinned oracle's `cr_classes.lst:147` says `HD:10`; CRB p.56 says d8).
    /// Everywhere else the order of the two tiers changes no die. Denominator: every census id
    /// (`class_census::census()`); printed with the counts.
    #[test]
    fn bespoke_and_converted_dice_agree_except_the_fs23_monk() {
        let census = crate::rules_core::class_census::census();
        let (mut both, mut bespoke_only, mut converted_only, mut neither) = (0, 0, 0, 0);
        let mut disagree = Vec::new();
        for class_id in census.keys() {
            let slug = class_id.strip_prefix("class:").expect("census ids are class:<slug>");
            let bespoke = crate::rules_core::durability::bespoke_hit_die(class_id).map(|(die, _)| die);
            let converted = class_chassis_sheet_rules::hit_die_from_package(slug).map(|(die, _)| die);
            match (bespoke, converted) {
                (Some(b), Some(c)) => {
                    both += 1;
                    if b != c {
                        disagree.push(format!("{class_id}: bespoke d{b}, converted d{c}"));
                    }
                }
                (Some(_), None) => bespoke_only += 1,
                (None, Some(_)) => converted_only += 1,
                (None, None) => neither += 1,
            }
        }
        eprintln!(
            "{} census ids: both tiers {both}, bespoke only {bespoke_only}, converted only {converted_only}, neither {neither}; disagree {disagree:?}",
            census.len()
        );
        assert_eq!(disagree, vec!["class:monk: bespoke d8, converted d10".to_owned()]);
    }

    /// Every class the multiclass fold used to read a chassis-record die for reads the same die
    /// now: the rule change moves only the classes that had none (the five above).
    #[test]
    fn every_chassis_record_die_is_unchanged_by_the_source_rule() {
        let census = crate::rules_core::class_census::census();
        let mut compared = 0;
        for class_id in census.keys() {
            let Some(record) = crate::rules_core::pilot_compute::multiclass_fold::chassis_record(class_id) else { continue };
            let Some(die) = record.hit_die else { continue };
            compared += 1;
            assert_eq!(hit_die_source(class_id).map(|s| s.die), Some(die), "{class_id}");
        }
        eprintln!("chassis-record dice compared: {compared} of {} census ids", census.len());
        assert!(compared > 0);
    }

    fn class_line_prose(class: &str, level: u8) -> String {
        let fixture = load_sweep_fixture().expect("census fixture");
        let input = crate::rules_core::class_seeds::input_for(&fixture, class, level);
        let package = crate::rules_core::sheet_rule_package::package().as_ref().expect("package");
        let computed = crate::rules_core::pilot_compute::compute_pilot_base_chassis(&input).with_sheet_rules(&input, package, &[]);
        let lines: Vec<_> = computed.sheet_lines.iter().filter(|l| l.kind == "class" && l.id.ends_with(&format!(":class:{class}"))).collect();
        assert_eq!(lines.len(), 1, "{class}: one class line, got {:?}", computed.sheet_lines.iter().filter(|l| l.kind == "class").map(|l| &l.id).collect::<Vec<_>>());
        lines[0].prose.clone()
    }

    /// The printed hit-die line is the computing source's. Monk prints d8 (the CRB table that
    /// computes its hit points), never the converted record's FS-23 `d10`; Unchained Rogue, whose
    /// record states no die, prints its base Rogue line's d8; Fighter's line is unchanged (d10).
    #[test]
    fn the_class_line_prints_the_die_its_hit_points_are_computed_from() {
        let monk = class_line_prose("monk", 5);
        assert!(monk.lines().any(|l| l == "Hit die: d8"), "{monk}");
        assert!(!monk.contains("d10"), "the FS-23 corpus die is not printed: {monk}");
        assert!(monk.contains("Skill ranks per level: 4"), "the rest of the record's prose stays: {monk}");
        let unchained_rogue = class_line_prose("unchained_rogue", 5);
        assert!(unchained_rogue.lines().any(|l| l == "Hit die: d8"), "{unchained_rogue}");
        let fighter = class_line_prose("fighter", 1);
        assert_eq!(fighter.lines().filter(|l| l.starts_with("Hit die: ")).collect::<Vec<_>>(), vec!["Hit die: d10"], "{fighter}");
    }
}
