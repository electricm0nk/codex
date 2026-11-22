//! The one place a catalog screen in this crate gets a record's words.
//!
//! # Why this module exists
//!
//! SD-35 `decisions.md §11` — "there should be nothing left of pcgen" on the live side. Before
//! this module, every catalog in this crate held the same shape: the compiled table carried the
//! ingest format's own description string, and the catalog parsed that string at run time on
//! the way to the screen. That is an ingest-format reader sitting in the code that prints a
//! character sheet, which the ruling forbids.
//!
//! The substitution those call sites performed already happens at ingest, in
//! `src/pcgen_import/sheet_rule/`. The converted [`SheetRule`](crate::rules_core::sheet_rule::SheetRule)
//! carries the record's words with **typed** holes, and
//! [`crate::rules_core::sheet_rule_catalog::catalog_description`] renders them with no character
//! in hand — a final number where the term is settled, the rule's own words where it is not
//! (`decisions.md §1`'s three permitted printed forms). This module is the join from a catalog
//! row to that record, and nothing else.
//!
//! # The join, and why it has four steps
//!
//! A catalog row names a book and a record key. The converted package addresses a rule by
//! `"<book>:<kind>:<slug>"`. So:
//!
//! 1. **Book-exact.** [`description_in_book`] builds that id from the row's own book directory
//!    and key and asks the package for it. This is the honest join: same book, same record.
//! 2. **The source row, for a key that is not a name at all.** A corpus row whose record
//!    carries no name reaches the compiled tables as
//!    `Codex-Named Unit (spell_inner_sea_world_guide_iswg_spells_lst_9)`; the converter resolves
//!    the same row's real name from the book's text (`Gorum's Armor`). Those rows can never join
//!    by name in either direction, so [`description_by_source_row`] joins them on the source
//!    file and line **both sides record** — the compiled key's own tail against the converted
//!    rule's `provenance.closure_rows`. A suffix two rules share resolves to neither.
//! 3. **By name, package-wide.** When steps 1 and 2 miss, [`description_for`] falls back to
//!    [`crate::rules_core::sheet_rule_catalog::catalog_description_by_name`], whose lookup is
//!    [`SheetRulePackage::find`](crate::rules_core::sheet_rule::SheetRulePackage::find) —
//!    `core_rulebook` first, then the lexicographically first book id. This is **the same
//!    resolution the character sheet itself performs** for a name several printings share; it is
//!    not a fuzzy matcher, and it never matches a slug the package does not literally hold.
//! 4. **The printed qualifier, dropped — [`base_name`].** A compiled table carries one row per
//!    *printed variant*; the converter writes one record per *record*. Core Rulebook's domain
//!    spell lists print `Nondetection (self only)`, `Planar Binding (Demons Only)`,
//!    `Speak with Animals (rodents only)`: the same spell, restricted by the domain that grants
//!    it. Those rows' own corpus description **is** the base spell's, which is why they read
//!    correctly before this module existed. Step 4 drops one trailing parenthesised group and
//!    retries steps 1 and 3 with what is left.
//!
//! Step 3 exists because a compiled table's book is sometimes a *printing* rather than a
//! converted book directory: `ultimate_magic_wordsofpower`'s three rows are Ultimate Magic
//! records, and `data/sheet_rules/` has no `ultimate_magic_wordsofpower/` directory at all. A
//! reprint resolving to the record it reprints is right; a row resolving to nothing is a
//! description a player used to be able to read and now cannot.
//!
//! **Step 4 is the bounded matcher this codebase's standing lesson permits, and nothing wider.**
//! *"A shared name never implies a shared thing — a fuzzy matcher belongs, if built at all, in
//! the caller, verified per-match against the owning record"* (`feat_catalog.rs`). This one is in
//! the caller's own crate, it is not fuzzy (one exact structural transformation, not a distance
//! metric), and every match is verified against the owning record because the retry is the same
//! exact-id/exact-slug lookup as steps 1 and 3 — a base name the package does not literally hold
//! resolves to nothing. It never strips more than one group, never strips a leading or interior
//! one, and never strips when nothing remains.
//!
//! 5. **A key two records share — [`description_of_the_one_disambiguated_variant`].** When two
//!    corpus records carry the same key the converter cannot write both to the same id, so it
//!    writes `<slug>__<hash>` for each and the bare slug holds nothing. Step 5 resolves such a
//!    key **only when exactly one of the variants states prose**; two that both state prose are
//!    two different rules and resolve to neither. Added in SD-35 `AT-35-E6-003-SWEEP` cycle 17,
//!    where it was the whole reason `Wizard ~ Spells` and `Master Of Many Styles ~ Perfect
//!    Style` had no description at all.
//!
//! **There is no step past these.** A key the package does not hold under any of the five serves
//! `None` — never a guessed, partial or neighbouring record's text.
//!
//! # What is NOT in here
//!
//! No token, no formula string, no argument tail, no positional placeholder, no ingest-format
//! vocabulary of any kind. The input is a book directory, a kind and a record key; the output is
//! English.

