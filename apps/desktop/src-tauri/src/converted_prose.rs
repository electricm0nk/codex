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
//! `src/pcgen_import/sheet_rule/`. The converted [`SheetRule`](codex::rules_core::sheet_rule::SheetRule)
//! carries the record's words with **typed** holes, and
//! [`codex::rules_core::sheet_rule_catalog::catalog_description`] renders them with no character
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
//!    [`codex::rules_core::sheet_rule_catalog::catalog_description_by_name`], whose lookup is
//!    [`SheetRulePackage::find`](codex::rules_core::sheet_rule::SheetRulePackage::find) —
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
//! **There is no step past these.** A key the package does not hold under any of the four serves
//! `None` — never a guessed, partial or neighbouring record's text.
//!
//! # What is NOT in here
//!
//! No token, no formula string, no argument tail, no positional placeholder, no ingest-format
//! vocabulary of any kind. The input is a book directory, a kind and a record key; the output is
//! English.

use codex::rules_core::corpus_loader::live_sheet_rules;
use codex::rules_core::sheet_rule::{slug, SheetRulePackage};
use codex::rules_core::sheet_rule_catalog::{catalog_description, catalog_description_by_name};
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
    let package = package()?;
    let rule = package.rule(&converted_id(book_dir, kind, key))?;
    catalog_description(package, rule)
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
    let token = source_row_suffix(key)?;
    let book = if book_dir == "beastiary" { "bestiary" } else { book_dir };
    let package = package()?;
    let suffixes = source_row_index().get(&(book.to_string(), kind.to_string()))?;
    let id = suffixes
        .iter()
        .find(|(suffix, _)| token.ends_with(suffix.as_str()))
        .map(|(_, id)| id)?;
    catalog_description(package, package.rule(id)?)
}

/// The description a catalog row serves, through the join in the module doc.
///
/// `None` is the honest answer for a row the package holds under none of the steps, and for a
/// record whose converted rule states a stat block and no descriptive prose at all.
pub fn description_for(book_dir: &str, kind: &str, key: &str) -> Option<String> {
    if let Some(text) = description_in_book(book_dir, kind, key) {
        return Some(text);
    }
    if let Some(text) = description_by_source_row(book_dir, kind, key) {
        return Some(text);
    }
    if let Some(text) = description_by_name(kind, key) {
        return Some(text);
    }
    let base = base_name(key)?;
    if let Some(text) = description_in_book(book_dir, kind, base) {
        return Some(text);
    }
    description_by_name(kind, base)
}

/// Step 3 alone: the package-wide by-name resolution, exactly as the character sheet performs
/// it.
pub fn description_by_name(kind: &str, key: &str) -> Option<String> {
    let package = package()?;
    catalog_description_by_name(package, kind, key)
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

    /// A restricted printing serves the record it restricts; a key nothing holds still
    /// serves nothing.
    #[test]
    fn a_restricted_printing_serves_its_base_record_and_an_unheld_key_serves_nothing() {
        let base = description_for("core_rulebook", "spell", "Nondetection")
            .expect("core_rulebook:spell:nondetection states no prose");
        assert_eq!(
            description_for("core_rulebook", "spell", "Nondetection (self only)").as_deref(),
            Some(base.as_str())
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
