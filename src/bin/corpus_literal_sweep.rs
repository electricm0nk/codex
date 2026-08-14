//! Corpus-literal byte-equality gate CLI: every shipped JSON record under
//! `data/corpus/**/*.json` must byte-match the PCGen `.lst` literal it cites.
//!
//! This is the check a `static` unit's bar has always named and nothing
//! performed. `wiring_class` calls a record `static` when its whole token
//! closure is literal magnitudes (`literal_magnitudes_only`), which makes its
//! bar "the shipped bytes equal the corpus bytes" — knowable without any
//! consumer-delta probe, and until now unverified, which is why those units
//! sit at `held` rather than `done`. This binary supplies that evidence. It
//! does not decide anyone's doneness verdict and it does not write anything.
//!
//! Comparison rules, and why each is what it is, live in
//! [`codex::rules_core::corpus_literal_sweep`]. This file is the walker: it
//! resolves the corpus, reads the records, assembles each one's token closure
//! from `wiring_class`'s own `.MOD` index, and reports.
//!
//! Exit codes: `0` clean, `1` at least one byte-level mismatch,
//! `2` an I/O, parse, or empty-population failure. An empty population is
//! deliberately exit `2` and not exit `0`: a sweep that examined nothing
//! asserts nothing, and this repo has already shipped two gates that reported
//! success while checking nothing.
//!
//! Usage: `corpus_literal_sweep [--repo-root <path>] [--corpus-root <path>]
//! [--quiet] [--max-report <n>]`
//!
//! `PCGEN_CORPUS_ROOT` selects the corpus, defaulting to
//! `$HOME/workspace/repos/pcgen/data` — the same HOME-relative default
//! `v06_work_inventory` uses, for the same reason (`workspace/` is synced, an
//! absolute other-user path is not).

use codex::rules_core::corpus_literal_sweep::{
    compare_digest, compare_tokens, parse_document, tab_tokens, token_closure, Finding,
    ProvenanceClaim, ShippedRecord, SweepTally, SYNTHESIZED_TOKEN_KEYS,
};
use codex::rules_core::wiring_class::build_mod_index;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const CORPUS_RECORDS_REL: &str = "data/corpus";
const BOOKS_RELATIVE: &str = "pathfinder/paizo/roleplaying_game";
const LABEL: &str = "corpus-literal-sweep";

