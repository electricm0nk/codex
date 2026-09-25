//! SD13-E5 progression family — table-driven rows for the two near-universal
//! negative-control shapes found by the Epic C2.1/C2.2 survey
//! (docs/release/SD-36-consolidation/epic-breakdown.md,
//! `/tmp/.../scratchpad/sd36/c2/c2sd13_survey*-out.txt`):
//!
//!   1. `recognition_negative_controls!` — "fighter (and, for Paladin,
//!      Fighter+Ranger) must not gain any of this class's namespaced
//!      explanations" (the `fighter_does_not_gain_<class>_level<N>_recognition`
//!      family).
//!   2. `multiclass_negative_controls!` — "this class's own fixture, widened
//!      to a Class+Fighter multiclass mix, must still withhold the bounded
//!      chassis explanation, and (since SD-36 Epic F3d) get the same receipt
//!      status and claim-blocking set as the class alone" (the
//!      `multiclass_<class>_level<N>_is_not_promoted_by_this_slice` family).
//!
//! Each macro expands ONE `#[test]` fn per row it is fed, with the row's
//! exact original fn name, inside the exact original per-(class,level)
//! module file (the macro is invoked once per file, from within that
//! file's own `mod`), so `cargo test -- --list` stays byte-identical to the
//! pre-rewrite roster. Every field a row carries (the explanation-id
//! prefixes/exact-ids checked, the `.replace()` from/to strings, and the
//! assertion messages) is the exact literal/ident token the corresponding
//! hand-written test body used — extracted mechanically
//! (`c2sd13_extract.py`), never re-derived or retyped by hand, and verified
//! byte-for-byte against the pre-rewrite source
//! (`c2sd13_extract.py` + the self-audit pass) before this file was written.
//!
//! Design note on "`const ROWS: &[Row]`": a `macro_rules!` cannot generate a
//! set of top-level `#[test] fn` items by iterating a *runtime* `const`
//! array — Rust macros are compile-time/syntactic. The "rows table" the
//! design doc (`docs/release/SD-36-consolidation/technical-design.md` §
//! "Pass C2") describes is expressed instead as the macro's own invocation
//! syntax at each per-file call site: one row per test, in the same
//! struct-literal shape a `const ROWS: &[Row]` would hold, immediately
//! followed by the macro invocation that turns each row into its `#[test]`.
//! That invocation block *is* the row data — see e.g.
//! `tests/sd13_progression/barbarian_level2.rs`'s trailing
//! `recognition_negative_controls! { ... }` / `multiclass_negative_controls!
//! { ... }` blocks.

/// Shape: `fighter_does_not_gain_<class>_level<N>_recognition`. A single
/// bounded other-class fixture (`FIGHTER_FIXTURE` for every row this macro
/// covers in this family) must never surface an explanation namespaced to
/// the class under test. `prefixes` are `starts_with` checks; `exact` are
/// `==` checks (either a string literal or a `const &str` identifier
/// already in scope in the invoking file, e.g. `MONK_EVASION_ID`) — exactly
/// the OR-clauses the original hand-written predicate closure used.
macro_rules! recognition_negative_controls {
    ( $(
        $name:ident($fixture:ident) {
            prefixes: [ $($p:literal),* $(,)? ],
            exact: [ $($x:expr),* $(,)? ],
            message: $msg:literal $(,)?
        }
    ),* $(,)? ) => {
        $(
            #[test]
            fn $name() {
                let checker_input = load($fixture);
                let checker_computation = compute_pilot_base_chassis(&checker_input);
                assert!(
                    !checker_computation
                        .explanations
                        .iter()
                        .any(|e| { false $(|| e.id.starts_with($p))* $(|| e.id == $x)* }),
                    $msg,
                    checker_computation.explanations
                );
            }
        )*
    };
}

