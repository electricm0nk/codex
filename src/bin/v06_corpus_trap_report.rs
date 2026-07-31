//! v0.6 alpha swarm: pre-ingest corpus trap report.
//!
//! **Why this exists.** Four of twenty-three PCGen books are ingested.
//! Every ingestion cycle so far has hit the *same* corpus traps —
//! `.MOD` rows counted as declarations, `#`-disabled rows read as live,
//! archetype-qualified `KEY:`s merged with the base record they only
//! share a display name with — rediscovered by hand, by a different
//! agent, every time. Nearly every count reported from those
//! rediscoveries was wrong on the first pass. This binary makes the
//! rediscovery mechanical: run it against a book *before* writing any
//! ingest code and it prints, per file and per line, every shape the
//! ingest has to handle and the specific miscount each one produces.
//!
//! The full catalogue of traps, and the corpus evidence confirming each,
//! lives on [`codex::pcgen_import::corpus_traps`]. This binary only
//! renders what that module reports.
//!
//! # Usage
//!
//! ```text
//! # Trap report for a book that has never been ingested:
//! cargo run --bin v06_corpus_trap_report -- ultimate_combat
//!
//! # Machine-readable, for a dashboard or a diff between runs:
//! cargo run --bin v06_corpus_trap_report -- ultimate_magic --json
//!
//! # Show every finding rather than a sample:
//! cargo run --bin v06_corpus_trap_report -- advanced_race_guide --examples 0
//!
//! # Never quote a book subtotal as a corpus total (trap 9):
//! cargo run --bin v06_corpus_trap_report -- --census WitchHex
//!
//! # Cross-check already-ingested caches against the lines they cite:
//! cargo run --bin v06_corpus_trap_report -- --audit
//! ```
//!
//! `PCGEN_CORPUS_ROOT` overrides the corpus location, defaulting to the
//! same `/home/ubuntu/workspace/repos/pcgen/data` the cache-generator
//! binaries (`gen_cache_acg` and siblings) already use. A book argument
//! may be a bare directory name under
//! `pathfinder/paizo/roleplaying_game` or an absolute path.
//!
//! # Exit codes
//!
//! `0` for a clean run, `1` for a usage or I/O failure, and `2` when
//! `--audit` finds a [`Severity::Defect`] in already-ingested content.
//! A corpus scan never exits non-zero for findings: upstream corpus shape
//! is data to handle, not a defect to fail on.
//!
//! This binary is an operator/ops surface (like `v06_class_state_dump`),
//! not an app runtime surface — nothing in the shipped app calls it.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use codex::pcgen_import::corpus_traps::{
    BookScan, Finding, Severity, Trap, audit_ingested_cache, concept_census, scan_book,
};

const DEFAULT_CORPUS_ROOT: &str = "/home/ubuntu/workspace/repos/pcgen/data";
const BOOKS_SUBDIR: &str = "pathfinder/paizo/roleplaying_game";
const DEFAULT_EXAMPLES: usize = 3;

/// Every trap the report renders, in the order the catalogue numbers them.
const REPORT_ORDER: &[Trap] = &[
    Trap::ModRecord,
    Trap::CopyRecord,
    Trap::DisabledLine,
    Trap::KeyDiffersFromName,
    Trap::ArchetypeScoped,
    Trap::SharedNameDistinctRecords,
    Trap::DefineZeroValueElsewhere,
    Trap::NamespacedKey,
    Trap::TokenDenseRecord,
    Trap::GoverningTokenHiddenByFilter,
    Trap::UnresolvableCitation,
];

#[derive(Debug)]
struct Args {
    book: Option<String>,
    census: Option<String>,
    audit: bool,
    json: bool,
    /// Findings printed per trap; 0 means all of them.
    examples: usize,
}

