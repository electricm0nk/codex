//! The merged, corpus-wide class-id census: every distinct class id the
//! engine's registries and the converted corpus know about, in one place.
//!
//! # Why this exists
//!
//! `docs/architecture/status.md`'s "Class/level compute coverage" table
//! (measured 2026-09-20) was built by a **temporary** integration test
//! (`tests/zz_class_census.rs`) that merged eight registries into one
//! `BTreeMap<slug, Row>`, ran the sweep, and was then deleted -- "no
//! committed binary keeps a merged corpus-wide class census live day to
//! day," as that doc's own words put it. Epic F (`docs/release/
//! SD-36-consolidation/epic-f-class-completion.md` §2, batch F0) makes that
//! merge a **permanent** instrument instead of a one-time throwaway, so the
//! next time this number needs re-deriving nobody has to reconstruct the
//! merge from prose. This module is F0a: the merged registry alone --
//! no sweep, no `--json` output, no CLI (`src/bin/class_census.rs` and the
//! sweep-and-compute half are later F0 steps).
//!
//! # The eight raw sources, and the ninth that never adds a new family
//!
//! Every class id comes from exactly one of these eight raw registries,
//! claimed in this precedence order (see [`ClassFamily`]):
//!
//! 1. [`ClassId::ALL`](crate::rules_core::rules_tables::crb::class_tables::ClassId::ALL) (CRB, `core_rulebook`)
//! 2. `ApgClassId::ALL` (APG, `advanced_players_guide`)
//! 3. `AcgClassId::ALL` (ACG, `advanced_class_guide`)
//! 4. `PuClassId::ALL` (Pathfinder Unchained, `pathfinder_unchained`)
//! 5. `UcClassId::ALL` (Ultimate Combat, `ultimate_combat`)
//! 6. `untabled_base_class_chassis::untabled_base_class_registry()` (untabled exotic base classes)
//! 7. `crb_untabled_class_chassis::covered_classes()` (CRB NPC / `Ex-*` classes)
//! 8. every converted class record, corpus-wide, whose principal rule's
//!    `tags` carry `"Prestige"` (Prestige) -- read through
//!    [`class_chassis_sheet_rules::records`], the same reader
//!    `generic_class_chassis` itself uses, over every real book directory
//!    `data/sheet_rules` actually holds (never a hand-typed book list; see
//!    [`prestige_scan_books`]).
//!
//! These eight are pairwise disjoint by construction: a converted record's
//! `TYPE:` heads make it exactly one of Base.PC (tabled), Base.NPC, `Ex-*`,
//! or Prestige, never two -- `no_class_id_sits_in_two_families` (below)
//! proves that holds for the current corpus rather than assuming it.
//!
//! `generic_class_chassis`'s own 78-class population (`covered_classes()`,
//! added by this batch) is a **ninth** source, folded in last. It never
//! introduces a new family: every id it names is already claimed by one of
//! the eight above (`docs/architecture/status.md`'s own evidence: 56 of its
//! 78 are Prestige rows, the other 22 are the Ultimate Combat classes and
//! 19 of the 20 untabled-exotic classes, each also reachable through
//! `generic_class_chassis`'s own 14-book population). Folding it in only
//! widens an existing entry's [`ClassCensusEntry::registries`] and
//! [`ClassCensusEntry::books`]. If a future corpus change makes it name an
//! id none of the eight claims, that id is recorded under
//! [`ClassFamily::GenericOnly`] rather than silently guessed into one of
//! the other eight -- `no_generic_only_stragglers_today` pins that this
//! is currently empty, so a regression is caught by name.

use std::collections::BTreeMap;
#[cfg(test)]
use std::collections::BTreeSet;

