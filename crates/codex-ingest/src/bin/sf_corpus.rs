//! Generate or check the Starfinder 1e corpus (SD-37 E3.4; `pcgen_import::sf_corpus`).
//!
//! ```text
//! cargo run --locked -p codex-ingest --bin sf_corpus -- --check   # the on-disk corpus is fresh
//! cargo run --locked -p codex-ingest --bin sf_corpus -- --write   # regenerate data/starfinder-1e/corpus
//! cargo run --locked -p codex-ingest --bin sf_corpus -- --dump <dir>
//! ```
//!
//! Reads `docs/work-inventory.starfinder-1e.json` and the pinned oracle through
//! `$PCGEN_CORPUS_ROOT` (`closure::corpus_root`). Exit 0 = PASS / written; 1 = stale; 2 = usage
//! or input error.

use std::path::PathBuf;

use codex::rules_core::game_system::GameSystem;
use codex_ingest::pcgen_import::sf_corpus;
use codex_ingest::pcgen_import::sheet_rule::closure::{self, PinnedTree};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mode = match args.as_slice() {
        [m] if m == "--check" || m == "--write" => (m.clone(), None),
        [m, d] if m == "--dump" => (m.clone(), Some(PathBuf::from(d))),
        _ => {
            eprintln!("usage: sf_corpus --check | --write | --dump <dir>");
            std::process::exit(2);
        }
    };
    let system = GameSystem::Starfinder1e;
    let repo = codex_ingest::repo_root();
    let out_dir = sf_corpus::corpus_dir(&repo, system);
    let tree = match PinnedTree::load_for(system, &closure::corpus_root()) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("sf_corpus: {e}");
            std::process::exit(2);
        }
    };
    let run = match sf_corpus::generate(&repo, &tree) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("sf_corpus: {e}");
            std::process::exit(2);
        }
    };
    let summary = format!(
        "records={} redacted={} renamed={} description_redacted={} files={}",
        run.records,
        run.redacted,
        run.renamed,
        run.description_redacted,
        run.files.len()
    );
    match (mode.0.as_str(), mode.1) {
        ("--check", _) => match sf_corpus::check(&out_dir, &run) {
            Ok(()) => println!("{summary} verdict=PASS"),
            Err(problems) => {
                for p in problems.iter().take(50) {
                    eprintln!("  {p}");
                }
                println!("{summary} verdict=FAIL problems={}", problems.len());
                std::process::exit(1);
            }
        },
        ("--dump", Some(d)) => {
            if d == out_dir {
                eprintln!("sf_corpus: --dump must not target the tracked corpus; pass a scratch directory");
                std::process::exit(2);
            }
            if let Err(e) = sf_corpus::write(&d, &run) {
                eprintln!("sf_corpus: writing {}: {e}", d.display());
                std::process::exit(2);
            }
            println!("{summary} -> {}", d.display());
        }
        _ => {
            if let Err(e) = sf_corpus::write(&out_dir, &run) {
                eprintln!("sf_corpus: writing {}: {e}", out_dir.display());
                std::process::exit(2);
            }
            println!("{summary} -> {}", out_dir.display());
        }
    }
}
