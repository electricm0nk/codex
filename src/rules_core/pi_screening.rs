//! Shared Product-Identity blacklist screening, per
//! `docs/governance/ogl-pi-blacklist.md` and `decisions.md §17`.
//!
//! **Why this module exists.** Before it did, the same 54-term blacklist
//! (`gen_book_cache.rs`'s `PI_BLACKLIST_TERMS`/`classify_field`,
//! `ingest_pu_classes.rs`'s and `ingest_races.rs`'s own
//! `PI_BLACKLIST_TERMS`/`pi_hits`) was forked three times, independently.
//! Three of the five corpus-cache writers had NO screening at all
//! (CRB's `gen_core_rulebook_cache.rs`, ACG/APG/Bestiary 1's
//! `cache_gen::{acg,apg,beastiary1}`), so their `license`/`pi_field`/
//! `pi_marker` fields only ever existed on disk via a post-hoc retrofit
//! pass the generator itself knew nothing about — the exact failure
//! mode `wiring_class`'s own `.MOD`-closure fix addressed for magnitude
//! detection, now addressed here for licensing. New callers use this
//! module; the three existing screened writers keep their own working
//! copies rather than being refactored onto this one, per the operator's
//! explicit call: touching already-correct screening code to de-duplicate
//! it risks creating two screening paths that can silently diverge, which
//! is the same class of bug this module exists to prevent from spreading
//! further.
//!
//! The term list itself is a bounded, documented heuristic
//! (`docs/governance/ogl-pi-blacklist.md` — DRAFT, operator-reviewable),
//! not an exhaustive legal review. It is reproduced here byte-for-byte
//! from `gen_book_cache.rs::PI_BLACKLIST_TERMS`, the most complete
//! of the three existing copies (20 deities + 34 place/nation names).

use crate::rules_core::shape_b_v1::{License, PI_MARKER_REDACTED, REDACTED_PI_MARKER};

/// The 20 canonical core Golarion deities plus 34 sampled setting
/// place/nation names — byte-identical to `gen_book_cache.rs`'s copy.
pub const PI_BLACKLIST_TERMS: &[&str] = &[
    "Iomedae", "Sarenrae", "Asmodeus", "Cayden Cailean", "Abadar", "Calistria", "Desna", "Erastil", "Gorum", "Gozreh",
    "Irori", "Lamashtu", "Nethys", "Norgorber", "Pharasma", "Rovagug", "Shelyn", "Torag", "Urgathoa", "Zon-Kuthon",
    "Golarion", "Absalom", "Cheliax", "Varisia", "Andoran", "Taldor", "Osirion", "Katapesh", "Ustalav", "Numeria",
    "Mwangi", "Tian Xia", "Avistan", "Garund", "Sarkoris", "Worldwound", "Vudra", "Kyonin", "Molthune", "Nidal",
    "Nirmathas", "Qadira", "Razmiran", "Rahadoum", "Galt", "Isger", "Lastwall", "Brevoy", "Druma", "Irrisen",
    "Jalmeray", "Thuvia", "Geb", "Nex",
    // Per-book addition, `ogl-pi-blacklist.md`'s per-book-override
    // template: ACG's own E2.0.8 retrofit found the example NPC name
    // "Jarn" embedded in `advanced_class_guide/spell/discern_next_of_kin.json`'s
    // flavor text while sampling that book's real description text -- not
    // one of the 20 deities, discovered per-book rather than pre-declared.
    // Folded into the shared list (rather than kept ACG-only) so this
    // module's single term list stays the actual union of every real hit
    // any book's retrofit has found, and a future book carrying the same
    // name is caught too.
    "Jarn",
];

/// `(license, pi_field, pi_marker, stored_value)` for one free-text field
/// value, per the PI-blacklist screen. A substring hit on any blacklist
/// term redacts the whole value to the literal marker
/// [`REDACTED_PI_MARKER`]; no hit is plain OGL. Mirrors
/// `gen_book_cache.rs::classify_field` exactly.
pub fn classify_field(field_name: &str, value: &str) -> (License, Option<String>, Option<String>, String) {
    for term in PI_BLACKLIST_TERMS {
        if value.contains(term) {
            return (
                License::PiRedacted,
                Some(field_name.to_string()),
                Some(PI_MARKER_REDACTED.to_string()),
                REDACTED_PI_MARKER.to_string(),
            );
        }
    }
    (License::Ogl, None, None, value.to_string())
}

/// [`classify_field`] over an `Option<&str>` field (a record whose free-text
/// field may be absent, e.g. no `description` token at all): `None` is
/// blanket OGL with nothing to redact, never scanned.
pub fn classify_optional_field(
    field_name: &str,
    value: Option<&str>,
) -> (License, Option<String>, Option<String>, Option<String>) {
    match value {
        None => (License::Ogl, None, None, None),
        Some(v) => {
            let (license, pi_field, pi_marker, stored) = classify_field(field_name, v);
            (license, pi_field, pi_marker, Some(stored))
        }
    }
}

/// A record with no free-text field at all (e.g. a class chassis record:
/// only mechanical `class_id`/`maxlevel`/BAB/save fields) — blanket OGL
/// per `ogl-pi-blacklist.md` §2.2, nothing to scan.
pub fn blanket_ogl() -> (License, Option<String>, Option<String>) {
    (License::Ogl, None, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_blacklist_term_is_plain_ogl() {
        let (license, pi_field, pi_marker, stored) =
            classify_field("description", "Deals 1d6 points of fire damage.");
        assert_eq!(license, License::Ogl);
        assert_eq!(pi_field, None);
        assert_eq!(pi_marker, None);
        assert_eq!(stored, "Deals 1d6 points of fire damage.");
    }

    #[test]
    fn a_deity_name_redacts_the_whole_value() {
        let (license, pi_field, pi_marker, stored) =
            classify_field("description", "As per Iomedae's blessing, you gain a +2 bonus.");
        assert_eq!(license, License::PiRedacted);
        assert_eq!(pi_field.as_deref(), Some("description"));
        assert_eq!(pi_marker.as_deref(), Some(PI_MARKER_REDACTED));
        assert_eq!(stored, REDACTED_PI_MARKER);
    }

    #[test]
    fn a_place_name_redacts_too() {
        let (license, ..) = classify_field("description", "You hail from Absalom.");
        assert_eq!(license, License::PiRedacted);
    }

    #[test]
    fn optional_field_absent_is_blanket_ogl_never_scanned() {
        let (license, pi_field, pi_marker, stored) = classify_optional_field("description", None);
        assert_eq!(license, License::Ogl);
        assert_eq!(pi_field, None);
        assert_eq!(pi_marker, None);
        assert_eq!(stored, None);
    }

    #[test]
    fn blanket_ogl_for_no_free_text_field_records() {
        assert_eq!(blanket_ogl(), (License::Ogl, None, None));
    }

    #[test]
    fn term_list_matches_the_reference_copy_plus_the_documented_acg_addition() {
        // 20 deities + 34 place/nation names (the shared 54-term list
        // every existing fork carries) + 1 ACG-specific per-book addition
        // ("Jarn", `ogl-pi-blacklist.md`'s per-book-override template).
        assert_eq!(PI_BLACKLIST_TERMS.len(), 55);
    }

    #[test]
    fn jarn_is_redacted() {
        let (license, ..) = classify_field("description", "an NPC named Jarn appears here");
        assert_eq!(license, License::PiRedacted);
    }
}
