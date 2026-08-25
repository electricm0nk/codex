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
    // SD-32 declared-pi-shipping-65-followups: a value that is ALREADY the
    // redaction marker carries no blacklist term to scan for (the marker
    // text itself is inert), so without this guard it fell through to the
    // `Ogl`/`None` return below -- stamping metadata that claims nothing
    // was ever redacted on a value that already IS the redacted form. This
    // is the root cause of 99 corpus records shipping `description:
    // "[redacted PI]"` with `license: "OGL"`/`pi_field: null`, verified
    // corpus-wide.
    if value == REDACTED_PI_MARKER {
        return (
            License::PiRedacted,
            Some(field_name.to_string()),
            Some(PI_MARKER_REDACTED.to_string()),
            REDACTED_PI_MARKER.to_string(),
        );
    }
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

// ---------------------------------------------------------------------------
// `decisions.md §12b`/T9-round-4 receipt: Rust-side port of
// `scripts/pi_scrub.py`'s `canonicalize`/`normalized_term_hit`/
// `blacklist_term_hit_including_concatenated` -- mirrors that module's
// semantics exactly (word-bounded, case-folded, bounded-OCR-normalized scan,
// OR an alphanumeric-normalized substring match for a blacklisted term
// concatenated PascalCase-style into another token's value with no
// separator, bounded to needles of at least 6 normalized characters).
// [`classify_field`] above is a DIFFERENT, older, deliberately-unbounded bare
// substring scan (its own doc comment explains why); this port is additive,
// used where a caller specifically needs the same guarantees the Python
// review/ingest scripts give (word-boundary + OCR fold with the length-bound
// concatenated fallback), not a replacement for `classify_field`'s callers.
//
// This is the SAME 61-term `PI_BLACKLIST_TERMS` above. `decisions.md §12b`
// (found by the `class_feature` lane, closed by the SD-32 integrity-sweep
// cycle, 2026-08-23): this copy and `pi_scrub.py`'s used to disagree by one
// term -- `pi_scrub.py`'s copy now carries the same trailing per-book
// addition this array does, and `tests/pi_blacklist_terms_rust_python_agree.rs`
// fails the build if the two ever diverge again. Every OTHER behaviour (fold
// table, word-boundary rule, concatenated-match bound) is a byte-identical
// port.

const SOFT_HYPHEN: char = '\u{ad}';

/// "Jarn" is the only blacklist term whose OCR-fold `rn`->`m` substitution
/// collides with an ordinary English word ("jam") -- mirrors
/// `pi_scrub.py::_RN_FOLD_EXEMPT_TERMS_CASEFOLD`/`_term_needs_rn_fold`.
fn term_needs_rn_fold(term: &str) -> bool {
    !term.eq_ignore_ascii_case("jarn")
}

/// "Galt" is the only blacklist term containing `l`, and its `l`->`i` fold
/// collides with the ordinary English word "gait" -- the SAME false-positive
/// class `term_needs_rn_fold` above exists for (Jarn/jam), found live
/// re-deriving `corpus_literal_sweep` against the pinned oracle
/// (t9-onboarding cycle, 2026-08-23): `advanced_players_guide/class_feature/
/// shifter_s_blessing/form_of_the_cat.json`'s DESC token, and three sibling
/// `class_feature` records whose KEY/ABILITY token restates a "<Name>'s
/// Gait"/"Steady Gait"-shaped ability name, all went `[redacted PI]` for a
/// fold collision, not a real PI hit. Mirrors
/// `pi_scrub.py::_CHAR_FOLD_EXEMPT_TERMS_CASEFOLD`/`_term_needs_char_fold`.
fn term_needs_char_fold(term: &str) -> bool {
    !term.eq_ignore_ascii_case("galt")
}

