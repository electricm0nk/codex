//! `SD31-W14-INTEGRATE-001` — the anti-reversion gate for `SD31-E6-F5-005`.
//!
//! **The CONFIRMED finding this file exists for.** That cycle narrowed 412
//! already-shipped equipment records' provenance from a web page to the pinned
//! oracle's own `.lst` row, moving the web citation intact into
//! `description_source`. The adversarial review then found that BOTH upstream
//! cache generators still emit the pre-repair shape —
//! `cache_gen::apg::generate_equipment` stamps `Source::WebSecondSource`
//! unconditionally, and `gen_core_rulebook_cache::equipment_source` returns
//! `web_second_source` for exactly the shape that was repaired (a real Rust
//! description with no `DESC:` token on the row). Neither generator knows the
//! `description_source` key, and no `verify.sh` stage runs either generator,
//! so re-running one silently reverts the repair and withdraws up to 346 board
//! units.
//!
//! Worse, the reformulated `sd26_cache_*` distribution test could not detect
//! it: it read `description_source` first and FELL BACK to `source.kind`, so a
//! regenerated record — which has no `description_source` and reads
//! `web_second_source` again — kept the same count and stayed green.
//!
//! This gate makes the reversion loud. It pins the repaired population by
//! book, in both directions: how many records carry the narrowed shape, and
//! how many still carry an un-narrowed `web_second_source` in `source`. A
//! regeneration that reverts the repair moves both numbers and fails here.
//!
//! It is deliberately a COUNT pin over the committed corpus rather than a
//! re-derivation, because a re-derivation would just re-run the same logic the
//! repair tool ran and agree with itself.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

fn corpus_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("data/corpus")
}

fn record_files(book: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![corpus_root().join(book)];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else { continue };
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().and_then(|n| n.to_str()) == Some("_parity") {
                    continue;
                }
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("json")
                && path.file_name().and_then(|n| n.to_str()) != Some("LICENSE.json")
            {
                out.push(path);
            }
        }
    }
    out
}

/// `(narrowed, un-narrowed)` for one book: how many records carry a
/// `description_source` whose kind is `web_second_source` (the repaired
/// shape), and how many still carry `web_second_source` in `source` itself.
fn provenance_split(book: &str) -> (usize, usize) {
    let mut narrowed = 0usize;
    let mut un_narrowed = 0usize;
    for path in record_files(book) {
        let Ok(text) = fs::read_to_string(&path) else { continue };
        let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
        if value.get("source").is_none() {
            continue;
        }
        if value["description_source"]["kind"] == "web_second_source" {
            narrowed += 1;
            assert_eq!(
                value["source"]["kind"], "lst_token",
                "{}: a narrowed record's own `source` must be the pinned oracle row it was \
                 narrowed to, not another web citation",
                path.display()
            );
        }
        if value["source"]["kind"] == "web_second_source" {
            un_narrowed += 1;
        }
    }
    (narrowed, un_narrowed)
}

/// The pinned population, re-derived 2026-08-18 at the wave-14 integration tip
/// with `python3` over `data/corpus/<book>/**/*.json`, counting
/// `description_source.kind` and `source.kind` separately.
///
/// `core_rulebook` reaching ZERO un-narrowed records is the load-bearing half:
/// every one of its 82 web-sourced equipmods was narrowed, so a single
/// `web_second_source` reappearing in `source` there IS the regeneration
/// reverting the repair.
const EXPECTED: &[(&str, usize, usize)] = &[
    // (book, narrowed `description_source`, remaining un-narrowed `source`)
    ("core_rulebook", 82, 0),
    ("advanced_players_guide", 330, 7),
    ("beastiary", 0, 1),
];