fn parse_args(argv: Vec<String>) -> Result<Args, String> {
    let mut args = Args {
        book: None,
        census: None,
        audit: false,
        json: false,
        examples: DEFAULT_EXAMPLES,
    };
    let mut it = argv.into_iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--json" => args.json = true,
            "--audit" => args.audit = true,
            "--census" => {
                args.census =
                    Some(it.next().ok_or_else(|| "--census needs a search string".to_string())?);
            }
            "--examples" => {
                let raw = it
                    .next()
                    .ok_or_else(|| "--examples needs a number".to_string())?;
                args.examples = raw
                    .parse()
                    .map_err(|_| format!("--examples wants a number, got `{raw}`"))?;
            }
            "--help" | "-h" => return Err(usage()),
            other if other.starts_with('-') => {
                return Err(format!("unknown flag `{other}`\n\n{}", usage()));
            }
            other => {
                if args.book.replace(other.to_string()).is_some() {
                    return Err(format!("only one book at a time; got a second: `{other}`"));
                }
            }
        }
    }
    if args.book.is_none() && args.census.is_none() && !args.audit {
        return Err(format!("nothing to do\n\n{}", usage()));
    }
    Ok(args)
}

fn usage() -> String {
    "usage: v06_corpus_trap_report <book> [--json] [--examples N]\n\
            v06_corpus_trap_report --census <string> [--json]\n\
            v06_corpus_trap_report --audit [--json]\n\
     \n\
     <book>  directory name under pathfinder/paizo/roleplaying_game, or an\n\
             absolute path. PCGEN_CORPUS_ROOT overrides the corpus location."
        .to_string()
}

fn corpus_root() -> PathBuf {
    PathBuf::from(
        std::env::var("PCGEN_CORPUS_ROOT").unwrap_or_else(|_| DEFAULT_CORPUS_ROOT.to_string()),
    )
}

fn resolve_book(root: &Path, book: &str) -> PathBuf {
    let candidate = PathBuf::from(book);
    if candidate.is_absolute() {
        candidate
    } else {
        root.join(BOOKS_SUBDIR).join(book)
    }
}

fn main() -> ExitCode {
    let args = match parse_args(std::env::args().skip(1).collect()) {
        Ok(a) => a,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(1);
        }
    };

    let root = corpus_root();
    if !root.is_dir() {
        eprintln!(
            "corpus root {} is not a directory; set PCGEN_CORPUS_ROOT",
            root.display()
        );
        return ExitCode::from(1);
    }

    if let Some(needle) = &args.census {
        let census = match concept_census(&root.join(BOOKS_SUBDIR), needle) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("census failed: {e}");
                return ExitCode::from(1);
            }
        };
        if args.json {
            println!(
                "{}",
                serde_json::json!({
                    "needle": census.needle,
                    "per_book": census.per_book,
                    "total": census.total,
                    "note": "a per-book subtotal is not a corpus total",
                })
            );
        } else {
            print!("{census}");
            println!(
                "\nQuoting any one row above as \"the\" count is trap 9. Name the scope."
            );
        }
    }

    if args.audit {
        let cache = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/corpus");
        let findings = match audit_ingested_cache(&cache, &root) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("cache audit failed: {e}");
                return ExitCode::from(1);
            }
        };
        let defects: Vec<&Finding> = findings
            .iter()
            .filter(|f| f.severity == Severity::Defect)
            .collect();
        if args.json {
            println!("{}", serde_json::json!({ "findings": findings_json(&findings) }));
        } else {
            print_audit(&findings, &defects);
        }
        if !defects.is_empty() {
            return ExitCode::from(2);
        }
    }

    if let Some(book) = &args.book {
        let dir = resolve_book(&root, book);
        if !dir.is_dir() {
            eprintln!("no such book directory: {}", dir.display());
            return ExitCode::from(1);
        }
        let scan = match scan_book(&dir) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("scan of {} failed: {e}", dir.display());
                return ExitCode::from(1);
            }
        };
        if args.json {
            println!("{}", book_json(&scan));
        } else {
            print_book(&scan, args.examples);
        }
    }

    ExitCode::SUCCESS
}

// ---------------------------------------------------------------------------
// Text rendering
// ---------------------------------------------------------------------------