use crate::rules_core::corpus_loader::live_sheet_rules;
use crate::rules_core::sheet_rule::{slug, SheetRulePackage};
use crate::rules_core::sheet_rule_catalog::{catalog_description, catalog_description_by_name};
use std::collections::BTreeMap;

/// The converted package this crate serves from, or `None` when
/// `data/sheet_rules/` is absent from the build (the loader resolves it from the library
/// crate's own manifest directory).
pub fn package() -> Option<&'static SheetRulePackage> {
    live_sheet_rules()
}

/// The converted rule id for a record: `"<book>:<kind>:<slug of key>"`.
///
/// `beastiary` is the corpus directory's historical spelling of the book the converter writes as
/// `bestiary` — the same one-line fold `class_feature_descriptions::converted_id` and
/// `companion_pool_catalog::converted_id` already carry.
pub fn converted_id(book_dir: &str, kind: &str, key: &str) -> String {
    let book = if book_dir == "beastiary" { "bestiary" } else { book_dir };
    format!("{book}:{kind}:{}", slug(key))
}

/// Step 1 of the join: the description of the `kind` record `key` **in `book_dir`**, or `None`
/// when that book holds no such record, or holds it with no descriptive prose.
pub fn description_in_book(book_dir: &str, kind: &str, key: &str) -> Option<String> {
    let id = id_in_book(book_dir, kind, key)?;
    catalog_description(package()?, package()?.rule(&id)?)
}

/// [`description_in_book`]'s own answer, as the rule id it resolved rather than the words.
///
/// Each step of the join has one of these. They exist because two callers ask two different
/// questions of the same join: a catalog screen wants the record's words, and
/// `pilot_compute::class_feature_grant_consumer` wants to know whether the converter settled
/// **every** term in them without a character in hand. Both must resolve the same record or
/// they are two joins, so the id form is the join and the description form is a view of it.
fn id_in_book(book_dir: &str, kind: &str, key: &str) -> Option<String> {
    let package = package()?;
    let id = converted_id(book_dir, kind, key);
    let rule = package.rule(&id)?;
    catalog_description(package, rule)?;
    Some(id)
}

/// Step 4's transformation: `"Nondetection (self only)"` → `Some("Nondetection")`.
///
/// Drops **one** trailing parenthesised group and the whitespace before it, and only when the
/// key genuinely ends in one and something is left over. `None` for every other shape — a key
/// with no group, a group that is not last, a key that is nothing but a group
/// (`"Codex-Named Unit (spell_isg_spells_lst_7)"` keeps its own identity and is correctly left
/// alone, because the part before the group is a placeholder the package does not hold either).
pub fn base_name(key: &str) -> Option<&str> {
    let trimmed = key.trim_end();
    if !trimmed.ends_with(')') {
        return None;
    }
    let open = trimmed.rfind(" (")?;
    let base = trimmed[..open].trim_end();
    if base.is_empty() {
        None
    } else {
        Some(base)
    }
}

/// The literal prefix of a compiled table's placeholder key — see [`source_row_suffix`].
const PLACEHOLDER_PREFIX: &str = "Codex-Named Unit (";

/// Step 2's key: the `<file stem>_lst_<line>` tail of a compiled table's **placeholder** key.
///
/// A corpus row whose own record carries no name reaches the compiled tables as
/// `Codex-Named Unit (spell_inner_sea_world_guide_iswg_spells_lst_9)` — the kind, the book, the
/// source file and the line, because that is all the transcription had to identify it with. The
/// converter resolves the same row's real name from the book's text (`Gorum's Armor`), so these
/// rows are exactly the population that can never join by name, in either direction.
///
/// They can join on **where they came from**, which both sides record: this returns
/// `iswg_spells_lst_9`, and a converted rule's `provenance.closure_rows` entry
/// `pathfinder/.../iswg_spells.lst:9` normalises to the same string
/// ([`closure_row_suffix`]). `None` for any key that is not a placeholder.
fn source_row_suffix(key: &str) -> Option<&str> {
    let inner = key.strip_prefix(PLACEHOLDER_PREFIX)?.strip_suffix(')')?;
    if inner.is_empty() {
        None
    } else {
        Some(inner)
    }
}