use crate::rules_core::pilot_compute::class_chassis_sheet_rules;
use crate::rules_core::pilot_compute::crb_untabled_class_chassis;
use crate::rules_core::pilot_compute::generic_class_chassis_covered_classes;
use crate::rules_core::pilot_compute::untabled_base_class_chassis::untabled_base_class_registry;
use crate::rules_core::rules_tables::acg::AcgClassId;
use crate::rules_core::rules_tables::apg::ApgClassId;
use crate::rules_core::rules_tables::crb::class_tables::ClassId;
use crate::rules_core::rules_tables::pathfinder_unchained::class_chassis::PuClassId;
use crate::rules_core::rules_tables::ultimate_combat::UcClassId;
use crate::support::paths::repo_root;

/// PF1's own character-level cap -- the ceiling every fully tabled base
/// class (CRB/APG/ACG/Pathfinder Unchained/Ultimate Combat) reads to, and
/// the same constant `docs/architecture/status.md`'s own evidence line
/// cites (`v06_class_state_dump` -> `max_level=20`).
const DEFAULT_TABLED_MAX_LEVEL: u8 = 20;

/// The partition family a census entry belongs to. A strict partition:
/// every class id is in exactly one family (`no_class_id_sits_in_two_families`).
/// Mirrors `docs/architecture/status.md`'s "Per-family breakdown" table
/// row for row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClassFamily {
    Crb,
    Apg,
    Acg,
    PathfinderUnchained,
    UltimateCombat,
    UntabledExoticBase,
    CrbNpcEx,
    Prestige,
    /// Never populated today (`no_generic_only_stragglers_today`) -- the
    /// escape hatch for a class `generic_class_chassis` names that none of
    /// the eight canonical sources claims, so such a class is recorded
    /// honestly instead of guessed into one of the other eight families.
    GenericOnly,
}

impl ClassFamily {
    pub fn label(self) -> &'static str {
        match self {
            ClassFamily::Crb => "CRB",
            ClassFamily::Apg => "APG",
            ClassFamily::Acg => "ACG",
            ClassFamily::PathfinderUnchained => "Pathfinder Unchained",
            ClassFamily::UltimateCombat => "Ultimate Combat",
            ClassFamily::UntabledExoticBase => "Untabled exotic base classes",
            ClassFamily::CrbNpcEx => "CRB NPC / Ex-* classes",
            ClassFamily::Prestige => "Prestige",
            ClassFamily::GenericOnly => "generic_class_chassis-only (unclaimed by any of the eight canonical sources)",
        }
    }
}

/// One distinct class id's merged census row.
#[derive(Debug, Clone)]
pub struct ClassCensusEntry {
    pub class_id: String,
    pub family: ClassFamily,
    /// Every book this id's chassis was actually found in, across every
    /// registry that named it (deduplicated, first-seen order).
    pub books: Vec<String>,
    /// Every raw source that named this id (deduplicated, first-seen
    /// order) -- how `every_registry_is_swept_once` shows overlaps.
    pub registries: Vec<&'static str>,
    pub max_level: u8,
    pub is_prestige: bool,
}

fn add_unique(list: &mut Vec<String>, value: &str) {
    if !list.iter().any(|v| v == value) {
        list.push(value.to_string());
    }
}

fn add_registry(list: &mut Vec<&'static str>, value: &'static str) {
    if !list.contains(&value) {
        list.push(value);
    }
}

/// Every real book directory under `data/sheet_rules`, sorted -- never a
/// hand-typed list. Reuses [`crate::rules_core::settled_corpus::corpus_book_dirs`],
/// the same dynamic directory-walk `data/corpus`'s own authoring-time
/// generator and whole-corpus parity proofs already use, pointed at
/// `data/sheet_rules` instead: both are "every real book directory under a
/// corpus root, skipping `_`-prefixed bookkeeping directories," which is
/// exactly what a corpus-wide Prestige-tag sweep needs.
pub fn prestige_scan_books() -> Vec<String> {
    let root = repo_root().join("data/sheet_rules");
    let mut books: Vec<String> = crate::rules_core::settled_corpus::corpus_book_dirs(&root)
        .into_iter()
        .filter_map(|path| path.file_name().map(|name| name.to_string_lossy().into_owned()))
        .collect();
    books.sort();
    books
}

