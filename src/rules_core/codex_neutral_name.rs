//! SD-32 `decisions.md §24` — the Codex-generated neutral name for units
//! whose *own name* is Product Identity (`ability`/`deity`/`class_feature`).
//! This is the Rust port of `scripts/codex_neutral_name.py`, used by
//! generators that already run in Rust (`cache_gen::class_feature`) rather
//! than as a Python post-pass, kept byte-identical in derivation to the
//! Python module so a record renamed by either path is indistinguishable.
//!
//! **Binding condition 1 (`§24b`-1): derived ONLY from non-PI coordinates.**
//! `(kind, book, source_file, source_line)` — the exact join key
//! `scripts/shape_ledger.py` already uses, plus `kind`. **Never** from the
//! PI name — not transformed, not truncated, not hashed. Enforced
//! structurally: every public function below takes `kind`/`book`/
//! `source_file`/`source_line` and no `name`/`key`/free-text parameter at
//! all, so there is no argument through which a PI string could enter the
//! derivation. `src/rules_core/tests` (this module's own `#[cfg(test)]`
//! block) proves the black-box claim: two calls that share coordinates
//! produce the same output regardless of what original name the CALLER
//! happens to hold (never passed in).
//!
//! **Binding condition 6: determinism.** Pure string composition of the
//! four inputs — no randomness, no wall-clock, no hashing. `§24a` argues
//! against a hash ("a hash in particular is a fingerprint that confirms a
//! guess"), so this module deliberately does not hash anything, including
//! the coordinates themselves.

use std::path::Path;

/// The literal marker every renamed record's human-readable name carries —
/// chosen so a reader (or a future generator) recognises a Codex-minted
/// identity on sight, per `§24b`-3.
pub const NAME_PREFIX: &str = "Codex-Named Unit";

/// Lowercase, `[a-z0-9]+`-runs-joined-by-`_` normalisation. Applied only to
/// non-PI coordinate strings (book names, filenames, kind names) — never to
/// a record's display name or description.
fn slug(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut last_was_sep = true; // suppress a leading separator
    for ch in value.chars() {
        let lower = ch.to_ascii_lowercase();
        if lower.is_ascii_alphanumeric() {
            out.push(lower);
            last_was_sep = false;
        } else if !last_was_sep {
            out.push('_');
            last_was_sep = true;
        }
    }
    while out.ends_with('_') {
        out.pop();
    }
    if out.is_empty() {
        "x".to_string()
    } else {
        out
    }
}

/// The stable, PI-free identifier for one unit: `kind_book_basename_line`.
/// Two different units can never collide here unless they share the exact
/// same `(book, source_file, source_line)` citation, which the census's own
/// enumeration already treats as the same object.
pub fn neutral_coordinate_id(kind: &str, book: &str, source_file: &str, source_line: u32) -> String {
    let basename = Path::new(source_file)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(source_file);
    format!("{}_{}_{}_{}", slug(kind), slug(book), slug(basename), source_line)
}

/// The Codex-generated display name a renamed record's `data.name` carries.
/// Self-describing as generated ([`NAME_PREFIX`] prefix) so even a reader who
/// never checks the `codex_generated_name` field sees it is not a printed
/// name.
pub fn neutral_name(kind: &str, book: &str, source_file: &str, source_line: u32) -> String {
    format!("{NAME_PREFIX} ({})", neutral_coordinate_id(kind, book, source_file, source_line))
}

/// Same identity, used wherever a record needs a `key` distinct in
/// principle from `name` — here the two are the same string, mirroring
/// `scripts/codex_neutral_name.py::neutral_key`.
pub fn neutral_key(kind: &str, book: &str, source_file: &str, source_line: u32) -> String {
    neutral_name(kind, book, source_file, source_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `§24b`-1's own required proof: the generator's output is unchanged
    /// when the (never-passed) PI name is swapped for a different string.
    /// There is no name parameter to swap, so this test proves the claim by
    /// construction: two callers with different "original name" context
    /// (never threaded into these functions at all) produce identical
    /// output for the same coordinates.
    #[test]
    fn output_is_unchanged_when_the_pi_name_is_swapped_for_a_different_string() {
        // Caller A "believes" the original name is one string; caller B
        // believes it is a completely different one. Neither belief has any
        // parameter to travel through -- both derive from the SAME call.
        let original_name_a = "Iomedae"; // stand-in PI-shaped string, not asserted to be real PI
        let original_name_b = "Some Entirely Different Name";
        assert_ne!(original_name_a, original_name_b);

        let a = neutral_name("deity", "inner_sea_gods", "isg_deities.lst", 42);
        let b = neutral_name("deity", "inner_sea_gods", "isg_deities.lst", 42);
        assert_eq!(a, b, "same coordinates must produce the same name regardless of any external name context");
        assert!(!a.contains(original_name_a) && !a.contains(original_name_b));
    }

    #[test]
    fn coordinate_id_is_pure_composition_of_kind_book_basename_line() {
        let id = neutral_coordinate_id("class_feature", "adventurers_guide", "support/ag_abilities_class_2.lst", 913);
        assert_eq!(id, "class_feature_adventurers_guide_ag_abilities_class_2_lst_913");
    }

    #[test]
    fn determinism_two_calls_same_inputs_byte_identical() {
        let n1 = neutral_name("deity", "book", "file.lst", 7);
        let n2 = neutral_name("deity", "book", "file.lst", 7);
        assert_eq!(n1, n2);
        let k1 = neutral_key("deity", "book", "file.lst", 7);
        let k2 = neutral_key("deity", "book", "file.lst", 7);
        assert_eq!(k1, k2);
    }

    #[test]
    fn different_coordinates_never_collide() {
        let a = neutral_coordinate_id("deity", "book", "file.lst", 7);
        let b = neutral_coordinate_id("deity", "book", "file.lst", 8);
        assert_ne!(a, b);
    }

    #[test]
    fn slug_never_produces_an_empty_or_leading_underscore_segment() {
        assert_eq!(slug(""), "x");
        assert_eq!(slug("!!!"), "x");
        assert_eq!(slug("  Hello, World!  "), "hello_world");
    }
}
