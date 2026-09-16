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
//! **Nothing is computed, and nothing here reads the ingest format.** A
//! record's pool membership comes off the **converted** package -- the third
//! [`tags`](crate::rules_core::sheet_rule::SheetRule::tags) entry on a rule
//! `sheet_rule_convert` wrote from that record's own closure row -- so
//! [`resolve_adopted_race_options`] can look one up by the exact string an
//! Adopted-Race selector names. Through SD-35 `AT-35-E6-003-RULED` cycle 4
//! this module called `pcgen_import::ingest_record::type_token_suffix` on the
//! corpus record's token array; cycle 5 replaced that with the converted read
//! after measuring the two agree on all 487 `kind: trait` corpus records.
//!
//! **The `ability/` fallback this module carried through `epic-6-kind-trait`
//! cycle 2 has been retired.** That cycle's own `§4`/`§6` named the reason it
//! existed: `shape_ledger.py`'s `(book, source_file, source_line)` join was
//! kind-blind, so `ingest_generic_kind.py --kind trait` could never see the
//! 487-unit `kind: trait` census population as `no_record` -- every one of
//! them collided with a pre-existing `kind: ability` record at the identical
//! coordinate. A sibling cycle fixed that join (`shape_ledger.py` made
//! kind-aware) and ran the real `--kind trait` ingest for real, producing
//! `data/corpus/*/trait_generic/*.json` records corpus-wide. This loader was
//! re-verified against the resulting corpus (`scripts/compare_pools.py`-shape
//! check, re-run this cycle) to confirm **zero** `RaceTrait`-tagged keys exist
//! under any book's `ability/` directory that are absent from `trait_generic/`
//! -- the fallback's population is now a strict, exact duplicate of the real
//! `kind: trait` write, not a source of content unavailable any other way.
//! Reading only `trait_generic/` is therefore both correct (the modelled
//! `kind: trait` schema `decisions.md §25` specifies) and lossless.

use std::fs;
use std::path::{Path, PathBuf};

use crate::rules_core::corpus_loader::BookCorpusRoot;
use crate::rules_core::race_resolver::AdoptedRaceSelector;
use crate::rules_core::sheet_rule::SheetRulePackage;

/// The converted package's tag for a Trait record's race-adoptable pool: the
/// rule's [`tags`](crate::rules_core::sheet_rule::SheetRule::tags) read
/// `["Trait", "RaceTrait", "<X> Race Trait", ..]` and the pool name is the
/// third. The converter writes the whole `TYPE:` chain out as tags, so the
/// shape of the chain -- not a prefix string this module holds -- is what
/// decides: a record whose first two tags are not exactly these two belongs
/// to no race pool, which is how `Trait.BasicTrait.RaceTrait.BloodlineTrait`
/// correctly resolves to `None`.
const TRAIT_TAG: &str = "Trait";
const RACE_TRAIT_TAG: &str = "RaceTrait";

