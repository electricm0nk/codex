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
    /// Child `ABILITYCATEGORY` name (upper) -> its declared parent `CATEGORY` (upper), for every
    /// `ABILITYCATEGORY:<name>...CATEGORY:<parent>...` row in the tree whose parent differs from
    /// its own name (a category that names itself under its own `CATEGORY:` token, e.g. `Special
    /// Ability`, is not a child of anything). `_pfs/` overlay rows are excluded, same as
    /// `define_index`/`bonus_var_index`.
    ///
    /// A child name declared with two DIFFERENT parents anywhere in the tree is left OUT of this
    /// map entirely -- [`super::ctx::resolve_rule_in`]'s parent retry must never guess between
    /// them, so an ambiguous child category resolves exactly like one with no known parent (a
    /// miss, which reaches `resolve_holdable_rule`'s own defect row, same as today).
    pub ability_category_parent: BTreeMap<String, String>,
    /// SD-36 F1c-3 (D6): a child `ABILITYCATEGORY`'s member filter -- name (upper) ->
    /// `TYPE:` tags, for every child category in [`Self::ability_category_parent`] that selects
    /// its members by `TYPE:` and names no explicit `ABILITYLIST:`. A name declared with two
    /// different tag sets is left out (never guessed), like an ambiguous parent.
    pub ability_category_type: BTreeMap<String, Vec<String>>,
    /// SD-36 F1c-5 (D8): a category's `POOL:` variable -- category name (upper) -> (the `POOL:`
    /// value upper, the declaring row), for every `ABILITYCATEGORY:` row carrying one. PCGen sizes
    /// the category's pool by that formula (`apg_abilitycategories.lst:267`
    /// `POOL:Pool_Summoner_Class_Selection`). A name declared with two different `POOL:` values is
    /// left out (never guessed).
    pub ability_category_pool: BTreeMap<String, (String, RowRef)>,
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

