//! The converter's INPUT: the pinned source tree read as rows, with the corrected closure
//! indexes (`SYNTHESIS.md` §C4, `blockers.md` B2/B5/B7/B9, `decisions.md` §15 R3).
//!
//! This is the tool side. It reads `.lst` rows from `$PCGEN_CORPUS_ROOT` (the pinned checkout,
//! `scripts/pcgen-oracle-pin.env`) and answers four questions for the converter:
//!
//! 1. **What rows make up one record's closure** -- the base row (by `source.path:line`), the
//!    plain base a `.COPY=` row inherits from (resolved corpus-wide), every `.MOD` row targeting
//!    the record's KEY (matched on (file family, CATEGORY, KEY-else-name), corpus-wide, with every
//!    file under a `_pfs/` directory skipped -- B9), and for a class the whole `CLASS:X` row set
//!    plus its numbered level lines (B5).
//! 2. **Who declares / contributes to a variable name**, case-insensitively, over every row in
//!    the tree -- including rows the corpus never ingested (B2).
//! 3. **The row for a record with no shipped token list** (B7): the base row read directly.
//! 4. **Which file family a row lives in**, by basename, so `.MOD` rows attach only to a record
//!    of the same object family.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// The corpus subtree every core PF1 book lives under, relative to `PCGEN_CORPUS_ROOT`.
pub const BOOKS_RELATIVE: &str = "pathfinder/paizo/roleplaying_game";

/// Books outside `roleplaying_game/` the corpus ships (mirrors `v06_work_inventory`).
pub const EXTRA_BOOK_DIRS: &[&str] = &[
    "pathfinder/dreamscarred_press/ultimate_psionics",
    "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_1",
    "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_2",
    "pathfinder/paizo/campaign_setting/inner_sea_world_guide",
    "pathfinder/paizo/campaign_setting/inner_sea_combat",
    "pathfinder/paizo/campaign_setting/inner_sea_faiths",
    "pathfinder/paizo/campaign_setting/inner_sea_gods",
    "pathfinder/paizo/campaign_setting/inner_sea_magic",
    "pathfinder/paizo/campaign_setting/inner_sea_races",
    "pathfinder/paizo/campaign_setting/inner_sea_temples",
    "pathfinder/paizo/campaign_setting/inner_sea_taverns",
    "pathfinder/paizo/campaign_setting/inner_sea_bestiary",
    "pathfinder/paizo/campaign_setting/inner_sea_intrigue",
];

/// Resolve the pinned corpus root: `$PCGEN_CORPUS_ROOT`, else `$HOME/workspace/repos/pcgen/data`.
pub fn corpus_root() -> PathBuf {
    match std::env::var("PCGEN_CORPUS_ROOT") {
        Ok(configured) => PathBuf::from(configured),
        Err(_) => {
            let home = std::env::var("HOME").expect("HOME must be set to locate the pinned corpus checkout");
            PathBuf::from(home).join("workspace/repos/pcgen/data")
        }
    }
}

/// One `.lst` file, held as lines.
pub struct LstFile {
    /// Path relative to the corpus root, forward slashes (`pathfinder/paizo/.../cr_feats.lst`).
    pub rel_path: String,
    pub book: String,
    pub family: FileFamily,
    pub is_pfs: bool,
    pub lines: Vec<String>,
}

/// The object family a `.lst` file declares, by basename (the same filename rule every other
/// corpus instrument uses; `field-name-is-not-field-meaning`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FileFamily {
    /// Abilities, class features, race traits, companions, feats: one namespace keyed by CATEGORY.
    Ability,
    Class,
    Equipment,
    EquipmentModifier,
    Spell,
    Race,
    Template,
    Domain,
    Deity,
    Skill,
    Language,
    Kit,
    Companion,
    Other,
}

