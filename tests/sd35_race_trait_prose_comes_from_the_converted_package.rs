//! SD-35 `AT-35-E6-003` — a racial trait's sentence is rendered from the **converted package**,
//! and it is the same sentence.
//!
//! # What moved
//!
//! `RaceTraitRecord::render_description` used to read the record's `DESC:` tokens and hand them
//! to the PCGen renderer at run time, and `same_row_display_values` used to read the row's
//! `DEFINE` bases and `BONUS:VAR` amounts out of the ingest arrays. Both are ingest-format
//! readers in the code that prints a character sheet, which `decisions.md §11` forbids and this
//! criterion closes. They read `data/sheet_rules/` now — the record's own words as plain-English
//! pieces with typed holes over our own `Expr`, its gates as converted `Applies`, and every
//! contribution to every variable in `_vars/<VarId>.json`.
//!
//! # Why this file is the evidence
//!
//! The conversion is only worth anything if the player reads the **same words**. So this renders
//! every racial-trait record in the whole shipped corpus **both ways** — the live path through
//! the converted package, and the same record's `DESC:` tokens through the tool-side PCGen
//! renderer seeded by the tool-side reading of the same row — and requires byte-identical text.
//!
//! The PCGen renderer is the **oracle**, and it stays exactly where `decisions.md §11` says an
//! oracle belongs: in `tests/`, never on the live side. This file is the only thing in the
//! racial-trait path that still names it.
//!
//! A disagreement here is not a tolerance to widen. It is either a converter defect (the record
//! converted wrong) or a renderer defect (the live reader reads it wrong), and both are this
//! criterion's business.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use codex::pcgen_import::bonus_chain_reader;
use codex::pcgen_import::pcgen_desc::{
    leaked_pcgen_syntax, render_pcgen_desc_tokens, PcgenDisplayValues,
};
use codex::pcgen_import::race_trait_tokens;
use codex::rules_core::corpus_loader::BookCorpusRoot;
use codex::rules_core::race_resolver::{load_race_corpus, RaceCorpus, RaceTraitRecord};

/// Every book directory under `data/corpus/`, so the population is the shipped corpus and not a
/// roster this file chose.
fn corpus_roots() -> Vec<(String, PathBuf)> {
    let mut roots: Vec<(String, PathBuf)> = std::fs::read_dir("data/corpus")
        .expect("data/corpus is readable")
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with('_') {
                return None;
            }
            Some((name, entry.path()))
        })
        .collect();
    roots.sort();
    roots
}

fn corpus() -> RaceCorpus {
    let owned = corpus_roots();
    let roots: Vec<BookCorpusRoot> = owned
        .iter()
        .map(|(book_id, dir)| BookCorpusRoot { book_id, dir: dir.as_path() })
        .collect();
    load_race_corpus(&roots)
}

/// The **oracle** side's reading of one row's own display values: the ingest arrays, folded the
/// way `same_row_display_values` folded them before this criterion moved it to the converted
/// `_vars` tables. A variable any other row also moves, or whose base is declared elsewhere,
/// does not resolve — that refusal is the property both sides must agree on.
fn oracle_same_row_values(record: &RaceTraitRecord) -> PcgenDisplayValues {
    let mut accumulator: BTreeMap<String, Option<i64>> = BTreeMap::new();
    for (name, base) in race_trait_tokens::same_row_defines(&record.data) {
        accumulator.insert(name, base);
    }
    for contribution in bonus_chain_reader::declared_bonuses(&record.data).var_contributions {
        let (name, amount) = (contribution.name, contribution.amount);
        match accumulator.get_mut(&name) {
            None => {
                accumulator.insert(name, None);
            }
            Some(slot) => {
                *slot = match (*slot, amount) {
                    (Some(current), Some(add)) => Some(current + add),
                    _ => None,
                };
            }
        }
    }
    let mut values = PcgenDisplayValues::new();
    for (name, resolved) in accumulator {
        if let Some(value) = resolved {
            values.set(&name, value);
        }
    }
    values
}

