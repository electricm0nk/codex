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
//! (`docs/governance/ogl-pi-blacklist.md` — SIGNED-OFF per SD-32
//! `decisions.md §19`), not an exhaustive legal review. It started as a
//! byte-for-byte reproduction of `gen_book_cache.rs::PI_BLACKLIST_TERMS`
//! (20 deities + 34 place/nation names) and has since grown by five
//! operator-approved per-book additions to 60 terms; see the term list's
//! own inline comments for each addition's provenance.

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
    // SD-32 `decisions.md §19a` amendment 3d (operator sign-off 2026-08-23,
    // `ogl-pi-blacklist.md` §2.3c): a citation of a PI term in a mechanical
    // `PREABILITY` prerequisite field redacts the citing record too.
    // Ported here (production Rust copy) by the T9-onboarding cycle that
    // transcribes corpus data under the amended, SIGNED-OFF blacklist --
    // `ogl-pi-blacklist.md`'s own frontmatter names this as the cycle
    // responsible for the bump from 57 to 60 terms.
    "Aldori", "Magaambya", "Magaambyan",
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
    // Per-book addition, `ogl-pi-blacklist.md`'s per-book-override
    // template: Inner Sea Gods's own SD31-E6-F10-001/SD31-W9-INTEGRATE-001
    // retrofit found the pinned oracle's OWN typo variants of two already-
    // blacklisted deity names surviving the exact-substring scan because
    // `classify_field` does not normalize case or spelling before
    // matching -- `isg_spells.lst:46`'s `FACTSET:Deity|Cayden CaiLean`
    // (capital L) shipped unredacted under `license:"OGL"` while its 51
    // correctly-spelled siblings redacted, and `isg_spells.lst:8`'s own
    // OCR of "Irori" as "lrori" (lowercase L) in `abstemiousness.json`'s
    // DESC shipped unredacted the same way. Folded into the shared list
    // per the same union-not-book-scoped rationale as "Jarn" above.
    "Cayden CaiLean",
    "lrori",
    // Per-book addition, `ogl-pi-blacklist.md`'s per-book-override
    // template, same shape as the two additions immediately above: the
    // `pi-key-rawtokens-screen` cycle's generic `data.key`/`data.raw_tokens`
    // corpus-wide audit (SD-32 card 11, 2026-08-23) found the pinned
    // oracle's OWN lowercase-possessive typo of an already-blacklisted
    // deity name at `isg_equip.lst:232` (coordinate, not the term itself --
    // see `ogl-pi-blacklist.md`'s per-book-override section for this
    // addition, which names it) surviving `classify_field`'s
    // case-SENSITIVE substring scan, because the capitalized canonical
    // spelling already on this list never matches the oracle's own
    // lowercase variant. Shipped unredacted in
    // `data/corpus/inner_sea_gods/equipment/wayfinder_of_zephyrs.json`'s
    // `data.description` and `raw_tokens[DESC]` until this cycle's fix.
    // Verified before adding: this lowercase variant (any case) occurs in
    // exactly one PCGen source file at exactly two lines -- one already
    // `NAMEISPI:YES`-excluded, the other this leak -- so this addition
    // widens redaction nowhere else. Folded into the shared list per the
    // same union-not-book-scoped rationale as the two additions above,
    // rather than case-folding the whole scan (evaluated and rejected: a
    // whole-list case-fold reopens the exact short-term/ordinary-word
    // collision `§19a`'s own normalization rule had to add word-boundary
    // matching to avoid (its own recorded incident, `ogl-pi-blacklist.md
    // §2.3a`), and this production copy deliberately has NO word-boundary
    // guard because real corpus identifiers concatenate a PI term into
    // another word with no separator, e.g. the concatenated identifier
    // shape `magaambya_is_redacted` below tests against).
    "gozreh's",
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

// --- PCGen's own per-record declaration ------------------------------------