/// A `provenance.closure_rows` entry, normalised to the tail a placeholder key carries:
/// `pathfinder/.../iswg_spells.lst:9` → `iswg_spells_lst_9`.
fn closure_row_suffix(row: &str) -> Option<String> {
    let (path, line) = row.rsplit_once(':')?;
    if line.is_empty() || !line.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let file = path.rsplit('/').next()?;
    if file.is_empty() {
        return None;
    }
    Some(format!("{}_{line}", file.replace('.', "_")))
}

/// `(book, kind)` → every `(source-row suffix, rule id)` the package holds for it, with the
/// suffixes two or more rules share dropped.
///
/// Built once per process from `provenance.closure_rows`. Dropping an ambiguous suffix is the
/// point: a placeholder row that could be either of two records resolves to neither, rather than
/// to whichever one sorted first.
fn source_row_index() -> &'static BTreeMap<(String, String), BTreeMap<String, String>> {
    static INDEX: std::sync::OnceLock<BTreeMap<(String, String), BTreeMap<String, String>>> =
        std::sync::OnceLock::new();
    INDEX.get_or_init(|| {
        let mut ambiguous: BTreeMap<(String, String), std::collections::BTreeSet<String>> =
            BTreeMap::new();
        let mut index: BTreeMap<(String, String), BTreeMap<String, String>> = BTreeMap::new();
        let Some(package) = package() else { return index };
        for (id, rule) in &package.rules {
            let book_kind = (rule.provenance.book.clone(), rule.provenance.kind.clone());
            for row in &rule.provenance.closure_rows {
                let Some(suffix) = closure_row_suffix(row) else { continue };
                let slot = index.entry(book_kind.clone()).or_default();
                match slot.get(&suffix) {
                    Some(existing) if existing == id => {}
                    Some(_) => {
                        ambiguous.entry(book_kind.clone()).or_default().insert(suffix);
                    }
                    None => {
                        slot.insert(suffix, id.clone());
                    }
                }
            }
        }
        for (book_kind, suffixes) in ambiguous {
            if let Some(slot) = index.get_mut(&book_kind) {
                for suffix in suffixes {
                    slot.remove(&suffix);
                }
            }
        }
        index
    })
}

/// Step 2 of the join: the rule a compiled table's placeholder key names, matched on the source
/// row both sides record. `None` for a key that is not a placeholder, and for a placeholder
/// whose source row the package holds under no rule or under more than one.
pub fn description_by_source_row(book_dir: &str, kind: &str, key: &str) -> Option<String> {
    let id = id_by_source_row(book_dir, kind, key)?;
    catalog_description(package()?, package()?.rule(&id)?)
}

/// [`description_by_source_row`]'s own answer, as the rule id it resolved.
fn id_by_source_row(book_dir: &str, kind: &str, key: &str) -> Option<String> {
    let token = source_row_suffix(key)?;
    let book = if book_dir == "beastiary" { "bestiary" } else { book_dir };
    let package = package()?;
    let suffixes = source_row_index().get(&(book.to_string(), kind.to_string()))?;
    let id = suffixes
        .iter()
        .find(|(suffix, _)| token.ends_with(suffix.as_str()))
        .map(|(_, id)| id)?;
    catalog_description(package, package.rule(id)?)?;
    Some(id.clone())
}

/// The description a catalog row serves, through the join in the module doc.
///
/// `None` is the honest answer for a row the package holds under none of the steps, and for a
/// record whose converted rule states a stat block and no descriptive prose at all.
pub fn description_for(book_dir: &str, kind: &str, key: &str) -> Option<String> {
    let package = package()?;
    let id = rule_id_for(book_dir, kind, key)?;
    catalog_description(package, package.rule(&id)?)
}