#[test]
fn the_narrowed_provenance_population_is_exactly_what_the_repair_left() {
    let mut drift: Vec<String> = Vec::new();
    for (book, want_narrowed, want_un_narrowed) in EXPECTED {
        let (narrowed, un_narrowed) = provenance_split(book);
        if narrowed != *want_narrowed || un_narrowed != *want_un_narrowed {
            drift.push(format!(
                "{book}: narrowed(description_source)={narrowed} (expected {want_narrowed}), \
                 un-narrowed(source)={un_narrowed} (expected {want_un_narrowed})"
            ));
        }
    }
    assert!(
        drift.is_empty(),
        "the SD31-E6-F5-005 provenance narrowing has drifted. If a cache generator was re-run, \
         it REVERTED the repair -- `cache_gen::apg::generate_equipment` and \
         `gen_core_rulebook_cache::equipment_source` both still emit the pre-repair \
         `web_second_source` shape and neither knows the `description_source` key \
         (SD31-W14-INTEGRATE-001, OPEN-ISSUES). Re-run \
         `cargo run --locked --bin repair_lst_provenance` before restating these numbers, and \
         do not restate them to match a regeneration.\n  {}",
        drift.join("\n  ")
    );
}

/// Every narrowed record's web citation must still be complete. The whole
/// point of moving rather than deleting it is that it is the OGL/attribution
/// record for the prose a player reads; a `description_source` missing its
/// `url` or `identity_match_basis` is a licensing artifact that lost its
/// content, which no count pin above would notice.
#[test]
fn every_narrowed_records_web_citation_is_still_whole() {
    let mut broken: Vec<String> = Vec::new();
    let mut seen = 0usize;
    for (book, ..) in EXPECTED {
        for path in record_files(book) {
            let Ok(text) = fs::read_to_string(&path) else { continue };
            let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
            if value["description_source"]["kind"] != "web_second_source" {
                continue;
            }
            seen += 1;
            for field in ["url", "fetched_at", "identity_match_basis"] {
                if !value["description_source"][field].is_string() {
                    broken.push(format!("{}: description_source.{field} missing", path.display()));
                }
            }
        }
    }
    assert!(broken.is_empty(), "{} incomplete web citations:\n  {}", broken.len(), broken.join("\n  "));
    assert_eq!(seen, 412, "the narrowed population is 412 records corpus-wide");
}

/// The field must survive a typed round-trip through the canonical record
/// struct. Before `SD31-W14-INTEGRATE-001` added it to
/// `shape_b_v1::CorpusRecordV1`, `description_source` was off-schema: the
/// struct carries no `deny_unknown_fields`, so deserializing and re-serializing
/// any repaired record silently DROPPED the web citation for good.
#[test]
fn description_source_survives_a_typed_round_trip_through_the_canonical_struct() {
    let sample = record_files("core_rulebook")
        .into_iter()
        .find(|p| {
            fs::read_to_string(p)
                .ok()
                .and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok())
                .is_some_and(|v| v["description_source"]["kind"] == "web_second_source")
        })
        .expect("at least one narrowed core_rulebook record");

    let text = fs::read_to_string(&sample).unwrap();
    let typed: codex::rules_core::shape_b_v1::CorpusRecordV1<serde_json::Value> =
        serde_json::from_str(&text).expect("a shipped record parses as the canonical struct");
    assert!(
        typed.description_source.is_some(),
        "{}: description_source was dropped by the typed parse",
        sample.display()
    );

    let round_tripped: serde_json::Value =
        serde_json::to_value(&typed).expect("re-serializes");
    let original: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(
        round_tripped["description_source"], original["description_source"],
        "{}: the web citation must survive serialize(deserialize(record)) byte for byte",
        sample.display()
    );
}

/// A regression map so a future cycle can see, per book and per kind, exactly
/// which records the repair touched — the count pins above tell you something
/// moved, this tells you where to look.
#[test]
fn the_narrowed_population_is_confined_to_equipment_kinds() {
    let mut by_kind: BTreeMap<String, usize> = BTreeMap::new();
    for (book, ..) in EXPECTED {
        for path in record_files(book) {
            let Ok(text) = fs::read_to_string(&path) else { continue };
            let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
            if value["description_source"]["kind"] != "web_second_source" {
                continue;
            }
            let kind = path
                .parent()
                .and_then(|p| {
                    let rel = p.strip_prefix(corpus_root().join(book)).ok()?;
                    rel.components().next().map(|c| c.as_os_str().to_string_lossy().into_owned())
                })
                .unwrap_or_else(|| "?".to_string());
            *by_kind.entry(kind).or_insert(0) += 1;
        }
    }
    assert_eq!(
        by_kind.keys().cloned().collect::<Vec<_>>(),
        vec!["equipment".to_string()],
        "the narrowing was scoped to equipment records; it has spread: {by_kind:?}"
    );
}