/// Shape: `multiclass_<class>_level<N>_is_not_promoted_by_this_slice`. The
/// class's own single-class fixture, widened to a Class+Fighter multiclass
/// mix via a literal `.replace(from, to)` (the exact strings the original
/// body used), must still withhold the bounded chassis explanation (same
/// predicate shape as `recognition_negative_controls!`) AND -- since SD-36
/// Epic F3d, a change of meaning from the former "must stay claim-blocked" --
/// STATUS PARITY: the mix's headless receipt status and claim-blocking set
/// (the `multiclass.<class>.` re-scope stripped) equal the class alone's.
///
/// SD-36 Epic C2 vacuity-guard addendum (operator ruling 2026-09-20): the
/// sibling `sd18_widening` family's own multiclass macro was found with 64
/// rows whose substitution silently no-op'd (a literal backslash-n instead
/// of a real newline collapsed both `class_level=` lines into one garbage
/// line), so the negative control passed for the wrong reason. This shape
/// is the same pattern, so it gets the same guards (added, none of the
/// pre-existing asserts below changed): the substitution must actually
/// fire, the mutated fixture must have exactly two `class_level=` lines,
/// and the LOADED character must carry exactly two class entries matching
/// what `$to` says.
macro_rules! multiclass_negative_controls {
    ( $(
        $name:ident($fixture:ident, $from:literal => $to:literal) {
            prefixes: [ $($p:literal),* $(,)? ],
            exact: [ $($x:expr),* $(,)? ],
            message: $msg:literal $(,)?
        }
    ),* $(,)? ) => {
        $(
            #[test]
            fn $name() {
                assert_eq!(
                    $fixture.matches($from).count(),
                    1,
                    "{}: fixture must contain '{}' exactly once before substitution",
                    stringify!($name), $from
                );
                let multiclass = $fixture.replace($from, $to);
                assert_ne!(
                    multiclass, $fixture,
                    "{}: substituting '{}' -> '{}' must change the fixture",
                    stringify!($name), $from, $to
                );
                let class_level_lines = multiclass
                    .lines()
                    .filter(|l| l.starts_with("class_level="))
                    .count();
                assert_eq!(
                    class_level_lines, 2,
                    "{}: multiclass fixture must have exactly two class_level lines after \
                     substitution, got {} in:\n{}",
                    stringify!($name), class_level_lines, multiclass
                );
                let input = load(&multiclass);
                let computation = compute_pilot_base_chassis(&input);
                assert_eq!(
                    input.chosen.class_levels.len(),
                    2,
                    "{}: loaded character must have exactly two class entries after multiclass \
                     substitution, got {:?}",
                    stringify!($name), input.chosen.class_levels
                );
                for (expect_class_id, expect_level) in crate::rows::parse_class_level_lines($to) {
                    assert!(
                        input
                            .chosen
                            .class_levels
                            .iter()
                            .any(|cl| cl.class_id == expect_class_id && cl.level == expect_level),
                        "{}: loaded character must have {} at level {} after multiclass \
                         substitution, got {:?}",
                        stringify!($name), expect_class_id, expect_level, input.chosen.class_levels
                    );
                }
                assert!(
                    !computation
                        .explanations
                        .iter()
                        .any(|e| { false $(|| e.id.starts_with($p))* $(|| e.id == $x)* }),
                    $msg,
                    computation.explanations
                );
                // SD-36 Epic F3d (decisions.md §14): assertion (b) is STATUS PARITY
                // with the class alone (was: "must stay claim-blocked in this slice"):
                // same receipt status as `$fixture` unmodified, same claim-blocking set
                // once the fold's `multiclass.<class>.` re-scope is stripped. The
                // vacuity guards above (two class_level lines, two loaded class entries
                // matching `$to`) keep this from passing on a single-class input.
                crate::common::assert_multiclass_status_parity(stringify!($name), $fixture, &multiclass);
            }
        )*
    };
}

/// Parses a `class:<class_id>:<level>` token into (class_id, level),
/// mirroring `apply_class_level`'s own trailing-colon split
/// (`src/rules_core/character_input.rs`) so the vacuity guard above checks
/// the LOADED character against the same rule the production parser uses.
fn parse_class_colon_level(token: &str) -> (&str, u8) {
    let (class_id, level_text) = token
        .rsplit_once(':')
        .unwrap_or_else(|| panic!("rows.rs: '{token}' is not a class:id:level token"));
    let level: u8 = level_text
        .parse()
        .unwrap_or_else(|_| panic!("rows.rs: '{token}' has a non-numeric level"));
    (class_id, level)
}

/// Parses every `class_level=class:<id>:<level>` line in a (possibly
/// multi-line) substitution string, in order.
pub(crate) fn parse_class_level_lines(text: &str) -> Vec<(String, u8)> {
    text.lines()
        .filter_map(|line| line.strip_prefix("class_level="))
        .map(|token| {
            let (class_id, level) = parse_class_colon_level(token);
            (class_id.to_owned(), level)
        })
        .collect()
}

pub(crate) use multiclass_negative_controls;
pub(crate) use recognition_negative_controls;