fn main() -> ExitCode {
    let mut repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut corpus_root: Option<PathBuf> = None;
    let mut quiet = false;
    let mut max_report = 40usize;
    let mut json_out: Option<PathBuf> = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--repo-root" => match args.next() {
                Some(v) => repo_root = PathBuf::from(v),
                None => return fatal("--repo-root needs a path"),
            },
            "--corpus-root" => match args.next() {
                Some(v) => corpus_root = Some(PathBuf::from(v)),
                None => return fatal("--corpus-root needs a path"),
            },
            "--max-report" => match args.next().and_then(|v| v.parse().ok()) {
                Some(v) => max_report = v,
                None => return fatal("--max-report needs a number"),
            },
            "--json-out" => match args.next() {
                Some(v) => json_out = Some(PathBuf::from(v)),
                None => return fatal("--json-out needs a path"),
            },
            "--quiet" => quiet = true,
            other => return fatal(&format!("unknown argument: {other}")),
        }
    }

    let corpus_root = match corpus_root {
        Some(explicit) => explicit,
        None => match std::env::var("PCGEN_CORPUS_ROOT") {
            Ok(configured) => PathBuf::from(configured),
            Err(_) => match std::env::var("HOME") {
                Ok(home) => PathBuf::from(home).join("workspace/repos/pcgen/data"),
                Err(_) => return fatal("HOME is unset; pass --corpus-root or set PCGEN_CORPUS_ROOT"),
            },
        },
    };
    let books_dir = corpus_root.join(BOOKS_RELATIVE);
    if !books_dir.is_dir() {
        return fatal(&format!(
            "corpus not found at {} -- set PCGEN_CORPUS_ROOT to a PCGen data/ checkout",
            books_dir.display()
        ));
    }
    let records_dir = repo_root.join(CORPUS_RECORDS_REL);
    if !records_dir.is_dir() {
        return fatal(&format!("shipped records not found at {}", records_dir.display()));
    }

    let mut sweep = Sweep::new(corpus_root);
    let json_files = find_json_files(&records_dir);
    if json_files.is_empty() {
        return fatal(&format!("no JSON records under {}", records_dir.display()));
    }

    // Pass 1: read every record into the sweep's shape, grouped by corpus
    // book, so each book's `.MOD` index is built once and only for books the
    // shipped records actually cite.
    let mut by_book: BTreeMap<String, Vec<ShippedRecord>> = BTreeMap::new();
    let mut claims: Vec<ProvenanceClaim> = Vec::new();
    for path in &json_files {
        sweep.tally.records_seen += 1;
        let rel = display_rel(path, &repo_root);
        let text = match std::fs::read_to_string(path) {
            Ok(t) => t,
            Err(e) => return fatal(&format!("cannot read {rel}: {e}")),
        };
        let parsed = match parse_document(&rel, &text) {
            Ok(p) => p,
            Err(e) => return fatal(&e),
        };
        if let Some(claim) = parsed.provenance {
            claims.push(claim);
        }
        if let Some(record) = parsed.record {
            let Some(book) = book_dir_of(&record.source_path) else {
                return fatal(&format!(
                    "{rel}: source.path {} is not <system>/<publisher>/<line>/<book>/<file>-shaped",
                    record.source_path
                ));
            };
            by_book.entry(book).or_default().push(record);
        }
    }

    let mut findings: Vec<Finding> = Vec::new();

    // Pass 2a: every provenance claim on the tree, token population or not.
    for claim in &claims {
        sweep.tally.digests_checked += 1;
        let corpus_file = sweep.corpus_root.join(&claim.source_path);
        if !corpus_file.is_file() {
            findings.push(Finding::CorpusFileMissing {
                record: claim.record_path.clone(),
                source_path: claim.source_path.clone(),
            });
            continue;
        }
        let digest = sweep.digest(&corpus_file);
        if let Some(f) = compare_digest(claim, &digest) {
            findings.push(f);
        }
    }

    // Pass 2b: the transcribed tokens, book by book.
    for (book, records) in &by_book {
        let mod_index = sweep.mod_index(book);
        // Only built for a book that really carries a synthesized token —
        // it is the whole book's token surface, and 9 records corpus-wide
        // need it (measured 2026-08-13).
        let needs_book_tokens = records.iter().any(|r| {
            r.tokens
                .iter()
                .any(|t| SYNTHESIZED_TOKEN_KEYS.iter().any(|(key, _)| *key == t.key))
        });
        let book_tokens =
            if needs_book_tokens { sweep.book_tokens(book) } else { BTreeSet::new() };

        for record in records {
            sweep.tally.records_examined += 1;
            let corpus_file = sweep.corpus_root.join(&record.source_path);
            // The base row is taken as an owned String rather than held as a
            // borrow of the line cache: `compare_tokens` below takes the
            // tally, which lives on the same cache-owning struct, and a gate
            // is not the place to fight the borrow checker over one avoided
            // allocation.
            let Some(line_count) = sweep.lines(&corpus_file).map(Vec::len) else {
                findings.push(Finding::CorpusFileMissing {
                    record: record.record_path.clone(),
                    source_path: record.source_path.clone(),
                });
                continue;
            };
            if record.source_line == 0 || record.source_line > line_count {
                findings.push(Finding::CorpusLineOutOfRange {
                    record: record.record_path.clone(),
                    source_path: record.source_path.clone(),
                    line: record.source_line,
                    file_lines: line_count,
                });
                continue;
            }
            let base_row = sweep
                .lines(&corpus_file)
                .map(|lines| lines[record.source_line - 1].clone())
                .unwrap_or_default();
            let closure = token_closure(&base_row, &record.identities, &mod_index);
            findings.extend(compare_tokens(record, &closure, &book_tokens, &mut sweep.tally));
        }
    }

    // A record with no corpus file at all is found once by the provenance
    // pass and again by the token pass. Both statements are true, but they
    // are the same fact, and a findings count that double-reports one defect
    // misleads in the direction this whole gate exists to avoid.
    findings.sort_by_key(Finding::describe);
    findings.dedup();

    let tally = &sweep.tally;
    if !quiet {
        println!(
            "{LABEL}: {} records examined of {} read, {} tokens compared ({} synthesized), {} digests checked, {} findings",
            tally.records_examined,
            tally.records_seen,
            tally.tokens_compared,
            tally.synthesized_tokens_compared,
            tally.digests_checked,
            findings.len(),
        );
    }

    // The zero-cases-ran guard, in the binary rather than only in the gate
    // script so no caller can get a false green. A population of zero is not
    // a clean tree, it is a broken sweep.
    //
    // It is conditioned on there being NO findings, and that condition is not
    // a softening: the self-test caught the first draft reporting "a sweep
    // that compares nothing proves nothing" over a record whose corpus line
    // was missing — a real, correctly-detected defect, described to the
    // reader as a broken instrument. Zero tokens compared *because every
    // record failed before its tokens could be reached* is the finding, and
    // it must be printed as one. Zero tokens compared with nothing to report
    // is still exit 2.
    if tally.records_examined == 0 || (tally.tokens_compared == 0 && findings.is_empty()) {
        eprintln!(
            "{LABEL}: examined {} records / {} tokens with nothing to report -- a sweep that compares nothing proves nothing",
            tally.records_examined, tally.tokens_compared
        );
        return ExitCode::from(2);
    }

    let clean = findings.is_empty();

    // Evidence for the doneness-verdict `static` done rung (operator
    // directive 2026-08-13, answering SD-32 decisions.md §2): a record is
    // only "verified" here if the WHOLE sweep came back CLEAN (a red sweep
    // proves nothing about any individual record -- one book's mismatch does
    // not tell you another book's records are fine, so it credits none) AND
    // this specific record went through pass 2b's token comparison
    // (`!record.tokens.is_empty()`) -- the digest-only pass (2a) is a weaker
    // bar and never credited here, same rule `static-sweep-coverage.py`
    // documents. `v06_work_inventory` joins this file's `(book, source_file,
    // source_line)` triples against its own units to decide which ones may
    // carry `literal-verified`; it does not trust anything this binary did
    // not itself byte-compare.
    if let Some(path) = &json_out {
        let mut entries: Vec<String> = Vec::new();
        if clean {
            for records in by_book.values() {
                for record in records {
                    if record.tokens.is_empty() {
                        continue;
                    }
                    // `v06_work_inventory`'s units carry the SHORT book name
                    // (e.g. `advanced_class_guide`), not the 4-segment
                    // `book_dir_of()` grouping key this binary uses
                    // internally (e.g. `pathfinder/paizo/roleplaying_game/
                    // advanced_class_guide`) -- that key is the immediate
                    // parent directory of `source_file`.
                    let source_path = Path::new(&record.source_path);
                    let (Some(source_file), Some(short_book)) = (
                        source_path.file_name().and_then(|f| f.to_str()),
                        source_path
                            .parent()
                            .and_then(|p| p.file_name())
                            .and_then(|f| f.to_str()),
                    ) else {
                        continue;
                    };
                    entries.push(format!(
                        "{{\"book\":{},\"source_file\":{},\"source_line\":{}}}",
                        json_string(short_book),
                        json_string(source_file),
                        record.source_line
                    ));
                }
            }
        }
        let body = format!(
            "{{\"clean\":{},\"records_examined\":{},\"verified\":[{}]}}\n",
            clean,
            tally.records_examined,
            entries.join(",")
        );
        if let Err(e) = std::fs::write(path, body) {
            return fatal(&format!("--json-out: cannot write {}: {e}", path.display()));
        }
    }

    if clean {
        println!("{LABEL}: CLEAN");
        return ExitCode::SUCCESS;
    }

    for finding in findings.iter().take(max_report) {
        println!("{LABEL}: MISMATCH {}", finding.describe());
    }
    if findings.len() > max_report {
        println!("{LABEL}: ... {} further findings suppressed", findings.len() - max_report);
    }
    let affected: BTreeSet<&str> = findings.iter().map(Finding::record).collect();
    println!("{LABEL}: {} findings across {} records", findings.len(), affected.len());
    ExitCode::from(1)
}