/// The oracle's whole answer for one record, including the two fallbacks the live path keeps:
/// a PI-redacted record serves its stored marker, and a record with no rendered prose at all
/// serves its stored description.
fn oracle_text(record: &RaceTraitRecord) -> String {
    if record.description_redacted {
        return record.data.description.clone().unwrap_or_default();
    }
    let tokens: Vec<&str> = race_trait_tokens::description_segments(&record.data);
    if tokens.is_empty() {
        return record.data.description.clone().unwrap_or_default();
    }
    let rendered = render_pcgen_desc_tokens(&tokens, &oracle_same_row_values(record));
    if rendered.text.is_empty() {
        return record.data.description.clone().unwrap_or_default();
    }
    rendered.text
}

/// **The gate.** Every racial-trait record in the shipped corpus renders the same sentence
/// through the converted package as it does through the PCGen renderer over its own `DESC:`
/// tokens.
///
/// Reports every disagreement, with both texts, rather than failing on the first: a count is
/// what tells a reader whether a change is one record's converter defect or a systematic one.
#[test]
fn every_racial_trait_renders_the_same_sentence_from_the_converted_package() {
    let corpus = corpus();
    let mut compared = 0usize;
    let mut disagreements: Vec<String> = Vec::new();

    for race_key in corpus.race_keys() {
        for record in corpus.traits_for(race_key) {
            compared += 1;
            let live = record.render_description(&record.same_row_display_values()).text;
            let oracle = oracle_text(record);
            if live != oracle {
                disagreements.push(format!(
                    "{}\n    converted: {live:?}\n    oracle:    {oracle:?}",
                    record.data.key
                ));
            }
        }
    }

    // The denominator, re-derived off disk every run: every racial-trait record every book
    // under `data/corpus/` loads, reachable from a race. A record that joins the corpus joins
    // this gate, and a drop in the count is a corpus regression rather than a quieter gate.
    assert_eq!(
        compared, 919,
        "the population is the whole shipped corpus's racial traits, not a sample"
    );
    assert!(
        disagreements.is_empty(),
        "{} of {compared} racial-trait records render different words through the converted \
         package than through the corpus DESC tokens:\n{}",
        disagreements.len(),
        disagreements.join("\n")
    );
}

/// No rendered racial-trait sentence carries ingest syntax to a player — the sheet rule
/// (`decisions.md §1`) stated as the property the converted package must make impossible.
#[test]
fn no_rendered_racial_trait_sentence_carries_ingest_syntax() {
    let corpus = corpus();
    let mut leaks: Vec<String> = Vec::new();
    for race_key in corpus.race_keys() {
        for record in corpus.traits_for(race_key) {
            let text = record.render_description(&record.same_row_display_values()).text;
            if let Some(leak) = leaked_pcgen_syntax(&text) {
                leaks.push(format!("{}: {leak}: {text}", record.data.key));
            }
        }
    }
    assert!(leaks.is_empty(), "{} leaks:\n{}", leaks.len(), leaks.join("\n"));
}

/// The converted variable id a live caller mints from a source name is the id the **converter**
/// wrote that variable's table under.
///
/// This is the join `display_values_with` stands on when it adds a feat's delta to a variable it
/// names, and it is a hash agreement rather than a transcription — so it either holds for every
/// name or is broken for all of them. Pinned against a real table on disk so a converter change
/// to how an id is minted cannot pass silently.
#[test]
fn a_var_id_minted_from_a_name_addresses_the_converters_own_table() {
    let package =
        codex::rules_core::corpus_loader::live_sheet_rules().expect("the converted package loads");
    let id = codex::rules_core::sheet_rule::var_id("Gnome_Hatred_AttackBonus");
    let table = package
        .vars
        .get(&id)
        .unwrap_or_else(|| panic!("the converter wrote no table under {id:?}"));
    assert!(
        table
            .declared_by
            .iter()
            .any(|rule| rule == "core_rulebook:race_trait:gnome_hatred"),
        "{id:?} is Gnome ~ Hatred's own variable: {:?}",
        table.declared_by
    );
}

/// The corpus directory this file reads is the one the converted package was built from.
#[test]
fn the_corpus_directory_exists() {
    assert!(Path::new("data/corpus").is_dir());
    assert!(Path::new("data/sheet_rules").is_dir());
}
