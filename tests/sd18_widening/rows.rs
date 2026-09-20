//! Row tables + macros for near-universal negative-control shapes shared
//! across many (class, level) modules in the sd18_widening family.
//!
//! SD-36 Epic C2 (`docs/release/SD-36-consolidation/epic-breakdown.md`
//! C2.1/C2.2, technical-design.md "Pass C2"): each per-class-per-level
//! module keeps being its own `mod` (declared from `main.rs`'s `roster!`),
//! so `cargo test --test sd18_widening -- --list` keeps emitting the exact
//! same `<module>::<fn_name>: test` entries as before this rewrite; only
//! the boilerplate BODY of the near-universal shapes moves here as data +
//! a macro. Bespoke tests are untouched (see AT-35-E1-003 / this epic's
//! progress log for the full accounting).
//!
//! THE SAFETY RULE (operator ruling 2026-09-20): every assert!/assert_eq!
//! in a converted test's old body survives here with the same expected
//! values and the same subject. For this shape (a negative control that a
//! Fighter-only fixture never surfaces any `class_chassis.<class>.` /
//! `class_feature.<class>.` explanation), the per-class variance — whether
//! the `class_feature.` prefix is also checked, and any extra exact-match
//! id (Sorcerer's un-namespaced spell-baseline record) — is DATA in the
//! row, not code, exactly mirroring the per-class exception lists already
//! used inline (e.g. the barbarian rage-execution exclusion in the sibling
//! `*_is_not_promoted_by_this_slice` bodies, left bespoke — see below).

/// Shared Fighter-only fixture for every `fighter_does_not_gain_*`
/// negative control below (previously duplicated verbatim as a per-file
/// `const FIGHTER_FIXTURE` in each of the 80 files that used it).
pub const FIGHTER_FIXTURE: &str = include_str!(
    "../fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

/// Row for the "Fighter does not gain `<class>`-namespaced recognition"
/// negative control (`fighter_does_not_gain_<class>_level<N>[_..]_recognition`).
/// Asserts a Fighter-only fixture's computation never surfaces any
/// explanation in the target class's namespace(s).
#[derive(Clone, Copy)]
pub struct FighterNegRow {
    pub class: &'static str,
    /// Whether `class_feature.<class>.` is also checked (most classes),
    /// or only `class_chassis.<class>.` (Paladin, Sorcerer).
    pub feature_prefix: bool,
    /// An extra exact-match id outside the class's own namespace prefix
    /// (Sorcerer's un-namespaced `class_chassis.spell_baseline.sorcerer`).
    pub extra_exact: Option<&'static str>,
}

/// One row per class that has this negative control (Fighter itself does
/// not test against itself, so 10 of the 11 sd18 classes appear here).
pub const FIGHTER_NEG_ROWS: &[FighterNegRow] = &[
    FighterNegRow { class: "barbarian", feature_prefix: true, extra_exact: None },
    FighterNegRow { class: "bard", feature_prefix: true, extra_exact: None },
    FighterNegRow { class: "cleric", feature_prefix: true, extra_exact: None },
    FighterNegRow { class: "druid", feature_prefix: true, extra_exact: None },
    FighterNegRow { class: "monk", feature_prefix: true, extra_exact: None },
    FighterNegRow { class: "paladin", feature_prefix: false, extra_exact: None },
    FighterNegRow { class: "ranger", feature_prefix: true, extra_exact: None },
    FighterNegRow { class: "rogue", feature_prefix: true, extra_exact: None },
    FighterNegRow {
        class: "sorcerer",
        feature_prefix: false,
        extra_exact: Some("class_chassis.spell_baseline.sorcerer"),
    },
    FighterNegRow { class: "wizard", feature_prefix: true, extra_exact: None },
];

pub fn find_fighter_neg_row(class: &str) -> FighterNegRow {
    *FIGHTER_NEG_ROWS
        .iter()
        .find(|r| r.class == class)
        .unwrap_or_else(|| panic!("rows.rs: no FighterNegRow for class '{class}'"))
}

/// Expands to one `#[test] fn $fn_name()` performing the exact assert! this
/// shape always performed, with the class-specific prefixes/exact-id
/// (previously spelled out per-file) pulled from `FIGHTER_NEG_ROWS` by
/// class name instead.
#[macro_export]
macro_rules! sd18_fighter_neg_control_test {
    ($fn_name:ident, $class:expr) => {
        #[test]
        fn $fn_name() {
            let row = $crate::rows::find_fighter_neg_row($class);
            let fighter = $crate::common::load($crate::rows::FIGHTER_FIXTURE);
            let fighter_computation =
                codex::rules_core::pilot_compute::compute_pilot_base_chassis(&fighter);
            let chassis_prefix = format!("class_chassis.{}.", row.class);
            let feature_prefix = format!("class_feature.{}.", row.class);
            assert!(
                !fighter_computation.explanations.iter().any(|e| {
                    e.id.starts_with(chassis_prefix.as_str())
                        || (row.feature_prefix && e.id.starts_with(feature_prefix.as_str()))
                        || row.extra_exact == Some(e.id.as_str())
                }),
                "the Fighter chassis must not surface any {}-namespaced explanation: {:?}",
                row.class,
                fighter_computation.explanations
            );
        }
    };
}


// ---------------------------------------------------------------------
// Shape B: `<class>_level_21_is_not_promoted_by_this_slice` (boundary
// negative control) — 38 rows.
// Shape C: `multiclass_<class>_level<N>_is_not_promoted_by_this_slice`
// — 64 rows. Both keyed by module (this family's
// per-(class,level) file, minus `.rs`) since the fn name itself repeats
// across sibling level modules with different row data.
//
// Genuinely bespoke exceptions NOT in these tables (left untouched in
// their own files, not forced into this shape — see
// sd18_widening-progress.md): the 5 Druid boundary/multiclass tests
// (custom `is_gated_druid_chassis_record` predicate + a 9-argument wolf
// companion stat-block assertion, nothing like this template), and the 19
// Fighter/Wizard multiclass tests where the SD-24/v0.6-swarm multiclass
// BAB-stacking widening flipped the assertion's own polarity (these now
// assert the class chassis DOES fire, not that it doesn't — a genuinely
// different assertion, not a per-class exception list entry).
// ---------------------------------------------------------------------

/// Row for the boundary `<class>_level_21_is_not_promoted_by_this_slice`
/// negative control: replaces the fixture's own class:level with class:21,
/// then asserts none of the class's explanations (prefixes/exact_ors, minus
/// excludes/exclude_prefixes) appear.
#[derive(Clone, Copy)]
pub struct NegControlRow {
    pub old_sub: &'static str,
    pub new_sub: &'static str,
    pub prefixes: &'static [&'static str],
    pub exact_ors: &'static [&'static str],
    pub excludes: &'static [&'static str],
    pub exclude_prefixes: &'static [&'static str],
    /// Message text with the trailing `": {:?}"` stripped (the macro
    /// re-appends it via real `{:?}` interpolation of
    /// `computation.explanations`, rather than storing that literal
    /// placeholder as inert data).
    pub msg: &'static str,
}

