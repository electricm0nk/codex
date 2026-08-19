//! The `kind=companion` half of the `derived` wiring class's
//! evaluator-vs-fixture check (`SD31-W15-COMPANION-001`) — the sibling of
//! `tests/derived_evaluator_fixture_check_monster.rs`,
//! `..._class_feature.rs` and `..._spell_range.rs`, over
//! `derived-evaluator-fixtures.json`'s `companion_entries` array.
//!
//! # Why this file exists (SD31-W15-INTEGRATE-001)
//!
//! The wave-15 adversarial review (reachability lens) confirmed a real gap:
//! `companion_entries` shipped its four provenance fields — `upstream_lst`,
//! `upstream_lst_sha256`, `upstream_line`, `corpus_field` — and **nothing in
//! the tree asserted any of them.** They were loaded into `CompanionFixture`
//! and used only to format error strings. The reviewer proved it by mutating
//! one committed row's provenance to a file that does not exist, a zeroed
//! sha256, line 999999 and a made-up token, leaving only the expected values
//! intact: the whole companion suite stayed green (17 passed / 0 failed). The
//! same mutation shape on the sibling `spell_range` family goes red on two
//! separate assertions.
//!
//! Under Decision 1(a) — *a gate that cannot fail is worse than no gate* —
//! the provenance half of that seam was not a gate at all. It is one now.
//! This file does not re-litigate the 117 credited units: the reviewer
//! independently re-derived every one of them against the pinned oracle with
//! zero failures. It closes the FORWARD risk, which is that the pinned oracle
//! moves, or a row is edited, and nothing notices.
//!
//! # The guarantees
//!
//! 1. **Anchored to the same upstream bytes** —
//!    [`companion_pinned_upstream_lst_still_hashes_to_the_pinned_sha256`]
//!    re-hashes each distinct `upstream_lst` fresh off the pinned oracle.
//! 2. **The pinned corpus field is really on the pinned line** —
//!    [`companion_pinned_corpus_field_is_byte_identical_to_the_upstream_lst`].
//! 3. **The expected values are re-derivable** —
//!    [`companion_expected_values_are_re_derivable_from_the_pinned_corpus_field`]
//!    re-parses the pinned token with a reference derivation written HERE,
//!    independent of both the Python generator and the engine's
//!    `parse_companion_strength_damage`, and recomputes every
//!    `(strength_modifier, damage_bonus)` pair with PF1's halve-and-round-DOWN
//!    convention.
//! 4. **One row per unit, and the array is not empty** —
//!    [`companion_entries_are_present_and_one_per_unit`]. An empty array would
//!    make every other assertion here vacuously true, which is the exact
//!    shape Decision 1(a) forbids.
//!
//! # Prove-it-can-fail
//!
//! Each guarantee is mutation-provable on the committed fixture in isolation:
//! zero one row's `upstream_lst_sha256` → guarantee 1 red; change one
//! `corpus_field` from `max(0,(STR/2))` to `STR` → guarantees 2 and 3 red;
//! delete a row → guarantee 4 red on the one-per-unit count; empty the array
//! → guarantee 4 red on the non-empty assertion.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use sha2::{Digest, Sha256};

use codex::rules_core::derived_evaluator_fixture_check::load_companion_fixtures;

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

/// A REFERENCE derivation of PF1's natural-attack Strength-damage rule,
/// written here so this file agrees with the fixture only if both agree with
/// the printed rule. Deliberately NOT a call into
/// `parse_companion_strength_damage`/`evaluate_companion_strength_damage`:
/// this is the third independent implementation (Python generator, engine
/// evaluator, this), so no two of them can silently share a bug.
///
/// PF1 CRB p.9: halve and round DOWN. Rust's `/` truncates toward zero and
/// disagrees on every negative odd modifier, so this uses an explicit floor.
fn reference_damage_bonus(shape: &str, strength_modifier: i32) -> Option<i32> {
    let floor_half = |n: i32| {
        if n >= 0 {
            n / 2
        } else {
            -(((-n) + 1) / 2)
        }
    };
    match shape {
        "half_strength_never_negative" => Some(floor_half(strength_modifier).max(0)),
        "full_strength" => Some(strength_modifier),
        "negated_full_strength" => Some(-strength_modifier),
        other => other.parse::<i32>().ok(),
    }
}