pub fn file_family(rel_path: &str) -> FileFamily {
    let base = rel_path.rsplit('/').next().unwrap_or(rel_path).to_ascii_lowercase();
    if base.contains("abilit") || base.contains("feat") {
        FileFamily::Ability
    } else if base.contains("class") {
        FileFamily::Class
    } else if base.contains("equipmod") {
        FileFamily::EquipmentModifier
    } else if base.contains("equip") {
        FileFamily::Equipment
    } else if base.contains("spell") || base.contains("power") {
        FileFamily::Spell
    } else if base.contains("race") || base.contains("bestiary") || base.contains("monster") {
        FileFamily::Race
    } else if base.contains("template") {
        FileFamily::Template
    } else if base.contains("domain") {
        FileFamily::Domain
    } else if base.contains("deit") {
        FileFamily::Deity
    } else if base.contains("skill") {
        FileFamily::Skill
    } else if base.contains("lang") {
        FileFamily::Language
    } else if base.contains("kit") {
        FileFamily::Kit
    } else if base.contains("companion") || base.contains("familiar") {
        FileFamily::Companion
    } else {
        FileFamily::Other
    }
}

/// A row citation: file index into [`PinnedTree::files`] + 1-based line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RowRef {
    pub file: usize,
    pub line: usize,
}

/// A numbered class level line: `<level>\t<tokens...>` under a `CLASS:` row.
#[derive(Debug, Clone)]
pub struct LevelLine {
    pub level: u8,
    pub row: RowRef,
}

/// The pinned tree with its indexes.
pub struct PinnedTree {
    pub root: PathBuf,
    pub book_paths: BTreeMap<String, PathBuf>,
    pub files: Vec<LstFile>,
    /// `(family, CATEGORY upper, KEY-else-name upper)` -> `.MOD` rows targeting it, tree order,
    /// `_pfs/` files excluded (B9 / R3).
    pub mod_index: BTreeMap<(FileFamily, String, String), Vec<RowRef>>,
    /// `(family, KEY-else-name upper)` -> the PLAIN row declaring it (first wins), for `.COPY=` bases.
    pub base_index: BTreeMap<(FileFamily, String), RowRef>,
    /// `(family, CATEGORY upper, KEY-else-name upper)` -> the plain row, for reference resolution
    /// of names the corpus does not hold.
    pub keyed_index: BTreeMap<(FileFamily, String, String), RowRef>,
    /// Variable NAME (upper) -> every row carrying `DEFINE:<name>|...`.
    pub define_index: BTreeMap<String, Vec<RowRef>>,
    /// Variable NAME (upper) -> every row carrying `BONUS:VAR|<name>|...` (including level lines).
    pub bonus_var_index: BTreeMap<String, Vec<RowRef>>,
    /// Class NAME (upper) -> every `CLASS:<name>` row (continuation rows included), tree order.
    pub class_rows: BTreeMap<String, Vec<RowRef>>,
    /// Class NAME (upper) -> its numbered level lines.
    pub level_lines: BTreeMap<String, Vec<LevelLine>>,
    /// Fact NAME (upper) -> `(value, row)` for every `FACT:<name>|<value>` token.
    pub fact_index: BTreeMap<String, Vec<(String, RowRef)>>,
    /// `_pfs/` files: every `(family, CATEGORY, KEY)` whose BASE row sits in an overlay file.
    pub pfs_base_keys: BTreeSet<(FileFamily, String, String)>,
}

/// Split a raw row into its name field and `(KEY, VALUE)` tokens, tab-separated. A field with
/// no `:` is kept as `(field, "")`. Empty fields (runs of tabs) are dropped.
pub fn tokenize_row(row: &str) -> (String, Vec<(String, String)>) {
    let mut fields = row.trim_end_matches('\r').split('\t').filter(|f| !f.trim().is_empty());
    let name = fields.next().unwrap_or("").trim().to_string();
    let tokens = fields
        .map(|f| {
            let f = f.trim();
            match f.find(':') {
                Some(i) => (f[..i].to_string(), f[i + 1..].to_string()),
                None => (f.to_string(), String::new()),
            }
        })
        .collect();
    (name, tokens)
}

/// The identity a row declares or modifies: `(category, key)` both upper-cased, plus whether it
/// is a `.MOD` row, a `.COPY=` row (with its base identity), or a plain row. A `CLASS:` row's
/// key is the class name; a `CATEGORY=X|Name.MOD` first field carries its category; otherwise
/// the category is the row's own `CATEGORY:` token (empty when absent) and the key is the row's
/// `KEY:` token when present, else the name field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowIdentity {
    pub category: String,
    pub key: String,
    pub shape: RowShape,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RowShape {
    Plain,
    Mod,
    /// `.COPY=<base>`: the base identity's key (upper).
    Copy(String),
    /// A numbered class level line.
    LevelLine(u8),
    /// A comment, blank, or `###` line.
    NotARecord,
}

