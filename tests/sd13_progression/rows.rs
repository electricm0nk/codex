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
//!      chassis explanation and stay claim-blocked" (the
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
/// predicate shape as `recognition_negative_controls!`) AND the computation
/// must stay claim-blocked.
macro_rules! multiclass_negative_controls {
    ( $(
        $name:ident($fixture:ident, $from:literal => $to:literal) {
            prefixes: [ $($p:literal),* $(,)? ],
            exact: [ $($x:expr),* $(,)? ],
            message: $msg:literal,
            blocked_message: $msg2:literal $(,)?
        }
    ),* $(,)? ) => {
        $(
            #[test]
            fn $name() {
                let multiclass = $fixture.replace($from, $to);
                let input = load(&multiclass);
                let computation = compute_pilot_base_chassis(&input);
                assert!(
                    !computation
                        .explanations
                        .iter()
                        .any(|e| { false $(|| e.id.starts_with($p))* $(|| e.id == $x)* }),
                    $msg,
                    computation.explanations
                );
                assert!(
                    computation.diagnostics.iter().any(|d| d.claim_blocking),
                    $msg2
                );
            }
        )*
    };
}

pub(crate) use multiclass_negative_controls;
pub(crate) use recognition_negative_controls;