/// What a PCGen row says about *itself*, read from its own tokens.
///
/// [`PI_BLACKLIST_TERMS`] above is a heuristic this program assembled — 60
/// names, documented as a bounded sample rather than a legal review. The corpus
/// states the same fact directly, per record, in two tokens the ingest path had
/// never read: `NAMEISPI:YES` and `DESCISPI:YES`.
///
/// **The two disagree, one-directionally.** Over the shipped `race_trait` tree
/// on 2026-08-12, 26 records declared `DESCISPI:YES`; the term list happened to
/// redact 18 of them because their prose contains a Golarion place name it
/// knows, and the remaining 8 published text the publisher had marked as
/// Product Identity (`Kodar Mountains`, `Earthfall`, `Ekujae`, `Gogpodda`,
/// `Omesta`, `Droskar`, `Abaddon`, `Inner Sea` — none on the list). A
/// declaration is not a heuristic and does not need to agree with one: where
/// the row declares, the row wins.
///
/// `NAMEISPI:YES` was first read by the monster lane for one book, in a
/// hand-built table (`rules_tables/inner_sea_world_guide/`), and reported to
/// this lane as corpus-wide territory. This is the shared reader that closes it
/// for every kind rather than for a second book.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DeclaredProductIdentity {
    /// `NAMEISPI:YES` — the record's **name** is Product Identity. A name
    /// cannot be redacted (it is the record's identity on every screen and half
    /// of its key), so the only way not to publish it is not to publish the
    /// row. See `docs/governance/ogl-pi-blacklist.md` §3 for the per-book
    /// override that could reclassify one, which is an operator decision.
    pub name: bool,
    /// `DESCISPI:YES` — the record's **description** is Product Identity. A
    /// description *can* be redacted and the record still works, so the row
    /// ships with [`REDACTED_PI_MARKER`] in place of its prose.
    pub description: bool,
}

impl DeclaredProductIdentity {
    /// `true` when the row declares anything at all — the cheap guard a caller
    /// uses before paying for a redaction path.
    pub fn any(self) -> bool {
        self.name || self.description
    }
}

/// Reads [`DeclaredProductIdentity`] off a row's `(key, value)` tokens.
///
/// Keys are matched case-insensitively and values are trimmed, because a PCGen
/// row is hand-maintained text: nothing guarantees the token is spelled exactly
/// `DESCISPI:YES` with no trailing whitespace, and a screen that silently
/// misses a declaration over a space is the failure this reader exists to
/// prevent. Any value other than `YES` is not a declaration — PCGen writes
/// `NAMEISPI:NO` explicitly on rows that are OGL — so it is read as absent
/// rather than as a hit.
pub fn declared_product_identity<I, K, V>(tokens: I) -> DeclaredProductIdentity
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
{
    let mut declared = DeclaredProductIdentity::default();
    for (key, value) in tokens {
        if !value.as_ref().trim().eq_ignore_ascii_case("YES") {
            continue;
        }
        let key = key.as_ref();
        if key.eq_ignore_ascii_case("NAMEISPI") {
            declared.name = true;
        } else if key.eq_ignore_ascii_case("DESCISPI") {
            declared.description = true;
        }
    }
    declared
}

