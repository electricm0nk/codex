//! Starfinder 1e book registry, licence exclusions and `.pcc` include structure (SD-37 E3.1).
//!
//! `docs/release/SD-37-starfinder-1e/decisions.md §6`: the converter's Starfinder book registry
//! lists only the books whose licence row in `docs/governance/license-matrix.md` says
//! **include**; the three excluded PCCs (SSRGG, LPJ Design, the core `_society` add-on) are
//! absent from it and are named in the exclusion record, so an excluded book cannot be read.
//!
//! The include-structure tests read the pinned oracle (`closure::corpus_root()`;
//! `$PCGEN_REPO_DIR` for the game mode). Like every other
//! corpus test in this crate they skip, loudly, when the checkout is absent.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use codex::rules_core::game_system::GameSystem;
use codex_ingest::pcgen_import::include_resolver::resolve_pcc_includes_from;
use codex_ingest::pcgen_import::sheet_rule::closure;
use codex_ingest::pcgen_import::system_books::{
    BOOK_PCCS, CAMPAIGN_SYSTEM_KINDS, EXCLUDED_BOOK_PCCS, load_game_mode, resolve_book_includes,
};

const IN_SCOPE_DIRS: [&str; 8] = [
    "starfinder/paizo/core",
    "starfinder/paizo/armory",
    "starfinder/paizo/character_operations_manual",
    "starfinder/paizo/pact_worlds",
    "starfinder/paizo/near_space",
    "starfinder/paizo/alien_archive",
    "starfinder/paizo/alien_archive_2",
    "starfinder/paizo/alien_archive_3",
];

