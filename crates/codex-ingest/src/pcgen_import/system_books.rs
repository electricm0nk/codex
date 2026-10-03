//! Per-system book registry by `.pcc` entry file, the licence exclusion record, the resolved
//! include structure of the registered books, and the game-mode loader (SD-37 E3.1, adopting
//! SD-35 C2.1 "the second PCGen-format reader").
//!
//! **Why a PCC registry.** Pathfinder's converter reads every `.lst` under one book subtree
//! ([`crate::pcgen_import::sheet_rule::closure::BOOKS_RELATIVE`]). Starfinder cannot be read
//! that way: `starfinder/paizo/` also holds the Starfinder Society guide, and the core
//! book's directory nests the Society add-on (`core/_society/`), both excluded by licence
//! (`docs/release/SD-37-starfinder-1e/decisions.md §6`, `docs/governance/license-matrix.md`).
//! So a Starfinder book is named by its `.pcc`, and its files are exactly the `.lst` that
//! `.pcc` includes. An excluded book is absent from [`BOOK_PCCS`] and named in
//! [`EXCLUDED_BOOK_PCCS`]; [`resolve_book_includes`] refuses any include that reaches one.
//!
//! **What this reads, not what it means.** The include structure carries each `.lst`'s
//! directive kind (`STAT:`, `SIZE:`, `VARIABLE:`, `DATATABLE:` …), and the game-mode loader
//! returns `system/gameModes/<mode>/*.lst` as tab-split rows. Interpreting those rows (the
//! formula system) is the next card's (E3.2).

use std::path::{Path, PathBuf};

use codex::rules_core::game_system::{BookRegistry, GameSystem, PerSystem};

use crate::pcgen_import::include_resolver::{IncludeResolution, ResolvedLstFile, resolve_pcc_includes_from};

/// One book: its directory and its `.pcc` entry file, both relative to `PCGEN_CORPUS_ROOT`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemBook {
    pub dir: &'static str,
    pub pcc: &'static str,
}

/// A book the corpus ships that the converter must never read, with the ruling that excludes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExcludedBook {
    pub book: SystemBook,
    pub reason: &'static str,
}

/// The books a system's converter reads, by `.pcc`: only books whose licence row in
/// `docs/governance/license-matrix.md` says **include**. Pathfinder is read by directory
/// (`closure::BOOKS_RELATIVE`) and registers none here.
pub const BOOK_PCCS: BookRegistry<SystemBook> = PerSystem {
    pathfinder_1e: &[],
    starfinder_1e: &[
        SystemBook { dir: "starfinder/paizo/core", pcc: "starfinder/paizo/core/_starfinder_core_rulebook.pcc" },
        SystemBook { dir: "starfinder/paizo/armory", pcc: "starfinder/paizo/armory/_starfinder_armory.pcc" },
        SystemBook {
            dir: "starfinder/paizo/character_operations_manual",
            pcc: "starfinder/paizo/character_operations_manual/_character_operations_manual.pcc",
        },
        SystemBook { dir: "starfinder/paizo/pact_worlds", pcc: "starfinder/paizo/pact_worlds/_starfinder_pact_worlds.pcc" },
        SystemBook { dir: "starfinder/paizo/near_space", pcc: "starfinder/paizo/near_space/_near_space.pcc" },
        SystemBook {
            dir: "starfinder/paizo/alien_archive",
            pcc: "starfinder/paizo/alien_archive/_starfinder_alien_archive.pcc",
        },
        SystemBook {
            dir: "starfinder/paizo/alien_archive_2",
            pcc: "starfinder/paizo/alien_archive_2/_starfinder_alien_archive_2.pcc",
        },
        SystemBook {
            dir: "starfinder/paizo/alien_archive_3",
            pcc: "starfinder/paizo/alien_archive_3/_starfinder_alien_archive_3.pcc",
        },
    ],
};

/// The books the sheet-rule converter and the corpus generator convert, per system: a subset of
/// [`BOOK_PCCS`]. Starfinder is tuned on the Core Rulebook alone before it goes wide
/// (`docs/release/SD-37-starfinder-1e/decisions.md §4`: E3.4 is the one-book proof, E3.5 adds
/// the other seven in one batch by widening this list). Pathfinder is read by directory and
/// registers none here.
pub const CONVERTED_BOOKS: BookRegistry<SystemBook> = PerSystem {
    pathfinder_1e: &[],
    starfinder_1e: &[SystemBook { dir: "starfinder/paizo/core", pcc: "starfinder/paizo/core/_starfinder_core_rulebook.pcc" }],
};

/// The PCGen game-mode directory each system's books load against (`system/gameModes/<mode>`).
pub const GAME_MODES: PerSystem<&str> = PerSystem { pathfinder_1e: "Pathfinder", starfinder_1e: "Starfinder" };

/// The id a converted book carries in the population, the corpus and the package: the last
/// segment of its directory (`starfinder/paizo/core` -> `core`), which is also the prefix of
/// every SF inventory unit id (`core:feat:toughness`).
pub fn book_id(book: &SystemBook) -> &'static str {
    book.dir.rsplit('/').next().unwrap_or(book.dir)
}