/// A `.COPY=` row's own tokens are applied **after** the tokens it inherits from the row it
/// copies -- PCGen's own order, the one this module's doc and `closure()`'s own comment already
/// state ("base -> own row -> mods").
///
/// SD-35 `AT-35-E6-003` cycle 10. The corpus ingest flattens a `.COPY=` record into ONE
/// `raw_tokens` array with the copy row's own tokens FIRST and the inherited ones after, and
/// `closure()` uses that shipped array in place of the base row's. `convert_token` assigns
/// last-wins for every metadata head, so the flattening silently handed the *inherited* value
/// to every head the copy row overrides. The visible cost: 473 corpus records whose copy row
/// states `VISIBLE:NO` (PCGen's own bookkeeping shadows -- e.g.
/// `Intelligent Item ~ Alignment / Lawful Good.COPY=Intelligent Item Alignment (LG)`, whose
/// inherited `VISIBLE:QUALIFY` won) converted with `print: true`, so the package said a row a
/// player never sees in PCGen's own item builder belongs on a character sheet.
///
/// This is a **stable partition, never a rewrite**: every shipped `(key, value)` pair the copy
/// row itself states moves to the end of the list, in its own order, and nothing else moves.
/// No token is added and none is dropped, so a PI-screened shipped list stays exactly as
/// screened -- the defect was ordering, not absence, and re-reading the unscreened pinned row
/// for content would be a different (and forbidden) change.
///
/// A base row that is not a `.COPY=` row is returned untouched.
fn copy_own_tokens_last(
    shipped: &[(String, String)],
    base_row_text: &str,
    base_identity: &RowIdentity,
) -> Vec<(String, String)> {
    if !matches!(base_identity.shape, RowShape::Copy(_)) {
        return shipped.to_vec();
    }
    let own: BTreeSet<(String, String)> = tokenize_row(base_row_text).1.into_iter().collect();
    if own.is_empty() {
        return shipped.to_vec();
    }
    let mut inherited: Vec<(String, String)> = Vec::with_capacity(shipped.len());
    let mut overrides: Vec<(String, String)> = Vec::new();
    for pair in shipped {
        if own.contains(pair) {
            overrides.push(pair.clone());
        } else {
            inherited.push(pair.clone());
        }
    }
    inherited.extend(overrides);
    inherited
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
            ability_category_parent: BTreeMap::new(),
            ability_category_type: BTreeMap::new(),
            ability_category_pool: BTreeMap::new(),
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
        let mut ability_category_parent: BTreeMap<String, String> = BTreeMap::new();
        let mut ability_category_ambiguous: BTreeSet<String> = BTreeSet::new();
        let mut ability_category_type: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut ability_category_listed: BTreeSet<String> = BTreeSet::new();
        let mut ability_category_pool: BTreeMap<String, (String, RowRef)> = BTreeMap::new();
        let mut ability_category_pool_ambiguous: BTreeSet<String> = BTreeSet::new();
        // `CATEGORY:Aligned Class` `BONUS:VAR` contributions (name upper, row, the row's own
        // `id.key` upper) buffered here instead of indexed inline -- see the comment where they
        // are pushed, below. Deferred to a second pass over `class_rows` because a row's OWNER
        // may live in a file the forward scan has not reached yet (`class_rows` is built file by
        // file, in the same single pass, and file order is not name order).
        let mut aligned_class_pending: Vec<(String, RowRef, String)> = Vec::new();
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
                        // Only a GENUINE `CLASS:` header row re-anchors `current_class`. Every
                        // other `Plain`/`Copy`-shaped row in a Class-family file -- notably
                        // `SUBCLASS:<School>` and `SUBCLASSLEVEL:<n>` rows, which `row_identity`
                        // also classifies as `Plain` (neither starts with `CLASS:`, neither is a
                        // pure-digit level line) -- must NOT clobber it. Before this fix, the
                        // last such row before a numbered level line silently stole every
                        // following level line's attribution (SD-36 Epic F silent-miss
                        // diagnosis: Wizard's own level-1 self-grant, `cr_classes.lst:301`, filed
                        // under `current_class="SUBCLASSLEVEL:1"` -- a key nothing ever looks
                        // up -- because it sits after eight `SUBCLASS:`/`SUBCLASSLEVEL:1` row
                        // pairs). Checked on the RAW row text (before `row_identity` strips the
                        // `CLASS:` prefix), so a `.COPY=`-shaped class header
                        // (`CLASS:X.COPY=Y`) still counts.
                        let is_class_header = file.family == FileFamily::Class && raw.trim_start().to_ascii_uppercase().starts_with("CLASS:");
                        if is_class_header {
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
                let (name, tokens) = tokenize_row(raw);
                // `ABILITYCATEGORY:<name>` rows declare a category, sometimes as a CHILD of
                // another (`CATEGORY:<parent>` token on the same row, differing from its own
                // name) -- the map `resolve_rule_in`'s parent retry reads, built here from the
                // tree's own rows, never a hardcoded list (`epic-f-class-completion.md` §3.1
                // item 1).
                let upper_name = name.to_ascii_uppercase();
                if let Some(rest) = upper_name.strip_prefix("ABILITYCATEGORY:") {
                    let own = rest.trim().to_string();
                    if !own.is_empty()
                        && let Some((_, v)) = tokens.iter().find(|(k, _)| k.eq_ignore_ascii_case("CATEGORY"))
                    {
                        let parent = v.trim().to_ascii_uppercase();
                        if !parent.is_empty() && parent != own {
                            match ability_category_parent.get(&own) {
                                Some(existing) if existing != &parent => {
                                    ability_category_ambiguous.insert(own.clone());
                                }
                                Some(_) => {}
                                None => {
                                    ability_category_parent.insert(own.clone(), parent);
                                }
                            }
                        }
                    }
                    if !own.is_empty()
                        && let Some((_, v)) = tokens.iter().find(|(k, _)| k == "POOL")
                    {
                        let pool = v.trim().to_ascii_uppercase();
                        match ability_category_pool.get(&own) {
                            Some((existing, _)) if existing != &pool => {
                                ability_category_pool_ambiguous.insert(own.clone());
                            }
                            Some(_) => {}
                            None => {
                                ability_category_pool.insert(own.clone(), (pool, row));
                            }
                        }
                    }
                    if !own.is_empty() {
                        if tokens.iter().any(|(k, _)| k.eq_ignore_ascii_case("ABILITYLIST")) {
                            ability_category_listed.insert(own.clone());
                        }
                        if let Some((_, v)) = tokens.iter().find(|(k, _)| k.eq_ignore_ascii_case("TYPE")) {
                            let tags: Vec<String> = v.split('.').map(str::trim).filter(|t| !t.is_empty()).map(str::to_string).collect();
                            match ability_category_type.get(&own) {
                                Some(existing) if existing != &tags => {
                                    ability_category_ambiguous.insert(own);
                                }
                                Some(_) => {}
                                None => {
                                    ability_category_type.insert(own, tags);
                                }
                            }
                        }
                    }
                }
                for (k, v) in &tokens {
                    match k.as_str() {
                        "DEFINE" => {
                            if let Some((name, _)) = v.split_once('|') {
                                define_index.entry(name.trim().to_ascii_uppercase()).or_default().push(row);
                            }
                        }
                        "BONUS" => {
                            if let Some(rest) = v.strip_prefix("VAR|")
                                && let Some((names, _)) = rest.split_once('|')
                            {
                                // `BONUS:VAR|<target>|...` names ONE OR MORE comma-separated
                                // target variables (PCGen's own grammar; `bonus_chain_reader.rs`'s
                                // `var_contributions` and `mod.rs`'s per-record
                                // `own_var_contribs` builder both already split on `,` for exactly
                                // this reason). Indexing the raw, unsplit field as one combined
                                // key (`"BloodrageStrBonus,BloodrageConBonus"`) instead of the two
                                // real names it names left `variable_rows("BloodrageStrBonus")`
                                // and `variable_rows("BloodrageConBonus")` unable to find this row
                                // at all under EITHER real name -- so `resolve_variable`'s
                                // same-record-only check saw only the DEFINE row and silently
                                // folded a genuinely multi-record, level-scaling variable
                                // (`Bloodrager ~ Bloodrage`'s Str line, gated identically to its
                                // already-correctly-Var Con/Will siblings by `Bloodrager ~
                                // Greater/Mighty Bloodrage`) to a flat, never-scaling constant
                                // (SD-36 Epic F1 merge-readiness blocker 2: the printed sheet
                                // block showed Con/Will upgrading at 11th/20th level while Str
                                // silently never did -- an internally inconsistent, PF1-wrong
                                // read). A mechanical grammar fix, not a per-class one: it applies
                                // identically to every multi-target `BONUS:VAR` row in the corpus.
                                //
                                // `CATEGORY:Aligned Class` rows (real PCGen shape,
                                // `isg_abilities.lst`: Inner Sea Gods' Evangelist prestige class
                                // re-exports EVERY base class's own level-tracking var under a
                                // MIRROR record of the same name -- `Fighter`/`Vigilante`
                                // `CATEGORY:Aligned Class` both carry
                                // `BONUS:VAR|<Class>_CFP_Level,<Class>LVL|EvangelistLVL-1`) are
                                // NEVER indexed inline here -- buffered instead, and resolved once
                                // `class_rows` (every file's own `CLASS:` headers) is complete
                                // below. Blocker 1c (stage 5) excluded every such row outright, but
                                // that over-reaches on the record that OWNS its own target var --
                                // Inner Sea Gods' `Winter Witch` row's `BONUS:VAR|WinterWitchLVL|
                                // EvangelistLVL-1` declares a name no OTHER row in the pinned tree
                                // ever claims (the Reign of Winter book that would is an Adventure
                                // Path, outside `BOOKS_RELATIVE`/`EXTRA_BOOK_DIRS`), so excluding it
                                // left `WinterWitchLVL` DEFINEd nowhere -- `resolve_variable`
                                // (`ctx.rs`) reads that as C1(c), undefined, and flattened
                                // `winter_witch#bonus1`'s caster level to a constant. The correct,
                                // mechanical (never per-class) line: an Aligned Class row's target
                                // is excluded only when some OTHER record -- a genuine `CLASS:`
                                // header anywhere in the tree, keyed the same as this row's own
                                // identity -- already owns that class name; Vigilante's and
                                // Fighter's own `CLASS:` rows exist elsewhere in the tree (real
                                // base classes an Aligned Class row merely mirrors), so those stay
                                // excluded exactly as blocker 1c intended.
                                if id.category == "ALIGNED CLASS" {
                                    for name in names.split(',') {
                                        aligned_class_pending.push((name.trim().to_ascii_uppercase(), row, id.key.clone()));
                                    }
                                } else {
                                    for name in names.split(',') {
                                        define_or_push(&mut bonus_var_index, name, row);
                                    }
                                }
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
        for name in ability_category_ambiguous.iter().chain(&ability_category_listed) {
            ability_category_type.remove(name);
        }
        ability_category_type.retain(|name, tags| !tags.is_empty() && ability_category_parent.contains_key(name));
        for name in &ability_category_ambiguous {
            ability_category_parent.remove(name);
        }
        for name in &ability_category_pool_ambiguous {
            ability_category_pool.remove(name);
        }
        // Resolve the buffered `Aligned Class` `BONUS:VAR` contributions now that `class_rows`
        // holds every file's `CLASS:` headers, tree-wide: a contribution is indexed only when NO
        // genuine `CLASS:<key>` row (the row's own identity key) exists anywhere else in the tree
        // -- otherwise it is a mirror of an externally-owned base class, excluded exactly as
        // blocker 1c intended.
        for (name, row, key) in aligned_class_pending {
            if !class_rows.contains_key(&key) {
                define_or_push(&mut bonus_var_index, &name, row);
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
        self.ability_category_parent = ability_category_parent;
        self.ability_category_type = ability_category_type;
        self.ability_category_pool = ability_category_pool;
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
            Some(t) if !t.is_empty() => copy_own_tokens_last(t, &base_row_text, &base_identity),
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
        // A class is found by the name its OWN base row declares (`CLASS:<name>`), never by the
        // corpus record's key: a class whose name is product identity ships a codex-named
        // placeholder key (`Codex-Named Unit (class_..._lst_136)`), and keying on it silently
        // dropped the class's continuation rows, level lines and `.MOD` rows -- its prerequisites,
        // class skills and `ABILITY:` grants (SD-36 F1c-3; 21 of 189 class records, e.g. Golden
        // Legionnaire's `ABILITY:Internal|AUTOMATIC|Weapon Prof ~ ...`, `ag_classes.lst:140`).
        let key_u = if family == FileFamily::Class
            && base_row_text.trim_start().to_ascii_uppercase().starts_with("CLASS:")
            && !base_identity.key.is_empty()
        {
            base_identity.key.clone()
        } else {
            key_u
        };
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

    /// A `PinnedTree` built directly from in-memory lines (no corpus checkout on disk), for
    /// indexer unit tests that must not pay the ~75s cold build / whole-tree-load cost the live
    /// oracle checkout carries.
    fn tree_from_lines(files: Vec<(&str, Vec<&str>)>) -> PinnedTree {
        let mut tree = PinnedTree {
            root: PathBuf::new(),
            book_paths: BTreeMap::new(),
            files: files
                .into_iter()
                .map(|(name, lines)| LstFile {
                    rel_path: name.to_string(),
                    book: "test".to_string(),
                    family: file_family(name),
                    is_pfs: false,
                    lines: lines.into_iter().map(|s| s.to_string()).collect(),
                })
                .collect(),
            mod_index: BTreeMap::new(),
            base_index: BTreeMap::new(),
            keyed_index: BTreeMap::new(),
            define_index: BTreeMap::new(),
            bonus_var_index: BTreeMap::new(),
            class_rows: BTreeMap::new(),
            level_lines: BTreeMap::new(),
            fact_index: BTreeMap::new(),
            pfs_base_keys: BTreeSet::new(),
            ability_category_parent: BTreeMap::new(),
            ability_category_type: BTreeMap::new(),
            ability_category_pool: BTreeMap::new(),
        };
        tree.build_indexes();
        tree
    }

    /// SD-36 Epic F1-1 -- the closure-indexer silent miss (`epic-f-class-completion.md` §0.3):
    /// eight `SUBCLASS:`/`SUBCLASSLEVEL:1` row pairs between a class's `CLASS:` header and its
    /// own level-1 self-grant must not steal `current_class` attribution, or the self-grant files
    /// under a `SUBCLASSLEVEL:1` key no real record ever looks up and silently vanishes from the
    /// class's own closure (no crash, no defect -- diagnosed on Wizard, `cr_classes.lst:277-301`).
    #[test]
    fn a_subclass_row_does_not_steal_current_class_from_its_level_lines() {
        let tree = tree_from_lines(vec![(
            "cr_classes.lst",
            vec![
                "CLASS:Wizard\tHD:6",
                "SUBCLASS:Abjurer\tCOST:0",
                "SUBCLASSLEVEL:1\tABILITY:Class|AUTOMATIC|Abjurer Bonus",
                "SUBCLASS:Evoker\tCOST:0",
                "SUBCLASSLEVEL:1\tABILITY:Class|AUTOMATIC|Evoker Bonus",
                "1\tABILITY:Class|AUTOMATIC|Wizard",
            ],
        )]);
        assert!(!tree.level_lines.contains_key("SUBCLASSLEVEL:1"), "no real class is ever keyed 'SUBCLASSLEVEL:1'; this key must not exist");
        let wizard_lines = tree.level_lines.get("WIZARD").expect("Wizard's own level-1 self-grant must attach to WIZARD");
        assert_eq!(wizard_lines.len(), 1, "exactly the one real level-1 line, not the SUBCLASS/SUBCLASSLEVEL rows");
        assert_eq!(wizard_lines[0].level, 1);
    }

    /// A genuine second `CLASS:` header row (a real continuation, e.g. Wizard's own second and
    /// third `CLASS:Wizard` declarations in the pinned oracle) still re-anchors `current_class`
    /// and is still recorded in `class_rows` -- the fix narrows what counts as a header, it does
    /// not stop recognising real ones.
    #[test]
    fn a_second_class_header_row_still_reanchors_current_class() {
        let tree = tree_from_lines(vec![(
            "cr_classes.lst",
            vec!["CLASS:Wizard\tHD:6", "CLASS:Wizard\tSPELLTYPE:Arcane", "1\tABILITY:Class|AUTOMATIC|Wizard"],
        )]);
        assert_eq!(tree.class_rows.get("WIZARD").map(Vec::len), Some(2), "two header rows, both recorded");
        let wizard_lines = tree.level_lines.get("WIZARD").unwrap();
        assert_eq!(wizard_lines.len(), 1);
    }

    /// A `.COPY=`-shaped class header (`CLASS:X.COPY=Y`) still counts as a header: the RAW-text
    /// check runs before `row_identity` strips the `CLASS:` prefix.
    #[test]
    fn a_copy_shaped_class_header_still_reanchors_current_class() {
        let tree = tree_from_lines(vec![("cr_classes.lst", vec!["CLASS:Ranger (Skirmisher).COPY=Ranger\tHD:8", "1\tABILITY:Class|AUTOMATIC|Ranger (Skirmisher)"])]);
        let lines = tree.level_lines.get("RANGER (SKIRMISHER)").expect("a .COPY= class header still re-anchors current_class");
        assert_eq!(lines.len(), 1);
    }

    /// §3.1 item 1 -- the child->parent `ABILITYCATEGORY` map, built from the tree's own rows
    /// only (never a hardcoded list).
    #[test]
    fn ability_category_parent_maps_a_child_to_its_declared_parent() {
        let tree = tree_from_lines(vec![("cr_abilitycategories.lst", vec!["ABILITYCATEGORY:Wizard Class Feature\tCATEGORY:Special Ability"])]);
        assert_eq!(tree.ability_category_parent.get("WIZARD CLASS FEATURE"), Some(&"SPECIAL ABILITY".to_string()));
    }

    /// A category that declares itself under its own `CATEGORY:` token (e.g. `Special Ability`)
    /// is not a child of anything and must not appear as one.
    #[test]
    fn ability_category_parent_excludes_a_self_referential_category() {
        let tree = tree_from_lines(vec![("cr_abilitycategories.lst", vec!["ABILITYCATEGORY:Special Ability\tCATEGORY:Special Ability"])]);
        assert!(!tree.ability_category_parent.contains_key("SPECIAL ABILITY"));
    }

    /// A child name declared under two DIFFERENT parents anywhere in the tree is ambiguous and
    /// must be left out of the map entirely -- the resolver must never guess between them.
    #[test]
    fn ability_category_parent_excludes_a_name_declared_under_two_different_parents() {
        let tree = tree_from_lines(vec![("cr_abilitycategories.lst", vec!["ABILITYCATEGORY:Weird\tCATEGORY:Special Ability", "ABILITYCATEGORY:Weird\tCATEGORY:Feat"])]);
        assert!(!tree.ability_category_parent.contains_key("WEIRD"), "two different declared parents for the same child name: ambiguous, must not guess");
    }

    /// The same child name declared with the SAME parent more than once (a real, harmless
    /// occurrence in the pinned oracle) is not ambiguous.
    #[test]
    fn ability_category_parent_tolerates_a_repeated_identical_declaration() {
        let tree = tree_from_lines(vec![("cr_abilitycategories.lst", vec!["ABILITYCATEGORY:Wizard Class Feature\tCATEGORY:Special Ability", "ABILITYCATEGORY:Wizard Class Feature\tCATEGORY:Special Ability"])]);
        assert_eq!(tree.ability_category_parent.get("WIZARD CLASS FEATURE"), Some(&"SPECIAL ABILITY".to_string()));
    }

    /// SD-36 Epic F1 merge-readiness blocker 2 -- `BONUS:VAR|<target>|...` names ONE OR MORE
    /// comma-separated target variables (real corpus shape: `Bloodrager ~ Bloodrage`'s own
    /// `BONUS:VAR|BloodrageStrBonus,BloodrageConBonus|4`, and its `Greater`/`Mighty Bloodrage`
    /// siblings' `...|2` continuations). `variable_rows` must find this ONE row under BOTH real
    /// target names, not only under the literal, unsplit `"X,Y"` string -- otherwise
    /// `resolve_variable`'s same-record-only check sees just the local `DEFINE` row for each name
    /// and silently folds a genuinely multi-record, level-scaling variable to a flat constant
    /// (diagnosed: Str froze at the base value while Con/Will, saved only by an unrelated
    /// external single-var reference elsewhere in the corpus, correctly stayed dynamic).
    #[test]
    fn variable_rows_finds_a_comma_separated_bonus_var_target_under_each_of_its_own_names() {
        let tree = tree_from_lines(vec![(
            "acg_abilities_class.lst",
            vec![
                "Bloodrage\tKEY:Bloodrager ~ Bloodrage\tCATEGORY:Special Ability\tDEFINE:BloodrageStrBonus|0\tDEFINE:BloodrageConBonus|0\tBONUS:VAR|BloodrageStrBonus,BloodrageConBonus|4",
                "Greater Bloodrage\tKEY:Bloodrager ~ Greater Bloodrage\tCATEGORY:Special Ability\tBONUS:VAR|BloodrageStrBonus,BloodrageConBonus|2",
                "Mighty Bloodrage\tKEY:Bloodrager ~ Mighty Bloodrage\tCATEGORY:Special Ability\tBONUS:VAR|BloodrageStrBonus,BloodrageConBonus|2",
            ],
        )]);
        let str_rows = tree.variable_rows("BloodrageStrBonus");
        let con_rows = tree.variable_rows("BloodrageConBonus");
        // The base row (DEFINE + BONUS:VAR, deduped to one) plus the Greater and Mighty
        // BONUS:VAR rows -- 3 distinct rows, for EACH name.
        assert_eq!(str_rows.len(), 3, "BloodrageStrBonus must be found on the base, Greater AND Mighty rows, not just the base row");
        assert_eq!(con_rows.len(), 3, "BloodrageConBonus must be found on the base, Greater AND Mighty rows, not just the base row");
        // Str and Con must see the SAME row set (both target names sit on every one of these
        // rows) -- the whole point of the bug being closed.
        assert_eq!(str_rows, con_rows);
    }

    /// SD-36 Epic F1 merge-readiness fallout: a `CATEGORY:Aligned Class` row (real PCGen shape,
    /// `isg_abilities.lst`: Inner Sea Gods' Evangelist prestige class re-exports EVERY base
    /// class's own level var under a mirror record of the same name, e.g.
    /// `BONUS:VAR|Vigilante_CFP_Level,VigilanteLVL|EvangelistLVL-1`) must NOT count toward
    /// `variable_rows`'s corpus-wide index -- it is a DIFFERENT class's (Evangelist's) own
    /// progression view, never part of the base class's own record family, and counting it
    /// defeats the in-record fold `class_chassis_sheet_rules.rs`'s empty-package `row_at` reader
    /// depends on for the base class's OWN chassis (a bare `<Class>LVL` var falling back to a
    /// cross-record `Var` there reads as an unconditional 0, not the class's real progression).
    #[test]
    fn variable_rows_excludes_an_aligned_class_minus_one_mirror_row() {
        let tree = tree_from_lines(vec![
            (
                "ui_classes.lst",
                vec!["CLASS:Vigilante\tHD:8\tDEFINE:VigilanteLVL|0\tBONUS:COMBAT|BASEAB|VigilanteLVL*3/4|TYPE=Base.REPLACE"],
            ),
            (
                "isg_abilities.lst",
                vec!["Vigilante\tCATEGORY:Aligned Class\tTYPE:Aligned Class\tBONUS:VAR|Vigilante_CFP_Level,VigilanteLVL|EvangelistLVL-1"],
            ),
        ]);
        let rows = tree.variable_rows("VigilanteLVL");
        assert_eq!(rows.len(), 1, "only the base class's own DEFINE row -- the Aligned Class mirror row is excluded: {rows:?}");
    }

    /// SD-36 Epic F1 merge-readiness blocker 1c correction: the exclusion above must not reach
    /// past the row's OWN file family. Inner Sea Gods' `Winter Witch` `CATEGORY:Aligned Class`
    /// row (real corpus shape, `abilities_rowpg.lst:24`) is the record that OWNS
    /// `WinterWitchLVL` -- no `CLASS:Winter Witch` row exists ANYWHERE in the pinned tree (the
    /// Reign of Winter Player's Guide that carries one is an Adventure Path book, outside both
    /// `BOOKS_RELATIVE` and `EXTRA_BOOK_DIRS`) -- so this row must stay indexed: excluding it
    /// turns `WinterWitchLVL` into a name DEFINEd nowhere in the tree, which `resolve_variable`
    /// (`ctx.rs`) reads as C1(c) (undefined -> `Const(0)` + an `undefined-variables` defect),
    /// flattening `inner_sea_gods:ability:winter_witch#bonus1`'s caster-level bonus from
    /// `EvangelistLVL - 1 - 2` to a flat, never-scaling `-2`.
    #[test]
    fn variable_rows_keeps_an_aligned_class_row_that_owns_its_own_target_var() {
        let tree = tree_from_lines(vec![(
            "abilities_rowpg.lst",
            vec![concat!(
                "Winter Witch\tCATEGORY:Aligned Class\tTYPE:Aligned Class\t",
                "PREABILITY:1,CATEGORY=Class,Winter Witch\tBONUS:PCLEVEL|Winter Witch|EvangelistLVL-1\t",
                "BONUS:VAR|WinterWitchLVL|EvangelistLVL-1\tADD:SPELLCASTER|Witch\t",
                "BONUS:CASTERLEVEL|Witch|WinterWitchLVL-2\tBONUS:PCLEVEL|Witch|var(\"WinterWitchLVL\")-2",
            )],
        )]);
        let rows = tree.variable_rows("WinterWitchLVL");
        assert_eq!(
            rows.len(),
            1,
            "no CLASS:Winter Witch row exists anywhere in the pinned tree -- this Aligned Class row is the ONLY declarer, so it must stay indexed: {rows:?}"
        );
    }
}
