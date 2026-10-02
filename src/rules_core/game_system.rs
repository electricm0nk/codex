//! The game system a package, a book list or a converter run belongs to.
//!
//! SD-37 Epic 1 (`docs/release/SD-37-starfinder-1e/decisions.md §7`): Pathfinder 1e and
//! Starfinder 1e share one engine, so every place that names a data root or a book list is
//! keyed by a [`GameSystem`]. Pathfinder's paths are unchanged (`data/sheet_rules/`,
//! `data/corpus/`); Starfinder's live under `data/starfinder-1e/`. The id strings are the
//! desktop's `RuleSetId` literals (`apps/desktop/src/characterHub/LandingScreen.tsx`) and the
//! save envelope's `game_system` field (`src/saved_character/mod.rs`).
//!
//! An unknown id is an error ([`UnknownGameSystem`]), never a fallback to a default system.

use std::fmt;
use std::path::{Path, PathBuf};

/// A supported game system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameSystem {
    Pathfinder1e,
    Starfinder1e,
}

/// An id that names no [`GameSystem`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownGameSystem {
    pub id: String,
}

impl fmt::Display for UnknownGameSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let known: Vec<&str> = GameSystem::ALL.iter().map(|s| s.id()).collect();
        write!(f, "unknown game system {:?}; known systems: {}", self.id, known.join(", "))
    }
}

impl std::error::Error for UnknownGameSystem {}

/// One system's package roots, joined onto a repo (or packaged-resource) root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageRoots {
    /// The converted sheet-rule package (`SheetRulePackage`).
    pub sheet_rules: PathBuf,
    /// The per-book converted corpus records.
    pub corpus: PathBuf,
}

impl GameSystem {
    /// Every system, in a fixed order (the index order of [`GameSystem::index`]).
    pub const ALL: [GameSystem; 2] = [GameSystem::Pathfinder1e, GameSystem::Starfinder1e];

    /// The wire id: `pathfinder-1e` or `starfinder-1e`.
    pub const fn id(self) -> &'static str {
        match self {
            GameSystem::Pathfinder1e => "pathfinder-1e",
            GameSystem::Starfinder1e => "starfinder-1e",
        }
    }

    /// The system an id names, or an error naming the id. Exact match only.
    pub fn from_id(id: &str) -> Result<GameSystem, UnknownGameSystem> {
        GameSystem::ALL
            .into_iter()
            .find(|s| s.id() == id)
            .ok_or_else(|| UnknownGameSystem { id: id.to_string() })
    }

    /// Position in [`GameSystem::ALL`], for per-system caches.
    pub const fn index(self) -> usize {
        match self {
            GameSystem::Pathfinder1e => 0,
            GameSystem::Starfinder1e => 1,
        }
    }

    /// The sheet-rule package directory, relative to the repo root.
    pub const fn sheet_rules_relative(self) -> &'static str {
        match self {
            GameSystem::Pathfinder1e => "data/sheet_rules",
            GameSystem::Starfinder1e => "data/starfinder-1e/sheet_rules",
        }
    }

    /// The converted corpus directory, relative to the repo root.
    pub const fn corpus_relative(self) -> &'static str {
        match self {
            GameSystem::Pathfinder1e => "data/corpus",
            GameSystem::Starfinder1e => "data/starfinder-1e/corpus",
        }
    }

    /// This system's package roots under `repo_root`.
    pub fn package_roots(self, repo_root: &Path) -> PackageRoots {
        PackageRoots {
            sheet_rules: repo_root.join(self.sheet_rules_relative()),
            corpus: repo_root.join(self.corpus_relative()),
        }
    }
}

impl fmt::Display for GameSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.id())
    }
}

impl std::str::FromStr for GameSystem {
    type Err = UnknownGameSystem;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        GameSystem::from_id(s)
    }
}

/// The repo root this process reads packages from, resolved at run time: `CODEX_REPO_ROOT`
/// when an operator or launcher sets it (the same variable the desktop's
/// `authoring_workbench::codex_repo_root` honours), else this crate's compile-time checkout.
pub fn runtime_repo_root() -> PathBuf {
    resolve_repo_root(std::env::var_os("CODEX_REPO_ROOT"))
}

/// [`runtime_repo_root`]'s rule as a pure function of the variable's value: a non-empty value
/// wins; unset or empty falls back to `CARGO_MANIFEST_DIR`.
pub fn resolve_repo_root(configured: Option<std::ffi::OsString>) -> PathBuf {
    match configured {
        Some(value) if !value.is_empty() => PathBuf::from(value),
        _ => PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    }
}

/// One value per game system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PerSystem<T> {
    pub pathfinder_1e: T,
    pub starfinder_1e: T,
}

impl<T: Copy> PerSystem<T> {
    /// The value for `system`.
    pub const fn get(&self, system: GameSystem) -> T {
        match system {
            GameSystem::Pathfinder1e => self.pathfinder_1e,
            GameSystem::Starfinder1e => self.starfinder_1e,
        }
    }
}

/// A book list keyed by game system. A system with no books registered has an empty list.
pub type BookRegistry<T> = PerSystem<&'static [T]>;