fn print_book(scan: &BookScan, examples: usize) {
    println!("═══ corpus trap report — {} ═══\n", scan.book);

    println!("Record shapes (scope: this book only — see --census before generalising)\n");
    println!(
        "  {:>8}  {:>8}  {:>8}  {:>8}  file",
        "DECLARES", ".COPY=", ".MOD", "#OFF"
    );
    for file in &scan.files {
        if file.declaring_lines() == 0 && file.modifications == 0 && file.disabled_records == 0 {
            continue;
        }
        println!(
            "  {:>8}  {:>8}  {:>8}  {:>8}  {}",
            file.declaring_lines(),
            file.copies,
            file.modifications,
            file.disabled_records,
            leaf(&file.path)
        );
    }
    println!(
        "  {:>8}  {:>8}  {:>8}  {:>8}  ── {} files",
        scan.declaring_lines(),
        scan.copies(),
        scan.modifications(),
        scan.disabled_records(),
        scan.files.len()
    );
    println!(
        "\n  DECLARES = declarations + `.COPY=` rows. `.MOD` rows declare nothing;\n  \
         adding them to DECLARES is the single most common miscount here.\n"
    );

    let counts = scan.counts_by_trap();
    println!("Findings by trap\n");
    for trap in REPORT_ORDER {
        let n = counts.get(trap).copied().unwrap_or(0);
        println!("  {:>7}  {}", n, trap.id());
    }
    println!();

    for trap in REPORT_ORDER {
        let hits: Vec<&Finding> = scan.findings_for(*trap).collect();
        if hits.is_empty() {
            continue;
        }
        println!("─── {} ({} findings) ───", trap.id(), hits.len());
        println!("  risk: {}\n", trap.miscount_risk());
        let show = if examples == 0 { hits.len() } else { examples.min(hits.len()) };
        for f in hits.iter().take(show) {
            println!("  {}:{}", leaf(&f.file), f.line);
            println!("    {} — {}", f.record, f.detail);
        }
        if show < hits.len() {
            println!("  … {} more (pass --examples 0 for all)", hits.len() - show);
        }
        println!();
    }

    let namespaces = scan.namespaces();
    if !namespaces.is_empty() {
        println!("─── KEY namespaces in this book ───");
        println!(
            "  A bare-leaf grep under these prefixes returns zero. Search\n  \
             `KEY:<namespace> ~ <leaf>`, not `KEY:<leaf>`.\n"
        );
        let mut rows: Vec<(&String, &usize)> = namespaces.iter().collect();
        rows.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        for (ns, n) in rows.iter().take(20) {
            println!("  {n:>6}  {ns}");
        }
        if rows.len() > 20 {
            println!("  … {} more namespaces", rows.len() - 20);
        }
        println!();
    }

    println!(
        "Everything above is legitimate upstream data. None of it is a defect;\n\
         the defect is mishandling it. Nothing here is a reason to fail a build."
    );
}

fn print_audit(findings: &[Finding], defects: &[&Finding]) {
    println!("═══ ingested-cache audit ═══\n");
    let mut by_trap: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    for f in findings {
        let entry = by_trap.entry(f.trap.id()).or_insert((0, 0));
        match f.severity {
            Severity::Trap => entry.0 += 1,
            Severity::Defect => entry.1 += 1,
        }
    }
    println!("  {:>7}  {:>7}  trap", "TRAP", "DEFECT");
    for (id, (t, d)) in &by_trap {
        println!("  {t:>7}  {d:>7}  {id}");
    }
    println!();

    if defects.is_empty() {
        println!("No defects: every ingested record's citation agrees with the line it names.");
        return;
    }
    println!("─── defects ({}) ───\n", defects.len());
    for f in defects {
        println!("  {}", leaf(&f.file));
        println!("    [{}] {} — {}", f.trap.id(), f.record, f.detail);
    }
}

fn leaf(path: &str) -> String {
    let parts: Vec<&str> = path.rsplit('/').take(2).collect();
    parts.into_iter().rev().collect::<Vec<_>>().join("/")
}

// ---------------------------------------------------------------------------
// JSON rendering
// ---------------------------------------------------------------------------