/// Minimal JSON string escaping for the `--json-out` report. Corpus book
/// names and `.lst` filenames are plain ASCII in this repo; this only
/// guards against a stray quote/backslash rather than implementing full
/// JSON escaping.
fn json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}

fn fatal(message: &str) -> ExitCode {
    eprintln!("{LABEL}: {message}");
    ExitCode::from(2)
}

/// The corpus-relative directory of the book a `source.path` belongs to.
///
/// PCGen files a book four segments deep — `<system>/<publisher>/<line>/<book>`
/// — and the line segment is NOT always `roleplaying_game`. Anchoring on that
/// one line was this sweep's own first defect: the 71 shipped records citing
/// `pathfinder/paizo/campaign_setting/inner_sea_races` were rejected outright,
/// and `v06_work_inventory` carries thirteen such directories in its
/// `additional_book_dirs`. Taking the first four segments is the rule the
/// corpus layout actually follows, and it needs no book list to maintain.
fn book_dir_of(source_path: &str) -> Option<String> {
    let segments: Vec<&str> = source_path.split('/').filter(|s| !s.is_empty()).collect();
    if segments.len() < 5 {
        return None;
    }
    Some(segments[..4].join("/"))
}

fn display_rel(path: &Path, repo_root: &Path) -> String {
    path.strip_prefix(repo_root).unwrap_or(path).to_string_lossy().replace('\\', "/")
}

