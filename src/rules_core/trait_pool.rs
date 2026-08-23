//! Real, book-agnostic loader + resolver for PF1e's chargen **Trait**
//! mechanic (`kind: trait`, `decisions.md §25`) -- the content an "Adopted
//! Race" selector (`race_resolver::adopted_race_choose_selectors`) picks
//! from.
//!
//! **Distinct content kind, distinct loader.** `race_resolver::RaceCorpus`
//! loads `kind: race`/`race_trait`; this module loads `kind: trait` records
//! from `data/corpus/<book>/trait_generic/*.json` -- the sibling-directory
//! shape `scripts/ingest_generic_kind.py` writes every `--kind` population
//! into (never inside the curated `<kind>/` directory itself; see that
//! script's own module doc comment for why). Kept separate rather than
//! folded into `RaceCorpus` because a Trait record belongs to no race at
//! all -- it is a flat, race-agnostic pool a selector of ANY adopted race
//! draws from, matched by its own `TYPE:Trait.RaceTrait.<X> Race Trait`
//! third dot-segment, never by `race_key` (Trait records carry none).
//!
//! Follows `corpus_loader.rs`'s own precedent exactly: the caller supplies
//! `BookCorpusRoot`s rather than this module hardcoding a book list, so it
//! stays agnostic to how many books carry Trait content at any given time
//! (`decisions.md §17`: a generic pass, not a per-book table).
//!
//! **Nothing is computed.** A pool record's own `raw_tokens` are read
//! verbatim off disk (the same `ingest_generic_kind.py` guarantee its module
//! doc comment states); this loader only indexes them by the `TYPE:` third
//! dot-segment so [`resolve_adopted_race_options`] can look one up by the
//! exact string an Adopted-Race selector's `CHOOSE:` token names.

use std::fs;
use std::path::{Path, PathBuf};

use crate::rules_core::corpus_loader::BookCorpusRoot;
use crate::rules_core::race_resolver::AdoptedRaceSelector;
use crate::rules_core::shape_b_v1::RawToken;

/// PCGen's `TYPE:` prefix every real Trait row's third dot-segment sits
/// behind: `TYPE:Trait.RaceTrait.<X> Race Trait` -> `"<X> Race Trait"`. The
/// same rule `v06_work_inventory.rs::refine_kind` and
/// `census_independent.py::_row_is_pf1_trait` already use to classify a row
/// into `Kind::Trait` in the first place, read here at the resolver layer
/// instead of the census layer.
const RACE_TRAIT_TYPE_PREFIX: &str = "Trait.RaceTrait.";

/// One `data/corpus/<book>/trait_generic/*.json` record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraitPoolRecord {
    pub book_id: String,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub raw_tokens: Vec<RawToken>,
}

impl TraitPoolRecord {
    /// The `<X> Race Trait` pool this record belongs to, read from its own
    /// `TYPE:` token(s) -- `None` for a Trait record that is not a
    /// race-adoptable pool member (PF1e also has non-race-scoped Traits,
    /// e.g. bare `TYPE:Trait` with no `RaceTrait.` component, which an
    /// Adopted-Race selector never references).
    pub fn race_trait_pool(&self) -> Option<&str> {
        self.raw_tokens
            .iter()
            .filter(|t| t.key == "TYPE")
            .find_map(|t| t.value.strip_prefix(RACE_TRAIT_TYPE_PREFIX))
    }
}

/// Every loaded `kind: trait` record, indexed by [`TraitPoolRecord::race_trait_pool`].
#[derive(Debug, Default)]
pub struct TraitPool {
    by_pool: std::collections::BTreeMap<String, Vec<TraitPoolRecord>>,
}

impl TraitPool {
    /// Every Trait record filed under the named `<X> Race Trait` pool,
    /// sorted by key. Empty is a legitimate, honestly-reported answer -- a
    /// pool this project has not yet ingested content for (or one PCGen
    /// itself never populates, like Rougarou's -- `decisions.md §25` cycle
    /// 1's own "proven empty" finding) resolves to nothing rather than
    /// fabricating a placeholder trait.
    pub fn pool_for(&self, pool: &str) -> Vec<&TraitPoolRecord> {
        self.by_pool.get(pool).map(|v| v.iter().collect()).unwrap_or_default()
    }

    /// Total record count across every loaded pool -- used by
    /// `reach_gate`/tests to confirm the loader found real content without
    /// duplicating this module's own indexing logic.
    pub fn total_records(&self) -> usize {
        self.by_pool.values().map(Vec::len).sum()
    }
}