/// The whole join, as the rule id it resolves — the five steps of the module doc, in order.
///
/// A step that resolves a rule stating **no** prose is not an answer; the walk continues, which
/// is exactly what the description form did when `catalog_description` returned `None` at a
/// step. [`description_for`] is this plus one `catalog_description` call.
pub fn rule_id_for(book_dir: &str, kind: &str, key: &str) -> Option<String> {
    if let Some(id) = id_in_book(book_dir, kind, key) {
        return Some(id);
    }
    if let Some(id) = id_by_source_row(book_dir, kind, key) {
        return Some(id);
    }
    if let Some(id) = id_by_name(kind, key) {
        return Some(id);
    }
    if let Some(base) = base_name(key) {
        if let Some(id) = id_in_book(book_dir, kind, base) {
            return Some(id);
        }
        if let Some(id) = id_by_name(kind, base) {
            return Some(id);
        }
    }
    id_of_the_one_disambiguated_variant(book_dir, kind, key)
}

/// The record's words **only when the converter settled every term in them** with no character
/// in hand — `None` when the join misses, when the record states no prose, and when the prose
/// carries a hole that stands on a character term.
///
/// SD-35 `AT-35-E6-003-SWEEP` cycle 17. `pilot_compute::class_feature_grant_consumer` admits a
/// grant fact on the strength of the sheet's Class Features section already being able to print
/// the record's sentence *without this character*. A sentence with an unsettled hole does not
/// meet that bar — it is the right thing to print on a catalog screen, where there is no
/// character at all, and the wrong thing to rely on when a character IS in hand and the
/// interpreter can state the real number. `[`description_for`]` answers the first question and
/// this answers the second; conflating them silently replaced 32 resolved, per-character
/// sentences with their unsettled form.
pub fn settled_description_for(book_dir: &str, kind: &str, key: &str) -> Option<String> {
    let package = package()?;
    let id = rule_id_for(book_dir, kind, key)?;
    let rule = package.rule(&id)?;
    if crate::rules_core::sheet_rule_catalog::prose_has_a_slot_no_character_settles(rule) {
        return None;
    }
    catalog_description(package, rule)
}

/// Step 5 of the join: a key two corpus records share, which the converter writes as
/// `<slug>__<hash>` rules and never under the bare slug.
///
/// The corpus holds two `core_rulebook` `class_feature` records both keyed `Wizard ~ Spells`,
/// and two `ultimate_combat` records both keyed `Master Of Many Styles ~ Perfect Style`. The
/// converter cannot write both to `…:class_feature:wizard_spells`, so it disambiguates each
/// with a content hash; the package then holds **no** rule at the bare slug and steps 1–4 all
/// miss. Before cycle 17 those rows lost their description entirely.
///
/// **It resolves only when exactly one variant states prose.** In both live cases the collision
/// is a record and its own token-only twin: one variant carries the book's sentence and the
/// other carries structure and no words at all, so "the one that has words" names a single
/// record and not a guess. When two variants both state prose they are two different rules and
/// this returns `None` — the same refusal [`description_by_source_row`] applies to an ambiguous
/// source row, and for the same reason. It never reaches outside the row's own book and kind,
/// and never matches a slug the package does not literally hold under the `__` form.
fn id_of_the_one_disambiguated_variant(book_dir: &str, kind: &str, key: &str) -> Option<String> {
    let package = package()?;
    let bare = converted_id(book_dir, kind, key);
    if package.rule(&bare).is_some() {
        return None;
    }
    let prefix = format!("{bare}__");
    let mut found: Option<String> = None;
    for (id, rule) in &package.rules {
        if !id.starts_with(&prefix) {
            continue;
        }
        if catalog_description(package, rule).is_none() {
            continue;
        }
        if found.is_some() {
            return None;
        }
        found = Some(id.clone());
    }
    found
}

/// Step 3 alone: the package-wide by-name resolution, exactly as the character sheet performs
/// it.
pub fn description_by_name(kind: &str, key: &str) -> Option<String> {
    let package = package()?;
    catalog_description_by_name(package, kind, key)
}