/// One raw source's claimed ids, keyed by class id, for the overlap check
/// `no_class_id_sits_in_two_families` runs before any precedence rule
/// papers over a real collision. Test-only: `census()` itself never needs
/// the raw, pre-merge sets, only `no_class_id_sits_in_two_families` does.
#[cfg(test)]
type RawClaims = BTreeMap<&'static str, BTreeSet<String>>;

#[cfg(test)]
fn raw_claims() -> RawClaims {
    let mut raw: RawClaims = BTreeMap::new();

    let crb: BTreeSet<String> = ClassId::ALL
        .iter()
        .map(|id| format!("class:{}", format!("{id:?}").to_lowercase()))
        .collect();
    raw.insert("crb_class_id", crb);

    let apg: BTreeSet<String> =
        ApgClassId::ALL.iter().map(|id| format!("class:{}", id.name())).collect();
    raw.insert("apg_class_id", apg);

    let acg: BTreeSet<String> =
        AcgClassId::ALL.iter().map(|id| format!("class:{}", id.name())).collect();
    raw.insert("acg_class_id", acg);

    let pu: BTreeSet<String> =
        PuClassId::ALL.iter().map(|id| format!("class:{}", id.name())).collect();
    raw.insert("pu_class_id", pu);

    let uc: BTreeSet<String> =
        UcClassId::ALL.iter().map(|id| format!("class:{}", id.name())).collect();
    raw.insert("uc_class_id", uc);

    let untabled: BTreeSet<String> =
        untabled_base_class_registry().iter().map(|meta| meta.class_id.clone()).collect();
    raw.insert("untabled_base_class_registry", untabled);

    let crb_npc_ex: BTreeSet<String> =
        crb_untabled_class_chassis::covered_classes().into_iter().map(|meta| meta.class_id).collect();
    raw.insert("crb_untabled_class_chassis", crb_npc_ex);

    let books = prestige_scan_books();
    let book_refs: Vec<&str> = books.iter().map(String::as_str).collect();
    let prestige: BTreeSet<String> = class_chassis_sheet_rules::records(&book_refs)
        .into_iter()
        .filter(|(_, chassis)| chassis.tags.iter().any(|t| t == "Prestige"))
        .map(|((_, slug), _)| format!("class:{slug}"))
        .collect();
    raw.insert("prestige_tagged_record", prestige);

    raw
}