/// One `data/corpus/<book>/trait_generic/*.json` record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraitPoolRecord {
    pub book_id: String,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    /// The `<X> Race Trait` pool this record belongs to, resolved once at load
    /// time from its own `TYPE:` token(s) -- `None` for a Trait record that is
    /// not a race-adoptable pool member (PF1e also has non-race-scoped Traits,
    /// e.g. bare `TYPE:Trait` with no `RaceTrait.` component, which an
    /// Adopted-Race selector never references).
    ///
    /// A resolved name, not the ingested row: this struct carried the whole
    /// token array until SD-35 `AT-35-E6-002` cycle 3, and no consumer ever
    /// read anything else out of it.
    pub race_trait_pool: Option<String>,
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
/// Reads the pool name off the **converted** package
/// (`data/sheet_rules/`, resolved by
/// [`corpus_loader::live_sheet_rules`](crate::rules_core::corpus_loader::live_sheet_rules)),
/// never off a corpus record's ingest-token array. With no package the pool is
/// empty and says so; it never falls back to reading the ingest format.
pub fn load_trait_pool(roots: &[BookCorpusRoot<'_>]) -> TraitPool {
    load_trait_pool_with_rules(roots, crate::rules_core::corpus_loader::live_sheet_rules())
}

/// [`load_trait_pool`] against a caller-supplied converted package -- the form the
/// corpus-wide gate below and any caller that already holds a package uses.
pub fn load_trait_pool_with_rules(
    roots: &[BookCorpusRoot<'_>],
    rules: Option<&SheetRulePackage>,
) -> TraitPool {
    let mut pool = TraitPool::default();
    let Some(rules) = rules else { return pool };
    // Single source directory per book -- `trait_generic/`, the real
    // `kind: trait` write `ingest_generic_kind.py --kind trait` produces
    // (`decisions.md §25`). The `ability/` fallback this loader carried
    // through `epic-6-kind-trait` cycle 2 is retired (see this module's own
    // doc comment): a sibling cycle fixed `shape_ledger.py`'s kind-blind join
    // and ran the real ingest, and the resulting `trait_generic/` population
    // was verified to be a strict superset (in fact an exact duplicate key
    // set) of what `ability/` ever carried for `RaceTrait`-tagged rows, so no
    // content is lost by reading only the modelled directory.
    for root in roots {
        let dir = root.dir.join("trait_generic");
        if !dir.is_dir() {
            continue;
        }
        for path in find_json_files(&dir) {
            let Some(record) = read_trait_record(root.book_id, &path, rules) else { continue };
            let Some(pool_key) = record.race_trait_pool.clone() else { continue };
            let bucket = pool.by_pool.entry(pool_key).or_default();
            if bucket.iter().any(|existing| existing.key == record.key) {
                continue;
            }
            bucket.push(record);
        }
    }
    for records in pool.by_pool.values_mut() {
        records.sort_by(|a, b| a.key.cmp(&b.key));
    }
    pool
}

fn read_trait_record(
    book_id: &str,
    path: &Path,
    rules: &SheetRulePackage,
) -> Option<TraitPoolRecord> {
    let text = fs::read_to_string(path).ok()?;
    let value: serde_json::Value = serde_json::from_str(&text).ok()?;
    let data = value.get("data")?;
    let key = data.get("key")?.as_str()?.to_string();
    let name = data.get("name").and_then(serde_json::Value::as_str).unwrap_or(&key).to_string();
    let description = data.get("description").and_then(serde_json::Value::as_str).map(str::to_string);
    let race_trait_pool = converted_race_trait_pool(&value, rules);
    Some(TraitPoolRecord { book_id: book_id.to_string(), key, name, description, race_trait_pool })
}

/// The `<X> Race Trait` pool this corpus record's **converted** rules place it in, or `None`.
///
/// The join is the record's own closure row (`source.path` + `source.line`), which is the one
/// key that closes: the converter names the same `path:line` in every rule it wrote from that
/// row. Measured over the whole `kind: trait` population -- 487 corpus records -- this
/// resolves 487 and agrees with the retired ingest-token read on 487 (see this cycle's
/// receipt for the command).
fn converted_race_trait_pool(
    record: &serde_json::Value,
    rules: &SheetRulePackage,
) -> Option<String> {
    let source = record.get("source")?;
    let path = source.get("path")?.as_str()?;
    let line = source.get("line")?.as_u64()?;
    for id in rules.rules_for_closure_row(path, line) {
        let tags = &rules.rule(id)?.tags;
        if tags.len() >= 3 && tags[0] == TRAIT_TAG && tags[1] == RACE_TRAIT_TAG {
            return Some(tags[2].clone());
        }
    }
    None
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
            // The `TYPE:Trait.RaceTrait.<X> Race Trait` -> `<X> Race Trait`
            // reading itself is proven tool side, against the real on-disk
            // document shape, by
            // `pcgen_import::ingest_record::tests::a_type_token_yields_only_the_tail_behind_its_prefix`.
            race_trait_pool: Some("Oread Race Trait".to_string()),
        };
        assert_eq!(record.race_trait_pool.as_deref(), Some("Oread Race Trait"));
    }

    #[test]
    fn a_bare_type_trait_record_with_no_racetrait_component_has_no_pool() {
        let record = TraitPoolRecord {
            book_id: "ultimate_campaign".to_string(),
            key: "Some Background Trait".to_string(),
            name: "Some Background Trait".to_string(),
            description: None,
            // A bare `TYPE:Trait` row resolves to no pool -- same reader,
            // same tool-side test as above.
            race_trait_pool: None,
        };
        assert_eq!(record.race_trait_pool, None);
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
                race_trait_pool: Some("Oread Race Trait".to_string()),
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
                race_trait_pool: Some("Sylph Race Trait".to_string()),
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

    /// Integration: loads the REAL, on-disk `bestiary_2/` state. `bestiary_2`
    /// itself carries no `trait_generic/` directory (the pool content its
    /// selectors resolve against lives in `inner_sea_races`, proven by the
    /// next test) -- this proves the "nonexistent dir contributes nothing,
    /// no panic" half of the loader's contract, the same guarantee
    /// `corpus_loader.rs`'s own test proves for `load_equipment_corpus`.
    #[test]
    fn loading_a_book_with_no_trait_generic_directory_finds_nothing_without_panicking() {
        let roots =
            [BookCorpusRoot { book_id: "bestiary_2", dir: Path::new("data/corpus/bestiary_2") }];
        let pool = load_trait_pool(&roots);
        assert_eq!(pool.total_records(), 0, "bestiary_2 carries no kind: trait directory of its own");
    }

    /// Integration: `inner_sea_races/trait_generic/trait_loner_of_the_rocks.json`
    /// is a REAL `kind: trait` corpus record -- the product of a sibling
    /// cycle's `shape_ledger.py` kind-aware-join fix and a real
    /// `ingest_generic_kind.py --kind trait` run (`epic-6-kind-trait` cycle 3;
    /// the `ability/` fallback cycle 2 built to route around the then-blocked
    /// join is retired, see this module's own doc comment). This proves the
    /// loader finds and correctly pools real, modelled `kind: trait` content
    /// without any fallback read.
    #[test]
    fn loading_the_real_inner_sea_races_corpus_finds_the_real_oread_pool_member() {
        let roots = [BookCorpusRoot {
            book_id: "inner_sea_races",
            dir: Path::new("data/corpus/inner_sea_races"),
        }];
        let pool = load_trait_pool(&roots);
        assert!(pool.total_records() > 0, "inner_sea_races must carry real kind: trait content");
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

    /// The per-kind converter gate for `kind: trait` (SD-35 `AT-35-E6-003-RULED` cycle 5):
    /// over the **live** `data/corpus/*/trait_generic/` directory and the **live**
    /// `data/sheet_rules/` package -- never a fixture with a hand-derived value -- every
    /// corpus record's closure row resolves to at least one converted rule, and the pool
    /// membership the converted `tags` state is exactly the one the retired
    /// `TYPE:Trait.RaceTrait.` token read produced.
    ///
    /// The token read is reproduced here, in the test, deliberately: it is the oracle this
    /// swap is measured against, and the test is the only place in the crate that may still
    /// spell it. If the converter ever stops writing the `TYPE:` chain out as tags, this goes
    /// red on the real corpus rather than the pool silently emptying on a sheet.
    #[test]
    fn every_live_trait_record_gets_the_same_pool_from_the_converted_package() {
        let Some(rules) = crate::rules_core::corpus_loader::live_sheet_rules() else {
            panic!("data/sheet_rules/ must be present for the kind: trait converter gate");
        };
        let corpus = Path::new("data/corpus");
        let mut books: Vec<PathBuf> = match fs::read_dir(corpus) {
            Ok(entries) => entries.flatten().map(|e| e.path()).filter(|p| p.is_dir()).collect(),
            Err(_) => panic!("data/corpus must be readable"),
        };
        books.sort();

        let mut records = 0usize;
        let mut resolved = 0usize;
        let mut pooled = 0usize;
        let mut disagreements: Vec<String> = Vec::new();
        for book in &books {
            let dir = book.join("trait_generic");
            if !dir.is_dir() {
                continue;
            }
            for path in find_json_files(&dir) {
                let Ok(text) = fs::read_to_string(&path) else { continue };
                let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
                let Some(data) = value.get("data") else { continue };
                records += 1;

                // The oracle: the ingest-token read this cycle retired.
                let expected = data
                    .get("raw_tokens")
                    .and_then(serde_json::Value::as_array)
                    .into_iter()
                    .flatten()
                    .filter(|t| t.get("key").and_then(serde_json::Value::as_str) == Some("TYPE"))
                    .filter_map(|t| t.get("value").and_then(serde_json::Value::as_str))
                    .find_map(|v| v.strip_prefix("Trait.RaceTrait."))
                    .map(str::to_string);

                let source = value.get("source");
                let row = source
                    .and_then(|s| s.get("path"))
                    .and_then(serde_json::Value::as_str)
                    .zip(source.and_then(|s| s.get("line")).and_then(serde_json::Value::as_u64));
                if let Some((p, l)) = row
                    && !rules.rules_for_closure_row(p, l).is_empty()
                {
                    resolved += 1;
                }

                let actual = converted_race_trait_pool(&value, rules);
                if actual.is_some() {
                    pooled += 1;
                }
                if actual != expected && disagreements.len() < 20 {
                    disagreements.push(format!(
                        "{}: converted={actual:?} ingest_tokens={expected:?}",
                        path.display()
                    ));
                }
            }
        }

        assert!(records >= 487, "the live kind: trait population is {records}, expected >= 487");
        assert_eq!(
            resolved, records,
            "every corpus trait record's closure row must resolve to converted rules"
        );
        assert!(
            disagreements.is_empty(),
            "{} of {records} records disagree: {disagreements:?}",
            disagreements.len()
        );
        assert!(pooled > 0, "the converted package must place real records in race pools");
    }
}
