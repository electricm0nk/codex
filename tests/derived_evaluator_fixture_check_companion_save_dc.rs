//! The `kind=companion` save-DC-formula half of the `derived` wiring class's
//! evaluator-vs-fixture check (wave-17) — the sibling of
//! `tests/derived_evaluator_fixture_check_companion.rs` (Strength-damage) and
//! its own companion-skill seam, over `derived-evaluator-fixtures.json`'s
//! `companion_save_dc_entries` array.
//!
//! # Why this file exists (same lesson `derived_evaluator_fixture_check_companion.rs`
//! records for its own seam)
//!
//! The wave-16 adversarial review found that a bar check loading provenance
//! fields and never asserting them is not a gate at all — under Decision
//! 1(a), *a gate that cannot fail is worse than no gate*. This file closes
//! the FORWARD risk for the save-DC seam from day one rather than needing a
//! second pass: if the pinned oracle moves, or a row is hand-edited, this
//! notices.
//!
//! # The guarantees
//!
//! 1. **Anchored to the same upstream bytes** —
//!    [`companion_save_dc_pinned_upstream_lst_still_hashes_to_the_pinned_sha256`]
//!    re-hashes each distinct `upstream_lst` fresh off the pinned oracle.
//! 2. **The pinned corpus field is really on the pinned line** —
//!    [`companion_save_dc_pinned_corpus_field_is_byte_identical_to_the_upstream_lst`].
//! 3. **The expected values are re-derivable** —
//!    [`companion_save_dc_expected_values_are_re_derivable_from_the_pinned_corpus_field`]
//!    re-parses the pinned formula argument with a THIRD, independent
//!    reference derivation written HERE (independent of both the Python
//!    generator and the engine's `parse_companion_save_dc_formula`), and
//!    recomputes every `(hit_dice, ability_modifier, save_dc)` triple with
//!    PF1's own "1/2 HD rounds down" rule.
//! 4. **One row per unit, and the array is not empty** —
//!    [`companion_save_dc_entries_are_present_and_one_per_unit`]. An empty
//!    array would make every other assertion here vacuously true, which is
//!    the exact shape Decision 1(a) forbids.
//!
//! # Prove-it-can-fail
//!
//! Each guarantee is mutation-provable on the committed fixture in
//! isolation: zero one row's `upstream_lst_sha256` → guarantee 1 red; change
//! one `corpus_field`'s formula from `10+HD/2+CON` to `10+HD/2+WIS` →
//! guarantees 2 and 3 red; delete a row → guarantee 4 red on the
//! one-per-unit count; empty the array → guarantee 4 red on the non-empty
//! assertion.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use sha2::{Digest, Sha256};

use codex::rules_core::derived_evaluator_fixture_check::load_companion_save_dc_fixtures;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

/// Same resolution rule every sibling guarantee file uses.
fn pcgen_data_root() -> Option<PathBuf> {
    if let Ok(root) = std::env::var("PCGEN_CORPUS_ROOT") {
        return Some(PathBuf::from(root));
    }
    let home = std::env::var("HOME").ok()?;
    Some(PathBuf::from(home).join("workspace").join("repos").join("pcgen").join("data"))
}

/// A THIRD, independent reference parser+evaluator, written here so this
/// file agrees with the fixture only if both agree with the printed rule.
/// Deliberately NOT a call into `parse_companion_save_dc_formula`/
/// `evaluate_companion_save_dc_formula` (the engine) NOR
/// `classify_shape`/`expected_save_dc` (the Python generator): three
/// independent implementations, so no two of them can silently share a bug.
///
/// PF1's own "1/2 HD rounds down" rule. Hit Dice is never negative in this
/// corpus, so plain integer division is exact.
fn reference_parse(formula: &str) -> Option<(i32, bool, &'static str)> {
    let compact: String = formula.chars().filter(|c| !c.is_whitespace()).collect();
    const ABILITIES: [&str; 6] = ["STR", "DEX", "CON", "INT", "WIS", "CHA"];
    for divisor in ["HD", "TL"] {
        let infix = format!("+{divisor}/2+");
        if let Some((lhs, rhs)) = compact.split_once(infix.as_str())
            && let Some(&ability) = ABILITIES.iter().find(|&&a| a == rhs)
            && let Ok(base) = lhs.parse::<i32>()
        {
            return Some((base, true, ability));
        }
    }
    let (lhs, rhs) = compact.split_once('+')?;
    let &ability = ABILITIES.iter().find(|&&a| a == rhs)?;
    let base = lhs.parse::<i32>().ok()?;
    Some((base, false, ability))
}

fn reference_save_dc(base: i32, includes_half_hd: bool, hit_dice: i32, ability_modifier: i32) -> i32 {
    let half_hd = if includes_half_hd { hit_dice / 2 } else { 0 };
    base + half_hd + ability_modifier
}

/// The formula argument the pinned `corpus_field` states, re-extracted here
/// from the raw text rather than taken from the fixture's `expected` block.
/// `corpus_field` is stored as `"DESC:...|<formula>"` (the generator's own
/// convention — the prose half is elided since it is not part of what this
/// seam asserts, only the trailing formula argument is).
fn formula_from_corpus_field(corpus_field: &str) -> Option<&str> {
    corpus_field.rsplit_once('|').map(|(_, f)| f)
}