/// The shape the pinned token itself states, re-parsed here from the raw
/// `corpus_field` bytes rather than taken from the fixture's `expected_shape`.
fn reference_shape(corpus_field: &str) -> Option<String> {
    let formula = corpus_field.rsplit_once("|DAMAGE|").map(|(_, f)| f)?;
    let f: String = formula.chars().filter(|c| !c.is_whitespace()).collect();
    // A PRE-gated bonus is a CONDITIONAL bonus; the seam refuses those rather
    // than rendering them as unconditional, so one must never be fixtured.
    if f.contains("|PRE") {
        return None;
    }
    Some(
        match f.as_str() {
            "max(0,(STR/2))" | "max(0,STR/2)" => "half_strength_never_negative",
            "STR" => "full_strength",
            "-STR" => "negated_full_strength",
            other => {
                if other.parse::<i32>().is_ok() {
                    "flat"
                } else {
                    return None;
                }
            }
        }
        .to_string(),
    )
}

#[test]
fn companion_entries_are_present_and_one_per_unit() {
    let fixtures = load_companion_fixtures(&repo_root());
    assert!(
        !fixtures.is_empty(),
        "companion_entries is EMPTY — every other guarantee in this file would \
         then pass vacuously, which is the Decision 1(a) shape this seam exists \
         to refuse"
    );
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for f in &fixtures {
        assert!(
            seen.insert(f.unit_id.as_str()),
            "companion_entries carries more than one row for {} — the seam clears \
             a unit only when its single row clears, so a duplicate would make \
             which row won undefined",
            f.unit_id
        );
        assert!(
            !f.expected_at.is_empty(),
            "{} pins no (strength_modifier, damage_bonus) pairs at all — a \
             fixture that asserts nothing cannot fail",
            f.unit_id
        );
    }
}

#[test]
fn companion_pinned_upstream_lst_still_hashes_to_the_pinned_sha256() {
    let Some(data_root) = pcgen_data_root() else {
        eprintln!("PCGEN_CORPUS_ROOT unset and no HOME; skipping");
        return;
    };
    let fixtures = load_companion_fixtures(&repo_root());
    assert!(!fixtures.is_empty(), "companion_entries must not be empty");
    // Hash each distinct file once; the fixture set names only a handful.
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
            "{}: {} has moved since this fixture was derived (pinned sha256 {}, \
             actual {}). Re-derive against the current pin rather than editing \
             the hash.",
            f.unit_id, f.upstream_lst, f.upstream_lst_sha256, actual
        );
        checked += 1;
    }
    assert!(checked > 0, "no fixture row was checked");
}

#[test]
fn companion_pinned_corpus_field_is_byte_identical_to_the_upstream_lst() {
    let Some(data_root) = pcgen_data_root() else {
        eprintln!("PCGEN_CORPUS_ROOT unset and no HOME; skipping");
        return;
    };
    let fixtures = load_companion_fixtures(&repo_root());
    assert!(!fixtures.is_empty(), "companion_entries must not be empty");
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
        assert!(
            line.contains(&f.corpus_field),
            "{}: pinned corpus_field {:?} does not appear on {}:{} — the fixture's \
             expected values were derived from bytes that are not there",
            f.unit_id,
            f.corpus_field,
            f.upstream_lst,
            f.upstream_line
        );
    }
}

#[test]
fn companion_expected_values_are_re_derivable_from_the_pinned_corpus_field() {
    let fixtures = load_companion_fixtures(&repo_root());
    assert!(!fixtures.is_empty(), "companion_entries must not be empty");
    let mut pairs_checked = 0usize;
    for f in &fixtures {
        let shape = reference_shape(&f.corpus_field).unwrap_or_else(|| {
            panic!(
                "{}: this file's INDEPENDENT reader cannot read a damage shape out of \
                 the pinned corpus_field {:?} — either the row is not this seam's \
                 shape, or it is PRE-gated and should never have been fixtured",
                f.unit_id, f.corpus_field
            )
        });
        assert_eq!(
            shape, f.expected_shape,
            "{}: the pinned corpus_field {:?} states shape {:?}, but the fixture \
             claims {:?}",
            f.unit_id, f.corpus_field, shape, f.expected_shape
        );
        for (strength_modifier, damage_bonus) in &f.expected_at {
            let expected = reference_damage_bonus(&shape, *strength_modifier)
                .expect("a shape this file recognised must evaluate");
            assert_eq!(
                expected, *damage_bonus,
                "{}: at Strength modifier {}, PF1's halve-and-round-down rule over \
                 {:?} gives {}, but the fixture pins {}",
                f.unit_id, strength_modifier, f.corpus_field, expected, damage_bonus
            );
            pairs_checked += 1;
        }
    }
    assert!(
        pairs_checked > 0,
        "no (strength_modifier, damage_bonus) pair was re-derived — vacuous"
    );
}