pub fn row_identity(row: &str) -> RowIdentity {
    let trimmed = row.trim_end_matches('\r');
    let first_nonblank = trimmed.trim_start();
    if first_nonblank.is_empty() || first_nonblank.starts_with('#') {
        return RowIdentity { category: String::new(), key: String::new(), shape: RowShape::NotARecord };
    }
    let (name, tokens) = tokenize_row(trimmed);
    if let Some(level) = name.split(|c: char| !c.is_ascii_digit()).next().filter(|s| !s.is_empty())
        && name.chars().all(|c| c.is_ascii_digit())
        && let Ok(l) = level.parse::<u8>()
    {
        return RowIdentity { category: String::new(), key: String::new(), shape: RowShape::LevelLine(l) };
    }
    if name.starts_with("SOURCE") || name.starts_with("LICENSE") {
        return RowIdentity { category: String::new(), key: String::new(), shape: RowShape::NotARecord };
    }
    let mut category = String::new();
    let mut base = name.clone();
    if let Some(rest) = base.strip_prefix("CATEGORY=")
        && let Some((cat, rest)) = rest.split_once('|')
    {
        category = cat.trim().to_ascii_uppercase();
        base = rest.to_string();
    }
    if let Some(rest) = base.strip_prefix("CLASS:") {
        base = rest.to_string();
    }
    let shape;
    if let Some(stripped) = base.strip_suffix(".MOD") {
        base = stripped.to_string();
        shape = RowShape::Mod;
    } else if let Some((b, copy_base)) = base.split_once(".COPY=") {
        let copy_from = copy_base.trim().to_ascii_uppercase();
        base = b.to_string();
        shape = RowShape::Copy(copy_from);
    } else {
        shape = RowShape::Plain;
    }
    if shape != RowShape::Mod && category.is_empty()
        && let Some((_, v)) = tokens.iter().find(|(k, _)| k == "CATEGORY")
    {
        category = v.trim().to_ascii_uppercase();
    }
    // A plain row's key is its KEY: token when present.
    let key = if shape == RowShape::Mod {
        base.trim().to_ascii_uppercase()
    } else if let Some((_, v)) = tokens.iter().find(|(k, _)| k == "KEY") {
        v.trim().to_ascii_uppercase()
    } else {
        base.trim().to_ascii_uppercase()
    };
    RowIdentity { category, key, shape }
}

fn walk_lst(dir: &Path, out: &mut Vec<PathBuf>) {
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
    out.sort();
}

impl PinnedTree {
    /// Read every `.lst` file under every known book directory and build the indexes.
    pub fn load(root: &Path) -> Result<PinnedTree, String> {
        let books_dir = root.join(BOOKS_RELATIVE);
        if !books_dir.is_dir() {
            return Err(format!(
                "pinned corpus not found at {} -- set PCGEN_CORPUS_ROOT to a PCGen data/ checkout",
                books_dir.display()
            ));
        }
        let mut book_paths: BTreeMap<String, PathBuf> = std::fs::read_dir(&books_dir)
            .map_err(|e| e.to_string())?
            .flatten()
            .filter(|e| e.path().is_dir())
            .map(|e| (e.file_name().to_string_lossy().into_owned(), e.path()))
            .collect();
        for extra in EXTRA_BOOK_DIRS {
            let path = root.join(extra);
            if !path.is_dir() {
                return Err(format!("extra book directory not found at {}", path.display()));
            }
            let id = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
            book_paths.insert(id, path);
        }
        let mut files: Vec<LstFile> = Vec::new();
        for (book, dir) in &book_paths {
            let mut paths = Vec::new();
            walk_lst(dir, &mut paths);
            for p in paths {
                let Ok(text) = std::fs::read_to_string(&p) else { continue };
                let rel = p.strip_prefix(root).unwrap_or(&p).to_string_lossy().replace('\\', "/");
                let is_pfs = rel.split('/').any(|seg| seg == "_pfs");
                files.push(LstFile {
                    family: file_family(&rel),
                    rel_path: rel,
                    book: book.clone(),
                    is_pfs,
                    lines: text.split('\n').map(|s| s.trim_end_matches('\r').to_string()).collect(),
                });
            }
        }
        let mut tree = PinnedTree {
            root: root.to_path_buf(),
            book_paths,
            files,
            mod_index: BTreeMap::new(),
            base_index: BTreeMap::new(),
            keyed_index: BTreeMap::new(),
            define_index: BTreeMap::new(),
            bonus_var_index: BTreeMap::new(),
            class_rows: BTreeMap::new(),
            level_lines: BTreeMap::new(),
            fact_index: BTreeMap::new(),
            pfs_base_keys: BTreeSet::new(),
        };
        tree.build_indexes();
        Ok(tree)
    }