fn find_json_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("json")
                && path.file_name().and_then(|f| f.to_str()) != Some("LICENSE.json")
            {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

/// Caches the corpus reads the two passes share.
struct Sweep {
    corpus_root: PathBuf,
    lines: BTreeMap<PathBuf, Option<Vec<String>>>,
    digests: BTreeMap<PathBuf, String>,
    tally: SweepTally,
}

impl Sweep {
    fn new(corpus_root: PathBuf) -> Self {
        Sweep {
            corpus_root,
            lines: BTreeMap::new(),
            digests: BTreeMap::new(),
            tally: SweepTally::default(),
        }
    }

    fn lines(&mut self, path: &Path) -> Option<&Vec<String>> {
        if !self.lines.contains_key(path) {
            let read = std::fs::read_to_string(path)
                .ok()
                .map(|t| t.split('\n').map(str::to_string).collect::<Vec<_>>());
            self.lines.insert(path.to_path_buf(), read);
        }
        self.lines.get(path).and_then(Option::as_ref)
    }

    fn digest(&mut self, path: &Path) -> String {
        if let Some(hit) = self.digests.get(path) {
            return hit.clone();
        }
        let digest = match std::fs::read(path) {
            Ok(bytes) => {
                let mut hasher = Sha256::new();
                hasher.update(&bytes);
                format!("{:x}", hasher.finalize())
            }
            Err(_) => String::new(),
        };
        self.digests.insert(path.to_path_buf(), digest.clone());
        digest
    }

    /// This book's `.MOD` rows, keyed by the record name they target.
    /// Delegates to `wiring_class::build_mod_index` — the `.MOD` discovery
    /// rule has exactly one implementation in this crate and this is not a
    /// second one.
    fn mod_index(&self, book_dir: &str) -> BTreeMap<String, Vec<String>> {
        let mut book_paths = BTreeMap::new();
        book_paths.insert(book_dir.to_string(), self.corpus_root.join(book_dir));
        build_mod_index(&book_paths)
            .into_iter()
            .map(|((_, name), rows)| (name, rows))
            .collect()
    }

    /// Every tab field of every `.lst` row in one book — the surface a
    /// synthesized token is checked against.
    fn book_tokens(&self, book_dir: &str) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        for path in lst_files(&self.corpus_root.join(book_dir)) {
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            for raw in text.split('\n') {
                for field in tab_tokens(raw) {
                    out.insert(field.to_string());
                }
                // Field 0 of a `.MOD` row is a head, but field 0 of an
                // ordinary row can still be the record's display name; a
                // synthesized token is never found there, so `tab_tokens`'
                // skip is correct and deliberate here too.
            }
        }
        out
    }
}

fn lst_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("lst") {
                out.push(path);
            }
        }
    }
    out
}
