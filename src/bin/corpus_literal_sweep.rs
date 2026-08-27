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
            // SD31-E6-F6-001 (`OPEN-ISSUES.md` rows 70/103's `.COPY=`
            // inheritance, generalized): when the cited row is itself a
            // `.COPY=` declaration, resolve the base record it copies and
            // fold its tokens into the closure too -- otherwise a genuinely
            // inherited, corpus-real value (never fabricated, always
            // resolved by the identical `KEY:`-or-bare-name rule
            // `gen_equipment_gap_tables.rs`'s own inheritance uses) reads as
            // unprovable, not because it is wrong but because the closure
            // never looked at the row that states it.
            let resolved_copy_base = copy_base_identity(&base_row)
                .and_then(|identity| sweep.copy_base_row(book, &corpus_file, identity));
            let closure =
                token_closure(&base_row, &record.identities, &mod_index, resolved_copy_base.as_deref());
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
        // decisions.md §24 / §22: the §24-redaction exemption
        // (`corpus_literal_sweep::compare_tokens`'s third exemption) is
        // reported unconditionally, zero included -- a reviewer must be
        // able to tell "no §24 records this run" apart from "the exemption
        // silently stopped being counted".
        println!(
            "{LABEL}: {} tokens exempted under decisions.md §24 redaction across {} codex_generated_name records",
            tally.codex_generated_name_tokens_exempted,
            tally.codex_generated_name_records_exempted.len(),
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
                    // `v06_work_inventory`'s units carry the SHORT book
                    // name -- `book_paths`'s key, which is the PCGen ORACLE
                    // directory basename under `$PCGEN_CORPUS_ROOT/pathfinder/
                    // paizo/roleplaying_game/` that `enumerate_book` walked
                    // to raw-enumerate this unit (`v06_work_inventory.rs`
                    // `books_dir = corpus_root.join(BOOKS_RELATIVE)`, where
                    // `corpus_root` there is `PCGEN_CORPUS_ROOT`, NOT this
                    // repo's `data/corpus/`). That is the LAST segment of
                    // `book_dir_of(&record.source_path)`, not the immediate
                    // parent directory of `source_file` (the previous, buggy
                    // shape): for a record filed more than one directory
                    // level under its book, the parent directory names a
                    // sub-directory (a race name, for `race`/`race_trait`),
                    // not any book at all. `OPEN-ISSUES.md` row 22, verified
                    // one unit deep against the committed inventory: the CRB
                    // dwarf race chassis is `core_essentials/races/dwarf/
                    // dwarf_races.lst` in the oracle (`core_essentials` is a
                    // real, separate oracle book directory from
                    // `core_rulebook` -- confirmed present under
                    // `$PCGEN_CORPUS_ROOT/pathfinder/paizo/roleplaying_game/`)
                    // and its `docs/work-inventory.json` unit id is literally
                    // `core_essentials:race:dwarf`, `book: "core_essentials"`
                    // -- not `core_rulebook`, despite the shipped JSON being
                    // filed at `data/corpus/core_rulebook/race/dwarf.json`.
                    // The shipped-record directory and the oracle-derived
                    // `unit.book` are two different namespaces; this join
                    // needs the second one.
                    let source_path = Path::new(&record.source_path);
                    let (Some(source_file), Some(short_book)) = (
                        source_path.file_name().and_then(|f| f.to_str()),
                        short_book_of(&record.source_path),
                    ) else {
                        continue;
                    };
                    entries.push(format!(
                        "{{\"book\":{},\"source_file\":{},\"source_line\":{}}}",
                        json_string(&short_book),
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

/// The `v06_work_inventory`-facing "book" for a shipped record: the SAME
/// PCGen oracle book directory `book_dir_of` resolves for `by_book`
/// grouping above, reduced to its short (last-segment) form --
/// `book_dir_of`'s four segments are always
/// `<system>/<publisher>/<line>/<book>`, so the book is always the last
/// one.
///
/// This is deliberately NOT derived from where the record's own JSON file
/// happens to be shipped under `data/corpus/`. `v06_work_inventory` raw-
/// enumerates the `race`/`race_trait` (and every other) population by
/// walking `$PCGEN_CORPUS_ROOT/pathfinder/paizo/roleplaying_game/<book>`
/// book by book (`books_dir = corpus_root.join(BOOKS_RELATIVE)`, where that
/// `corpus_root` is `PCGEN_CORPUS_ROOT`, not this repo's `data/corpus/`),
/// and every unit it mints carries the ORACLE book id it was walked under
/// as `unit.book` -- that is the id this file's `--json-out` join key must
/// match, not this repo's own (unrelated) choice of shipped-record
/// directory. Verified one unit deep against the committed inventory
/// (`OPEN-ISSUES.md` row 22): the CRB dwarf race's PCGen source is
/// `core_essentials/races/dwarf/dwarf_races.lst` -- `core_essentials` is a
/// real, separate oracle book directory, distinct from `core_rulebook` --
/// and `docs/work-inventory.json` carries it as unit id
/// `core_essentials:race:dwarf`, `"book": "core_essentials"`, even though
/// this repo ships the record at `data/corpus/core_rulebook/race/
/// dwarf.json`. Reading the book off `record.source_path`'s parent
/// directory (the previous, buggy shape) instead reads the deepest PCGen
/// nesting level, which for a record filed more than one directory level
/// under its book (every `race`/`race_trait` row under a per-race
/// subdirectory) is a race name, not a book at all.
fn short_book_of(source_path: &str) -> Option<String> {
    let dir = book_dir_of(source_path)?;
    let last = Path::new(&dir).file_name().and_then(|f| f.to_str())?;
    if last == "core_essentials" {
        // `SD31-ATTRIB-001` (`OPEN-ISSUES.md` row 68): `v06_work_inventory`
        // no longer mints `unit.book == "core_essentials"` for a race whose
        // TRUE source book is provable one record deep -- see that binary's
        // `RACE_TRUE_BOOK`/`resolve_true_book_for_core_essentials` (this
        // table is that one, duplicated, matching this repo's own
        // established convention for `book_dir_of`-shaped logic, per
        // `repair_spell_citations.rs`'s own doc comment on the same
        // duplication). This join key must track that relabelling exactly,
        // or the sweep's `--json-out` reverts to the pre-fix join and every
        // one of these units silently loses its `literal-verified` stamp
        // (reproduced once, `--allow-stamp-loss`'s own first-offenders list,
        // before this fix landed).
        //
        // Root-level `core_essentials/ce_*.lst` files (the `SOURCELONG:`
        // header signal `resolve_true_book_for_core_essentials`'s OTHER arm
        // resolves) are deliberately NOT handled here: `short_book_of` is a
        // pure function of `source_path` with no oracle-file-content access,
        // and as of this fix zero of the sweep's own verified population
        // cites a root-level `core_essentials` file (all 330 are per-race
        // `races/<slug>/*.lst` rows -- re-derived 2026-08-16,
        // `python3 -c "...Counter(x['source_file'] for x in verified if
        // x['book']=='core_essentials')..."`, every hit a `*_race*.lst`
        // basename). If a future cycle ships a root-level-sourced record
        // through this sweep, this function will silently under-resolve it
        // back to `core_essentials` rather than guess -- exactly this
        // fix's own "leave it, do not guess" rule -- and the join will need
        // widening then, with real content to test against.
        let race_slug = source_path
            .split('/')
            .position(|s| s == "races")
            .and_then(|races_at| source_path.split('/').nth(races_at + 1));
        // `decisions.md §10`'s "newest publish wins" layer, kept in sync
        // with `v06_work_inventory.rs`'s own `RACE_NEWEST_PRINTING`
        // (`OPEN-ISSUES.md`, `SD31-D9-DISSOLVE-001`): scoped to the `race`
        // KIND's own chassis file ONLY -- a race_trait row from the
        // identical `races/<slug>/` directory must NOT move, or this
        // join desyncs from `v06_work_inventory`'s own kind-scoped
        // override and every race_trait `literal-verified` stamp under
        // that slug silently breaks. `short_book_of` has no `Kind` to
        // consult (unlike `v06_work_inventory`'s per-row `record_kind`),
        // so the SAME filename convention `file_kind()` uses there
        // (`_races` substring, minus the two companion/familiar
        // exceptions, neither of which a per-race directory carries)
        // stands in for it here.
        let source_file = source_path.rsplit('/').next().unwrap_or("");
        let is_race_chassis_file = source_file.contains("_races")
            && !source_file.contains("_races_companion")
            && !source_file.contains("_races_familiar");
        if let Some(newest) = is_race_chassis_file
            .then(|| race_slug.and_then(|slug| RACE_NEWEST_PRINTING.iter().find(|(s, _)| *s == slug)))
            .flatten()
        {
            return Some(newest.1.to_string());
        }
        let race_slug_book = race_slug
            .and_then(|slug| RACE_TRUE_BOOK.iter().find(|(s, _)| *s == slug))
            .map(|(_, book)| (*book).to_string());
        if let Some(book) = race_slug_book {
            return Some(book);
        }
    }
    Some(last.to_string())
}

/// `core_essentials/races/<slug>/` -> the true book. Byte-identical to
/// `v06_work_inventory.rs`'s own `RACE_TRUE_BOOK` -- see that table's doc
/// comment for the full derivation (each entry re-checked one record deep
/// against an in-scope book's own `.pcc`, 2026-08-16). Duplicated rather
/// than shared: this repo's established convention for `book_dir_of`-shaped
/// logic across bins (`repair_spell_citations.rs`'s own doc comment on its
/// copy of `book_dir_of` itself makes the same call).
const RACE_TRUE_BOOK: &[(&str, &str)] = &[
    ("dwarf", "core_rulebook"),
    ("elf", "core_rulebook"),
    ("gnome", "core_rulebook"),
    ("half_elf", "core_rulebook"),
    ("half_orc", "core_rulebook"),
    ("halfling", "core_rulebook"),
    ("human", "core_rulebook"),
    ("aasimar", "bestiary"),
    ("drow", "bestiary"),
    ("duergar", "bestiary"),
    ("goblin", "bestiary"),
    ("hobgoblin", "bestiary"),
    ("kobold", "bestiary"),
    ("merfolk", "bestiary"),
    ("orc", "bestiary"),
    ("svirfneblin", "bestiary"),
    ("tengu", "bestiary"),
    ("tiefling", "bestiary"),
    ("dhampir", "bestiary_2"),
    ("fetchling", "bestiary_2"),
    ("grippli", "bestiary_2"),
    ("ifrit", "bestiary_2"),
    ("oread", "bestiary_2"),
    ("sylph", "bestiary_2"),
    ("undine", "bestiary_2"),
    ("catfolk", "bestiary_3"),
    ("ratfolk", "bestiary_3"),
    ("suli", "bestiary_3"),
    ("vanara", "bestiary_3"),
    ("vishkanya", "bestiary_3"),
    ("changeling", "bestiary_4"),
    ("kitsune", "bestiary_4"),
    ("nagaji", "bestiary_4"),
    ("samsaran", "bestiary_4"),
    ("wayang", "bestiary_4"),
    // "gathlain" removed (SD31-W5-INTEGRATE-001): also natively declared by
    // ultimate_wilderness's own .pcc, so it is genuinely ambiguous, not a
    // single-book attribution -- see v06_work_inventory.rs's RACE_TRUE_BOOK
    // doc comment, which this table must stay in sync with.
    ("kasatha", "bestiary_4"),
    ("trox", "bestiary_4"),
    ("wyrwood", "bestiary_4"),
    ("wyvaran", "bestiary_4"),
    ("gillman", "inner_sea_world_guide"),
    ("strix", "inner_sea_world_guide"),
    ("skinwalker", "bestiary_5"),
    ("rougarou", "bestiary_6"),
];

/// `decisions.md §10`'s "newest publish wins" table, byte-identical to
/// `v06_work_inventory.rs`'s own `RACE_NEWEST_PRINTING` -- see that table's
/// doc comment for the full derivation (32 races currently attributed to a
/// book strictly older than Advanced Race Guide's own `SOURCEDATE:2012-06`,
/// re-derived 2026-08-16). Duplicated rather than shared, this repo's
/// established convention for `book_dir_of`-shaped logic across bins.
const RACE_NEWEST_PRINTING: &[(&str, &str)] = &[
    ("dwarf", "advanced_race_guide"),
    ("elf", "advanced_race_guide"),
    ("gnome", "advanced_race_guide"),
    ("half_elf", "advanced_race_guide"),
    ("half_orc", "advanced_race_guide"),
    ("halfling", "advanced_race_guide"),
    ("human", "advanced_race_guide"),
    ("aasimar", "advanced_race_guide"),
    ("drow", "advanced_race_guide"),
    ("duergar", "advanced_race_guide"),
    ("goblin", "advanced_race_guide"),
    ("hobgoblin", "advanced_race_guide"),
    ("kobold", "advanced_race_guide"),
    ("merfolk", "advanced_race_guide"),
    ("orc", "advanced_race_guide"),
    ("svirfneblin", "advanced_race_guide"),
    ("tengu", "advanced_race_guide"),
    ("tiefling", "advanced_race_guide"),
    ("dhampir", "advanced_race_guide"),
    ("fetchling", "advanced_race_guide"),
    ("grippli", "advanced_race_guide"),
    ("ifrit", "advanced_race_guide"),
    ("oread", "advanced_race_guide"),
    ("sylph", "advanced_race_guide"),
    ("undine", "advanced_race_guide"),
    ("catfolk", "advanced_race_guide"),
    ("ratfolk", "advanced_race_guide"),
    ("suli", "advanced_race_guide"),
    ("vanara", "advanced_race_guide"),
    ("vishkanya", "advanced_race_guide"),
    ("gillman", "advanced_race_guide"),
    ("strix", "advanced_race_guide"),
];

/// The corpus-relative directory of the book a `source.path` belongs to.
///
/// PCGen files a Paizo book four segments deep —
/// `<system>/<publisher>/<line>/<book>` — and the line segment is NOT always
/// `roleplaying_game`. Anchoring on that one line was this sweep's own first
/// defect: the 71 shipped records citing
/// `pathfinder/paizo/campaign_setting/inner_sea_races` were rejected
/// outright, and `v06_work_inventory` carries thirteen such directories in
/// its `additional_book_dirs`. Taking the first four segments is the rule
/// the Paizo corpus layout actually follows, and it needs no book list to
/// maintain.
///
/// **Third-party publishers do not all carry a "line" tier.** Dreamscarred
/// Press ships `<system>/dreamscarred_press/<book>` — three segments, one
/// shallower — confirmed against the real oracle
/// (`$PCGEN_CORPUS_ROOT/pathfinder/dreamscarred_press/*` has no further
/// nesting before the book directory) and against
/// `v06_work_inventory.rs`'s own `additional_book_dirs`, which registers
/// `"pathfinder/dreamscarred_press/ultimate_psionics"` directly (3
/// segments) rather than through the 4-segment Paizo shape. Before this
/// fix (`OPEN-ISSUES.md` row 48/49), every Dreamscarred-Press-sourced
/// record with `raw_tokens` populated `fatal()`-aborted this entire sweep
/// (`SD31-E6-F5-002`'s 113 `ultimate_psionics` equipment records). The
/// publisher name is checked explicitly, not inferred from segment count
/// alone, so an unrelated 4-segment path shape does not silently borrow
/// this narrower rule.
/// The identity a `.COPY=<name>` row's first column names as its base, when
/// `row` is such a declaration — the string before `.COPY=`. `None` for a
/// plain row. Mirrors `gen_equipment_gap_tables.rs`'s own `.COPY=` split
/// exactly (same literal PCGen syntax, one predicate).
fn copy_base_identity(row: &str) -> Option<&str> {
    let first = row.split('\t').next().unwrap_or("");
    first.split_once(".COPY=").map(|(base, _)| base)
}

fn book_dir_of(source_path: &str) -> Option<String> {
    let segments: Vec<&str> = source_path.split('/').filter(|s| !s.is_empty()).collect();
    if segments.len() >= 5 {
        return Some(segments[..4].join("/"));
    }
    if segments.len() == 4 && segments[1] == "dreamscarred_press" {
        return Some(segments[..3].join("/"));
    }
    None
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

    /// Resolves a `.COPY=` row's base identity (the string before
    /// `.COPY=`) to the PLAIN (non-`.COPY=`) row that declares it — the
    /// identical `KEY:`-token-or-bare-name resolution
    /// `gen_equipment_gap_tables.rs`'s own `collect_base_fields` uses for
    /// the SAME relationship, so a `.COPY=` row's shipped inherited fields
    /// and this check's own closure agree on what "the base" means. `None`
    /// when no plain row states that identity — never fabricated, and a
    /// `.COPY=` row is never itself matched (mirrors the generator's own
    /// "at most one hop" rule).
    ///
    /// **Same-file first, always** (`SD33-R9-CORPUS-SWEEP`, real corpus
    /// reproduction: `ultimate_equipment/equipment/hellscourge.json`,
    /// `ue_equip_arms_armor.lst:496` `Scorpion Whip.COPY=Hellscourge`). A
    /// bare weapon name is not a unique identity across a book's OWN files
    /// — `ue_profs_weapon.lst:79` also declares a plain `Scorpion Whip` row
    /// (a weapon-PROFICIENCY definition, `TYPE:` only, no `COST:`/`WT:`/
    /// `DAMAGE:`/…), a structurally different PCGen record kind that
    /// happens to share the bare name. Scanning the whole book in
    /// `std::fs::read_dir`'s own (unsorted, filesystem-order-dependent)
    /// walk let that unrelated proficiency row win the "first match" race
    /// on this checkout, silently EXCLUDING the real equipment base row
    /// (`ue_equip_arms_armor.lst:349`, same file as the citing `.COPY=`
    /// row) from the closure — the independent enricher this sweep checks
    /// against (`enrich_equipment_raw_tokens.rs::find_copy_base`) never has
    /// this failure mode because it only ever parses the ONE cited `.lst`
    /// file, never the whole book. Checking the citing record's own file
    /// FIRST makes the two tools agree by construction whenever an
    /// unambiguous same-file base exists (the common case, confirmed for
    /// every one of this defect's 9 real corpus instances by hand against
    /// the pinned oracle bytes) — this is a **superset** of the previous
    /// behavior, not a narrowing: a same-file match, when one exists, is
    /// always at least as correct as an unsorted book-wide first-match, and
    /// every book file (own file included) is still eligible, so a record
    /// with no same-file base still resolves exactly as before via the
    /// (now sorted, so deterministic) book-wide fallback below.
    fn copy_base_row(&mut self, book_dir: &str, own_file: &Path, base_identity: &str) -> Option<String> {
        if let Some(line) = self.copy_base_row_in_file(own_file, base_identity) {
            return Some(line);
        }
        let mut files = lst_files(&self.corpus_root.join(book_dir));
        files.sort();
        for path in files {
            if path.as_path() == own_file {
                continue; // already checked above
            }
            if let Some(line) = self.copy_base_row_in_file(&path, base_identity) {
                return Some(line);
            }
        }
        None
    }

    /// One file's own plain rows only — the shared search `copy_base_row`
    /// runs first against the citing record's own file, then (sorted, for
    /// determinism) against the rest of the book.
    fn copy_base_row_in_file(&mut self, path: &Path, base_identity: &str) -> Option<String> {
        let lines = self.lines(path)?.clone();
        for line in &lines {
            let fields: Vec<&str> = line.split('\t').collect();
            let Some(first) = fields.first() else { continue };
            let first = first.trim();
            if first.is_empty() || first.contains(".COPY=") {
                continue;
            }
            let key_token = fields.iter().find_map(|f| f.trim().strip_prefix("KEY:"));
            let matches = match key_token {
                Some(key) => key == base_identity,
                None => first == base_identity,
            };
            if matches {
                return Some(line.clone());
            }
        }
        None
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

#[cfg(test)]
mod short_book_of_tests {
    use super::{short_book_of, RACE_NEWEST_PRINTING, RACE_TRUE_BOOK};

    /// The defect `OPEN-ISSUES.md` row 22 traced one unit deep: the CRB
    /// dwarf race's PCGen source is
    /// `pathfinder/paizo/roleplaying_game/core_essentials/races/dwarf/
    /// dwarf_races.lst` -- one directory level deeper than a flat book
    /// layout. The OLD (pre-row-22) code read
    /// `source_path.parent().file_name()`, which for this row is `"dwarf"`
    /// — a race name, not a book, so no join could ever match. Row 22's own
    /// fix (this function's original shape) read the book off the same
    /// 4-segment `book_dir_of` grouping, landing on `"core_essentials"` --
    /// correct relative to the OLD bug, but itself `OPEN-ISSUES.md` row 68's
    /// defect: `core_essentials` is a PCGen packaging directory, not a book.
    /// `SD31-ATTRIB-001` closes that second layer: Dwarf's true FIRST
    /// printing is Core Rulebook (`core_rulebook.pcc`'s own 7), read off
    /// `RACE_TRUE_BOOK` -- and `decisions.md §10`'s later "newest publish
    /// wins" layer (`RACE_NEWEST_PRINTING`) moves it once more, to
    /// Advanced Race Guide, the newer of Dwarf's two printings.
    #[test]
    fn crb_race_chassis_resolves_to_its_newest_printing_not_core_essentials_or_the_race_name() {
        assert_eq!(
            short_book_of(
                "pathfinder/paizo/roleplaying_game/core_essentials/races/dwarf/dwarf_races.lst"
            ),
            Some("advanced_race_guide".to_string())
        );
    }

    /// A race NOT on `RACE_NEWEST_PRINTING` (Bestiary-4-native, no older
    /// printing to supersede) still resolves to its true FIRST book, same
    /// as before `decisions.md §10`'s layer existed -- the override is
    /// opt-in per race, never blanket.
    #[test]
    fn a_race_absent_from_newest_printing_keeps_resolving_to_its_true_book() {
        assert_eq!(
            short_book_of(
                "pathfinder/paizo/roleplaying_game/core_essentials/races/kasatha/kasatha_races.lst"
            ),
            Some("bestiary_4".to_string())
        );
    }

    /// **The join-desync `decisions.md §10`'s layer would create if this
    /// file's own copy were kind-blind.** A race_trait row from the
    /// IDENTICAL `races/dwarf/` directory (a `_abilities_race` file, not a
    /// `_races` chassis file) must stay on Dwarf's true FIRST book --
    /// `v06_work_inventory.rs`'s own `record_kind == Kind::Race` guard
    /// scopes the override to the chassis kind only, and this file's
    /// filename-based stand-in (`is_race_chassis_file`) must agree, or
    /// every Dwarf `race_trait` unit's `literal-verified` stamp silently
    /// breaks the moment `v06_work_inventory` re-attributes `race` but not
    /// `race_trait`.
    #[test]
    fn a_race_trait_file_from_the_same_race_directory_is_not_moved_by_the_newest_printing_layer() {
        assert_eq!(
            short_book_of(
                "pathfinder/paizo/roleplaying_game/core_essentials/races/dwarf/dwarf_abilities_race.lst"
            ),
            Some("core_rulebook".to_string())
        );
    }

    /// A flat-filed record (the shape the old code happened to get right,
    /// because for a flat book layout `source_path.parent().file_name()`
    /// and `book_dir_of`'s last segment agree) must keep resolving the same
    /// way — no regression on the population the bug never touched.
    #[test]
    fn flat_filed_record_resolves_to_its_own_book_same_as_before() {
        assert_eq!(
            short_book_of(
                "pathfinder/paizo/roleplaying_game/advanced_race_guide/arg_abilities_race.lst"
            ),
            Some("advanced_race_guide".to_string())
        );
        assert_eq!(
            short_book_of(
                "pathfinder/paizo/roleplaying_game/ultimate_equipment/ue_equip_magic_items.lst"
            ),
            Some("ultimate_equipment".to_string())
        );
    }

    /// A per-race-nested `race_trait` record resolves to its own real
    /// SOURCE book, not the intermediate race-name directory PCGen's source
    /// nests through, and (`SD31-ATTRIB-001`, `OPEN-ISSUES.md` row 68) not
    /// to `core_essentials` either -- Tiefling's true book is Bestiary 1
    /// (`advanced_race_guide.pcc`'s own `# B1 races` section), and this is
    /// exactly the shape row 68 named: before this fix every one of these
    /// resolved to `core_essentials` and silently hid which book they
    /// belonged to.
    #[test]
    fn nested_race_trait_resolves_to_its_true_book_not_core_essentials() {
        assert_eq!(
            short_book_of(
                "pathfinder/paizo/roleplaying_game/core_essentials/races/tiefling/tiefling_abilities_race_subrace.lst"
            ),
            Some("bestiary".to_string())
        );
    }

    /// The race-name-directory is still not swallowed as a book id for a
    /// race this fix cannot yet attribute (`monkey_goblin`: two in-scope
    /// books, `bestiary_6` and `inner_sea_bestiary`, natively declare it --
    /// see `RACE_TRUE_BOOK`'s own doc comment) -- it stays `core_essentials`,
    /// never the directory name `monkey_goblin` itself.
    #[test]
    fn an_ambiguous_race_still_resolves_to_core_essentials_not_its_own_directory_name() {
        assert_eq!(
            short_book_of(
                "pathfinder/paizo/roleplaying_game/core_essentials/races/monkey_goblin/monkey_goblin_abilities_race.lst"
            ),
            Some("core_essentials".to_string())
        );
    }

    /// **Enumerated, not assumed:** no two real, currently-shipped records
    /// carry the same `.lst` basename under two DIFFERENT real oracle
    /// books (`find_basename_collisions.py`-equivalent one-liner over every
    /// `data/corpus/**/*.json`'s `source.path`, grouped by
    /// `book_dir_of`'s last segment: 0 basenames shared across >1 book).
    /// Nor does any race name appear as a `races/<name>/` subdirectory
    /// under more than one top-level oracle book (`core_essentials` is
    /// currently the ONLY oracle book with this nesting shape at all —
    /// enumerated over `$PCGEN_CORPUS_ROOT/pathfinder/paizo/
    /// roleplaying_game/*/races/`). So this repo's real corpus has no case
    /// today where `short_book_of` must disambiguate a genuine same-name
    /// collision between two books. The synthetic test below proves the
    /// function is correct anyway, for the day one exists.
    #[test]
    fn synthetic_collision_two_different_books_sharing_a_nested_directory_name_resolve_correctly()
    {
        // Both records nest through a subdirectory literally named
        // `shared_name` -- the OLD `source_path.parent().file_name()` code
        // would read `"shared_name"` for BOTH, collapsing two different
        // real books into one indistinguishable (and wrong) string.
        let a = short_book_of(
            "pathfinder/paizo/roleplaying_game/book_alpha/sub/shared_name/x.lst",
        );
        let b = short_book_of(
            "pathfinder/paizo/roleplaying_game/book_beta/sub/shared_name/x.lst",
        );
        assert_eq!(a, Some("book_alpha".to_string()));
        assert_eq!(b, Some("book_beta".to_string()));
        assert_ne!(a, b, "two different books must never resolve to the same book id");
    }

    #[test]
    fn rejects_a_source_path_not_shaped_system_publisher_line_book_file() {
        assert_eq!(short_book_of("too/short.lst"), None);
        assert_eq!(short_book_of(""), None);
    }

    /// `OPEN-ISSUES.md` row 48/49: Dreamscarred Press ships with no "line"
    /// tier (`<system>/dreamscarred_press/<book>/<file>`, one segment
    /// shallower than Paizo's shape) -- confirmed against the real oracle
    /// directory layout and against `v06_work_inventory.rs`'s own
    /// `additional_book_dirs` registration of
    /// `"pathfinder/dreamscarred_press/ultimate_psionics"` (3 segments).
    /// Before this fix every such record with `raw_tokens` populated
    /// `fatal()`-aborted the whole sweep.
    #[test]
    fn dreamscarred_press_four_segment_path_resolves_one_tier_shallower() {
        assert_eq!(
            short_book_of("pathfinder/dreamscarred_press/ultimate_psionics/up_equipmods.lst"),
            Some("ultimate_psionics".to_string())
        );
    }

    /// The narrower Dreamscarred-Press rule must not swallow an unrelated
    /// 4-segment path from a different (or malformed) publisher -- the
    /// publisher name is checked explicitly, not inferred from segment
    /// count alone.
    #[test]
    fn a_four_segment_non_dreamscarred_press_path_still_rejects() {
        assert_eq!(short_book_of("pathfinder/some_other_press/book/file.lst"), None);
    }

    /// Regression, corpus-wide: for every shipped `race`/`race_trait`
    /// record's real `source.path` today, `short_book_of` must resolve to
    /// EXACTLY the same book `book_dir_of` (the binary's own pre-existing,
    /// trusted `by_book` grouping function) resolves -- UNLESS the record's
    /// oracle book is `core_essentials` and its race slug is in
    /// `RACE_TRUE_BOOK`, in which case `short_book_of` must resolve to that
    /// table's entry instead (`SD31-ATTRIB-001`, `OPEN-ISSUES.md` row 68 --
    /// this is the fix, not a regression: before it, EVERY one of these
    /// disagreed silently with the true book and this test's own prior
    /// unconditional-agreement assertion was pinning that defect in place)
    /// -- and, for `race`-kind records ONLY, `decisions.md §10`'s later
    /// `RACE_NEWEST_PRINTING` layer wins over `RACE_TRUE_BOOK` when the
    /// slug is on that table too (never for `race_trait`, which stays on
    /// its true FIRST book). Every record outside both resolved sets still
    /// can never disagree with the grouping the rest of this binary already
    /// relies on.
    #[test]
    fn every_shipped_race_source_path_agrees_with_book_dir_of_or_the_resolved_true_book() {
        let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let corpus_root = repo_root.join("data/corpus");
        let mut checked = 0usize;
        for kind_dir in ["race", "race_trait"] {
            let mut stack = vec![corpus_root.clone()];
            while let Some(dir) = stack.pop() {
                let Ok(entries) = std::fs::read_dir(&dir) else { continue };
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        stack.push(path);
                        continue;
                    }
                    if path.extension().and_then(|e| e.to_str()) != Some("json") {
                        continue;
                    }
                    if path.file_name().and_then(|f| f.to_str()) == Some("LICENSE.json") {
                        continue;
                    }
                    let rel =
                        path.strip_prefix(repo_root).unwrap().to_string_lossy().replace('\\', "/");
                    let segs: Vec<&str> = rel.split('/').collect();
                    if segs.len() < 4 || segs[0] != "data" || segs[1] != "corpus" || segs[3] != kind_dir
                    {
                        continue;
                    }
                    let Ok(text) = std::fs::read_to_string(&path) else { continue };
                    let Ok(parsed) = super::parse_document(&rel, &text) else { continue };
                    let Some(record) = parsed.record else { continue };
                    let raw_expected = super::book_dir_of(&record.source_path)
                        .and_then(|dir| std::path::Path::new(&dir).file_name().map(|f| f.to_string_lossy().into_owned()));
                    let race_slug = record
                        .source_path
                        .split('/')
                        .position(|s| s == "races")
                        .and_then(|i| record.source_path.split('/').nth(i + 1));
                    let resolved_expected = if raw_expected.as_deref() == Some("core_essentials") {
                        let newest = if kind_dir == "race" {
                            race_slug
                                .and_then(|slug| RACE_NEWEST_PRINTING.iter().find(|(s, _)| *s == slug))
                                .map(|(_, book)| book.to_string())
                        } else {
                            None
                        };
                        newest
                            .or_else(|| {
                                race_slug
                                    .and_then(|slug| RACE_TRUE_BOOK.iter().find(|(s, _)| *s == slug))
                                    .map(|(_, book)| book.to_string())
                            })
                            .or(raw_expected)
                    } else {
                        raw_expected
                    };
                    assert_eq!(
                        short_book_of(&record.source_path),
                        resolved_expected,
                        "record {rel} (source.path {}) disagreed with book_dir_of/RACE_TRUE_BOOK",
                        record.source_path
                    );
                    checked += 1;
                }
            }
        }
        assert!(checked > 0, "no race/race_trait records found under {}", corpus_root.display());
    }
}

/// **`SD33-R9-CORPUS-SWEEP`, mutation proof against the real production
/// function.** Real corpus reproduction of
/// `ultimate_equipment/equipment/hellscourge.json`: `ue_equip_arms_armor.lst`
/// (the citing `.COPY=` row's own file) also carries the correct, full
/// "Scorpion Whip" equipment base row, while a SEPARATE file in the same
/// book, `ue_profs_weapon.lst` (a weapon-proficiency list, a different
/// PCGen record kind), carries an unrelated, minimal "Scorpion Whip" row
/// under the identical bare name. `std::fs::read_dir`'s own (unsorted)
/// order let the wrong file win the old "first match across the whole
/// book" walk on the real checkout; deliberately naming the decoy file so
/// it sorts BEFORE the citing file (`a_decoy...` < `z_own...`) reproduces
/// that failure mode independently of any one filesystem's real
/// `read_dir` order, so this test is not flaky the way trusting real
/// `read_dir` order would be.
#[cfg(test)]
mod copy_base_row_tests {
    use super::Sweep;
    use std::fs;
    use std::path::PathBuf;

    struct Scratch {
        book_dir: PathBuf,
    }

    impl Scratch {
        fn new(name: &str) -> Self {
            let base = std::env::temp_dir()
                .join(format!("codex_corpus_literal_sweep_copy_base_{name}_{}", std::process::id()));
            let _ = fs::remove_dir_all(&base);
            let book_dir = base.join("book");
            fs::create_dir_all(&book_dir).unwrap();
            Scratch { book_dir }
        }

        fn write(&self, file_name: &str, contents: &str) -> PathBuf {
            let path = self.book_dir.join(file_name);
            fs::write(&path, contents).unwrap();
            path
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(self.book_dir.parent().unwrap());
        }
    }

    #[test]
    fn copy_base_row_prefers_the_citing_records_own_file_over_a_same_named_decoy_elsewhere_in_the_book() {
        let scratch = Scratch::new("prefers_own_file");
        // Sorts BEFORE the citing file below, and is what the pre-fix
        // book-wide-first-match walk would have returned.
        scratch.write("a_decoy_profs_weapon.lst", "Scorpion Whip\tTYPE:Exotic.Melee.Light.Slashing\n");
        let own_file = scratch.write(
            "z_own_equip_arms_armor.lst",
            "Scorpion Whip\tPROFICIENCY:WEAPON|Scorpion Whip\tCOST:5\tWT:3\tCRITMULT:x2\tDAMAGE:1d4\n\
             Scorpion Whip.COPY=Hellscourge\n",
        );

        let mut sweep = Sweep::new(scratch.book_dir.parent().unwrap().to_path_buf());
        let resolved = sweep.copy_base_row("book", &own_file, "Scorpion Whip");

        assert_eq!(
            resolved.as_deref(),
            Some("Scorpion Whip\tPROFICIENCY:WEAPON|Scorpion Whip\tCOST:5\tWT:3\tCRITMULT:x2\tDAMAGE:1d4"),
            "must resolve to the real equipment row in the citing record's OWN file, not the \
             decoy weapon-proficiency row in a different file, regardless of directory scan order"
        );
    }

    #[test]
    fn copy_base_row_still_falls_back_to_the_rest_of_the_book_when_no_same_file_base_exists() {
        let scratch = Scratch::new("falls_back");
        let base_file = scratch.write("elsewhere.lst", "Widget\tCOST:9\tWT:1\n");
        let own_file = scratch.write("citing.lst", "Widget.COPY=Gizmo\n");

        let mut sweep = Sweep::new(scratch.book_dir.parent().unwrap().to_path_buf());
        let resolved = sweep.copy_base_row("book", &own_file, "Widget");

        assert_eq!(
            resolved.as_deref(),
            Some("Widget\tCOST:9\tWT:1"),
            "a record with no same-file base must still resolve via the book-wide fallback, \
             exactly as before this fix"
        );
        let _ = base_file; // keep the write alive/named for readability
    }
}