fn findings_json(findings: &[Finding]) -> Vec<serde_json::Value> {
    findings
        .iter()
        .map(|f| {
            serde_json::json!({
                "file": f.file,
                "line": f.line,
                "trap": f.trap.id(),
                "severity": f.severity.to_string(),
                "record": f.record,
                "detail": f.detail,
                "miscount_risk": f.trap.miscount_risk(),
            })
        })
        .collect()
}

fn book_json(scan: &BookScan) -> serde_json::Value {
    let files: Vec<serde_json::Value> = scan
        .files
        .iter()
        .map(|f| {
            serde_json::json!({
                "path": f.path,
                "declaring_lines": f.declaring_lines(),
                "declarations": f.declarations,
                "copies": f.copies,
                "modifications": f.modifications,
                "disabled_records": f.disabled_records,
                "continuations": f.continuations,
                "directives": f.directives,
                "bonus_var_tokens": f.bonus_var_tokens,
                "bonus_var_records": f.bonus_var_records,
                "findings": findings_json(&f.findings().cloned().collect::<Vec<_>>()),
            })
        })
        .collect();

    let counts: BTreeMap<&str, usize> = scan
        .counts_by_trap()
        .into_iter()
        .map(|(t, n)| (t.id(), n))
        .collect();

    serde_json::json!({
        "book": scan.book,
        "scope": "single book directory; not a corpus total",
        "totals": {
            "declaring_lines": scan.declaring_lines(),
            "declarations": scan.declarations(),
            "copies": scan.copies(),
            "modifications": scan.modifications(),
            "disabled_records": scan.disabled_records(),
        },
        "counts_by_trap": counts,
        "namespaces": scan.namespaces(),
        "files": files,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bare_book_name_resolves_under_the_roleplaying_game_directory() {
        let root = PathBuf::from("/corpus");
        assert_eq!(
            resolve_book(&root, "ultimate_combat"),
            PathBuf::from("/corpus/pathfinder/paizo/roleplaying_game/ultimate_combat")
        );
    }

    #[test]
    fn an_absolute_book_path_is_used_verbatim() {
        let root = PathBuf::from("/corpus");
        assert_eq!(
            resolve_book(&root, "/elsewhere/uc"),
            PathBuf::from("/elsewhere/uc")
        );
    }

    #[test]
    fn no_arguments_is_a_usage_error_rather_than_a_silent_success() {
        assert!(parse_args(vec![]).is_err());
    }

    #[test]
    fn a_second_book_argument_is_rejected_rather_than_silently_dropped() {
        let err = parse_args(vec!["uc".into(), "um".into()]).unwrap_err();
        assert!(err.contains("only one book"), "got: {err}");
    }

    #[test]
    fn examples_zero_means_show_everything() {
        let args = parse_args(vec!["uc".into(), "--examples".into(), "0".into()]).unwrap();
        assert_eq!(args.examples, 0);
    }

    #[test]
    fn census_requires_its_search_string() {
        assert!(parse_args(vec!["--census".into()]).is_err());
    }

    #[test]
    fn every_trap_variant_appears_in_the_report_order() {
        // A trap added to the catalogue but forgotten here would be
        // silently invisible in the report, which is exactly the failure
        // mode this whole binary exists to prevent.
        let all = [
            Trap::ModRecord,
            Trap::DisabledLine,
            Trap::KeyDiffersFromName,
            Trap::ArchetypeScoped,
            Trap::SharedNameDistinctRecords,
            Trap::DefineZeroValueElsewhere,
            Trap::NamespacedKey,
            Trap::TokenDenseRecord,
            Trap::GoverningTokenHiddenByFilter,
            Trap::CopyRecord,
            Trap::UnresolvableCitation,
        ];
        for trap in all {
            assert!(
                REPORT_ORDER.contains(&trap),
                "{trap:?} is missing from REPORT_ORDER and would never be printed"
            );
        }
        assert_eq!(REPORT_ORDER.len(), all.len());
    }

    #[test]
    fn leaf_keeps_the_book_directory_so_a_filename_is_unambiguous() {
        assert_eq!(leaf("/a/b/ultimate_combat/uc_feats.lst"), "ultimate_combat/uc_feats.lst");
    }
}
