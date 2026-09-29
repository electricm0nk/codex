//! PCC include graph and LST reference resolver for SD-17 Slice A.
//!
//! This module composes the existing GE-03 PCC parser rather than shadowing it:
//! `PCC:` directives are still parsed by [`crate::pcgen_import::pcc`], while
//! this pass resolves those directives into deterministic graph edges and flat
//! LST file references.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::pcgen_import::pcc::{PccDiagnosticKind, parse_pcc_entry};

/// Result of resolving one PCC entry-file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncludeResolution {
    /// The filesystem root used for `@/` and `*/` PCGen path resolution.
    pub corpus_root: PathBuf,
    /// The root PCC file that was resolved.
    pub source_pcc_path: PathBuf,
    /// PCC files successfully read and parsed in deterministic DFS preorder.
    pub pcc_files: Vec<ResolvedPccFile>,
    /// Directed `PCC:` include edges, preserving source-order traversal.
    pub pcc_edges: Vec<ResolvedPccEdge>,
    /// Deterministic flat LST references emitted by resolved PCC files.
    pub lst_files: Vec<ResolvedLstFile>,
    /// Non-fatal diagnostics found while resolving includes.
    pub diagnostics: Vec<IncludeDiagnostic>,
}

/// A PCC node parsed by the resolver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedPccFile {
    /// Absolute normalized path to the PCC file.
    pub path: PathBuf,
}

/// A directed include edge from one PCC file to another.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedPccEdge {
    /// Absolute normalized source PCC path.
    pub source_path: PathBuf,
    /// Absolute normalized target PCC path.
    pub target_path: PathBuf,
    /// One-based line number of the source `PCC:` directive.
    pub line_number: usize,
    /// Raw directive text preserved as evidence.
    pub raw_directive: String,
    /// Raw target text parsed by the GE-03 PCC parser.
    pub raw_target: String,
}

/// A flat LST reference discovered in a resolved PCC file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedLstFile {
    /// Absolute normalized path to the LST file.
    pub path: PathBuf,
    /// PCC file that emitted this LST reference.
    pub source_pcc_path: PathBuf,
    /// One-based source line of the LST reference.
    pub line_number: usize,
    /// PCC directive kind, e.g. `CLASS`, `RACE`, `SPELL`, `DATACONTROL`.
    pub kind: String,
    /// Raw target text from the directive before path resolution.
    pub raw_target: String,
}

/// Include-resolution diagnostic kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncludeDiagnosticKind {
    /// A `PCC:` directive with no include target, propagated from the GE-03 parser.
    MalformedInclude,
    /// An include or LST target resolved to a missing file.
    MissingTarget,
    /// A `PCC:` include points back to a file already on the active DFS stack.
    CycleDetected,
    /// A source PCC file could not be read.
    ReadFailed,
}

/// Structured diagnostic emitted by the include resolver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncludeDiagnostic {
    /// Absolute normalized source PCC path for the diagnostic.
    pub source_path: PathBuf,
    /// One-based source line when the diagnostic came from a directive.
    pub line_number: Option<usize>,
    /// Raw source line when available.
    pub raw_line: String,
    /// Diagnostic classification.
    pub kind: IncludeDiagnosticKind,
    /// Human-readable diagnostic message.
    pub message: String,
}

/// Resolve a PCC entry-file using an inferred corpus root.
///
/// This convenience API walks upward from `source_pcc_path` and uses the deepest
/// ancestor containing `_universal`, `homebrew`, or `pathfinder` directories as
/// the corpus root. Call [`resolve_pcc_includes_from`] when the corpus root is
/// known explicitly.
pub fn resolve_pcc_includes(
    source_pcc_path: &Path,
) -> Result<IncludeResolution, IncludeDiagnostic> {
    let source_pcc_path = normalize_absolute_path(source_pcc_path);
    let corpus_root = infer_corpus_root(&source_pcc_path).unwrap_or_else(|| {
        source_pcc_path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."))
    });
    resolve_pcc_includes_from(corpus_root, source_pcc_path)
}