/// [`description_by_name`]'s own answer, as the rule id it resolved.
fn id_by_name(kind: &str, key: &str) -> Option<String> {
    let package = package()?;
    let id = package.find(kind, &slug(key))?;
    catalog_description(package, package.rule(id)?)?;
    Some(id.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The package has to be loadable at all, or every catalog in this crate serves `None` and
    /// every population ratchet below reads zero for a reason that has nothing to do with the
    /// records. Fail loudly here rather than silently everywhere else.
    #[test]
    fn the_converted_package_loads_and_holds_rules() {
        let package = package().expect(
            "data/sheet_rules/ did not load -- every catalog in this crate serves its \
             descriptions from it",
        );
        assert!(
            package.rules.len() > 60_000,
            "converted package holds only {} rules",
            package.rules.len()
        );
    }

    /// The id shape is the package's own, proved against a record the package really holds
    /// rather than asserted.
    #[test]
    fn a_book_exact_id_resolves_a_real_record() {
        let package = package().unwrap();
        let id = converted_id("core_rulebook", "spell", "Wall of Thorns");
        assert_eq!(id, "core_rulebook:spell:wall_of_thorns");
        assert!(package.rule(&id).is_some(), "{id} is not in the package");
    }

    /// `beastiary` is folded; nothing else is rewritten.
    #[test]
    fn the_bestiary_spelling_is_the_only_book_fold() {
        assert_eq!(converted_id("beastiary", "monster", "Goblin"), "bestiary:monster:goblin");
        assert_eq!(
            converted_id("inner_sea_gods", "spell", "Goblin"),
            "inner_sea_gods:spell:goblin"
        );
    }

    /// Step 4 drops one trailing printed qualifier, and nothing else.
    #[test]
    fn the_printed_qualifier_step_is_exactly_one_trailing_group() {
        assert_eq!(base_name("Nondetection (self only)"), Some("Nondetection"));
        assert_eq!(base_name("Planar Binding (Demons Only)"), Some("Planar Binding"));
        // Two groups: one is dropped, not both.
        assert_eq!(
            base_name("Plane Shift (self only/to Shadow or Material Plane)"),
            Some("Plane Shift")
        );
        assert_eq!(base_name("Summon Monster III (lantern archon only)"), Some("Summon Monster III"));
        // No trailing group, an interior one, and a key that is nothing but a group.
        assert_eq!(base_name("Wall of Thorns"), None);
        assert_eq!(base_name("Special Ability ~ Amorphous ~ Armor"), None);
        assert_eq!(base_name("(self only)"), None);
    }

    /// A restricted printing the package holds as its own record serves **that** record; one the
    /// package does not hold falls back to the record it restricts; a key nothing holds still
    /// serves nothing.
    ///
    /// **Corrected in SD-35 `AT-35-E6-003-SWEEP` cycle 17, against the package rather than
    /// against this test's memory of it.** The assertion used to be that
    /// `Nondetection (self only)` serves `Nondetection`'s words. It does not, and should not:
    /// the converter writes `core_rulebook:spell:nondetection_self_only` as a record of its own,
    /// so step 1 of the join answers directly and the restricted printing states the restricted
    /// text. Step 4 is still load-bearing — `Planar Binding (Demons Only)` has no record of its
    /// own — and is what this now exercises. Re-derive which printings the package holds:
    /// `ls data/sheet_rules/core_rulebook/spell/ | grep -E 'nondetection|planar_binding'`.
    #[test]
    fn a_restricted_printing_serves_its_own_record_then_its_base_and_an_unheld_key_serves_nothing()
    {
        // Held as its own record: step 1, not step 4.
        let restricted = description_for("core_rulebook", "spell", "Nondetection (self only)")
            .expect("core_rulebook:spell:nondetection_self_only states no prose");
        let base = description_for("core_rulebook", "spell", "Nondetection")
            .expect("core_rulebook:spell:nondetection states no prose");
        assert_ne!(
            restricted, base,
            "the package holds the restricted printing as its own record; serving the base \
             record's words for it would throw the restriction away"
        );

        // A printed qualifier the package holds under no book at all: steps 1–3 miss and step 4
        // drops the group, serving the record the printing restricts. `Planar Binding (Demons
        // Only)` is deliberately NOT the example — the package holds it, in
        // `advanced_players_guide`, so step 3 answers it and step 4 never runs.
        assert_eq!(
            description_for("core_rulebook", "spell", "Nondetection (while aboard a ship)")
                .as_deref(),
            Some(base.as_str()),
            "step 4 must drop a qualifier the package holds nowhere and serve the base record"
        );

        assert_eq!(
            description_for("core_rulebook", "spell", "Not A Real Spell (self only)"),
            None
        );
    }

    /// The fallback resolves a reprint to the record it reprints, and refuses a name the
    /// package does not hold.
    #[test]
    fn the_by_name_fallback_resolves_a_reprint_and_refuses_an_unheld_name() {
        assert!(
            description_for("ultimate_magic_wordsofpower", "spell", "Burst Fire Blast").is_some(),
            "the Words of Power printing of an Ultimate Magic spell lost its description"
        );
        assert_eq!(
            description_for("core_rulebook", "spell", "Not A Real Spell Name At All"),
            None
        );
    }
}