impl<T: 'static> PerSystem<&'static [T]> {
    /// A registry holding Pathfinder 1e books only; Starfinder registers its own books in
    /// the card that ingests them.
    pub const fn pathfinder_only(pathfinder_1e: &'static [T]) -> Self {
        PerSystem { pathfinder_1e, starfinder_1e: &[] }
    }

    /// The books registered for `system`, in registry order.
    pub const fn books(&self, system: GameSystem) -> &'static [T] {
        match system {
            GameSystem::Pathfinder1e => self.pathfinder_1e,
            GameSystem::Starfinder1e => self.starfinder_1e,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    use std::path::{Path, PathBuf};

    /// Each system id resolves to exactly one root: `from_id(id())` is the identity, no two
    /// systems share an id, and no two systems share a sheet-rule or corpus directory.
    #[test]
    fn game_system_root_each_id_resolves_to_exactly_one_root() {
        let repo = Path::new("/repo");
        let mut ids = BTreeSet::new();
        let mut sheet_dirs = BTreeSet::new();
        let mut corpus_dirs = BTreeSet::new();
        for system in GameSystem::ALL {
            assert_eq!(GameSystem::from_id(system.id()), Ok(system));
            assert!(ids.insert(system.id()), "duplicate id {}", system.id());
            let roots = system.package_roots(repo);
            assert!(sheet_dirs.insert(roots.sheet_rules.clone()), "shared sheet-rule root {:?}", roots.sheet_rules);
            assert!(corpus_dirs.insert(roots.corpus.clone()), "shared corpus root {:?}", roots.corpus);
            assert_ne!(roots.sheet_rules, roots.corpus);
        }
        assert_eq!(ids.len(), GameSystem::ALL.len());
    }

    /// `decisions.md §7`: the Pathfinder root is the current path, unchanged. Starfinder lives
    /// under `data/starfinder-1e/`, and its id matches the desktop's `RuleSetId` literal.
    #[test]
    fn game_system_root_pathfinder_is_the_current_path() {
        let repo = Path::new("/repo");
        let pf = GameSystem::Pathfinder1e.package_roots(repo);
        assert_eq!(pf.sheet_rules, PathBuf::from("/repo/data/sheet_rules"));
        assert_eq!(pf.corpus, PathBuf::from("/repo/data/corpus"));
        assert_eq!(GameSystem::Pathfinder1e.id(), "pathfinder-1e");
        let sf = GameSystem::Starfinder1e.package_roots(repo);
        assert_eq!(sf.sheet_rules, PathBuf::from("/repo/data/starfinder-1e/sheet_rules"));
        assert_eq!(sf.corpus, PathBuf::from("/repo/data/starfinder-1e/corpus"));
        assert_eq!(GameSystem::Starfinder1e.id(), "starfinder-1e");
    }

    /// An unknown id is an error naming the id and the known ids -- never a fallback to a
    /// default system.
    #[test]
    fn game_system_root_unknown_id_is_an_error_not_a_fallback() {
        for bad in ["", "pathfinder", "Pathfinder-1e", "pathfinder-2e", "starfinder-2e", " pathfinder-1e"] {
            let err = GameSystem::from_id(bad).expect_err(bad);
            assert_eq!(err.id, bad);
            let message = err.to_string();
            assert!(message.contains("pathfinder-1e") && message.contains("starfinder-1e"), "{message}");
        }
    }

    /// The root is resolved at run time: an operator-set `CODEX_REPO_ROOT` wins over the
    /// compile-time checkout path; an unset or empty value falls back to the compile-time path.
    #[test]
    fn game_system_root_resolves_at_runtime_before_the_compile_time_path() {
        let compiled = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        assert_eq!(resolve_repo_root(Some("/elsewhere/codex".into())), PathBuf::from("/elsewhere/codex"));
        assert_eq!(resolve_repo_root(None), compiled);
        assert_eq!(resolve_repo_root(Some("".into())), compiled);
    }

    /// The live Pathfinder package is the one `live_sheet_rules()` has always returned: the
    /// same `data/sheet_rules/` load, reached through the system-keyed entry point.
    #[test]
    fn game_system_root_live_pathfinder_package_is_the_default_package() {
        let keyed = crate::rules_core::corpus_loader::live_sheet_rules_for(GameSystem::Pathfinder1e);
        let default = crate::rules_core::corpus_loader::live_sheet_rules();
        assert!(keyed.is_some(), "data/sheet_rules/ carries the Pathfinder package");
        assert!(std::ptr::eq(keyed.unwrap(), default.unwrap()));
    }

    /// A registry built from a Pathfinder book list returns that list, unchanged and in order,
    /// for Pathfinder, and registers no book for any other system.
    #[test]
    fn book_registry_keys_books_by_system() {
        const PF: &[&str] = &["core_rulebook", "advanced_players_guide"];
        const REGISTRY: BookRegistry<&str> = BookRegistry::pathfinder_only(PF);
        assert_eq!(REGISTRY.books(GameSystem::Pathfinder1e), PF);
        assert!(REGISTRY.books(GameSystem::Starfinder1e).is_empty());
        let both = PerSystem { pathfinder_1e: Some("pathfinder/x"), starfinder_1e: None };
        assert_eq!(both.get(GameSystem::Pathfinder1e), Some("pathfinder/x"));
        assert_eq!(both.get(GameSystem::Starfinder1e), None);
    }
}