/// [`resolve_book_includes`] for the [`CONVERTED_BOOKS`] only, with the same exclusion guard.
pub fn resolve_converted_book_includes(system: GameSystem, corpus_root: &Path) -> Result<BookIncludes, String> {
    let converted = CONVERTED_BOOKS.books(system);
    if converted.is_empty() {
        return Err(format!("no converted book is registered for game system {system} (system_books::CONVERTED_BOOKS)"));
    }
    if let Some(b) = converted.iter().find(|b| !BOOK_PCCS.books(system).contains(b)) {
        return Err(format!("converted book {} is not a registered book (system_books::BOOK_PCCS)", b.dir));
    }
    resolve_registered(system, corpus_root, converted, EXCLUDED_BOOK_PCCS.books(system))
}

/// The books a system's corpus ships that its converter must not read: every licence row that
/// says **exclude**.
pub const EXCLUDED_BOOK_PCCS: BookRegistry<ExcludedBook> = PerSystem {
    pathfinder_1e: &[],
    starfinder_1e: &[
        ExcludedBook {
            book: SystemBook {
                dir: "starfinder/paizo/starfinder_society_rules",
                pcc: "starfinder/paizo/starfinder_society_rules/_sfs.pcc",
            },
            reason: "Starfinder Society Roleplaying Guild Guide: its COPYRIGHT reads \"All Rights Reserved\" \
                     (decisions.md §6; license-matrix.md SF row: exclude)",
        },
        ExcludedBook {
            book: SystemBook {
                dir: "starfinder/lpj_design/infinite_space",
                pcc: "starfinder/lpj_design/infinite_space/themes/theme_outlaw_crew/_lpj9304.pcc",
            },
            reason: "third-party (LPJ Design); its OGL claim is not verified \
                     (decisions.md §6, safe default SD-a; license-matrix.md SF row: exclude)",
        },
        ExcludedBook {
            book: SystemBook { dir: "starfinder/paizo/core/_society", pcc: "starfinder/paizo/core/_society/_.pcc" },
            reason: "Society guide's Core Rulebook mods, nested in the core book's directory; declares no licence \
                     (decisions.md §6, safe default SD-a; license-matrix.md SF row: exclude)",
        },
    ],
};

/// The campaign-loaded system-file directives: the `.lst` files a book's `.pcc` loads that
/// define the game's stats, sizes, saves, alignments, variables, data tables, dynamic objects
/// and global modifiers (rather than records such as classes or spells).
pub const CAMPAIGN_SYSTEM_KINDS: [&str; 8] =
    ["ALIGNMENT", "DATATABLE", "DYNAMIC", "GLOBALMODIFIER", "SAVE", "SIZE", "STAT", "VARIABLE"];

/// The resolved include structure of every registered book of one system.
#[derive(Debug, Clone)]
pub struct BookIncludes {
    pub system: GameSystem,
    /// One resolution per registered book, in registry order.
    pub books: Vec<(SystemBook, IncludeResolution)>,
    /// Every `.lst` the registered books include, first reference wins, in registry order.
    pub lst_files: Vec<ResolvedLstFile>,
}

impl BookIncludes {
    /// The campaign-loaded system files ([`CAMPAIGN_SYSTEM_KINDS`]).
    pub fn campaign_system_files(&self) -> impl Iterator<Item = &ResolvedLstFile> {
        self.lst_files.iter().filter(|f| CAMPAIGN_SYSTEM_KINDS.contains(&f.kind.as_str()))
    }
}

/// Resolve every registered book of `system` from `corpus_root` (a PCGen `data/` checkout).
///
/// Errors, naming the cause: a system with no books in [`BOOK_PCCS`]; a registered `.pcc` that
/// is missing or unreadable; any include diagnostic (missing target, cycle, malformed
/// directive); an include that reaches a `.pcc` or `.lst` inside an excluded book.
pub fn resolve_book_includes(system: GameSystem, corpus_root: &Path) -> Result<BookIncludes, String> {
    let registered = BOOK_PCCS.books(system);
    if registered.is_empty() {
        return Err(format!(
            "no book .pcc is registered for game system {system} (system_books::BOOK_PCCS)"
        ));
    }
    resolve_registered(system, corpus_root, registered, EXCLUDED_BOOK_PCCS.books(system))
}