/// Case-fold + bounded OCR-confusion fold (`l`/`1`/`!` -> `i`, `0` -> `o`,
/// `rn` -> `m` unless `apply_rn_fold` is false; the `l`/`1`/`!`/`0` table
/// itself skipped when `apply_char_fold` is false). Mirrors
/// `pi_scrub.py::canonicalize` exactly, including fold order (rn-fold before
/// the character table, so `rn` is never itself re-split by the l/1/!/0
/// substitutions -- neither table produces or consumes those bytes, so order
/// is actually immaterial here, but kept identical to the Python source for
/// auditability).
fn canonicalize(s: &str, apply_rn_fold: bool, apply_char_fold: bool) -> String {
    let folded = s.to_lowercase().replace(SOFT_HYPHEN, "-");
    let folded = if apply_rn_fold { folded.replace("rn", "m") } else { folded };
    if !apply_char_fold {
        return folded;
    }
    folded
        .chars()
        .map(|c| match c {
            'l' | '1' | '!' => 'i',
            '0' => 'o',
            other => other,
        })
        .collect()
}

/// `true` when `needle` occurs in `haystack` with an ASCII-alphanumeric
/// boundary (or string edge) on both sides -- mirrors the Python regex
/// `(?<![a-z0-9])term(?![a-z0-9])` against an already-canonicalized (lowercase
/// -only) haystack, so checking `is_ascii_alphanumeric()` on either boundary
/// byte reproduces the same `[a-z0-9]` class Python excludes.
pub(crate) fn word_bounded_contains(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return false;
    }
    let hb = haystack.as_bytes();
    let nb = needle.as_bytes();
    if nb.len() > hb.len() {
        return false;
    }
    for start in 0..=(hb.len() - nb.len()) {
        if &hb[start..start + nb.len()] != nb {
            continue;
        }
        let left_ok = start == 0 || !hb[start - 1].is_ascii_alphanumeric();
        let right = start + nb.len();
        let right_ok = right == hb.len() || !hb[right].is_ascii_alphanumeric();
        if left_ok && right_ok {
            return true;
        }
    }
    false
}

/// Every blacklist term whose canonicalized form appears, word-bounded, in
/// the canonicalized `free_text` -- mirrors `pi_scrub.py::normalized_term_hits`.
///
/// PERFORMANCE: `canonicalize(free_text, ...)` only ever depends on the
/// (`needs_rn_fold`, `needs_char_fold`) pair, not on which term is being
/// checked -- and across the current [`PI_BLACKLIST_TERMS`], only 2 terms
/// (`term_needs_rn_fold`/`term_needs_char_fold`'s own documented Jarn/Galt
/// exceptions) diverge from the common `(true, true)` case, so there are at
/// most 3 distinct pairs in practice, never 61. The naive per-term loop
/// re-canonicalized the SAME `free_text` up to 61 times regardless -- for
/// `declared-pi-audit`'s CHECK C, called once per shipped string across the
/// full corpus (order-of-a-million calls at the current widened population),
/// that repeated re-allocation (two `to_lowercase`/`replace` passes per call)
/// was the dominant cost of a stage that hung at 99.9% CPU for minutes with
/// no output. Caching each distinct canonicalized form the first time this
/// call needs it -- keyed on the same two booleans the naive version already
/// branched on -- collapses that to at most 3 canonicalizations of
/// `free_text` per call, with byte-for-byte identical output (same terms,
/// same order, same hits): a pure memoization of a pure function of
/// `free_text` and the two fold flags, nothing about which terms match or in
/// what order changes.
pub fn normalized_term_hits(free_text: &str) -> Vec<&'static str> {
    if free_text.trim().is_empty() {
        return Vec::new();
    }
    let mut hits = Vec::new();
    // At most 3 entries ever populated: (true,true) [59 terms], (false,true)
    // [Jarn], (true,false) [Galt] -- see the doc comment above.
    let mut canon_text_cache: Vec<((bool, bool), String)> = Vec::with_capacity(3);
    for term in PI_BLACKLIST_TERMS {
        let needs_rn_fold = term_needs_rn_fold(term);
        let needs_char_fold = term_needs_char_fold(term);
        let canon_term = canonicalize(term, needs_rn_fold, needs_char_fold);
        if canon_term.is_empty() {
            continue;
        }
        let key = (needs_rn_fold, needs_char_fold);
        let idx = match canon_text_cache.iter().position(|(k, _)| *k == key) {
            Some(i) => i,
            None => {
                canon_text_cache.push((key, canonicalize(free_text, needs_rn_fold, needs_char_fold)));
                canon_text_cache.len() - 1
            }
        };
        if word_bounded_contains(&canon_text_cache[idx].1, &canon_term) {
            hits.push(*term);
        }
    }
    hits
}