/// Loads every `kind: trait` record from every given book's
/// `trait_generic/` directory. A book with no such directory contributes
/// nothing and is not an error -- the identical shape
/// `corpus_loader::load_equipment_corpus` already uses, so this is safe to
/// call against a book that has not (yet) had `ingest_generic_kind.py
/// --kind trait` run against it.
pub fn load_trait_pool(roots: &[BookCorpusRoot<'_>]) -> TraitPool {
    let mut pool = TraitPool::default();
    // Two source directories per book, `trait_generic/` preferred over
    // `ability/`, deduplicated by the record's own `key`.
    //
    // **Why `ability/` too.** `ingest_generic_kind.py --kind trait`'s
    // `no_record`-ledger-gated design cannot yet see this population:
    // `shape_ledger.py`'s `(book, source_file, source_line)` join is
    // kind-blind, and a prior ingest pass (predating `Kind::Trait`,
    // `epic-6-kind-trait_cycle-1_cycle_receipt.md §1`) already wrote these
    // SAME LST rows under `kind: ability` -- so the join finds THAT record at
    // the same coordinate and reports `matched`/`no_formula_tokens`, never
    // `no_record`, even though no `kind: trait` record exists anywhere.
    // Confirmed directly: `data/corpus/inner_sea_races/ability/loner_of_the_
    // rocks.json` carries `TYPE:Trait.RaceTrait.Oread Race Trait` verbatim,
    // the exact real content `epic-6-kind-trait_cycle-1_cycle_receipt.md §1`
    // named (`isr_abilities.lst:78`) -- genuinely on disk, correctly shaped
    // (identical `key`/`name`/`description`/`raw_tokens` schema to a generic-
    // kind record), just filed under the wrong content-kind directory by an
    // ingest pass this cycle did not run and does not own. Reading it here is
    // NOT a corpus write (`data/corpus/**` stays hand-untouched) and is not a
    // claim that the `ability` filing is correct -- it is this loader
    // refusing to leave real, already-shipped, correctly-parseable content
    // invisible to the resolver merely because a housekeeping re-file has not
    // happened yet. `trait_generic/` is checked FIRST and wins on a key
    // collision, so the moment a future cycle runs the generic-kind ingest
    // (or retires these `ability` duplicates), this loader needs no change.
    for root in roots {
        for dir_name in ["trait_generic", "ability"] {
            let dir = root.dir.join(dir_name);
            if !dir.is_dir() {
                continue;
            }
            for path in find_json_files(&dir) {
                let Some(record) = read_trait_record(root.book_id, &path) else { continue };
                let Some(pool_key) = record.race_trait_pool().map(str::to_string) else { continue };
                let bucket = pool.by_pool.entry(pool_key).or_default();
                if bucket.iter().any(|existing| existing.key == record.key) {
                    continue;
                }
                bucket.push(record);
            }
        }
    }
    for records in pool.by_pool.values_mut() {
        records.sort_by(|a, b| a.key.cmp(&b.key));
    }
    pool
}

fn read_trait_record(book_id: &str, path: &Path) -> Option<TraitPoolRecord> {
    let text = fs::read_to_string(path).ok()?;
    let value: serde_json::Value = serde_json::from_str(&text).ok()?;
    let data = value.get("data")?;
    let key = data.get("key")?.as_str()?.to_string();
    let name = data.get("name").and_then(serde_json::Value::as_str).unwrap_or(&key).to_string();
    let description = data.get("description").and_then(serde_json::Value::as_str).map(str::to_string);
    let raw_tokens: Vec<RawToken> = data
        .get("raw_tokens")
        .and_then(serde_json::Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(|entry| {
                    let k = entry.get("key")?.as_str()?.to_string();
                    let v = entry.get("value")?.as_str()?.to_string();
                    Some(RawToken { key: k, value: v })
                })
                .collect()
        })
        .unwrap_or_default();
    Some(TraitPoolRecord { book_id: book_id.to_string(), key, name, description, raw_tokens })
}

fn find_json_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        let Ok(entries) = fs::read_dir(&current) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();
            if path.is_dir() {
                stack.push(path);
            } else if file_name == "LICENSE.json" {
                continue;
            } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

/// One resolved Trait member of an Adopted-Race option's pool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdoptedRaceTraitGrant {
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub book_id: String,
}

/// One "Adopted Race" selector, resolved against a loaded [`TraitPool`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedAdoptedRaceOption {
    pub key: String,
    pub name: String,
    pub book_id: String,
    pub adopted_race: String,
    /// Real Trait records this option's pool resolves to. Empty is honest
    /// and expected for a race whose pool this project has not (yet)
    /// ingested, or which PCGen itself never populates (Rougarou).
    pub grants: Vec<AdoptedRaceTraitGrant>,
    /// `true` when the selector's own `CHOOSE:` token did not carry a
    /// readable pool suffix at all -- a malformed-row finding this struct
    /// surfaces rather than silently treating as "empty pool".
    pub malformed_choose_token: bool,
}