/// Every distinct class id known to the engine, merged from the eight raw
/// registries plus `generic_class_chassis`'s own population. See this
/// module's doc comment for the precedence rule and why the ninth source
/// never introduces a new family (short of the honestly-named
/// `ClassFamily::GenericOnly` escape hatch).
pub fn census() -> BTreeMap<String, ClassCensusEntry> {
    let mut entries: BTreeMap<String, ClassCensusEntry> = BTreeMap::new();

    let claim = |entries: &mut BTreeMap<String, ClassCensusEntry>,
                 class_id: String,
                 family: ClassFamily,
                 book: &str,
                 registry: &'static str,
                 max_level: u8,
                 is_prestige: bool| {
        entries
            .entry(class_id.clone())
            .and_modify(|e| {
                add_unique(&mut e.books, book);
                add_registry(&mut e.registries, registry);
            })
            .or_insert_with(|| ClassCensusEntry {
                class_id,
                family,
                books: vec![book.to_string()],
                registries: vec![registry],
                max_level,
                is_prestige,
            });
    };

    for id in ClassId::ALL {
        let class_id = format!("class:{}", format!("{id:?}").to_lowercase());
        claim(&mut entries, class_id, ClassFamily::Crb, "core_rulebook", "crb_class_id", DEFAULT_TABLED_MAX_LEVEL, false);
    }
    for id in ApgClassId::ALL {
        let class_id = format!("class:{}", id.name());
        claim(&mut entries, class_id, ClassFamily::Apg, "advanced_players_guide", "apg_class_id", DEFAULT_TABLED_MAX_LEVEL, false);
    }
    for id in AcgClassId::ALL {
        let class_id = format!("class:{}", id.name());
        claim(&mut entries, class_id, ClassFamily::Acg, "advanced_class_guide", "acg_class_id", DEFAULT_TABLED_MAX_LEVEL, false);
    }
    for id in PuClassId::ALL {
        let class_id = format!("class:{}", id.name());
        claim(&mut entries, class_id, ClassFamily::PathfinderUnchained, "pathfinder_unchained", "pu_class_id", DEFAULT_TABLED_MAX_LEVEL, false);
    }
    for id in UcClassId::ALL {
        let class_id = format!("class:{}", id.name());
        claim(&mut entries, class_id, ClassFamily::UltimateCombat, "ultimate_combat", "uc_class_id", DEFAULT_TABLED_MAX_LEVEL, false);
    }
    for meta in untabled_base_class_registry() {
        claim(
            &mut entries,
            meta.class_id.clone(),
            ClassFamily::UntabledExoticBase,
            &meta.source_book,
            "untabled_base_class_registry",
            meta.max_level,
            false,
        );
    }
    for meta in crb_untabled_class_chassis::covered_classes() {
        let slug = meta.class_id.strip_prefix("class:").unwrap_or(&meta.class_id).to_string();
        let max_level = class_chassis_sheet_rules::record("core_rulebook", &slug)
            .map(|record| record.max_level)
            .unwrap_or(DEFAULT_TABLED_MAX_LEVEL);
        claim(
            &mut entries,
            meta.class_id.clone(),
            ClassFamily::CrbNpcEx,
            "core_rulebook",
            "crb_untabled_class_chassis",
            max_level,
            false,
        );
    }

    let books = prestige_scan_books();
    let book_refs: Vec<&str> = books.iter().map(String::as_str).collect();
    for ((book, slug), chassis) in class_chassis_sheet_rules::records(&book_refs) {
        if !chassis.tags.iter().any(|t| t == "Prestige") {
            continue;
        }
        let class_id = format!("class:{slug}");
        claim(&mut entries, class_id, ClassFamily::Prestige, &book, "prestige_tagged_record", chassis.max_level, true);
    }

    // The ninth source: `generic_class_chassis`'s own population. Folded in
    // last, and never claims a family a prior source did not already
    // establish -- see this module's doc comment. A class id it names that
    // none of the eight above claimed is recorded under
    // `ClassFamily::GenericOnly`, never silently folded into an existing
    // family.
    for meta in generic_class_chassis_covered_classes() {
        let is_prestige = meta.tags.iter().any(|t| t == "Prestige");
        entries
            .entry(meta.class_id.clone())
            .and_modify(|e| {
                add_unique(&mut e.books, &meta.book);
                add_registry(&mut e.registries, "generic_class_chassis");
            })
            .or_insert_with(|| ClassCensusEntry {
                class_id: meta.class_id.clone(),
                family: ClassFamily::GenericOnly,
                books: vec![meta.book.clone()],
                registries: vec!["generic_class_chassis"],
                max_level: meta.max_level,
                is_prestige,
            });
    }

    entries
}