/// First blacklist term whose canonicalized form appears, word-bounded, in
/// `free_text` -- mirrors `pi_scrub.py::normalized_term_hit`.
pub fn normalized_term_hit(free_text: &str) -> Option<&'static str> {
    normalized_term_hits(free_text).into_iter().next()
}

/// The minimum normalized-character length a blacklist term must reach
/// before the alphanumeric-normalized (no-separator) concatenated check
/// applies to it -- mirrors `pi_scrub.py::_MIN_NORMALIZED_NEEDLE_LEN`. Below
/// this bound, a short blacklist term (already covered at its ordinary,
/// separated occurrences by the word-bounded scan above) risks
/// over-redacting a value by coincidence.
const MIN_NORMALIZED_NEEDLE_LEN: usize = 6;

/// Lowercase, then strip every byte that is not ASCII alphanumeric -- mirrors
/// `pi_scrub.py::_normalize` (`re.sub(r"[^a-z0-9]", "", s.lower())`). Used
/// ONLY to build the short, known needle/term forms
/// ([`PI_BLACKLIST_TERMS`]'s own normalized forms below) -- never on a
/// haystack VALUE being scanned for a hit; see [`alnum_normalize_haystack`]
/// for why.
fn alnum_normalize(s: &str) -> String {
    s.to_lowercase().chars().filter(|c| c.is_ascii_alphanumeric()).collect()
}

/// Value-side normalization for the concatenated-form check: strips
/// punctuation the way [`alnum_normalize`] does, but PRESERVES real
/// whitespace as a hard separator. Mirrors `pi_scrub.py::_normalize_haystack`
/// exactly -- that fix (`decisions.md §26`-adjacent, the
/// `hidden_wand.json`/"Andoran" incident) was never ported to this Rust copy
/// until the t9-onboarding `corpus_literal_sweep` unblock cycle
/// (2026-08-23): the OLD (whitespace-stripping) normalization manufactured
/// the substring "andoran" out of the three separate, real English words
/// "Commando", "Ranger", "Trap" once every space was deleted
/// (`ultimate_wilderness/class_feature/commando/ranger_trap.json`'s KEY
/// token went `[redacted PI]` for exactly this reason, live on this corpus,
/// before this fix). This check exists to catch a term truly joined with NO
/// separator at all -- a PCGen `BONUS`/`DEFINE` variable identifier or a
/// `TYPE:` value never contains whitespace to begin with -- so preserving
/// whitespace here costs that genuine catch nothing while it stops natural
/// -language prose whose words merely happen to concatenate into a term once
/// whitespace is deleted from being treated as a no-separator join it never
/// was.
fn alnum_normalize_haystack(s: &str) -> String {
    s.to_lowercase().chars().filter(|c| c.is_ascii_alphanumeric() || c.is_whitespace()).collect()
}

/// [`normalized_term_hit`] (word-bounded, OCR-normalized), OR -- if that
/// finds nothing -- an alphanumeric-normalized (no-separator) substring
/// match against [`PI_BLACKLIST_TERMS`], bounded to
/// [`MIN_NORMALIZED_NEEDLE_LEN`] normalized characters, with real whitespace
/// in `value` still acting as a boundary ([`alnum_normalize_haystack`]).
/// Mirrors `pi_scrub.py::blacklist_term_hit_including_concatenated` exactly:
/// this is the check that catches a blacklisted term concatenated
/// PascalCase-style into another token's value with no separator
/// (`AldoriDefensiveParryLVL`, `CalistrianHunter`), which
/// `normalized_term_hit`'s word-boundary requirement alone cannot see.
pub fn blacklist_term_hit_including_concatenated(value: &str) -> Option<&'static str> {
    if value.is_empty() {
        return None;
    }
    if let Some(hit) = normalized_term_hit(value) {
        return Some(hit);
    }
    let norm_value = alnum_normalize_haystack(value);
    if norm_value.trim().is_empty() {
        return None;
    }
    for term in PI_BLACKLIST_TERMS {
        let canon_term = alnum_normalize(term);
        if canon_term.chars().count() >= MIN_NORMALIZED_NEEDLE_LEN && norm_value.contains(&canon_term) {
            return Some(term);
        }
    }
    None
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