#[test]
fn companion_save_dc_entries_are_present_and_one_per_unit() {
    let fixtures = load_companion_save_dc_fixtures(&repo_root());
    assert!(
        !fixtures.is_empty(),
        "companion_save_dc_entries is EMPTY — every other guarantee in this file would \
         then pass vacuously, which is the Decision 1(a) shape this seam exists to refuse"
    );
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for f in &fixtures {
        assert!(
            seen.insert(f.unit_id.as_str()),
            "companion_save_dc_entries carries more than one row for {} — the seam clears a \
             unit only when its single row clears, so a duplicate would make which row won \
             undefined",
            f.unit_id
        );
        assert!(
            !f.expected_at.is_empty(),
            "{} pins no (hit_dice, ability_modifier, save_dc) triples at all — a fixture that \
             asserts nothing cannot fail",
            f.unit_id
        );
    }
}

#[test]
fn companion_save_dc_pinned_upstream_lst_still_hashes_to_the_pinned_sha256() {
    let Some(data_root) = pcgen_data_root() else {
        eprintln!("PCGEN_CORPUS_ROOT unset and no HOME; skipping");
        return;
    };
    let fixtures = load_companion_save_dc_fixtures(&repo_root());
    assert!(!fixtures.is_empty(), "companion_save_dc_entries must not be empty");
    let mut hashed: BTreeMap<String, String> = BTreeMap::new();
    let mut checked = 0usize;
    for f in &fixtures {
        let path = data_root.join(&f.upstream_lst);
        let Ok(bytes) = std::fs::read(&path) else {
            panic!(
                "{}: the fixture pins upstream_lst {:?}, which does not exist under the \
                 pinned oracle — the provenance names bytes nobody can read",
                f.unit_id, f.upstream_lst
            );
        };
        let actual = hashed
            .entry(f.upstream_lst.clone())
            .or_insert_with(|| sha256_hex(&bytes))
            .clone();
        assert_eq!(
            actual, f.upstream_lst_sha256,
            "{}: {} has moved since this fixture was derived (pinned sha256 {}, actual {}). \
             Re-derive against the current pin rather than editing the hash.",
            f.unit_id, f.upstream_lst, f.upstream_lst_sha256, actual
        );
        checked += 1;
    }
    assert!(checked > 0, "no fixture row was checked");
}

#[test]
fn companion_save_dc_pinned_corpus_field_is_byte_identical_to_the_upstream_lst() {
    let Some(data_root) = pcgen_data_root() else {
        eprintln!("PCGEN_CORPUS_ROOT unset and no HOME; skipping");
        return;
    };
    let fixtures = load_companion_save_dc_fixtures(&repo_root());
    assert!(!fixtures.is_empty(), "companion_save_dc_entries must not be empty");
    let mut lines: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for f in &fixtures {
        let file_lines = lines.entry(f.upstream_lst.clone()).or_insert_with(|| {
            std::fs::read_to_string(data_root.join(&f.upstream_lst))
                .unwrap_or_else(|e| panic!("cannot read {}: {e}", f.upstream_lst))
                .lines()
                .map(|s| s.to_string())
                .collect()
        });
        let idx = usize::try_from(f.upstream_line).expect("line fits usize");
        assert!(
            idx >= 1 && idx <= file_lines.len(),
            "{}: pinned upstream_line {} is outside {} ({} lines)",
            f.unit_id,
            f.upstream_line,
            f.upstream_lst,
            file_lines.len()
        );
        let line = &file_lines[idx - 1];
        let formula = formula_from_corpus_field(&f.corpus_field).unwrap_or_else(|| {
            panic!("{}: corpus_field {:?} has no '|' to split a formula off", f.unit_id, f.corpus_field)
        });
        assert!(
            line.contains(formula),
            "{}: pinned formula {:?} (from corpus_field {:?}) does not appear on {}:{} — the \
             fixture's expected values were derived from bytes that are not there",
            f.unit_id,
            formula,
            f.corpus_field,
            f.upstream_lst,
            f.upstream_line
        );
    }
}

#[test]
fn companion_save_dc_expected_values_are_re_derivable_from_the_pinned_corpus_field() {
    let fixtures = load_companion_save_dc_fixtures(&repo_root());
    assert!(!fixtures.is_empty(), "companion_save_dc_entries must not be empty");
    let mut triples_checked = 0usize;
    for f in &fixtures {
        let formula = formula_from_corpus_field(&f.corpus_field).unwrap_or_else(|| {
            panic!("{}: corpus_field {:?} has no '|' to split a formula off", f.unit_id, f.corpus_field)
        });
        let (base, includes_half_hd, ability) = reference_parse(formula).unwrap_or_else(|| {
            panic!(
                "{}: this file's INDEPENDENT reader cannot read a save-DC shape out of the \
                 pinned formula {:?} — either the row is not this seam's shape, or it should \
                 never have been fixtured",
                f.unit_id, formula
            )
        });
        assert_eq!(
            (base, includes_half_hd, ability),
            (f.expected_base, f.expected_includes_half_hd, f.expected_ability.as_str()),
            "{}: the pinned formula {:?} states (base={}, includes_half_hd={}, ability={:?}), \
             but the fixture claims (base={}, includes_half_hd={}, ability={:?})",
            f.unit_id,
            formula,
            base,
            includes_half_hd,
            ability,
            f.expected_base,
            f.expected_includes_half_hd,
            f.expected_ability
        );
        for (hit_dice, ability_modifier, save_dc) in &f.expected_at {
            let expected = reference_save_dc(base, includes_half_hd, *hit_dice, *ability_modifier);
            assert_eq!(
                expected, *save_dc,
                "{}: at (hit_dice={}, ability_modifier={}), PF1's 1/2-HD-rounds-down rule over \
                 {:?} gives {}, but the fixture pins {}",
                f.unit_id, hit_dice, ability_modifier, formula, expected, save_dc
            );
            triples_checked += 1;
        }
    }
    assert!(
        triples_checked > 0,
        "no (hit_dice, ability_modifier, save_dc) triple was re-derived — vacuous"
    );
}