pub const BOUNDARY_NEG_ROWS: &[(&str, NegControlRow)] = &[
    ("barbarian_level12", NegControlRow { old_sub: "class:barbarian:12", new_sub: "class:barbarian:21", prefixes: &["class_chassis.barbarian.", "class_feature.barbarian."], exact_ors: &[], excludes: &["class_feature.barbarian.rage_execution.not_raging"], exclude_prefixes: &[], msg: "level-21 Barbarian must not gain any bounded barbarian explanation" }),
    ("barbarian_level13", NegControlRow { old_sub: "class:barbarian:13", new_sub: "class:barbarian:21", prefixes: &["class_chassis.barbarian.", "class_feature.barbarian."], exact_ors: &[], excludes: &["class_feature.barbarian.rage_execution.not_raging"], exclude_prefixes: &[], msg: "level-21 Barbarian must not gain any bounded barbarian explanation" }),
    ("barbarian_level14", NegControlRow { old_sub: "class:barbarian:14", new_sub: "class:barbarian:21", prefixes: &["class_chassis.barbarian.", "class_feature.barbarian."], exact_ors: &[], excludes: &["class_feature.barbarian.rage_execution.not_raging"], exclude_prefixes: &[], msg: "level-21 Barbarian must not gain any bounded barbarian explanation" }),
    ("barbarian_level15", NegControlRow { old_sub: "class:barbarian:15", new_sub: "class:barbarian:21", prefixes: &["class_chassis.barbarian.", "class_feature.barbarian."], exact_ors: &[], excludes: &["class_feature.barbarian.rage_execution.not_raging"], exclude_prefixes: &[], msg: "level-21 Barbarian must not gain any bounded barbarian explanation" }),
    ("barbarian_level18", NegControlRow { old_sub: "class:barbarian:18", new_sub: "class:barbarian:21", prefixes: &["class_chassis.barbarian.", "class_feature.barbarian."], exact_ors: &[], excludes: &["class_feature.barbarian.rage_execution.not_raging"], exclude_prefixes: &[], msg: "level-21 Barbarian must not gain any bounded barbarian explanation" }),
    ("barbarian_level20", NegControlRow { old_sub: "class:barbarian:20", new_sub: "class:barbarian:21", prefixes: &["class_chassis.barbarian.", "class_feature.barbarian."], exact_ors: &[], excludes: &["class_feature.barbarian.rage_execution.not_raging"], exclude_prefixes: &[], msg: "level-21 Barbarian must not gain any bounded barbarian explanation" }),
    ("bard_level11_inspire", NegControlRow { old_sub: "class:bard:11", new_sub: "class:bard:21", prefixes: &["class_chassis.bard.", "class_feature.bard."], exact_ors: &["class_chassis.spell_baseline.bard"], excludes: &["class_feature.bard.bardic_performance_execution.not_performing", "class_feature.bard.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "level-21 Bard must not gain any bounded bard explanation" }),
    ("bard_level12", NegControlRow { old_sub: "class:bard:12", new_sub: "class:bard:21", prefixes: &["class_chassis.bard.", "class_feature.bard."], exact_ors: &["class_chassis.spell_baseline.bard"], excludes: &["class_feature.bard.bardic_performance_execution.not_performing", "class_feature.bard.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "level-21 Bard must not gain any bounded bard explanation" }),
    ("bard_level13", NegControlRow { old_sub: "class:bard:13", new_sub: "class:bard:21", prefixes: &["class_chassis.bard.", "class_feature.bard."], exact_ors: &["class_chassis.spell_baseline.bard"], excludes: &["class_feature.bard.bardic_performance_execution.not_performing", "class_feature.bard.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "level-21 Bard must not gain any bounded bard explanation" }),
    ("bard_level14", NegControlRow { old_sub: "class:bard:14", new_sub: "class:bard:21", prefixes: &["class_chassis.bard.", "class_feature.bard."], exact_ors: &["class_chassis.spell_baseline.bard"], excludes: &["class_feature.bard.bardic_performance_execution.not_performing", "class_feature.bard.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "level-21 Bard must not gain any bounded bard explanation" }),
    ("bard_level18", NegControlRow { old_sub: "class:bard:18", new_sub: "class:bard:21", prefixes: &["class_chassis.bard.", "class_feature.bard."], exact_ors: &["class_chassis.spell_baseline.bard"], excludes: &["class_feature.bard.bardic_performance_execution.not_performing", "class_feature.bard.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "level-21 Bard must not gain any bounded bard explanation" }),
    ("bard_level20", NegControlRow { old_sub: "class:bard:20", new_sub: "class:bard:21", prefixes: &["class_chassis.bard.", "class_feature.bard."], exact_ors: &["class_chassis.spell_baseline.bard"], excludes: &["class_feature.bard.bardic_performance_execution.not_performing", "class_feature.bard.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "level-21 Bard must not gain any bounded bard explanation" }),
    ("cleric_level11", NegControlRow { old_sub: "class:cleric:11", new_sub: "class:cleric:21", prefixes: &["class_chassis.cleric.", "class_feature.cleric."], exact_ors: &["class_chassis.spell_baseline.cleric"], excludes: &["class_feature.domain.good_touch_of_good_not_active", "class_feature.cleric.weapon_and_armor_proficiency", "class_feature.cleric.domain.generic.healing_domain.rebuke_death.rebukedeathtimes", "class_feature.cleric.aura.strength_level"], exclude_prefixes: &[], msg: "level-21 Cleric must not gain any bounded cleric explanation" }),
    ("cleric_level12", NegControlRow { old_sub: "class:cleric:12", new_sub: "class:cleric:21", prefixes: &["class_chassis.cleric.", "class_feature.cleric."], exact_ors: &["class_chassis.spell_baseline.cleric"], excludes: &["class_feature.domain.good_touch_of_good_not_active", "class_feature.cleric.weapon_and_armor_proficiency", "class_feature.cleric.domain.generic.healing_domain.rebuke_death.rebukedeathtimes", "class_feature.cleric.aura.strength_level"], exclude_prefixes: &[], msg: "level-21 Cleric must not gain any bounded cleric explanation" }),
    ("cleric_level13", NegControlRow { old_sub: "class:cleric:13", new_sub: "class:cleric:21", prefixes: &["class_chassis.cleric.", "class_feature.cleric."], exact_ors: &["class_chassis.spell_baseline.cleric"], excludes: &["class_feature.domain.good_touch_of_good_not_active", "class_feature.cleric.weapon_and_armor_proficiency", "class_feature.cleric.domain.generic.healing_domain.rebuke_death.rebukedeathtimes", "class_feature.cleric.aura.strength_level"], exclude_prefixes: &[], msg: "level-21 Cleric must not gain any bounded cleric explanation" }),
    ("cleric_level14", NegControlRow { old_sub: "class:cleric:14", new_sub: "class:cleric:21", prefixes: &["class_chassis.cleric.", "class_feature.cleric."], exact_ors: &["class_chassis.spell_baseline.cleric"], excludes: &["class_feature.domain.good_touch_of_good_not_active", "class_feature.cleric.weapon_and_armor_proficiency", "class_feature.cleric.domain.generic.healing_domain.rebuke_death.rebukedeathtimes", "class_feature.cleric.aura.strength_level"], exclude_prefixes: &[], msg: "level-21 Cleric must not gain any bounded cleric explanation" }),
    ("cleric_level18", NegControlRow { old_sub: "class:cleric:18", new_sub: "class:cleric:21", prefixes: &["class_chassis.cleric.", "class_feature.cleric."], exact_ors: &["class_chassis.spell_baseline.cleric"], excludes: &["class_feature.domain.good_touch_of_good_not_active", "class_feature.cleric.weapon_and_armor_proficiency", "class_feature.cleric.domain.generic.healing_domain.rebuke_death.rebukedeathtimes", "class_feature.cleric.aura.strength_level"], exclude_prefixes: &[], msg: "level-21 Cleric must not gain any bounded cleric explanation" }),
    ("cleric_level19", NegControlRow { old_sub: "class:cleric:19", new_sub: "class:cleric:21", prefixes: &["class_chassis.cleric.", "class_feature.cleric."], exact_ors: &["class_chassis.spell_baseline.cleric"], excludes: &["class_feature.domain.good_touch_of_good_not_active", "class_feature.cleric.weapon_and_armor_proficiency", "class_feature.cleric.domain.generic.healing_domain.rebuke_death.rebukedeathtimes", "class_feature.cleric.aura.strength_level"], exclude_prefixes: &[], msg: "level-21 Cleric must not gain any bounded cleric explanation" }),
    ("cleric_level20", NegControlRow { old_sub: "class:cleric:20", new_sub: "class:cleric:21", prefixes: &["class_chassis.cleric.", "class_feature.cleric."], exact_ors: &["class_chassis.spell_baseline.cleric"], excludes: &["class_feature.domain.good_touch_of_good_not_active", "class_feature.cleric.weapon_and_armor_proficiency", "class_feature.cleric.domain.generic.healing_domain.rebuke_death.rebukedeathtimes", "class_feature.cleric.aura.strength_level"], exclude_prefixes: &[], msg: "level-21 Cleric must not gain any bounded cleric explanation" }),
    ("paladin_level12", NegControlRow { old_sub: "class:paladin:12", new_sub: "class:paladin:21", prefixes: &["class_chassis.paladin."], exact_ors: &[], excludes: &[], exclude_prefixes: &[], msg: "level-21 Paladin must not gain any bounded paladin chassis explanation" }),
    ("paladin_level13", NegControlRow { old_sub: "class:paladin:13", new_sub: "class:paladin:21", prefixes: &["class_chassis.paladin."], exact_ors: &[], excludes: &[], exclude_prefixes: &[], msg: "level-21 Paladin must not gain any bounded paladin chassis explanation" }),
    ("paladin_level14", NegControlRow { old_sub: "class:paladin:14", new_sub: "class:paladin:21", prefixes: &["class_chassis.paladin."], exact_ors: &[], excludes: &[], exclude_prefixes: &[], msg: "level-21 Paladin must not gain any bounded paladin chassis explanation" }),
    ("paladin_level20", NegControlRow { old_sub: "class:paladin:20", new_sub: "class:paladin:21", prefixes: &["class_chassis.paladin."], exact_ors: &[], excludes: &[], exclude_prefixes: &[], msg: "level-21 Paladin must not gain any bounded paladin chassis explanation" }),
    ("ranger_level12", NegControlRow { old_sub: "class:ranger:12", new_sub: "class:ranger:21", prefixes: &["class_chassis.ranger.", "class_feature.ranger."], exact_ors: &[], excludes: &["class_feature.ranger.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "level-21 Ranger must not gain any bounded ranger chassis explanation" }),
    ("ranger_level13", NegControlRow { old_sub: "class:ranger:13", new_sub: "class:ranger:21", prefixes: &["class_chassis.ranger.", "class_feature.ranger."], exact_ors: &[], excludes: &["class_feature.ranger.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "level-21 Ranger must not gain any bounded ranger chassis explanation" }),
    ("ranger_level14", NegControlRow { old_sub: "class:ranger:14", new_sub: "class:ranger:21", prefixes: &["class_chassis.ranger.", "class_feature.ranger."], exact_ors: &[], excludes: &["class_feature.ranger.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "level-21 Ranger must not gain any bounded ranger chassis explanation" }),
    ("ranger_level20", NegControlRow { old_sub: "class:ranger:20", new_sub: "class:ranger:21", prefixes: &["class_chassis.ranger.", "class_feature.ranger."], exact_ors: &[], excludes: &["class_feature.ranger.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "level-21 Ranger must not gain any bounded ranger chassis explanation" }),
    ("rogue_level20", NegControlRow { old_sub: "class:rogue:20", new_sub: "class:rogue:21", prefixes: &["class_chassis.rogue.", "class_feature.rogue."], exact_ors: &[], excludes: &["class_feature.rogue.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "level-21 Rogue must not gain any bounded rogue explanation (PF1 has no 21st character \
         level; this is a pure implementation-gate check)" }),
    ("sorcerer_level11", NegControlRow { old_sub: "class:sorcerer:11", new_sub: "class:sorcerer:21", prefixes: &["class_chassis.sorcerer."], exact_ors: &["class_chassis.spell_baseline.sorcerer"], excludes: &[], exclude_prefixes: &[], msg: "level-21 Sorcerer must not gain any bounded sorcerer chassis explanation" }),
    ("sorcerer_level12", NegControlRow { old_sub: "class:sorcerer:12", new_sub: "class:sorcerer:21", prefixes: &["class_chassis.sorcerer."], exact_ors: &["class_chassis.spell_baseline.sorcerer"], excludes: &[], exclude_prefixes: &[], msg: "level-21 Sorcerer must not gain any bounded sorcerer chassis explanation" }),
    ("sorcerer_level13", NegControlRow { old_sub: "class:sorcerer:13", new_sub: "class:sorcerer:21", prefixes: &["class_chassis.sorcerer."], exact_ors: &["class_chassis.spell_baseline.sorcerer"], excludes: &[], exclude_prefixes: &[], msg: "level-21 Sorcerer must not gain any bounded sorcerer chassis explanation" }),
    ("sorcerer_level14", NegControlRow { old_sub: "class:sorcerer:14", new_sub: "class:sorcerer:21", prefixes: &["class_chassis.sorcerer."], exact_ors: &["class_chassis.spell_baseline.sorcerer"], excludes: &[], exclude_prefixes: &[], msg: "level-21 Sorcerer must not gain any bounded sorcerer chassis explanation" }),
    ("sorcerer_level20", NegControlRow { old_sub: "class:sorcerer:20", new_sub: "class:sorcerer:21", prefixes: &["class_chassis.sorcerer."], exact_ors: &["class_chassis.spell_baseline.sorcerer"], excludes: &[], exclude_prefixes: &[], msg: "level-21 Sorcerer must not gain any bounded sorcerer chassis explanation (PF1 has no \
         21st character level)" }),
    ("wizard_level11", NegControlRow { old_sub: "class:wizard:11", new_sub: "class:wizard:21", prefixes: &["class_chassis.wizard.", "class_feature.wizard."], exact_ors: &["class_chassis.spell_baseline.wizard"], excludes: &["class_feature.wizard.weapon_and_armor_proficiency"], exclude_prefixes: &["class_feature.wizard.corpus_record."], msg: "level-21 Wizard must not gain any bounded wizard explanation" }),
    ("wizard_level12", NegControlRow { old_sub: "class:wizard:12", new_sub: "class:wizard:21", prefixes: &["class_chassis.wizard.", "class_feature.wizard."], exact_ors: &["class_chassis.spell_baseline.wizard"], excludes: &["class_feature.wizard.weapon_and_armor_proficiency"], exclude_prefixes: &["class_feature.wizard.corpus_record."], msg: "level-21 Wizard must not gain any bounded wizard explanation" }),
    ("wizard_level13", NegControlRow { old_sub: "class:wizard:13", new_sub: "class:wizard:21", prefixes: &["class_chassis.wizard.", "class_feature.wizard."], exact_ors: &["class_chassis.spell_baseline.wizard"], excludes: &["class_feature.wizard.weapon_and_armor_proficiency"], exclude_prefixes: &["class_feature.wizard.corpus_record."], msg: "level-21 Wizard must not gain any bounded wizard explanation" }),
    ("wizard_level14", NegControlRow { old_sub: "class:wizard:14", new_sub: "class:wizard:21", prefixes: &["class_chassis.wizard.", "class_feature.wizard."], exact_ors: &["class_chassis.spell_baseline.wizard"], excludes: &["class_feature.wizard.weapon_and_armor_proficiency"], exclude_prefixes: &["class_feature.wizard.corpus_record."], msg: "level-21 Wizard must not gain any bounded wizard explanation" }),
    ("wizard_level20", NegControlRow { old_sub: "class:wizard:20", new_sub: "class:wizard:21", prefixes: &["class_chassis.wizard.", "class_feature.wizard."], exact_ors: &["class_chassis.spell_baseline.wizard"], excludes: &["class_feature.wizard.weapon_and_armor_proficiency"], exclude_prefixes: &["class_feature.wizard.corpus_record."], msg: "level-21 Wizard must not gain any bounded wizard explanation" }),
];

pub fn find_boundary_row(module: &str) -> NegControlRow {
    BOUNDARY_NEG_ROWS
        .iter()
        .find(|(m, _)| *m == module)
        .unwrap_or_else(|| panic!("rows.rs: no BOUNDARY_NEG_ROWS entry for module '{module}'"))
        .1
}

/// Expands to one `#[test] fn $fn_name()` performing the exact
/// boundary-negative-control assert this shape always performed.
#[macro_export]
macro_rules! sd18_boundary_neg_control_test {
    ($fn_name:ident, $module:expr, $fixture:expr) => {
        #[test]
        fn $fn_name() {
            let row = $crate::rows::find_boundary_row($module);
            let modified = $fixture.replace(row.old_sub, row.new_sub);
            let computation = $crate::support::compute(&modified);
            assert!(
                !computation.explanations.iter().any(|e| {
                    let or_match = row.prefixes.iter().any(|p| e.id.starts_with(p))
                        || row.exact_ors.iter().any(|x| e.id == *x);
                    let excluded = row.excludes.iter().any(|x| e.id == *x)
                        || row.exclude_prefixes.iter().any(|p| e.id.starts_with(p));
                    or_match && !excluded
                }),
                "{}: {:?}",
                row.msg,
                computation.explanations
            );
        }
    };
}

/// Row for the `multiclass_<class>_level<N>_is_not_promoted_by_this_slice`
/// negative control: same shape as `NegControlRow` plus the second
/// claim-blocking assert every one of these tests also performs.
#[derive(Clone, Copy)]
pub struct MulticlassNegControlRow {
    pub old_sub: &'static str,
    pub new_sub: &'static str,
    pub prefixes: &'static [&'static str],
    pub exact_ors: &'static [&'static str],
    pub excludes: &'static [&'static str],
    pub exclude_prefixes: &'static [&'static str],
    pub msg: &'static str,
    pub cbmsg: &'static str,
}

pub const MULTICLASS_NEG_ROWS: &[(&str, MulticlassNegControlRow)] = &[
    ("barbarian_level12", MulticlassNegControlRow { old_sub: "class_level=class:barbarian:12", new_sub: "class_level=class:barbarian:12\nclass_level=class:fighter:1", prefixes: &["class_chassis.barbarian.", "class_feature.barbarian."], exact_ors: &[], excludes: &["class_feature.barbarian.rage_execution.not_raging"], exclude_prefixes: &[], msg: "multiclass Barbarian must not gain any bounded barbarian explanation", cbmsg: "multiclass Barbarian must stay claim-blocked in this slice" }),
    ("barbarian_level13", MulticlassNegControlRow { old_sub: "class_level=class:barbarian:13", new_sub: "class_level=class:barbarian:13\nclass_level=class:fighter:1", prefixes: &["class_chassis.barbarian.", "class_feature.barbarian."], exact_ors: &[], excludes: &["class_feature.barbarian.rage_execution.not_raging"], exclude_prefixes: &[], msg: "multiclass Barbarian must not gain any bounded barbarian explanation", cbmsg: "multiclass Barbarian must stay claim-blocked in this slice" }),
    ("barbarian_level14", MulticlassNegControlRow { old_sub: "class_level=class:barbarian:14", new_sub: "class_level=class:barbarian:14\nclass_level=class:fighter:1", prefixes: &["class_chassis.barbarian.", "class_feature.barbarian."], exact_ors: &[], excludes: &["class_feature.barbarian.rage_execution.not_raging"], exclude_prefixes: &[], msg: "multiclass Barbarian must not gain any bounded barbarian explanation", cbmsg: "multiclass Barbarian must stay claim-blocked in this slice" }),
    ("barbarian_level15", MulticlassNegControlRow { old_sub: "class_level=class:barbarian:15", new_sub: "class_level=class:barbarian:15\nclass_level=class:fighter:1", prefixes: &["class_chassis.barbarian.", "class_feature.barbarian."], exact_ors: &[], excludes: &["class_feature.barbarian.rage_execution.not_raging"], exclude_prefixes: &[], msg: "multiclass Barbarian must not gain any bounded barbarian explanation", cbmsg: "multiclass Barbarian must stay claim-blocked in this slice" }),
    ("barbarian_level16", MulticlassNegControlRow { old_sub: "class_level=class:barbarian:16", new_sub: "class_level=class:barbarian:16\nclass_level=class:fighter:1", prefixes: &["class_chassis.barbarian.", "class_feature.barbarian."], exact_ors: &[], excludes: &["class_feature.barbarian.rage_execution.not_raging"], exclude_prefixes: &[], msg: "multiclass Barbarian must not gain any bounded barbarian explanation", cbmsg: "multiclass Barbarian must stay claim-blocked in this slice" }),
    ("barbarian_level17", MulticlassNegControlRow { old_sub: "class_level=class:barbarian:17", new_sub: "class_level=class:barbarian:17\nclass_level=class:fighter:1", prefixes: &["class_chassis.barbarian.", "class_feature.barbarian."], exact_ors: &[], excludes: &["class_feature.barbarian.rage_execution.not_raging"], exclude_prefixes: &[], msg: "multiclass Barbarian must not gain any bounded barbarian explanation", cbmsg: "multiclass Barbarian must stay claim-blocked in this slice" }),
    ("barbarian_level18", MulticlassNegControlRow { old_sub: "class_level=class:barbarian:18", new_sub: "class_level=class:barbarian:18\nclass_level=class:fighter:1", prefixes: &["class_chassis.barbarian.", "class_feature.barbarian."], exact_ors: &[], excludes: &["class_feature.barbarian.rage_execution.not_raging"], exclude_prefixes: &[], msg: "multiclass Barbarian must not gain any bounded barbarian explanation", cbmsg: "multiclass Barbarian must stay claim-blocked in this slice" }),
    ("barbarian_level19", MulticlassNegControlRow { old_sub: "class_level=class:barbarian:19", new_sub: "class_level=class:barbarian:19\nclass_level=class:fighter:1", prefixes: &["class_chassis.barbarian.", "class_feature.barbarian."], exact_ors: &[], excludes: &["class_feature.barbarian.rage_execution.not_raging"], exclude_prefixes: &[], msg: "multiclass Barbarian must not gain any bounded barbarian explanation", cbmsg: "multiclass Barbarian must stay claim-blocked in this slice" }),
    ("barbarian_level20", MulticlassNegControlRow { old_sub: "class_level=class:barbarian:20", new_sub: "class_level=class:barbarian:20\nclass_level=class:fighter:1", prefixes: &["class_chassis.barbarian.", "class_feature.barbarian."], exact_ors: &[], excludes: &["class_feature.barbarian.rage_execution.not_raging"], exclude_prefixes: &[], msg: "multiclass Barbarian must not gain any bounded barbarian explanation", cbmsg: "multiclass Barbarian must stay claim-blocked in this slice" }),
    ("bard_level11_inspire", MulticlassNegControlRow { old_sub: "class_level=class:bard:11", new_sub: "class_level=class:bard:11\nclass_level=class:fighter:1", prefixes: &["class_chassis.bard.", "class_feature.bard."], exact_ors: &[], excludes: &["class_feature.bard.bardic_performance_execution.not_performing", "class_feature.bard.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Bard must not gain any bounded bard explanation", cbmsg: "multiclass Bard must stay claim-blocked in this slice" }),
    ("bard_level12", MulticlassNegControlRow { old_sub: "class_level=class:bard:12", new_sub: "class_level=class:bard:12\nclass_level=class:fighter:1", prefixes: &["class_chassis.bard.", "class_feature.bard."], exact_ors: &[], excludes: &["class_feature.bard.bardic_performance_execution.not_performing", "class_feature.bard.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Bard must not gain any bounded bard explanation", cbmsg: "multiclass Bard must stay claim-blocked in this slice" }),
    ("bard_level13", MulticlassNegControlRow { old_sub: "class_level=class:bard:13", new_sub: "class_level=class:bard:13\nclass_level=class:fighter:1", prefixes: &["class_chassis.bard.", "class_feature.bard."], exact_ors: &[], excludes: &["class_feature.bard.bardic_performance_execution.not_performing", "class_feature.bard.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Bard must not gain any bounded bard explanation", cbmsg: "multiclass Bard must stay claim-blocked in this slice" }),
    ("bard_level14", MulticlassNegControlRow { old_sub: "class_level=class:bard:14", new_sub: "class_level=class:bard:14\nclass_level=class:fighter:1", prefixes: &["class_chassis.bard.", "class_feature.bard."], exact_ors: &[], excludes: &["class_feature.bard.bardic_performance_execution.not_performing", "class_feature.bard.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Bard must not gain any bounded bard explanation", cbmsg: "multiclass Bard must stay claim-blocked in this slice" }),
    ("bard_level15", MulticlassNegControlRow { old_sub: "class_level=class:bard:15", new_sub: "class_level=class:bard:15\nclass_level=class:fighter:1", prefixes: &["class_chassis.bard.", "class_feature.bard."], exact_ors: &[], excludes: &["class_feature.bard.bardic_performance_execution.not_performing", "class_feature.bard.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Bard must not gain any bounded bard explanation", cbmsg: "multiclass Bard must stay claim-blocked in this slice" }),
    ("bard_level16", MulticlassNegControlRow { old_sub: "class_level=class:bard:16", new_sub: "class_level=class:bard:16\nclass_level=class:fighter:1", prefixes: &["class_chassis.bard.", "class_feature.bard."], exact_ors: &[], excludes: &["class_feature.bard.bardic_performance_execution.not_performing", "class_feature.bard.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Bard must not gain any bounded bard explanation", cbmsg: "multiclass Bard must stay claim-blocked in this slice" }),
    ("bard_level17", MulticlassNegControlRow { old_sub: "class_level=class:bard:17", new_sub: "class_level=class:bard:17\nclass_level=class:fighter:1", prefixes: &["class_chassis.bard.", "class_feature.bard."], exact_ors: &[], excludes: &["class_feature.bard.bardic_performance_execution.not_performing", "class_feature.bard.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Bard must not gain any bounded bard explanation", cbmsg: "multiclass Bard must stay claim-blocked in this slice" }),
    ("bard_level18", MulticlassNegControlRow { old_sub: "class_level=class:bard:18", new_sub: "class_level=class:bard:18\nclass_level=class:fighter:1", prefixes: &["class_chassis.bard.", "class_feature.bard."], exact_ors: &[], excludes: &["class_feature.bard.bardic_performance_execution.not_performing", "class_feature.bard.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Bard must not gain any bounded bard explanation", cbmsg: "multiclass Bard must stay claim-blocked in this slice" }),
    ("bard_level19", MulticlassNegControlRow { old_sub: "class_level=class:bard:19", new_sub: "class_level=class:bard:19\nclass_level=class:fighter:1", prefixes: &["class_chassis.bard.", "class_feature.bard."], exact_ors: &[], excludes: &["class_feature.bard.bardic_performance_execution.not_performing", "class_feature.bard.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Bard must not gain any bounded bard explanation", cbmsg: "multiclass Bard must stay claim-blocked in this slice" }),
    ("bard_level20", MulticlassNegControlRow { old_sub: "class_level=class:bard:20", new_sub: "class_level=class:bard:20\nclass_level=class:fighter:1", prefixes: &["class_chassis.bard.", "class_feature.bard."], exact_ors: &[], excludes: &["class_feature.bard.bardic_performance_execution.not_performing", "class_feature.bard.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Bard must not gain any bounded bard explanation", cbmsg: "multiclass Bard must stay claim-blocked in this slice" }),
    ("cleric_level11", MulticlassNegControlRow { old_sub: "class_level=class:cleric:11", new_sub: "class_level=class:cleric:11\nclass_level=class:fighter:1", prefixes: &["class_chassis.cleric.", "class_feature.cleric."], exact_ors: &[], excludes: &["class_feature.domain.good_touch_of_good_not_active", "class_feature.cleric.weapon_and_armor_proficiency", "class_feature.cleric.domain.generic.healing_domain.rebuke_death.rebukedeathtimes", "class_feature.cleric.aura.strength_level"], exclude_prefixes: &[], msg: "multiclass Cleric must not gain any bounded cleric explanation", cbmsg: "multiclass Cleric must stay claim-blocked in this slice" }),
    ("cleric_level12", MulticlassNegControlRow { old_sub: "class_level=class:cleric:12", new_sub: "class_level=class:cleric:12\nclass_level=class:fighter:1", prefixes: &["class_chassis.cleric.", "class_feature.cleric."], exact_ors: &[], excludes: &["class_feature.domain.good_touch_of_good_not_active", "class_feature.cleric.weapon_and_armor_proficiency", "class_feature.cleric.domain.generic.healing_domain.rebuke_death.rebukedeathtimes", "class_feature.cleric.aura.strength_level"], exclude_prefixes: &[], msg: "multiclass Cleric must not gain any bounded cleric explanation", cbmsg: "multiclass Cleric must stay claim-blocked in this slice" }),
    ("cleric_level13", MulticlassNegControlRow { old_sub: "class_level=class:cleric:13", new_sub: "class_level=class:cleric:13\nclass_level=class:fighter:1", prefixes: &["class_chassis.cleric.", "class_feature.cleric."], exact_ors: &[], excludes: &["class_feature.domain.good_touch_of_good_not_active", "class_feature.cleric.weapon_and_armor_proficiency", "class_feature.cleric.domain.generic.healing_domain.rebuke_death.rebukedeathtimes", "class_feature.cleric.aura.strength_level"], exclude_prefixes: &[], msg: "multiclass Cleric must not gain any bounded cleric explanation", cbmsg: "multiclass Cleric must stay claim-blocked in this slice" }),
    ("cleric_level14", MulticlassNegControlRow { old_sub: "class_level=class:cleric:14", new_sub: "class_level=class:cleric:14\nclass_level=class:fighter:1", prefixes: &["class_chassis.cleric.", "class_feature.cleric."], exact_ors: &[], excludes: &["class_feature.domain.good_touch_of_good_not_active", "class_feature.cleric.weapon_and_armor_proficiency", "class_feature.cleric.domain.generic.healing_domain.rebuke_death.rebukedeathtimes", "class_feature.cleric.aura.strength_level"], exclude_prefixes: &[], msg: "multiclass Cleric must not gain any bounded cleric explanation", cbmsg: "multiclass Cleric must stay claim-blocked in this slice" }),
    ("cleric_level15", MulticlassNegControlRow { old_sub: "class_level=class:cleric:15", new_sub: "class_level=class:cleric:15\nclass_level=class:fighter:1", prefixes: &["class_chassis.cleric.", "class_feature.cleric."], exact_ors: &[], excludes: &["class_feature.domain.good_touch_of_good_not_active", "class_feature.cleric.weapon_and_armor_proficiency", "class_feature.cleric.domain.generic.healing_domain.rebuke_death.rebukedeathtimes", "class_feature.cleric.aura.strength_level"], exclude_prefixes: &[], msg: "multiclass Cleric must not gain any bounded cleric explanation", cbmsg: "multiclass Cleric must stay claim-blocked in this slice" }),
    ("cleric_level16", MulticlassNegControlRow { old_sub: "class_level=class:cleric:16", new_sub: "class_level=class:cleric:16\nclass_level=class:fighter:1", prefixes: &["class_chassis.cleric.", "class_feature.cleric."], exact_ors: &[], excludes: &["class_feature.domain.good_touch_of_good_not_active", "class_feature.cleric.weapon_and_armor_proficiency", "class_feature.cleric.domain.generic.healing_domain.rebuke_death.rebukedeathtimes", "class_feature.cleric.aura.strength_level"], exclude_prefixes: &[], msg: "multiclass Cleric must not gain any bounded cleric explanation", cbmsg: "multiclass Cleric must stay claim-blocked in this slice" }),
    ("cleric_level17", MulticlassNegControlRow { old_sub: "class_level=class:cleric:17", new_sub: "class_level=class:cleric:17\nclass_level=class:fighter:1", prefixes: &["class_chassis.cleric.", "class_feature.cleric."], exact_ors: &[], excludes: &["class_feature.domain.good_touch_of_good_not_active", "class_feature.cleric.weapon_and_armor_proficiency", "class_feature.cleric.domain.generic.healing_domain.rebuke_death.rebukedeathtimes", "class_feature.cleric.aura.strength_level"], exclude_prefixes: &[], msg: "multiclass Cleric must not gain any bounded cleric explanation", cbmsg: "multiclass Cleric must stay claim-blocked in this slice" }),
    ("cleric_level18", MulticlassNegControlRow { old_sub: "class_level=class:cleric:18", new_sub: "class_level=class:cleric:18\nclass_level=class:fighter:1", prefixes: &["class_chassis.cleric.", "class_feature.cleric."], exact_ors: &[], excludes: &["class_feature.domain.good_touch_of_good_not_active", "class_feature.cleric.weapon_and_armor_proficiency", "class_feature.cleric.domain.generic.healing_domain.rebuke_death.rebukedeathtimes", "class_feature.cleric.aura.strength_level"], exclude_prefixes: &[], msg: "multiclass Cleric must not gain any bounded cleric explanation", cbmsg: "multiclass Cleric must stay claim-blocked in this slice" }),
    ("cleric_level19", MulticlassNegControlRow { old_sub: "class_level=class:cleric:19", new_sub: "class_level=class:cleric:19\nclass_level=class:fighter:1", prefixes: &["class_chassis.cleric.", "class_feature.cleric."], exact_ors: &[], excludes: &["class_feature.domain.good_touch_of_good_not_active", "class_feature.cleric.weapon_and_armor_proficiency", "class_feature.cleric.domain.generic.healing_domain.rebuke_death.rebukedeathtimes", "class_feature.cleric.aura.strength_level"], exclude_prefixes: &[], msg: "multiclass Cleric must not gain any bounded cleric explanation", cbmsg: "multiclass Cleric must stay claim-blocked in this slice" }),
    ("cleric_level20", MulticlassNegControlRow { old_sub: "class_level=class:cleric:20", new_sub: "class_level=class:cleric:20\nclass_level=class:fighter:1", prefixes: &["class_chassis.cleric.", "class_feature.cleric."], exact_ors: &[], excludes: &["class_feature.domain.good_touch_of_good_not_active", "class_feature.cleric.weapon_and_armor_proficiency", "class_feature.cleric.domain.generic.healing_domain.rebuke_death.rebukedeathtimes", "class_feature.cleric.aura.strength_level"], exclude_prefixes: &[], msg: "multiclass Cleric must not gain any bounded cleric explanation", cbmsg: "multiclass Cleric must stay claim-blocked in this slice" }),
    ("monk_level12", MulticlassNegControlRow { old_sub: "class_level=class:monk:12", new_sub: "class_level=class:monk:12\nclass_level=class:fighter:1", prefixes: &["class_chassis.monk.", "class_feature.monk."], exact_ors: &[], excludes: &[], exclude_prefixes: &[], msg: "multiclass Monk must not gain any bounded monk explanation", cbmsg: "multiclass Monk must stay claim-blocked in this slice" }),
    ("paladin_level12", MulticlassNegControlRow { old_sub: "class_level=class:paladin:12", new_sub: "class_level=class:paladin:12\nclass_level=class:fighter:1", prefixes: &["class_chassis.paladin."], exact_ors: &[], excludes: &[], exclude_prefixes: &[], msg: "multiclass Paladin must not gain any bounded paladin chassis explanation", cbmsg: "multiclass Paladin must stay claim-blocked in this slice" }),
    ("paladin_level13", MulticlassNegControlRow { old_sub: "class_level=class:paladin:13", new_sub: "class_level=class:paladin:13\nclass_level=class:fighter:1", prefixes: &["class_chassis.paladin."], exact_ors: &[], excludes: &[], exclude_prefixes: &[], msg: "multiclass Paladin must not gain any bounded paladin chassis explanation", cbmsg: "multiclass Paladin must stay claim-blocked in this slice" }),
    ("paladin_level14", MulticlassNegControlRow { old_sub: "class_level=class:paladin:14", new_sub: "class_level=class:paladin:14\nclass_level=class:fighter:1", prefixes: &["class_chassis.paladin."], exact_ors: &[], excludes: &[], exclude_prefixes: &[], msg: "multiclass Paladin must not gain any bounded paladin chassis explanation", cbmsg: "multiclass Paladin must stay claim-blocked in this slice" }),
    ("paladin_level15", MulticlassNegControlRow { old_sub: "class_level=class:paladin:15", new_sub: "class_level=class:paladin:15\nclass_level=class:fighter:1", prefixes: &["class_chassis.paladin."], exact_ors: &[], excludes: &[], exclude_prefixes: &[], msg: "multiclass Paladin must not gain any bounded paladin chassis explanation", cbmsg: "multiclass Paladin must stay claim-blocked in this slice" }),
    ("paladin_level16", MulticlassNegControlRow { old_sub: "class_level=class:paladin:16", new_sub: "class_level=class:paladin:16\nclass_level=class:fighter:1", prefixes: &["class_chassis.paladin."], exact_ors: &[], excludes: &[], exclude_prefixes: &[], msg: "multiclass Paladin must not gain any bounded paladin chassis explanation", cbmsg: "multiclass Paladin must stay claim-blocked in this slice" }),
    ("paladin_level17", MulticlassNegControlRow { old_sub: "class_level=class:paladin:17", new_sub: "class_level=class:paladin:17\nclass_level=class:fighter:1", prefixes: &["class_chassis.paladin."], exact_ors: &[], excludes: &[], exclude_prefixes: &[], msg: "multiclass Paladin must not gain any bounded paladin chassis explanation", cbmsg: "multiclass Paladin must stay claim-blocked in this slice" }),
    ("paladin_level18", MulticlassNegControlRow { old_sub: "class_level=class:paladin:18", new_sub: "class_level=class:paladin:18\nclass_level=class:fighter:1", prefixes: &["class_chassis.paladin."], exact_ors: &[], excludes: &[], exclude_prefixes: &[], msg: "multiclass Paladin must not gain any bounded paladin chassis explanation", cbmsg: "multiclass Paladin must stay claim-blocked in this slice" }),
    ("paladin_level19", MulticlassNegControlRow { old_sub: "class_level=class:paladin:19", new_sub: "class_level=class:paladin:19\nclass_level=class:fighter:1", prefixes: &["class_chassis.paladin."], exact_ors: &[], excludes: &[], exclude_prefixes: &[], msg: "multiclass Paladin must not gain any bounded paladin chassis explanation", cbmsg: "multiclass Paladin must stay claim-blocked in this slice" }),
    ("paladin_level20", MulticlassNegControlRow { old_sub: "class_level=class:paladin:20", new_sub: "class_level=class:paladin:20\nclass_level=class:fighter:1", prefixes: &["class_chassis.paladin."], exact_ors: &[], excludes: &[], exclude_prefixes: &[], msg: "multiclass Paladin must not gain any bounded paladin chassis explanation", cbmsg: "multiclass Paladin must stay claim-blocked in this slice" }),
    ("ranger_level12", MulticlassNegControlRow { old_sub: "class_level=class:ranger:12", new_sub: "class_level=class:ranger:12\nclass_level=class:fighter:1", prefixes: &["class_chassis.ranger.", "class_feature.ranger."], exact_ors: &[], excludes: &["class_feature.ranger.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Ranger must not gain any bounded ranger chassis explanation", cbmsg: "multiclass Ranger must stay claim-blocked in this slice" }),
    ("ranger_level13", MulticlassNegControlRow { old_sub: "class_level=class:ranger:13", new_sub: "class_level=class:ranger:13\nclass_level=class:fighter:1", prefixes: &["class_chassis.ranger.", "class_feature.ranger."], exact_ors: &[], excludes: &["class_feature.ranger.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Ranger must not gain any bounded ranger chassis explanation", cbmsg: "multiclass Ranger must stay claim-blocked in this slice" }),
    ("ranger_level14", MulticlassNegControlRow { old_sub: "class_level=class:ranger:14", new_sub: "class_level=class:ranger:14\nclass_level=class:fighter:1", prefixes: &["class_chassis.ranger.", "class_feature.ranger."], exact_ors: &[], excludes: &["class_feature.ranger.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Ranger must not gain any bounded ranger chassis explanation", cbmsg: "multiclass Ranger must stay claim-blocked in this slice" }),
    ("ranger_level15", MulticlassNegControlRow { old_sub: "class_level=class:ranger:15", new_sub: "class_level=class:ranger:15\nclass_level=class:fighter:1", prefixes: &["class_chassis.ranger.", "class_feature.ranger."], exact_ors: &[], excludes: &["class_feature.ranger.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Ranger must not gain any bounded ranger chassis explanation", cbmsg: "multiclass Ranger must stay claim-blocked in this slice" }),
    ("ranger_level18", MulticlassNegControlRow { old_sub: "class_level=class:ranger:18", new_sub: "class_level=class:ranger:18\nclass_level=class:fighter:1", prefixes: &["class_chassis.ranger.", "class_feature.ranger."], exact_ors: &[], excludes: &["class_feature.ranger.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Ranger must not gain any bounded ranger chassis explanation", cbmsg: "multiclass Ranger must stay claim-blocked in this slice" }),
    ("ranger_level19", MulticlassNegControlRow { old_sub: "class_level=class:ranger:19", new_sub: "class_level=class:ranger:19\nclass_level=class:fighter:1", prefixes: &["class_chassis.ranger.", "class_feature.ranger."], exact_ors: &[], excludes: &["class_feature.ranger.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Ranger must not gain any bounded ranger chassis explanation", cbmsg: "multiclass Ranger must stay claim-blocked in this slice" }),
    ("rogue_level12", MulticlassNegControlRow { old_sub: "class_level=class:rogue:12", new_sub: "class_level=class:rogue:12\nclass_level=class:fighter:1", prefixes: &["class_chassis.rogue.", "class_feature.rogue."], exact_ors: &[], excludes: &["class_feature.rogue.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Rogue must not gain any bounded rogue explanation", cbmsg: "multiclass Rogue must stay claim-blocked in this slice" }),
    ("rogue_level13", MulticlassNegControlRow { old_sub: "class_level=class:rogue:13", new_sub: "class_level=class:rogue:13\nclass_level=class:fighter:1", prefixes: &["class_chassis.rogue.", "class_feature.rogue."], exact_ors: &[], excludes: &["class_feature.rogue.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Rogue must not gain any bounded rogue explanation", cbmsg: "multiclass Rogue must stay claim-blocked in this slice" }),
    ("rogue_level14", MulticlassNegControlRow { old_sub: "class_level=class:rogue:14", new_sub: "class_level=class:rogue:14\nclass_level=class:fighter:1", prefixes: &["class_chassis.rogue.", "class_feature.rogue."], exact_ors: &[], excludes: &["class_feature.rogue.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Rogue must not gain any bounded rogue explanation", cbmsg: "multiclass Rogue must stay claim-blocked in this slice" }),
    ("rogue_level15", MulticlassNegControlRow { old_sub: "class_level=class:rogue:15", new_sub: "class_level=class:rogue:15\nclass_level=class:fighter:1", prefixes: &["class_chassis.rogue.", "class_feature.rogue."], exact_ors: &[], excludes: &["class_feature.rogue.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Rogue must not gain any bounded rogue explanation", cbmsg: "multiclass Rogue must stay claim-blocked in this slice" }),
    ("rogue_level16", MulticlassNegControlRow { old_sub: "class_level=class:rogue:16", new_sub: "class_level=class:rogue:16\nclass_level=class:fighter:1", prefixes: &["class_chassis.rogue.", "class_feature.rogue."], exact_ors: &[], excludes: &["class_feature.rogue.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Rogue must not gain any bounded rogue explanation", cbmsg: "multiclass Rogue must stay claim-blocked in this slice" }),
    ("rogue_level17", MulticlassNegControlRow { old_sub: "class_level=class:rogue:17", new_sub: "class_level=class:rogue:17\nclass_level=class:fighter:1", prefixes: &["class_chassis.rogue.", "class_feature.rogue."], exact_ors: &[], excludes: &["class_feature.rogue.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Rogue must not gain any bounded rogue explanation", cbmsg: "multiclass Rogue must stay claim-blocked in this slice" }),
    ("rogue_level18", MulticlassNegControlRow { old_sub: "class_level=class:rogue:18", new_sub: "class_level=class:rogue:18\nclass_level=class:fighter:1", prefixes: &["class_chassis.rogue.", "class_feature.rogue."], exact_ors: &[], excludes: &["class_feature.rogue.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Rogue must not gain any bounded rogue explanation", cbmsg: "multiclass Rogue must stay claim-blocked in this slice" }),
    ("rogue_level19", MulticlassNegControlRow { old_sub: "class_level=class:rogue:19", new_sub: "class_level=class:rogue:19\nclass_level=class:fighter:1", prefixes: &["class_chassis.rogue.", "class_feature.rogue."], exact_ors: &[], excludes: &["class_feature.rogue.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Rogue must not gain any bounded rogue explanation", cbmsg: "multiclass Rogue must stay claim-blocked in this slice" }),
    ("rogue_level20", MulticlassNegControlRow { old_sub: "class_level=class:rogue:20", new_sub: "class_level=class:rogue:20\nclass_level=class:fighter:1", prefixes: &["class_chassis.rogue.", "class_feature.rogue."], exact_ors: &[], excludes: &["class_feature.rogue.weapon_and_armor_proficiency"], exclude_prefixes: &[], msg: "multiclass Rogue must not gain any bounded rogue explanation", cbmsg: "multiclass Rogue must stay claim-blocked in this slice" }),
    ("sorcerer_level11", MulticlassNegControlRow { old_sub: "class_level=class:sorcerer:11", new_sub: "class_level=class:sorcerer:11\nclass_level=class:fighter:1", prefixes: &["class_chassis.sorcerer."], exact_ors: &["class_chassis.spell_baseline.sorcerer"], excludes: &[], exclude_prefixes: &[], msg: "multiclass Sorcerer must not gain any bounded sorcerer chassis explanation", cbmsg: "multiclass Sorcerer must stay claim-blocked in this slice" }),
    ("sorcerer_level12", MulticlassNegControlRow { old_sub: "class_level=class:sorcerer:12", new_sub: "class_level=class:sorcerer:12\nclass_level=class:fighter:1", prefixes: &["class_chassis.sorcerer."], exact_ors: &["class_chassis.spell_baseline.sorcerer"], excludes: &[], exclude_prefixes: &[], msg: "multiclass Sorcerer must not gain any bounded sorcerer chassis explanation", cbmsg: "multiclass Sorcerer must stay claim-blocked in this slice" }),
    ("sorcerer_level13", MulticlassNegControlRow { old_sub: "class_level=class:sorcerer:13", new_sub: "class_level=class:sorcerer:13\nclass_level=class:fighter:1", prefixes: &["class_chassis.sorcerer."], exact_ors: &["class_chassis.spell_baseline.sorcerer"], excludes: &[], exclude_prefixes: &[], msg: "multiclass Sorcerer must not gain any bounded sorcerer chassis explanation", cbmsg: "multiclass Sorcerer must stay claim-blocked in this slice" }),
    ("sorcerer_level14", MulticlassNegControlRow { old_sub: "class_level=class:sorcerer:14", new_sub: "class_level=class:sorcerer:14\nclass_level=class:fighter:1", prefixes: &["class_chassis.sorcerer."], exact_ors: &["class_chassis.spell_baseline.sorcerer"], excludes: &[], exclude_prefixes: &[], msg: "multiclass Sorcerer must not gain any bounded sorcerer chassis explanation", cbmsg: "multiclass Sorcerer must stay claim-blocked in this slice" }),
    ("sorcerer_level15", MulticlassNegControlRow { old_sub: "class_level=class:sorcerer:15", new_sub: "class_level=class:sorcerer:15\nclass_level=class:fighter:1", prefixes: &["class_chassis.sorcerer."], exact_ors: &["class_chassis.spell_baseline.sorcerer"], excludes: &[], exclude_prefixes: &[], msg: "multiclass Sorcerer must not gain any bounded sorcerer chassis explanation", cbmsg: "multiclass Sorcerer must stay claim-blocked in this slice" }),
    ("sorcerer_level16", MulticlassNegControlRow { old_sub: "class_level=class:sorcerer:16", new_sub: "class_level=class:sorcerer:16\nclass_level=class:fighter:1", prefixes: &["class_chassis.sorcerer."], exact_ors: &["class_chassis.spell_baseline.sorcerer"], excludes: &[], exclude_prefixes: &[], msg: "multiclass Sorcerer must not gain any bounded sorcerer chassis explanation", cbmsg: "multiclass Sorcerer must stay claim-blocked in this slice" }),
    ("sorcerer_level17", MulticlassNegControlRow { old_sub: "class_level=class:sorcerer:17", new_sub: "class_level=class:sorcerer:17\nclass_level=class:fighter:1", prefixes: &["class_chassis.sorcerer."], exact_ors: &["class_chassis.spell_baseline.sorcerer"], excludes: &[], exclude_prefixes: &[], msg: "multiclass Sorcerer must not gain any bounded sorcerer chassis explanation", cbmsg: "multiclass Sorcerer must stay claim-blocked in this slice" }),
    ("sorcerer_level18", MulticlassNegControlRow { old_sub: "class_level=class:sorcerer:18", new_sub: "class_level=class:sorcerer:18\nclass_level=class:fighter:1", prefixes: &["class_chassis.sorcerer."], exact_ors: &["class_chassis.spell_baseline.sorcerer"], excludes: &[], exclude_prefixes: &[], msg: "multiclass Sorcerer must not gain any bounded sorcerer chassis explanation", cbmsg: "multiclass Sorcerer must stay claim-blocked in this slice" }),
    ("sorcerer_level19", MulticlassNegControlRow { old_sub: "class_level=class:sorcerer:19", new_sub: "class_level=class:sorcerer:19\nclass_level=class:fighter:1", prefixes: &["class_chassis.sorcerer."], exact_ors: &["class_chassis.spell_baseline.sorcerer"], excludes: &[], exclude_prefixes: &[], msg: "multiclass Sorcerer must not gain any bounded sorcerer chassis explanation", cbmsg: "multiclass Sorcerer must stay claim-blocked in this slice" }),
    ("sorcerer_level20", MulticlassNegControlRow { old_sub: "class_level=class:sorcerer:20", new_sub: "class_level=class:sorcerer:20\nclass_level=class:fighter:1", prefixes: &["class_chassis.sorcerer."], exact_ors: &["class_chassis.spell_baseline.sorcerer"], excludes: &[], exclude_prefixes: &[], msg: "multiclass Sorcerer must not gain any bounded sorcerer chassis explanation", cbmsg: "multiclass Sorcerer must stay claim-blocked in this slice" }),
];

pub fn find_multiclass_row(module: &str) -> MulticlassNegControlRow {
    MULTICLASS_NEG_ROWS
        .iter()
        .find(|(m, _)| *m == module)
        .unwrap_or_else(|| panic!("rows.rs: no MULTICLASS_NEG_ROWS entry for module '{module}'"))
        .1
}

#[macro_export]
macro_rules! sd18_multiclass_neg_control_test {
    ($fn_name:ident, $module:expr, $fixture:expr) => {
        #[test]
        fn $fn_name() {
            let row = $crate::rows::find_multiclass_row($module);
            let modified = $fixture.replace(row.old_sub, row.new_sub);
            let computation = $crate::support::compute(&modified);
            assert!(
                !computation.explanations.iter().any(|e| {
                    let or_match = row.prefixes.iter().any(|p| e.id.starts_with(p))
                        || row.exact_ors.iter().any(|x| e.id == *x);
                    let excluded = row.excludes.iter().any(|x| e.id == *x)
                        || row.exclude_prefixes.iter().any(|p| e.id.starts_with(p));
                    or_match && !excluded
                }),
                "{}: {:?}",
                row.msg,
                computation.explanations
            );
            assert!(
                computation.diagnostics.iter().any(|d| d.claim_blocking),
                "{}",
                row.cbmsg
            );
        }
    };
}