/// SD-32 declared-pi-shipping-65-followups: a NARROW, guarded-path
/// metadata-only fix for a record already on disk whose `description` is
/// already the redaction marker but whose `license`/`pi_field` were never
/// stamped to say so (the root cause `classify_field`'s marker-guard above
/// now prevents going forward; this is what un-does the SAME defect on the
/// 99 records it already produced before that guard existed). Every
/// generator here is no-clobber on an existing file, so there is no way to
/// route an already-shipped record back through the writer's normal
/// from-scratch path without either deleting it first (a hand-edit-adjacent
/// operation this repo's doctrine reserves for a real content change, not a
/// metadata correction) or reconciling ONLY the three stamp fields in
/// place, which is what this function computes.
///
/// Returns `None` when there is nothing to fix (no description, description
/// is not the marker, or the stamp is already correct) — the caller's
/// signal to leave the file completely untouched, exactly as its own
/// no-clobber rule already promises for every other case. Returns
/// `Some((license, pi_field, pi_marker))` otherwise: `PiRedacted`, the
/// existing `pi_field` list with `"description"` unioned in (never
/// dropping an existing entry, e.g. a prior `"name"` redaction from
/// `decisions.md §24`), and the standard marker.
pub fn reconcile_description_pi_stamp(
    description: Option<&str>,
    license: License,
    pi_field: Option<&str>,
) -> Option<(License, Option<String>, Option<String>)> {
    if description != Some(REDACTED_PI_MARKER) {
        return None;
    }
    let already_correct =
        license == License::PiRedacted && pi_field.is_some_and(|f| f.split(',').any(|part| part == "description"));
    if already_correct {
        return None;
    }
    let new_pi_field = match pi_field.filter(|f| !f.is_empty()) {
        Some(existing) if existing.split(',').any(|part| part == "description") => existing.to_string(),
        Some(existing) => format!("{existing},description"),
        None => "description".to_string(),
    };
    Some((License::PiRedacted, Some(new_pi_field), Some(PI_MARKER_REDACTED.to_string())))
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- `blacklist_term_hit_including_concatenated` (Python-port scan) ---

    #[test]
    fn concatenated_scan_finds_nothing_in_ordinary_text() {
        assert_eq!(blacklist_term_hit_including_concatenated("Deals 1d6 points of fire damage."), None);
    }

    #[test]
    fn concatenated_scan_catches_an_ordinary_separated_occurrence() {
        // Falls through to `normalized_term_hit` (word-bounded) -- the same
        // shape `classify_field` already catches, proving the two checks
        // agree on the easy case.
        assert_eq!(blacklist_term_hit_including_concatenated("As per Iomedae's blessing"), Some("Iomedae"));
    }

    #[test]
    fn concatenated_scan_catches_a_pascalcase_concatenated_identifier() {
        // `normalized_term_hit` alone cannot see this: "Aldori" is
        // immediately followed by "D" with no boundary. This is the live
        // shape found in `adventurers_guide/class_feature/aldori_defender/
        // defensive_parry.json`'s own `DEFINE` token.
        assert_eq!(blacklist_term_hit_including_concatenated("AldoriDefensiveParryLVL|0"), Some("Aldori"));
        assert_eq!(blacklist_term_hit_including_concatenated("VAR|CalistrianHunterLVL|1"), Some("Calistria"));
    }

    #[test]
    fn concatenated_scan_does_not_over_redact_a_short_term_by_coincidence() {
        // "Nex" (3 normalized chars) is below `MIN_NORMALIZED_NEEDLE_LEN`
        // (6), so the concatenated fallback never fires for it -- only the
        // word-bounded scan can catch "Nex", and that scan already refuses
        // to match inside "next" (no boundary).
        assert_eq!(blacklist_term_hit_including_concatenated("the next round"), None);
    }

    #[test]
    fn concatenated_scan_word_boundary_still_refuses_next_for_nex() {
        assert_eq!(normalized_term_hit("the next round"), None);
    }

    #[test]
    fn concatenated_scan_catches_the_ocr_lrori_variant() {
        assert_eq!(blacklist_term_hit_including_concatenated("Sometimes lrori smiles"), Some("Irori"));
    }

    #[test]
    fn concatenated_scan_jarn_still_catches_a_literal_plain_spelling() {
        assert_eq!(blacklist_term_hit_including_concatenated("an NPC named Jarn appears"), Some("Jarn"));
    }

    #[test]
    fn concatenated_scan_jarn_rn_fold_does_not_catch_an_ordinary_jam() {
        // `decisions.md §26`'s own recorded false-positive class: "jam" is
        // an ordinary word, and "Jarn" is exempted from the rn->m fold so
        // this must NOT match.
        assert_eq!(blacklist_term_hit_including_concatenated("out of a tight jam"), None);
    }

    // ---- t9-onboarding cycle (2026-08-23), corpus_literal_sweep unblock:
    // the "Galt"/"gait" l-fold collision, same class as Jarn/jam above.

    #[test]
    fn concatenated_scan_galt_char_fold_does_not_catch_an_ordinary_gait() {
        // Real reproduction: `advanced_players_guide/class_feature/
        // shifter_s_blessing/form_of_the_cat.json`'s DESC token, pinned
        // oracle `apg_abilities_class.lst:2827`.
        assert_eq!(
            blacklist_term_hit_including_concatenated(
                "his gait more deliberate and graceful"
            ),
            None
        );
    }

    #[test]
    fn concatenated_scan_galt_still_catches_a_literal_plain_spelling() {
        assert_eq!(
            blacklist_term_hit_including_concatenated(
                "The rebels of Galt overthrew their aristocracy"
            ),
            Some("Galt")
        );
    }

    #[test]
    fn word_boundary_alone_does_not_prevent_the_galt_gait_collision() {
        // Proves the negative claim directly, mirroring Jarn/jam's proof:
        // an already-word-bounded scan with NO l-fold exemption still
        // matches "gait".
        let canon_text = canonicalize("his gait more deliberate", false, true);
        let canon_galt_full_fold = canonicalize("Galt", false, true); // "gait"
        assert!(word_bounded_contains(&canon_text, &canon_galt_full_fold));
    }

    // ---- t9-onboarding cycle (2026-08-23): the "Andoran"/whitespace-
    // stripping collision this same cycle found and fixed in
    // `alnum_normalize_haystack`.

    #[test]
    fn concatenated_scan_does_not_manufacture_andoran_across_real_word_boundaries() {
        // Real reproduction: `ultimate_wilderness/class_feature/commando/
        // ranger_trap.json`'s KEY token, pinned oracle
        // `uw_abilities_class.lst:635` (`KEY:Commando ~ Ranger Trap`).
        // "Commando" + "Ranger" concatenate to "...mandoranger..." (containing
        // "andoran") ONLY if real whitespace is stripped before the scan --
        // whitespace-preserving normalization must refuse this.
        assert_eq!(
            blacklist_term_hit_including_concatenated("Commando ~ Ranger Trap"),
            None
        );
    }

    #[test]
    fn concatenated_scan_still_catches_a_genuinely_no_separator_andoran_identifier() {
        // The genuine catch this check exists for must survive: a PCGen
        // variable identifier that concatenates the term with NO real
        // whitespace anywhere.
        assert_eq!(
            blacklist_term_hit_including_concatenated("AndoranCitizenshipLVL"),
            Some("Andoran")
        );
    }

    #[test]
    fn concatenated_scan_empty_value_is_never_a_hit() {
        assert_eq!(blacklist_term_hit_including_concatenated(""), None);
    }

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

    /// SD-32 declared-pi-shipping-65-followups: a value that is ALREADY the
    /// redaction marker (e.g. a static `rules_tables` literal a prior pass
    /// hand-redacted, or a value some upstream step already blanked) must
    /// stamp `PiRedacted`/the field name/the marker -- not fall through the
    /// term scan as ordinary prose. `REDACTED_PI_MARKER` itself ("[redacted
    /// PI]") contains no blacklist term, so before this fix `classify_field`
    /// silently classified an already-redacted value as plain `Ogl` with
    /// `pi_field: None`, shipping the marker text with metadata that claims
    /// nothing was ever redacted -- the exact "description already redacted
    /// but license/pi_field never stamped" shape found live in 99 corpus
    /// records across 9 (book, kind) pairs (`bestiary_4/monster_ability` 65
    /// of them), verified via `cargo run --locked --bin
    /// declared_pi_shipping_audit` plus a corpus-wide re-derivation that
    /// does not depend on the exact source-line declaration.
    #[test]
    fn a_value_already_equal_to_the_marker_stamps_redacted_not_plain_ogl() {
        let (license, pi_field, pi_marker, stored) = classify_field("description", REDACTED_PI_MARKER);
        assert_eq!(license, License::PiRedacted);
        assert_eq!(pi_field.as_deref(), Some("description"));
        assert_eq!(pi_marker.as_deref(), Some(PI_MARKER_REDACTED));
        assert_eq!(stored, REDACTED_PI_MARKER);
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

    // --- `reconcile_description_pi_stamp` (SD-32 declared-pi-shipping-65) --

    #[test]
    fn reconcile_fixes_a_marker_description_shipped_as_plain_ogl() {
        // The exact 65-record `bestiary_4/monster_ability` shape: marker in
        // place, but license/pi_field never stamped.
        let fixed = reconcile_description_pi_stamp(Some(REDACTED_PI_MARKER), License::Ogl, None);
        assert_eq!(
            fixed,
            Some((License::PiRedacted, Some("description".to_string()), Some(PI_MARKER_REDACTED.to_string())))
        );
    }

    #[test]
    fn reconcile_unions_description_into_an_existing_pi_field_list_without_dropping_it() {
        // The 9 `inner_sea_gods/equipment` renamed records: `pi_field:
        // "name"` from the §24 rename, description ALSO the marker but
        // never added to the list.
        let fixed = reconcile_description_pi_stamp(Some(REDACTED_PI_MARKER), License::PiRedacted, Some("name"));
        assert_eq!(
            fixed,
            Some((License::PiRedacted, Some("name,description".to_string()), Some(PI_MARKER_REDACTED.to_string())))
        );
    }

    #[test]
    fn reconcile_is_a_no_op_when_the_stamp_is_already_correct() {
        let fixed =
            reconcile_description_pi_stamp(Some(REDACTED_PI_MARKER), License::PiRedacted, Some("description"));
        assert_eq!(fixed, None, "an already-correct record must be left completely untouched");
    }

    #[test]
    fn reconcile_is_a_no_op_when_the_stamp_already_lists_description_among_others() {
        let fixed = reconcile_description_pi_stamp(
            Some(REDACTED_PI_MARKER),
            License::PiRedacted,
            Some("description,name,raw_tokens"),
        );
        assert_eq!(fixed, None);
    }

    #[test]
    fn reconcile_is_a_no_op_for_an_ordinary_unredacted_description() {
        let fixed = reconcile_description_pi_stamp(Some("Deals 1d6 points of fire damage."), License::Ogl, None);
        assert_eq!(fixed, None, "must never touch a record with no marker to reconcile");
    }

    #[test]
    fn reconcile_is_a_no_op_for_an_absent_description() {
        let fixed = reconcile_description_pi_stamp(None, License::Ogl, None);
        assert_eq!(fixed, None);
    }
}