/// [`classify_optional_field`], with the row's own declaration taking
/// precedence over the term list.
///
/// `declared` is the value of the matching [`DeclaredProductIdentity`] field
/// for `field_name`. When it is `true` the field is redacted whatever the term
/// scan says; when it is `false` the term scan still runs, because a row that
/// declares nothing can still contain a deity name (`ogl-pi-blacklist.md` §2 is
/// explicit that the corpus's own markers are incomplete, which is why the term
/// list exists at all). The two screens are a union, never a substitution.
pub fn classify_optional_field_declared(
    field_name: &str,
    value: Option<&str>,
    declared: bool,
) -> (License, Option<String>, Option<String>, Option<String>) {
    match (value, declared) {
        (None, _) => (License::Ogl, None, None, None),
        (Some(_), true) => (
            License::PiRedacted,
            Some(field_name.to_string()),
            Some(PI_MARKER_REDACTED.to_string()),
            Some(REDACTED_PI_MARKER.to_string()),
        ),
        (Some(v), false) => {
            let (license, pi_field, pi_marker, stored) = classify_field(field_name, v);
            (license, pi_field, pi_marker, Some(stored))
        }
    }
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
        // ("Jarn", `ogl-pi-blacklist.md`'s per-book-override template) + 2
        // Inner Sea Gods-specific per-book additions ("Cayden CaiLean",
        // "lrori" -- the oracle's own typo/OCR variants of two already-
        // blacklisted deity names) + 3 SD-32 `decisions.md §19a` amendment
        // 3d additions ("Aldori", "Magaambya", "Magaambyan" -- operator
        // sign-off 2026-08-23, `ogl-pi-blacklist.md` §2.3c), bringing this
        // production copy to parity with the SIGNED-OFF 60-term list, + 1
        // more per-book addition (`pi-key-rawtokens-screen` cycle,
        // 2026-08-23, see this array's own trailing entry's comment and
        // `ogl-pi-blacklist.md`'s per-book-override section for what it is
        // and why) -- 61 total, one term ahead of the SIGNED-OFF list until
        // a future sign-off cycle folds this addition into
        // `ogl-pi-blacklist.md §2.3c`'s own term count the way the three
        // `§19a` additions were.
        assert_eq!(PI_BLACKLIST_TERMS.len(), 61);
    }

    /// The `pi-key-rawtokens-screen` cycle's addition (2026-08-23) redacts
    /// the oracle's own lowercase-possessive typo of the deity name at
    /// index 9 -- built HERE by lowercasing that pre-existing entry and
    /// appending a possessive, never by indexing the new array entry
    /// itself, so this test independently re-derives the target string
    /// instead of just echoing back whatever the array's last slot holds
    /// (a position-based echo would spuriously pass against ANY entry, not
    /// specifically the one this cycle added -- confirmed live: an
    /// earlier draft of this test kept passing after the real fix was
    /// reverted, because it re-read whatever the (now different) last
    /// entry was rather than this specific derived string). Mutation-proved
    /// RED with the fix reverted (`git stash`-free repro: temporarily
    /// deleting this array's last entry), GREEN restored.
    #[test]
    fn the_oracle_s_lowercase_possessive_typo_of_the_index_9_deity_is_redacted() {
        let canonical_deity = PI_BLACKLIST_TERMS[9];
        let oracle_typo_variant = format!("{}'s", canonical_deity.to_lowercase());
        let text = format!("engraved with themes of {oracle_typo_variant} oceanic aspect");
        // Sanity: the pre-existing capitalized entry alone must NOT already
        // catch this lowercase variant -- otherwise this test would pass
        // for a reason unrelated to this cycle's addition.
        assert!(!text.contains(canonical_deity));
        let (license, ..) = classify_field("description", &text);
        assert_eq!(
            license,
            License::PiRedacted,
            "the oracle's own lowercase-possessive typo of an already-blacklisted deity name must redact"
        );
    }

    #[test]
    fn aldori_is_redacted() {
        let (license, ..) = classify_field("value", "1,FEAT=Aldori Dueling Sword Proficiency");
        assert_eq!(license, License::PiRedacted);
    }

    #[test]
    fn magaambya_is_redacted() {
        let (license, ..) = classify_field("value", "SpecialQuality.MagaambyaReward");
        assert_eq!(license, License::PiRedacted);
    }

    #[test]
    fn jarn_is_redacted() {
        let (license, ..) = classify_field("description", "an NPC named Jarn appears here");
        assert_eq!(license, License::PiRedacted);
    }

    #[test]
    fn cayden_cailean_s_oracle_typo_variant_is_redacted() {
        // isg_spells.lst:46's own FACTSET:Deity|Cayden CaiLean (capital L)
        // must redact exactly like the correctly-spelled form does --
        // this is the exact shape that shipped unredacted before this fix.
        let (license, ..) = classify_field("value", "Deity|Cayden CaiLean");
        assert_eq!(license, License::PiRedacted);
    }

    #[test]
    fn irori_s_oracle_ocr_typo_variant_is_redacted() {
        // isg_spells.lst:8's own OCR of "Irori" as "lrori" (lowercase L)
        // must redact like the correctly-spelled form does.
        let (license, ..) = classify_field("description", "Sometimes lrori smiles on his worshipers");
        assert_eq!(license, License::PiRedacted);
    }

    // --- PCGen's own per-record declaration --------------------------------

    #[test]
    fn a_row_with_no_pi_tokens_declares_nothing() {
        let declared = declared_product_identity([("KEY", "Elf ~ Keen Senses"), ("DESC", "+2 Perception.")]);
        assert_eq!(declared, DeclaredProductIdentity::default());
        assert!(!declared.any());
    }

    #[test]
    fn nameispi_and_descispi_are_read_independently() {
        let name_only = declared_product_identity([("NAMEISPI", "YES")]);
        assert!(name_only.name && !name_only.description);
        let desc_only = declared_product_identity([("DESCISPI", "YES")]);
        assert!(desc_only.description && !desc_only.name);
        let both = declared_product_identity([("NAMEISPI", "YES"), ("DESCISPI", "YES")]);
        assert!(both.name && both.description && both.any());
    }

    /// PCGen writes `NAMEISPI:NO` explicitly on OGL rows. Reading any non-`YES`
    /// value as a hit would redact the whole corpus.
    #[test]
    fn an_explicit_no_is_not_a_declaration() {
        let declared = declared_product_identity([("NAMEISPI", "NO"), ("DESCISPI", "NO")]);
        assert_eq!(declared, DeclaredProductIdentity::default());
    }

    /// A `.lst` row is hand-maintained text; a screen that misses a declaration
    /// over a space or a lowercase key is the failure this reader exists to
    /// prevent.
    #[test]
    fn the_token_is_matched_case_insensitively_and_trimmed() {
        let declared = declared_product_identity([("descispi", " yes ")]);
        assert!(declared.description);
    }

    /// The point of the whole reader: this description carries no blacklist
    /// term, so the term scan alone publishes it.
    #[test]
    fn a_declared_description_is_redacted_even_though_no_blacklist_term_appears_in_it() {
        let prose = "these tieflings are tied to the daemons of Abaddon";
        assert_eq!(classify_field("description", prose).0, License::Ogl, "the term list does not know it");

        let (license, pi_field, pi_marker, stored) =
            classify_optional_field_declared("description", Some(prose), true);
        assert_eq!(license, License::PiRedacted);
        assert_eq!(pi_field.as_deref(), Some("description"));
        assert_eq!(pi_marker.as_deref(), Some(PI_MARKER_REDACTED));
        assert_eq!(stored.as_deref(), Some(REDACTED_PI_MARKER));
    }

    /// The two screens are a union, never a substitution: a row declaring
    /// nothing is still scanned.
    #[test]
    fn an_undeclared_description_still_goes_through_the_term_scan() {
        let (license, ..) =
            classify_optional_field_declared("description", Some("blessed by Iomedae"), false);
        assert_eq!(license, License::PiRedacted);
        let (clean, ..) = classify_optional_field_declared("description", Some("Deals 1d6 fire."), false);
        assert_eq!(clean, License::Ogl);
    }

    #[test]
    fn a_declared_but_absent_field_has_nothing_to_redact() {
        let (license, pi_field, pi_marker, stored) = classify_optional_field_declared("description", None, true);
        assert_eq!(license, License::Ogl);
        assert_eq!((pi_field, pi_marker, stored), (None, None, None));
    }
}