/// Resolve a PCC entry-file using an explicit corpus root for PCGen `@/` and
/// `*/` path semantics.
pub fn resolve_pcc_includes_from(
    corpus_root: impl AsRef<Path>,
    source_pcc_path: impl AsRef<Path>,
) -> Result<IncludeResolution, IncludeDiagnostic> {
    let corpus_root = normalize_absolute_path(corpus_root.as_ref());
    let source_pcc_path = normalize_absolute_path(source_pcc_path.as_ref());
    if !source_pcc_path.is_file() {
        return Err(IncludeDiagnostic {
            source_path: source_pcc_path.clone(),
            line_number: None,
            raw_line: String::new(),
            kind: IncludeDiagnosticKind::ReadFailed,
            message: format!("source PCC '{}' is not readable", source_pcc_path.display()),
        });
    }

    let mut resolver = IncludeResolver::new(corpus_root, source_pcc_path.clone());
    resolver.visit(&source_pcc_path);
    Ok(resolver.into_resolution())
}

struct IncludeResolver {
    resolution: IncludeResolution,
    states: HashMap<PathBuf, VisitState>,
    stack: Vec<PathBuf>,
    seen_lst: HashSet<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VisitState {
    Visiting,
    Visited,
}

impl IncludeResolver {
    fn new(corpus_root: PathBuf, source_pcc_path: PathBuf) -> Self {
        Self {
            resolution: IncludeResolution {
                corpus_root,
                source_pcc_path,
                pcc_files: Vec::new(),
                pcc_edges: Vec::new(),
                lst_files: Vec::new(),
                diagnostics: Vec::new(),
            },
            states: HashMap::new(),
            stack: Vec::new(),
            seen_lst: HashSet::new(),
        }
    }

    fn into_resolution(self) -> IncludeResolution {
        self.resolution
    }

    fn visit(&mut self, source_path: &Path) {
        if matches!(self.states.get(source_path), Some(VisitState::Visited)) {
            return;
        }

        let input_text = match fs::read_to_string(source_path) {
            Ok(input_text) => input_text,
            Err(error) => {
                self.resolution.diagnostics.push(IncludeDiagnostic {
                    source_path: source_path.to_path_buf(),
                    line_number: None,
                    raw_line: String::new(),
                    kind: IncludeDiagnosticKind::ReadFailed,
                    message: format!("could not read PCC '{}': {error}", source_path.display()),
                });
                return;
            }
        };

        self.states
            .insert(source_path.to_path_buf(), VisitState::Visiting);
        self.stack.push(source_path.to_path_buf());
        self.resolution.pcc_files.push(ResolvedPccFile {
            path: source_path.to_path_buf(),
        });

        let source_identity = relative_identity(&self.resolution.corpus_root, source_path);
        let parsed = parse_pcc_entry(&source_identity, &input_text);
        self.propagate_parser_diagnostics(source_path, parsed.diagnostics);
        let mut includes_by_line = parsed
            .includes
            .into_iter()
            .map(|include| (include.line_number, include))
            .collect::<HashMap<_, _>>();

        for (index, raw_line) in input_text.lines().enumerate() {
            let line_number = index + 1;
            if let Some(include) = includes_by_line.remove(&line_number) {
                self.resolve_include_edge(source_path, include);
            } else {
                self.collect_lst_reference_line(source_path, line_number, raw_line);
            }
        }

        self.stack.pop();
        self.states
            .insert(source_path.to_path_buf(), VisitState::Visited);
    }

    fn propagate_parser_diagnostics(
        &mut self,
        source_path: &Path,
        diagnostics: Vec<crate::pcgen_import::pcc::PccDiagnostic>,
    ) {
        for diagnostic in diagnostics {
            let kind = match diagnostic.kind {
                PccDiagnosticKind::MalformedInclude => IncludeDiagnosticKind::MalformedInclude,
            };
            self.resolution.diagnostics.push(IncludeDiagnostic {
                source_path: source_path.to_path_buf(),
                line_number: Some(diagnostic.line_number),
                raw_line: diagnostic.raw_line,
                kind,
                message: diagnostic.message,
            });
        }
    }