/// Resolves every [`AdoptedRaceSelector`] against a loaded [`TraitPool`] --
/// the combining step [`crate::rules_core::race_resolver::adoptive_parentage_options`]
/// performs for its own (different) shape, kept in this module because the
/// pool half is a different content kind `race_resolver` does not load.
pub fn resolve_adopted_race_options(
    selectors: &[AdoptedRaceSelector],
    pool: &TraitPool,
) -> Vec<ResolvedAdoptedRaceOption> {
    selectors
        .iter()
        .map(|selector| match &selector.pool_type_suffix {
            Some(suffix) => {
                let grants = pool
                    .pool_for(suffix)
                    .into_iter()
                    .map(|record| AdoptedRaceTraitGrant {
                        key: record.key.clone(),
                        name: record.name.clone(),
                        description: record.description.clone(),
                        book_id: record.book_id.clone(),
                    })
                    .collect();
                ResolvedAdoptedRaceOption {
                    key: selector.key.clone(),
                    name: selector.name.clone(),
                    book_id: selector.book_id.clone(),
                    adopted_race: selector.adopted_race.clone(),
                    grants,
                    malformed_choose_token: false,
                }
            }
            None => ResolvedAdoptedRaceOption {
                key: selector.key.clone(),
                name: selector.name.clone(),
                book_id: selector.book_id.clone(),
                adopted_race: selector.adopted_race.clone(),
                grants: Vec::new(),
                malformed_choose_token: true,
            },
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn oread_selector() -> AdoptedRaceSelector {
        AdoptedRaceSelector {
            key: "Adopted Race ~ Oread".to_string(),
            name: "Oread".to_string(),
            book_id: "bestiary_2".to_string(),
            adopted_race: "Oread".to_string(),
            pool_type_suffix: Some("Oread Race Trait".to_string()),
        }
    }

    #[test]
    fn a_trait_pool_record_reads_its_own_race_trait_pool_from_its_type_token() {
        let record = TraitPoolRecord {
            book_id: "inner_sea_races".to_string(),
            key: "Oread ~ Something".to_string(),
            name: "Something".to_string(),
            description: Some("desc".to_string()),
            raw_tokens: vec![RawToken {
                key: "TYPE".to_string(),
                value: "Trait.RaceTrait.Oread Race Trait".to_string(),
            }],
        };
        assert_eq!(record.race_trait_pool(), Some("Oread Race Trait"));
    }

    #[test]
    fn a_bare_type_trait_record_with_no_racetrait_component_has_no_pool() {
        let record = TraitPoolRecord {
            book_id: "ultimate_campaign".to_string(),
            key: "Some Background Trait".to_string(),
            name: "Some Background Trait".to_string(),
            description: None,
            raw_tokens: vec![RawToken { key: "TYPE".to_string(), value: "Trait".to_string() }],
        };
        assert_eq!(record.race_trait_pool(), None);
    }

    #[test]
    fn resolving_an_empty_pool_is_honest_never_fabricated() {
        // No corpus content loaded at all -- the real state of this repo
        // today, pending `docs/work-inventory.json`'s regen
        // (`epic-6-kind-trait_cycle-1_cycle_receipt.md §3`). `grants` must be
        // empty, not a guessed/synthesized member.
        let pool = TraitPool::default();
        let resolved = resolve_adopted_race_options(&[oread_selector()], &pool);
        assert_eq!(resolved.len(), 1);
        assert!(resolved[0].grants.is_empty());
        assert!(!resolved[0].malformed_choose_token, "the selector itself parsed fine -- only its pool is empty");
    }

    #[test]
    fn resolving_a_populated_pool_returns_its_real_members_matched_by_the_exact_type_suffix() {
        let mut pool = TraitPool::default();
        pool.by_pool.insert(
            "Oread Race Trait".to_string(),
            vec![TraitPoolRecord {
                book_id: "inner_sea_races".to_string(),
                key: "Oread ~ Meditative".to_string(),
                name: "Meditative".to_string(),
                description: Some("You gain a +2 trait bonus on Sense Motive checks.".to_string()),
                raw_tokens: vec![],
            }],
        );
        // A DIFFERENT pool must never leak into Oread's resolution.
        pool.by_pool.insert(
            "Sylph Race Trait".to_string(),
            vec![TraitPoolRecord {
                book_id: "inner_sea_races".to_string(),
                key: "Sylph ~ Something Else".to_string(),
                name: "Something Else".to_string(),
                description: None,
                raw_tokens: vec![],
            }],
        );
        let resolved = resolve_adopted_race_options(&[oread_selector()], &pool);
        assert_eq!(resolved.len(), 1);
        assert_eq!(resolved[0].grants.len(), 1, "must resolve ONLY the Oread pool, not Sylph's");
        assert_eq!(resolved[0].grants[0].key, "Oread ~ Meditative");
        assert_eq!(resolved[0].grants[0].name, "Meditative");
        assert_eq!(resolved[0].grants[0].book_id, "inner_sea_races");
    }

    #[test]
    fn a_selector_with_no_readable_choose_pool_suffix_is_flagged_not_silently_emptied() {
        let malformed = AdoptedRaceSelector {
            key: "Adopted Race ~ Broken".to_string(),
            name: "Broken".to_string(),
            book_id: "bestiary_2".to_string(),
            adopted_race: "Broken".to_string(),
            pool_type_suffix: None,
        };
        let pool = TraitPool::default();
        let resolved = resolve_adopted_race_options(&[malformed], &pool);
        assert!(resolved[0].malformed_choose_token, "a genuinely unreadable CHOOSE token must be flagged");
        assert!(resolved[0].grants.is_empty());
    }

    /// Integration: loads the REAL, on-disk `bestiary_2/race_trait/` +
    /// `trait_generic/` state. As of this cycle no `trait_generic/`
    /// directory exists anywhere in the committed corpus yet
    /// (`docs/work-inventory.json` regen still blocked -- see
    /// `epic-6-kind-trait_cycle-1_cycle_receipt.md §3`), so this proves the
    /// loader does not panic against that real, current, contentless state
    /// -- the same "nonexistent dir contributes nothing" guarantee
    /// `corpus_loader.rs`'s own test proves for `load_equipment_corpus`.
    #[test]
    fn loading_the_real_corpus_today_finds_no_trait_generic_content_yet_without_panicking() {
        // `bestiary_2` carries neither a `trait_generic/` directory nor any
        // `ability/*.json` record whose TYPE names a `RaceTrait` pool -- it
        // is not one of the 6 books `epic-6-kind-trait_cycle-1_cycle_
        // receipt.md §1` found real Trait content in. This proves the
        // "nothing here, no panic" half of the loader's contract; the next
        // test proves the "real content, found and resolved" half.
        let roots =
            [BookCorpusRoot { book_id: "bestiary_2", dir: Path::new("data/corpus/bestiary_2") }];
        let pool = load_trait_pool(&roots);
        assert_eq!(pool.total_records(), 0, "bestiary_2 carries no Trait pool content of either shape");
    }

    /// Integration: `inner_sea_races/ability/loner_of_the_rocks.json` is a
    /// REAL, already-shipped, on-disk corpus record (predates this cycle,
    /// filed under `kind: ability` by an ingest pass that ran before
    /// `Kind::Trait` existed) carrying `TYPE:Trait.RaceTrait.Oread Race
    /// Trait` verbatim -- the exact record
    /// `epic-6-kind-trait_cycle-1_cycle_receipt.md §1` names
    /// (`isr_abilities.lst:78`). This proves the loader's `ability/`
    /// fallback (this module's own doc comment on `load_trait_pool`) finds
    /// and correctly pools REAL corpus content today, without waiting on
    /// `ingest_generic_kind.py --kind trait`'s still-blocked `no_record`
    /// discovery (`shape_ledger.py`'s kind-blind join already reports this
    /// exact row `matched`, so that path alone would never surface it).
    #[test]
    fn loading_the_real_inner_sea_races_corpus_finds_the_real_oread_pool_member() {
        let roots = [BookCorpusRoot {
            book_id: "inner_sea_races",
            dir: Path::new("data/corpus/inner_sea_races"),
        }];
        let pool = load_trait_pool(&roots);
        assert!(pool.total_records() > 0, "inner_sea_races must carry real, already-shipped Trait content");
        let oread_pool = pool.pool_for("Oread Race Trait");
        assert!(
            oread_pool.iter().any(|r| r.key == "Trait ~ Loner of the Rocks"),
            "the real, on-disk Oread pool member must be found: {:?}",
            oread_pool.iter().map(|r| &r.key).collect::<Vec<_>>()
        );
        let member = oread_pool.iter().find(|r| r.key == "Trait ~ Loner of the Rocks").unwrap();
        assert_eq!(member.name, "Loner of the Rocks");
        assert!(member.description.as_deref().is_some_and(|d| d.contains("Heal and Survival")));
        assert_eq!(member.book_id, "inner_sea_races");
    }
}