fn resolve_registered(
    system: GameSystem,
    corpus_root: &Path,
    registered: &[SystemBook],
    excluded: &[ExcludedBook],
) -> Result<BookIncludes, String> {
    let excluded_by = |path: &Path| -> Option<&ExcludedBook> {
        let rel = path.strip_prefix(corpus_root).ok()?;
        excluded.iter().find(|e| rel.starts_with(e.book.dir))
    };
    let mut books = Vec::new();
    let mut lst_files: Vec<ResolvedLstFile> = Vec::new();
    let mut seen = std::collections::BTreeSet::<PathBuf>::new();
    for book in registered {
        let resolution = resolve_pcc_includes_from(corpus_root, corpus_root.join(book.pcc))
            .map_err(|d| format!("{}: {}", book.pcc, d.message))?;
        if let Some(d) = resolution.diagnostics.first() {
            return Err(format!(
                "{}: {} include diagnostic(s), first: {}",
                book.pcc,
                resolution.diagnostics.len(),
                d.message
            ));
        }
        for pcc in &resolution.pcc_files {
            if let Some(e) = excluded_by(&pcc.path) {
                return Err(format!("{} includes {} of excluded book {}", book.pcc, pcc.path.display(), e.book.dir));
            }
        }
        for lst in &resolution.lst_files {
            if let Some(e) = excluded_by(&lst.path) {
                return Err(format!("{} includes {} of excluded book {}", book.pcc, lst.path.display(), e.book.dir));
            }
            if seen.insert(lst.path.clone()) {
                lst_files.push(lst.clone());
            }
        }
        books.push((*book, resolution));
    }
    Ok(BookIncludes { system, books, lst_files })
}

/// One non-blank, non-comment row of a game-mode `.lst`: its tab-separated tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameModeRow {
    /// One-based source line.
    pub line_number: usize,
    /// Tab-separated tokens, empty tokens dropped, each trimmed.
    pub tokens: Vec<String>,
}

/// One `system/gameModes/<mode>/*.lst` file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameModeFile {
    /// File name (`level.lst`).
    pub name: String,
    pub path: PathBuf,
    pub rows: Vec<GameModeRow>,
}

/// Load the `.lst` files directly inside `<pcgen_repo_dir>/system/gameModes/<mode>`, sorted by
/// name. A missing directory, or one with no `.lst`, is an error naming the path.
pub fn load_game_mode(pcgen_repo_dir: &Path, mode: &str) -> Result<Vec<GameModeFile>, String> {
    let dir = pcgen_repo_dir.join("system/gameModes").join(mode);
    let entries = std::fs::read_dir(&dir).map_err(|e| format!("game mode {mode} at {}: {e}", dir.display()))?;
    let mut paths: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().is_some_and(|x| x == "lst"))
        .collect();
    paths.sort();
    if paths.is_empty() {
        return Err(format!("game mode {mode} at {} has no .lst file", dir.display()));
    }
    paths
        .into_iter()
        .map(|path| {
            let bytes = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            let text = String::from_utf8_lossy(&bytes);
            Ok(GameModeFile {
                name: path.file_name().unwrap_or_default().to_string_lossy().into_owned(),
                rows: game_mode_rows(&text),
                path,
            })
        })
        .collect()
}

fn game_mode_rows(text: &str) -> Vec<GameModeRow> {
    text.lines()
        .enumerate()
        .filter_map(|(i, line)| {
            let content = line.trim();
            if content.is_empty() || content.starts_with('#') {
                return None;
            }
            let tokens: Vec<String> =
                line.split('\t').map(str::trim).filter(|t| !t.is_empty()).map(str::to_string).collect();
            Some(GameModeRow { line_number: i + 1, tokens })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The exclusion guard: a registered book whose `.pcc` reaches a file of an excluded book
    /// is refused by name, even though the file exists and the include resolves.
    #[test]
    fn an_include_into_an_excluded_book_is_refused() {
        let root = std::env::temp_dir().join(format!("system-books-guard-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("sys/good")).unwrap();
        std::fs::create_dir_all(root.join("sys/good/_addon")).unwrap();
        std::fs::write(root.join("sys/good/_addon/x.lst"), "A\n").unwrap();
        std::fs::write(root.join("sys/good/g.lst"), "B\n").unwrap();
        std::fs::write(root.join("sys/good/good.pcc"), "STAT:g.lst\nABILITY:_addon/x.lst\n").unwrap();
        let good = SystemBook { dir: "sys/good", pcc: "sys/good/good.pcc" };
        let addon = ExcludedBook { book: SystemBook { dir: "sys/good/_addon", pcc: "sys/good/_addon/a.pcc" }, reason: "test" };
        let err = resolve_registered(GameSystem::Starfinder1e, &root, &[good], &[addon]).unwrap_err();
        assert!(err.contains("excluded book sys/good/_addon"), "{err}");
        let ok = resolve_registered(GameSystem::Starfinder1e, &root, &[good], &[]).unwrap();
        assert_eq!(ok.lst_files.len(), 2);
        assert_eq!(ok.campaign_system_files().count(), 1);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn game_mode_rows_split_on_tabs_and_skip_comments_and_blanks() {
        let rows = game_mode_rows("# comment\n\nSTAT:10\t\tCOST:0\r\n  \nLOAD:0|0\n");
        assert_eq!(
            rows,
            vec![
                GameModeRow { line_number: 3, tokens: vec!["STAT:10".into(), "COST:0".into()] },
                GameModeRow { line_number: 5, tokens: vec!["LOAD:0|0".into()] },
            ]
        );
    }
}