/// `census()`'s entries grouped by family, counted -- the partition
/// `every_registry_is_swept_once` prints in its own assertion message.
pub fn family_counts(entries: &BTreeMap<String, ClassCensusEntry>) -> BTreeMap<ClassFamily, usize> {
    let mut counts: BTreeMap<ClassFamily, usize> = BTreeMap::new();
    for entry in entries.values() {
        *counts.entry(entry.family).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    fn partition_message(entries: &BTreeMap<String, ClassCensusEntry>) -> String {
        let counts = family_counts(entries);
        let mut parts: Vec<String> =
            counts.iter().map(|(family, count)| format!("{}={count}", family.label())).collect();
        parts.sort();
        format!("measured total={} ({})", entries.len(), parts.join(", "))
    }

    #[test]
    fn every_registry_is_swept_once() {
        let entries = census();
        // Every id appears exactly once -- guaranteed by construction
        // (`BTreeMap<String, _>`), but a duplicate `class_id` field inside
        // two different map slots (a bug in the merge, not the type) would
        // still be a real defect, so check it directly rather than trust
        // the map shape alone.
        let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
        for (key, entry) in &entries {
            assert_eq!(key, &entry.class_id, "map key must equal the entry's own class_id");
            assert!(seen_ids.insert(entry.class_id.as_str()), "{} swept more than once: {}", entry.class_id, partition_message(&entries));
        }
        // Every entry names at least one registry and one book -- a class
        // id with zero provenance would mean the merge inserted it without
        // ever calling `claim`, which should be unreachable.
        for entry in entries.values() {
            assert!(!entry.registries.is_empty(), "{}: no registry recorded ({})", entry.class_id, partition_message(&entries));
            assert!(!entry.books.is_empty(), "{}: no book recorded ({})", entry.class_id, partition_message(&entries));
        }
        // The measured partition by family, always computed into `message`
        // so a future failure names exactly which family moved. Re-derive
        // with `cargo test --locked -j 2 --lib
        // class_census::tests::every_registry_is_swept_once -- --nocapture`
        // (temporarily insert `panic!("{message}")` above this line to see
        // it on a green run).
        let message = partition_message(&entries);
        let counts = family_counts(&entries);
        // Measured 2026-09-21, independently in this code (not copied from
        // the doc): Crb=11, Apg=6, Acg=10, PathfinderUnchained=4,
        // UltimateCombat=3, UntabledExoticBase=20, CrbNpcEx=7, Prestige=74.
        // 11+6+10+4=31 tabled + 3 + 20 + 7 + 74 = 135, exactly
        // `docs/architecture/status.md`'s own "31+3+20+7+74 = 135" partition
        // (measured there 2026-09-20 by a now-deleted throwaway test this
        // module replaces with a permanent one). Asserted per family, not
        // only as a total, so a family that shrinks while another grows by
        // the same amount cannot hide behind an unchanged total. No
        // `scripts/retro.py correction` needed: the measured total already
        // equals the previously published figure -- if a future run of
        // this test goes red, that IS the corpus movement to log.
        assert_eq!(counts.get(&ClassFamily::Crb).copied().unwrap_or(0), 11, "{message}");
        assert_eq!(counts.get(&ClassFamily::Apg).copied().unwrap_or(0), 6, "{message}");
        assert_eq!(counts.get(&ClassFamily::Acg).copied().unwrap_or(0), 10, "{message}");
        assert_eq!(counts.get(&ClassFamily::PathfinderUnchained).copied().unwrap_or(0), 4, "{message}");
        assert_eq!(counts.get(&ClassFamily::UltimateCombat).copied().unwrap_or(0), 3, "{message}");
        assert_eq!(counts.get(&ClassFamily::UntabledExoticBase).copied().unwrap_or(0), 20, "{message}");
        assert_eq!(counts.get(&ClassFamily::CrbNpcEx).copied().unwrap_or(0), 7, "{message}");
        assert_eq!(counts.get(&ClassFamily::Prestige).copied().unwrap_or(0), 74, "{message}");
        assert_eq!(entries.len(), 135, "measured census total moved off the previously published 135 -- {message}");
    }

    #[test]
    fn prestige_list_matches_the_entry_requirement_fixture() {
        use crate::rules_core::pilot_compute::prestige_class_entry_gate::prestige_class_entry_requirements;

        let entries = census();
        let census_prestige: BTreeSet<String> = entries
            .values()
            .filter(|e| e.family == ClassFamily::Prestige)
            .map(|e| e.class_id.clone())
            .collect();
        let fixture_prestige: BTreeSet<String> =
            prestige_class_entry_requirements().iter().map(|req| req.class_id.clone()).collect();

        let only_in_census: Vec<&String> = census_prestige.difference(&fixture_prestige).collect();
        let only_in_fixture: Vec<&String> = fixture_prestige.difference(&census_prestige).collect();
        assert!(
            only_in_census.is_empty() && only_in_fixture.is_empty(),
            "prestige list mismatch (census {} of fixture {}): only in census = {:?}, only in fixture = {:?}",
            census_prestige.len(),
            fixture_prestige.len(),
            only_in_census,
            only_in_fixture,
        );
        assert_eq!(census_prestige.len(), 74, "measured prestige-tagged class count, of the fixture's own 74");
    }

    #[test]
    fn no_class_id_sits_in_two_families() {
        // Checks the RAW sources, before the first-claimed-wins precedence
        // in `census()` has a chance to paper over a real collision: if two
        // of the eight canonical sources ever name the same id, that is a
        // genuine defect (a class the corpus tags two incompatible ways at
        // once), not something the merge should silently resolve.
        let raw = raw_claims();
        let mut offenders: BTreeMap<String, Vec<&'static str>> = BTreeMap::new();
        let names: Vec<&'static str> = raw.keys().copied().collect();
        for i in 0..names.len() {
            for j in (i + 1)..names.len() {
                let a = &raw[names[i]];
                let b = &raw[names[j]];
                for id in a.intersection(b) {
                    offenders.entry(id.clone()).or_default().extend([names[i], names[j]]);
                }
            }
        }
        assert!(
            offenders.is_empty(),
            "class id(s) claimed by more than one canonical source: {offenders:?}"
        );

        // And the merged census itself: every entry's family must be one
        // the class id's own raw membership actually supports (catches a
        // `claim()` call site accidentally using the wrong `ClassFamily`
        // constant for its own loop).
        let entries = census();
        for entry in entries.values() {
            if entry.family == ClassFamily::GenericOnly {
                continue;
            }
            let registry_for_family = match entry.family {
                ClassFamily::Crb => "crb_class_id",
                ClassFamily::Apg => "apg_class_id",
                ClassFamily::Acg => "acg_class_id",
                ClassFamily::PathfinderUnchained => "pu_class_id",
                ClassFamily::UltimateCombat => "uc_class_id",
                ClassFamily::UntabledExoticBase => "untabled_base_class_registry",
                ClassFamily::CrbNpcEx => "crb_untabled_class_chassis",
                ClassFamily::Prestige => "prestige_tagged_record",
                ClassFamily::GenericOnly => unreachable!(),
            };
            assert!(
                entry.registries.contains(&registry_for_family),
                "{}: family {} but its own claiming registry {:?} is not in registries {:?}",
                entry.class_id,
                entry.family.label(),
                registry_for_family,
                entry.registries,
            );
        }
    }

    #[test]
    fn no_generic_only_stragglers_today() {
        // `docs/architecture/status.md`'s own evidence: `generic_class_
        // chassis`'s 78 classes are fully subsumed by the other eight
        // sources (56 Prestige + 19 of 20 untabled-exotic + 3 Ultimate
        // Combat = 78). This pins that today; if it ever goes red, a real
        // new class surfaced through `generic_class_chassis` alone and
        // needs a named family decision, not a silent default.
        let entries = census();
        let stragglers: Vec<&str> = entries
            .values()
            .filter(|e| e.family == ClassFamily::GenericOnly)
            .map(|e| e.class_id.as_str())
            .collect();
        assert!(stragglers.is_empty(), "generic_class_chassis named a class no other source claims: {stragglers:?}");
    }

    #[test]
    fn prestige_scan_books_is_measured_not_guessed() {
        let books = prestige_scan_books();
        assert!(books.contains(&"core_rulebook".to_string()));
        assert!(!books.iter().any(|b| b.starts_with('_')), "book list must exclude bookkeeping dirs: {books:?}");
        // Measured directly: `find data/sheet_rules -maxdepth 1 -type d
        // ! -path data/sheet_rules ! -name '_*' | wc -l`.
        assert!(books.len() >= 30, "measured book-dir count looks too small: {} ({:?})", books.len(), books);
    }
}