    fn build_indexes(&mut self) {
        let mut mod_index: BTreeMap<(FileFamily, String, String), Vec<RowRef>> = BTreeMap::new();
        let mut base_index: BTreeMap<(FileFamily, String), RowRef> = BTreeMap::new();
        let mut keyed_index: BTreeMap<(FileFamily, String, String), RowRef> = BTreeMap::new();
        let mut define_index: BTreeMap<String, Vec<RowRef>> = BTreeMap::new();
        let mut bonus_var_index: BTreeMap<String, Vec<RowRef>> = BTreeMap::new();
        let mut class_rows: BTreeMap<String, Vec<RowRef>> = BTreeMap::new();
        let mut level_lines: BTreeMap<String, Vec<LevelLine>> = BTreeMap::new();
        let mut fact_index: BTreeMap<String, Vec<(String, RowRef)>> = BTreeMap::new();
        let mut pfs_base_keys = BTreeSet::new();
        for (fi, file) in self.files.iter().enumerate() {
            let mut current_class: Option<String> = None;
            for (li, raw) in file.lines.iter().enumerate() {
                let row = RowRef { file: fi, line: li + 1 };
                let id = row_identity(raw);
                match &id.shape {
                    RowShape::NotARecord => continue,
                    RowShape::LevelLine(level) => {
                        if let Some(cls) = &current_class {
                            level_lines.entry(cls.clone()).or_default().push(LevelLine { level: *level, row });
                        }
                    }
                    RowShape::Mod => {
                        if !file.is_pfs {
                            mod_index.entry((file.family, id.category.clone(), id.key.clone())).or_default().push(row);
                        }
                    }
                    RowShape::Plain | RowShape::Copy(_) => {
                        if file.family == FileFamily::Class {
                            current_class = Some(id.key.clone());
                            class_rows.entry(id.key.clone()).or_default().push(row);
                        }
                        if matches!(id.shape, RowShape::Plain) {
                            base_index.entry((file.family, id.key.clone())).or_insert(row);
                            keyed_index.entry((file.family, id.category.clone(), id.key.clone())).or_insert(row);
                            if file.is_pfs {
                                pfs_base_keys.insert((file.family, id.category.clone(), id.key.clone()));
                            }
                        }
                    }
                }
                // Variable and fact declarations: every row shape, every file (B2 needs the
                // never-ingested rows too; `_pfs/` rows are excluded so an overlay-only DEFINE
                // does not resolve as a real declarer).
                if file.is_pfs {
                    continue;
                }
                let (_, tokens) = tokenize_row(raw);
                for (k, v) in &tokens {
                    match k.as_str() {
                        "DEFINE" => {
                            if let Some((name, _)) = v.split_once('|') {
                                define_index.entry(name.trim().to_ascii_uppercase()).or_default().push(row);
                            }
                        }
                        "BONUS" => {
                            if let Some(rest) = v.strip_prefix("VAR|")
                                && let Some((name, _)) = rest.split_once('|')
                            {
                                define_or_push(&mut bonus_var_index, name, row);
                            }
                        }
                        "FACT" => {
                            if let Some((name, val)) = v.split_once('|') {
                                fact_index
                                    .entry(name.trim().to_ascii_uppercase())
                                    .or_default()
                                    .push((val.trim().to_string(), row));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        self.mod_index = mod_index;
        self.base_index = base_index;
        self.keyed_index = keyed_index;
        self.define_index = define_index;
        self.bonus_var_index = bonus_var_index;
        self.class_rows = class_rows;
        self.level_lines = level_lines;
        self.fact_index = fact_index;
        self.pfs_base_keys = pfs_base_keys;
    }

    pub fn row_text(&self, r: RowRef) -> &str {
        self.files[r.file].lines.get(r.line - 1).map(|s| s.as_str()).unwrap_or("")
    }

    pub fn cite(&self, r: RowRef) -> String {
        format!("{}:{}", self.files[r.file].rel_path, r.line)
    }

    pub fn file_index(&self, rel_path: &str) -> Option<usize> {
        self.files.iter().position(|f| f.rel_path == rel_path)
    }

    /// The `.MOD` rows targeting `(family, category, key)`, corpus-wide, tree order.
    pub fn mods_for(&self, family: FileFamily, category: &str, key: &str) -> &[RowRef] {
        self.mod_index
            .get(&(family, category.to_ascii_uppercase(), key.to_ascii_uppercase()))
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Every row that declares or contributes to variable `name` (case-insensitive).
    pub fn variable_rows(&self, name: &str) -> Vec<RowRef> {
        let key = name.trim().to_ascii_uppercase();
        let mut out: Vec<RowRef> = Vec::new();
        if let Some(v) = self.define_index.get(&key) {
            out.extend(v.iter().copied());
        }
        if let Some(v) = self.bonus_var_index.get(&key) {
            out.extend(v.iter().copied());
        }
        out.sort();
        out.dedup();
        out
    }

    pub fn is_declared(&self, name: &str) -> bool {
        self.define_index.contains_key(&name.trim().to_ascii_uppercase())
    }
}

fn define_or_push(index: &mut BTreeMap<String, Vec<RowRef>>, name: &str, row: RowRef) {
    index.entry(name.trim().to_ascii_uppercase()).or_default().push(row);
}

/// One record's closure: its rows in PCGen application order.
#[derive(Debug, Clone, Default)]
pub struct Closure {
    pub rows: Vec<ClosureRow>,
    /// The record's own rows (base, continuation, copy base) as citations -- what "same record"
    /// means for the variable fold.
    pub own_rows: BTreeSet<RowRef>,
    pub overlay: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ClosureRow {
    pub cite: String,
    pub row: Option<RowRef>,
    pub tokens: Vec<(String, String)>,
    pub kind: ClosureRowKind,
    /// A gate every token on this row carries in addition to its own (class level lines).
    pub level_gate: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClosureRowKind {
    Base,
    CopyBase,
    Mod,
    LevelLine,
}

impl PinnedTree {
    /// Build the closure for one record whose base row is `rel_path:line`. `shipped_tokens`
    /// are the corpus record's own (already product-identity-screened) tokens when it has any;
    /// they replace the base row's tokens, and the base row is still cited. `category`/`key` are
    /// the record's identity for `.MOD` matching.
    pub fn closure(
        &self,
        rel_path: &str,
        line: usize,
        shipped_tokens: Option<&[(String, String)]>,
        category: &str,
        key: &str,
        copy_base_key: Option<&str>,
    ) -> Closure {
        let mut out = Closure::default();
        let file_idx = self.file_index(rel_path);
        let family = file_idx.map(|i| self.files[i].family).unwrap_or_else(|| file_family(rel_path));
        let base_ref = file_idx.map(|i| RowRef { file: i, line });
        let cat_u = category.to_ascii_uppercase();
        let key_u = key.to_ascii_uppercase();
        if let Some(i) = file_idx
            && self.files[i].is_pfs
        {
            out.overlay = Some("pfs".to_string());
        }
        // `.COPY=` base first (PCGen order: base -> own row -> mods).
        let base_row_text = base_ref.map(|r| self.row_text(r).to_string()).unwrap_or_default();
        let base_identity = row_identity(&base_row_text);
        let copy_from = copy_base_key
            .map(|s| s.to_ascii_uppercase())
            .or_else(|| if let RowShape::Copy(b) = &base_identity.shape { Some(b.clone()) } else { None });
        if let Some(copy_key) = copy_from
            && let Some(r) = self.base_index.get(&(family, copy_key))
        {
            let (_, tokens) = tokenize_row(self.row_text(*r));
            out.own_rows.insert(*r);
            out.rows.push(ClosureRow { cite: self.cite(*r), row: Some(*r), tokens, kind: ClosureRowKind::CopyBase, level_gate: None });
        }
        // Own row(s).
        let own_tokens = match shipped_tokens {
            Some(t) if !t.is_empty() => t.to_vec(),
            _ => tokenize_row(&base_row_text).1,
        };
        if let Some(r) = base_ref {
            out.own_rows.insert(r);
        }
        out.rows.push(ClosureRow {
            cite: base_ref.map(|r| self.cite(r)).unwrap_or_else(|| format!("{rel_path}:{line}")),
            row: base_ref,
            tokens: own_tokens,
            kind: ClosureRowKind::Base,
            level_gate: None,
        });
        // A class: continuation rows and numbered level lines (B5).
        if family == FileFamily::Class {
            if let Some(rows) = self.class_rows.get(&key_u) {
                for r in rows {
                    if Some(*r) == base_ref {
                        continue;
                    }
                    if file_idx.is_some_and(|i| i != r.file) {
                        // A same-named class in another file is another book's twin; its
                        // rows attach through `.MOD` semantics only when PCGen would merge
                        // them, which it does not for two plain declarations.
                        continue;
                    }
                    let (_, tokens) = tokenize_row(self.row_text(*r));
                    out.own_rows.insert(*r);
                    out.rows.push(ClosureRow { cite: self.cite(*r), row: Some(*r), tokens, kind: ClosureRowKind::Base, level_gate: None });
                }
            }
            if let Some(lines) = self.level_lines.get(&key_u) {
                for ll in lines {
                    if file_idx.is_some_and(|i| i != ll.row.file) {
                        continue;
                    }
                    let (_, tokens) = tokenize_row(self.row_text(ll.row));
                    out.own_rows.insert(ll.row);
                    out.rows.push(ClosureRow {
                        cite: self.cite(ll.row),
                        row: Some(ll.row),
                        tokens,
                        kind: ClosureRowKind::LevelLine,
                        level_gate: Some(ll.level),
                    });
                }
            }
        }
        // `.MOD` rows, corpus-wide, KEY-matched, `_pfs/` excluded (B9 / R3).
        let mod_family = match family {
            FileFamily::Companion => FileFamily::Ability,
            other => other,
        };
        for r in self.mods_for(mod_family, &cat_u, &key_u) {
            let (_, tokens) = tokenize_row(self.row_text(*r));
            out.rows.push(ClosureRow { cite: self.cite(*r), row: Some(*r), tokens, kind: ClosureRowKind::Mod, level_gate: None });
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_identity_reads_category_key_and_shape() {
        let id = row_identity("CATEGORY=Special Ability|Foo.MOD\tDESC:x");
        assert_eq!(id, RowIdentity { category: "SPECIAL ABILITY".into(), key: "FOO".into(), shape: RowShape::Mod });
        let id = row_identity("Bar\tKEY:Real Key\tCATEGORY:FEAT\tTYPE:General");
        assert_eq!(id.category, "FEAT");
        assert_eq!(id.key, "REAL KEY");
        assert_eq!(id.shape, RowShape::Plain);
        let id = row_identity("CLASS:Fighter\tHD:10");
        assert_eq!(id.key, "FIGHTER");
        assert_eq!(id.shape, RowShape::Plain);
        let id = row_identity("3\tABILITY:Class|AUTOMATIC|Fighter Armor Training");
        assert_eq!(id.shape, RowShape::LevelLine(3));
        let id = row_identity("Longsword.COPY=Longsword (Base)\tKEY:Longsword");
        assert_eq!(id.shape, RowShape::Copy("LONGSWORD (BASE)".into()));
        assert_eq!(id.key, "LONGSWORD");
        assert_eq!(row_identity("# comment").shape, RowShape::NotARecord);
        assert_eq!(row_identity("SOURCELONG:Core Rulebook\tSOURCESHORT:CR").shape, RowShape::NotARecord);
    }

    #[test]
    fn file_family_is_by_basename() {
        assert_eq!(file_family("x/cr_abilities_class.lst"), FileFamily::Ability);
        assert_eq!(file_family("x/cr_feats.lst"), FileFamily::Ability);
        assert_eq!(file_family("x/cr_classes.lst"), FileFamily::Class);
        assert_eq!(file_family("x/cr_equipmods.lst"), FileFamily::EquipmentModifier);
        assert_eq!(file_family("x/cr_equip_arms_armor.lst"), FileFamily::Equipment);
        assert_eq!(file_family("x/b1_races.lst"), FileFamily::Race);
    }
}