const EXCLUDED_DIRS: [&str; 3] = [
    "starfinder/paizo/starfinder_society_rules",
    "starfinder/lpj_design/infinite_space",
    "starfinder/paizo/core/_society",
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn corpus_root() -> Option<PathBuf> {
    let root = closure::corpus_root();
    if root.join("starfinder").is_dir() {
        Some(root)
    } else {
        eprintln!("skipped: no starfinder/ under {} (run scripts/fetch-pcgen-oracle.sh)", root.display());
        None
    }
}

fn pcgen_repo_dir(corpus_root: &Path) -> PathBuf {
    match std::env::var("PCGEN_REPO_DIR") {
        Ok(r) => PathBuf::from(r),
        Err(_) => corpus_root.join(".."),
    }
}

/// The SF rows of `license-matrix.md` (E0.2's deliverable), as (book dir, include?).
fn licence_matrix_sf_rows() -> Vec<(String, bool)> {
    let path = repo_root().join("docs/governance/license-matrix.md");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    text.lines()
        .filter(|l| l.starts_with("| `starfinder/"))
        .map(|l| {
            let dir = l.split('`').nth(1).expect("book dir in backticks").to_string();
            let include = l.contains("**include**");
            assert!(include || l.contains("**exclude**"), "SF licence row has no include/exclude verdict: {l}");
            (dir, include)
        })
        .collect()
}

#[test]
fn sf_registry_lists_exactly_the_eight_in_scope_books() {
    let dirs: BTreeSet<&str> = BOOK_PCCS.books(GameSystem::Starfinder1e).iter().map(|b| b.dir).collect();
    assert_eq!(dirs, IN_SCOPE_DIRS.into_iter().collect::<BTreeSet<_>>());
    assert_eq!(BOOK_PCCS.books(GameSystem::Starfinder1e).len(), 8, "one registry entry per book");
    for book in BOOK_PCCS.books(GameSystem::Starfinder1e) {
        assert!(book.pcc.starts_with(book.dir) && book.pcc.ends_with(".pcc"), "{book:?}");
    }
}

#[test]
fn sf_excluded_pccs_are_absent_from_the_registry_and_named_in_the_exclusion_record() {
    let registered: BTreeSet<&str> = BOOK_PCCS.books(GameSystem::Starfinder1e).iter().map(|b| b.pcc).collect();
    let excluded = EXCLUDED_BOOK_PCCS.books(GameSystem::Starfinder1e);
    let excluded_dirs: BTreeSet<&str> = excluded.iter().map(|e| e.book.dir).collect();
    assert_eq!(excluded_dirs, EXCLUDED_DIRS.into_iter().collect::<BTreeSet<_>>());
    for e in excluded {
        assert!(!registered.contains(e.book.pcc), "excluded PCC {} is in the registry", e.book.pcc);
        assert!(e.reason.contains("decisions.md §6"), "exclusion of {} cites no ruling", e.book.dir);
    }
}

/// The code registry and E0.2's licence matrix agree row for row: include ⇔ registered,
/// exclude ⇔ in the exclusion record. A book added to either side alone fails here.
#[test]
fn sf_registry_agrees_with_the_licence_matrix() {
    let rows = licence_matrix_sf_rows();
    assert_eq!(rows.len(), 11, "E0.2: 11 SF licence rows");
    let include: BTreeSet<String> = rows.iter().filter(|r| r.1).map(|r| r.0.clone()).collect();
    let exclude: BTreeSet<String> = rows.iter().filter(|r| !r.1).map(|r| r.0.clone()).collect();
    let registered: BTreeSet<String> =
        BOOK_PCCS.books(GameSystem::Starfinder1e).iter().map(|b| b.dir.to_string()).collect();
    let recorded: BTreeSet<String> =
        EXCLUDED_BOOK_PCCS.books(GameSystem::Starfinder1e).iter().map(|e| e.book.dir.to_string()).collect();
    assert_eq!(registered, include);
    assert_eq!(recorded, exclude);
}

/// All 12 SF `.pcc` (CUI F-2) parse and every include resolves; their union covers all 131 SF
/// `.lst` (CUI F-1) plus the one shared `_universal/races.lst`.
#[test]
fn sf_all_twelve_pccs_parse_and_every_include_resolves() {
    let Some(root) = corpus_root() else { return };
    let mut pccs = Vec::new();
    walk(&root.join("starfinder"), "pcc", &mut pccs);
    assert_eq!(pccs.len(), 12, "CUI F-2");
    let mut union = BTreeSet::new();
    for pcc in &pccs {
        let res = resolve_pcc_includes_from(&root, pcc).expect("readable PCC");
        assert!(res.diagnostics.is_empty(), "{}: {:?}", pcc.display(), res.diagnostics);
        union.extend(res.lst_files.into_iter().map(|f| f.path));
    }
    let mut all_lst = Vec::new();
    walk(&root.join("starfinder"), "lst", &mut all_lst);
    assert_eq!(all_lst.len(), 131, "CUI F-1");
    let sf: BTreeSet<PathBuf> = union.iter().filter(|p| p.starts_with(root.join("starfinder"))).cloned().collect();
    assert_eq!(sf, all_lst.into_iter().collect::<BTreeSet<_>>(), "every SF .lst is reached by some PCC");
    let outside: Vec<_> = union.iter().filter(|p| !p.starts_with(root.join("starfinder"))).collect();
    assert_eq!(outside, vec![&root.join("_universal/races.lst")]);
}

/// The registry's books resolve to the 123 in-scope SF `.lst` (131 minus SSRGG 5, LPJ 2,
/// core `_society` 1); none of them lies under an excluded book; the campaign-loaded system
/// files (`STAT:` … `GLOBALMODIFIER:`) are the Core Rulebook's eight plus five other books'
/// `VARIABLE:` files (13; near_space and alien_archive_3 load none).
#[test]
fn sf_registry_books_resolve_to_123_in_scope_lst() {
    let Some(root) = corpus_root() else { return };
    let structure = resolve_book_includes(GameSystem::Starfinder1e, &root).expect("SF include structure");
    let sf: Vec<String> = structure
        .lst_files
        .iter()
        .filter_map(|f| f.path.strip_prefix(&root).ok())
        .map(|p| p.to_string_lossy().into_owned())
        .filter(|p| p.starts_with("starfinder/"))
        .collect();
    if let Ok(out) = std::env::var("SF_INCLUDE_LIST_OUT") {
        std::fs::write(out, sf.join("\n") + "\n").expect("write list");
    }
    assert_eq!(sf.len(), 123, "131 SF .lst minus 8 excluded");
    for p in &sf {
        assert!(!EXCLUDED_DIRS.iter().any(|d| p.starts_with(&format!("{d}/"))), "excluded file read: {p}");
    }
    let system: BTreeSet<(String, String)> = structure
        .campaign_system_files()
        .map(|f| (f.kind.clone(), f.path.file_name().unwrap().to_string_lossy().into_owned()))
        .collect();
    let expected: BTreeSet<(String, String)> = [
        ("ALIGNMENT", "scr__align.lst"),
        ("DATATABLE", "scr__datatables.lst"),
        ("DYNAMIC", "scr__dynamic.lst"),
        ("GLOBALMODIFIER", "scr__globalmodifiers.lst"),
        ("SAVE", "scr__saves.lst"),
        ("SIZE", "scr__sizes.lst"),
        ("STAT", "scr__stats.lst"),
        ("VARIABLE", "scr__variables.lst"),
        ("VARIABLE", "sa__variables.lst"),
        ("VARIABLE", "scom__variables.lst"),
        ("VARIABLE", "spw__variables.lst"),
        ("VARIABLE", "saa__variables.lst"),
        ("VARIABLE", "saa2__variables.lst"),
    ]
    .into_iter()
    .map(|(k, f)| (k.to_string(), f.to_string()))
    .collect();
    assert_eq!(system, expected);
    assert_eq!(CAMPAIGN_SYSTEM_KINDS.len(), 8);
}

/// Pathfinder's books are read by directory (`closure::BOOKS_RELATIVE`), not by PCC: asking
/// this registry for them is refused by name, never answered with Starfinder's books.
#[test]
fn sf_pathfinder_has_no_pcc_registry_and_is_refused_by_name() {
    assert!(BOOK_PCCS.books(GameSystem::Pathfinder1e).is_empty());
    let err = resolve_book_includes(GameSystem::Pathfinder1e, Path::new("/nonexistent")).unwrap_err();
    assert!(err.contains("pathfinder-1e") && err.contains("BOOK_PCCS"), "{err}");
}

/// `system/gameModes/Starfinder` (CUI F-18): the 10 `.lst` load as tokenised rows.
#[test]
fn sf_game_mode_loads_ten_lst() {
    let Some(root) = corpus_root() else { return };
    let mode = load_game_mode(&pcgen_repo_dir(&root), "Starfinder").expect("Starfinder game mode");
    let names: Vec<&str> = mode.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(
        names,
        [
            "codeControl.lst",
            "equipmentslots.lst",
            "level.lst",
            "load.lst",
            "migration.lst",
            "miscinfo.lst",
            "pointbuymethods_system.lst",
            "rules.lst",
            "sizeAdjustment.lst",
            "statsandchecks.lst",
        ]
    );
    let pointbuy = mode.iter().find(|f| f.name == "pointbuymethods_system.lst").unwrap();
    assert_eq!(pointbuy.rows[0].tokens, ["STAT:10", "COST:0"]);
    assert!(load_game_mode(&pcgen_repo_dir(&root), "NoSuchMode").is_err());
}

fn walk(dir: &Path, ext: &str, out: &mut Vec<PathBuf>) {
    for e in std::fs::read_dir(dir).unwrap().flatten() {
        let p = e.path();
        if p.is_dir() {
            walk(&p, ext, out);
        } else if p.extension().is_some_and(|x| x == ext) {
            out.push(p);
        }
    }
}