    fn resolve_include_edge(
        &mut self,
        source_path: &Path,
        include: crate::pcgen_import::pcc::PccIncludeEdge,
    ) {
        let target_path =
            resolve_pcgen_path(&self.resolution.corpus_root, source_path, &include.target);
        self.resolution.pcc_edges.push(ResolvedPccEdge {
            source_path: source_path.to_path_buf(),
            target_path: target_path.clone(),
            line_number: include.line_number,
            raw_directive: include.raw_directive.clone(),
            raw_target: include.target.clone(),
        });

        match self.states.get(&target_path).copied() {
            Some(VisitState::Visited) => {}
            Some(VisitState::Visiting) => {
                self.resolution.diagnostics.push(IncludeDiagnostic {
                    source_path: source_path.to_path_buf(),
                    line_number: Some(include.line_number),
                    raw_line: include.raw_directive,
                    kind: IncludeDiagnosticKind::CycleDetected,
                    message: format!(
                        "PCC include cycle detected: {}",
                        self.cycle_path(&target_path)
                    ),
                });
            }
            None => {
                if target_path.is_file() {
                    self.visit(&target_path);
                } else {
                    self.resolution.diagnostics.push(IncludeDiagnostic {
                        source_path: source_path.to_path_buf(),
                        line_number: Some(include.line_number),
                        raw_line: include.raw_directive,
                        kind: IncludeDiagnosticKind::MissingTarget,
                        message: format!(
                            "PCC include target '{}' resolved from '{}' does not exist",
                            relative_identity(&self.resolution.corpus_root, &target_path),
                            relative_identity(&self.resolution.corpus_root, source_path),
                        ),
                    });
                }
            }
        }
    }

    fn collect_lst_reference_line(
        &mut self,
        source_path: &Path,
        line_number: usize,
        raw_line: &str,
    ) {
        let content = raw_line.trim_start();
        if content.is_empty() || content.starts_with('#') || content.starts_with("PCC:") {
            return;
        }
        let Some((kind, rest)) = content.split_once(':') else {
            return;
        };
        let raw_target = rest.split('|').next().unwrap_or_default().trim();
        if !raw_target.to_ascii_lowercase().contains(".lst") {
            return;
        }
        let target_path = resolve_pcgen_path(&self.resolution.corpus_root, source_path, raw_target);
        if !target_path.is_file() {
            self.resolution.diagnostics.push(IncludeDiagnostic {
                source_path: source_path.to_path_buf(),
                line_number: Some(line_number),
                raw_line: raw_line.to_string(),
                kind: IncludeDiagnosticKind::MissingTarget,
                message: format!(
                    "LST target '{}' resolved from '{}' does not exist",
                    relative_identity(&self.resolution.corpus_root, &target_path),
                    relative_identity(&self.resolution.corpus_root, source_path),
                ),
            });
            return;
        }
        if self.seen_lst.insert(target_path.clone()) {
            self.resolution.lst_files.push(ResolvedLstFile {
                path: target_path,
                source_pcc_path: source_path.to_path_buf(),
                line_number,
                kind: kind.trim().to_string(),
                raw_target: raw_target.to_string(),
            });
        }
    }

    fn cycle_path(&self, target_path: &Path) -> String {
        let start_index = self
            .stack
            .iter()
            .position(|path| path == target_path)
            .unwrap_or(0);
        let mut cycle = self.stack[start_index..].to_vec();
        cycle.push(target_path.to_path_buf());
        cycle
            .iter()
            .map(|path| relative_identity(&self.resolution.corpus_root, path))
            .collect::<Vec<_>>()
            .join(" -> ")
    }
}

fn infer_corpus_root(source_pcc_path: &Path) -> Option<PathBuf> {
    let mut best = None;
    for ancestor in source_pcc_path.ancestors() {
        if ancestor.join("_universal").is_dir()
            || ancestor.join("homebrew").is_dir()
            || ancestor.join("pathfinder").is_dir()
        {
            best = Some(ancestor.to_path_buf());
        }
    }
    best
}

fn resolve_pcgen_path(corpus_root: &Path, source_path: &Path, raw_target: &str) -> PathBuf {
    let first_target = raw_target.split('|').next().unwrap_or(raw_target).trim();
    let normalized_target = first_target.replace('\\', "/");
    let without_at = normalized_target
        .strip_prefix('@')
        .unwrap_or(&normalized_target);
    let resolved = if let Some(rest) = without_at.strip_prefix("*/") {
        corpus_root.join(rest)
    } else if without_at.starts_with('/') {
        corpus_root.join(without_at.trim_start_matches('/'))
    } else {
        source_path
            .parent()
            .map(|parent| parent.join(without_at))
            .unwrap_or_else(|| PathBuf::from(without_at))
    };
    normalize_absolute_path(&resolved)
}

fn normalize_absolute_path(path: &Path) -> PathBuf {
    let mut normalized = if path.is_absolute() {
        PathBuf::new()
    } else {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    };
    for component in path.components() {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(Component::RootDir.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::Normal(part) => normalized.push(part),
        }
    }
    normalized
}

fn relative_identity(corpus_root: &Path, path: &Path) -> String {
    path.strip_prefix(corpus_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
