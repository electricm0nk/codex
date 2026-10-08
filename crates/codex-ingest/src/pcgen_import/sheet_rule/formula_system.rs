//! The formula-system reader (SD-37 E3.2): PCGen's newer variable engine as the Starfinder
//! data uses it -- `MODIFY:` / `MODIFYOTHER:` tokens, the `VARIABLE:` files' `GLOBAL:` /
//! `LOCAL:` / `CHANNEL:` declarations, the `DATACONTROL:` files' `FUNCTION:` and
//! `DYNAMICSCOPE:` rows, the `DYNAMIC:` objects and the `DATATABLE:` tables.
//!
//! This is a different engine from the one [`super::formula`] reads (`BONUS:`/`DEFINE:`, the JEP
//! grammar): Pathfinder carries 35 `MODIFY*` tokens, Starfinder 1,954
//! (`docs/release/SD-37-starfinder-1e/content-unit-inventory.md` F-9).
//!
//! **What "read" means here.** Each token is split the way `plugin/lsttokens/ModifyLst.java`
//! and `ModifyOtherLst.java` split it (`|`, with `(...)`/`[...]` grouping), then checked the way
//! PCGen's loader checks it, citing the pinned oracle
//! (`scripts/pcgen-oracle-pin.env` `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`):
//!
//! - the variable is declared, and legal in the scope of the object that carries the token
//!   (`VariableLibrary::isLegalVariableID`; scopes from each object's `getLocalScopeName()`:
//!   `Equipment` -> `PC.EQUIPMENT`, `EquipmentModifier` -> `PC.EQUIPMENT.PART`, `PCStat` ->
//!   `PC.STAT`, `Race` -> `PC.RACE`, `SizeAdjustment` -> `PC.SIZE`, `Skill` -> `PC.SKILL`,
//!   `PCCheck` -> `PC.SAVE`, everything else global `PC`; a child scope draws from its parent);
//! - variable names match case-insensitively (`PCGen-Formula` `VariableManager.variableDefs` is
//!   a `CaseInsensitiveMap`; so is `SimpleFunctionLibrary.functionMap`);
//! - a `LOCAL:`/`GLOBAL:` with no `FORMAT=` is `NUMBER` (`plugin/lsttokens/variable/LocalToken.java`:
//!   "Defaults to NUMBER");
//! - the modifier is one the variable's format has (`plugin/modifier/<format>/*ModifierFactory`:
//!   NUMBER has ADD DIVIDE MAX MIN MULTIPLY SET; BOOLEAN, ORDEREDPAIR, STRING, DICE have SET);
//! - `MODIFYOTHER:` names a non-global scope and a grouping that exists in it (a `DYNAMIC:`
//!   object of that category for a `DYNAMICSCOPE:` scope);
//! - the value parses as a formula whose identifiers are declared variables legal in the scope,
//!   whose functions exist (the PCGen-Formula built-ins, the PCGen plugin functions, and the
//!   data's own `FUNCTION:` rows), whose `input("x")` names a declared `CHANNEL:` and whose
//!   `lookup("t", …)` names a declared `DATATABLE`;
//! - the variable has a sheet role in [`VARIABLE_ROLES`] (compute vs print, the paper-sheet rule).
//!
//! A token that passes is [`Disposition::Mapped`]; any other is [`Disposition::Refused`] with the
//! failed check named. **Nothing is evaluated here** and no rule file is written: what each
//! mapped role computes, and whether that is right for the game, is the SF mapping table's
//! (E3.3) and the SF chassis's (E4).
//!
//! The census ([`census_json`], written by `bin/sf_formula_census.rs`) is what
//! `scripts/token_coverage.py --sf-formula` sums. It carries file paths, lines, the token's
//! parsed parts and its disposition -- never a record name.

use std::collections::BTreeMap;
use std::path::Path;

use codex::rules_core::game_system::GameSystem;
use serde::Serialize;
use serde_json::{Value, json};

use crate::pcgen_import::system_books::resolve_book_includes;

/// `MODIFY:` (on the carrying object's own scope) or `MODIFYOTHER:` (on another scope's objects).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ModifyKind {
    Modify,
    ModifyOther,
}

/// The paper-sheet rule (`decisions.md §5`): a value that feeds a sheet total is computed;
/// everything else is printed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SheetUse {
    Compute,
    Print,
}

/// A parsed formula-system value.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum FsExpr {
    /// A numeric literal, kept as written (`-0`, `2.5`).
    Num(String),
    Bool(bool),
    Str(String),
    /// A declared variable, by its declared spelling.
    Var(String),
    /// An ORDEREDPAIR literal `x,y`.
    Pair(String, String),
    Neg(Box<FsExpr>),
    Not(Box<FsExpr>),
    Bin { op: String, lhs: Box<FsExpr>, rhs: Box<FsExpr> },
    Call { name: String, args: Vec<FsExpr> },
}

/// Mapped (every check passed) or refused (the failed check, named).
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "disposition", rename_all = "lowercase")]
pub enum Disposition {
    Mapped {
        /// The declared variable, by its declared spelling.
        variable: String,
        format: String,
        role: String,
        sheet: SheetUse,
        value: FsExpr,
    },
    Refused {
        reason: String,
    },
}

/// One `MODIFY:`/`MODIFYOTHER:` tab field.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ModifyToken {
    /// `.lst` path relative to the corpus root.
    pub file: String,
    /// One-based line.
    pub line: usize,
    /// One-based tab field.
    pub field: usize,
    pub kind: ModifyKind,
    /// The `.pcc` directive kind of the file (`EQUIPMENT`, `STAT` …).
    pub record_kind: String,
    /// The scope the token is analysed in: the carrying object's, or `MODIFYOTHER`'s first arg.
    pub scope: String,
    /// `MODIFYOTHER`'s grouping (`Walk`).
    pub grouping: Option<String>,
    /// The variable as written.
    pub var: String,
    pub op: String,
    /// The value text as written.
    pub value: String,
    pub priority: Option<i64>,
    /// Flattened into the token's JSON (`"disposition": "mapped"|"refused"` + its fields).
    #[serde(flatten)]
    pub disposition: Disposition,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VariableDecl {
    pub name: String,
    /// `PC` for `GLOBAL:`, else the `LOCAL:` scope.
    pub scope: String,
    pub format: String,
    pub file: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChannelDecl {
    pub name: String,
    pub scope: String,
    pub format: String,
    pub file: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FunctionDecl {
    pub name: String,
    /// The `VALUE:` formula as written.
    pub value: String,
    pub file: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DataTable {
    pub name: String,
    pub columns: Vec<String>,
    pub formats: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub file: String,
    pub line: usize,
}

/// Everything the `VARIABLE:`, `DATACONTROL:`, `DYNAMIC:` and `DATATABLE:` files declare.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Declarations {
    pub variables: Vec<VariableDecl>,
    pub channels: Vec<ChannelDecl>,
    pub functions: Vec<FunctionDecl>,
    /// `DYNAMICSCOPE:` categories (`MOVEMENT`), each a scope `PC.<category>`.
    pub dynamic_scopes: Vec<String>,
    /// `DYNAMIC:` objects as (category, name).
    pub dynamic_objects: Vec<(String, String)>,
    pub datatables: Vec<DataTable>,
}

/// The reader's output for one system.
#[derive(Debug, Clone, PartialEq)]
pub struct FormulaSystemReading {
    pub system: GameSystem,
    /// Every `.lst` read, relative to the corpus root, in registry order.
    pub files_read: Vec<String>,
    pub declarations: Declarations,
    pub tokens: Vec<ModifyToken>,
}

/// The sheet role of each formula-system variable the Starfinder data modifies, keyed by the
/// declared name (matched case-insensitively). A variable with no row refuses by name.
///
/// Compute rows are the ability chain (`scr__stats.lst`: `Score` = the stat channel's input,
/// `Mod` = `d20Mod(Score)`, `STR`…`CHA` / `STRSCORE`…`CHASCORE` its global aliases) and movement
/// speed (a sheet number that armour adjusts). Every other row is a printed fact of an item or
/// a race.
pub const VARIABLE_ROLES: &[(&str, &str, SheetUse)] = &[
    ("Score", "ability_score", SheetUse::Compute),
    ("Mod", "ability_modifier", SheetUse::Compute),
    ("STR", "ability_modifier_alias", SheetUse::Compute),
    ("DEX", "ability_modifier_alias", SheetUse::Compute),
    ("CON", "ability_modifier_alias", SheetUse::Compute),
    ("INT", "ability_modifier_alias", SheetUse::Compute),
    ("WIS", "ability_modifier_alias", SheetUse::Compute),
    ("CHA", "ability_modifier_alias", SheetUse::Compute),
    ("STRSCORE", "ability_score_alias", SheetUse::Compute),
    ("DEXSCORE", "ability_score_alias", SheetUse::Compute),
    ("CONSCORE", "ability_score_alias", SheetUse::Compute),
    ("INTSCORE", "ability_score_alias", SheetUse::Compute),
    ("WISSCORE", "ability_score_alias", SheetUse::Compute),
    ("CHASCORE", "ability_score_alias", SheetUse::Compute),
    ("Speed", "movement_speed", SheetUse::Compute),
    ("ItemLevel", "item_level", SheetUse::Print),
    ("UpgradeSlot", "armor_upgrade_slots", SheetUse::Print),
    ("UpgradeSlotTaken", "upgrade_slots_used", SheetUse::Print),
    ("WeaponSlot", "armor_weapon_slots", SheetUse::Print),
    ("AeonUpgradeSlots", "aeon_upgrade_slots", SheetUse::Print),
    ("AndroidUpgradeSlotTaken", "android_upgrade_slots_used", SheetUse::Print),
    ("Face", "space", SheetUse::Print),
    ("Race_Reach", "reach", SheetUse::Print),
    ("Race_Hands", "hands", SheetUse::Print),
    ("Race_Legs", "legs", SheetUse::Print),
    ("RaceType_Humanoid", "humanoid_body_plan", SheetUse::Print),
    // SD-37 E3.4: a weapon's `PART:1|MODIFY:Damage|SET|1d6` (scope `PC.EQUIPMENT.PART`, format
    // DICE). Its tab field starts `PART:`, not `MODIFY:`, so E3.2's census predicate never
    // reached it and this table had no row; the converter prints the dice literally.
    ("Damage", "weapon_damage", SheetUse::Print),
];

/// PCGen-Formula built-ins (`pcgen/base/formula/function/*Function.java`) and the PCGen plugin
/// functions (`plugin/function/*Function.java`), lower-cased.
const KNOWN_FUNCTIONS: &[&str] = &[
    "abs", "ceil", "floor", "if", "max", "min", "round", "value", "isempty", "ispresent", "length", "slice",
    "getoptional", "get", "getfact", "getother", "group", "input", "key", "listall", "lookup",
];

/// Modifier identifications per variable format (`plugin/modifier/<format>/`).
fn legal_ops(format: &str) -> &'static [&'static str] {
    match format {
        "NUMBER" => &["ADD", "DIVIDE", "MAX", "MIN", "MULTIPLY", "SET"],
        "BOOLEAN" | "ORDEREDPAIR" | "STRING" | "DICE" => &["SET"],
        _ => &[],
    }
}

/// The scope a `.pcc` directive kind's objects analyse their own `MODIFY:` in.
pub fn scope_of_record_kind(kind: &str) -> &'static str {
    match kind {
        "EQUIPMENT" => "PC.EQUIPMENT",
        "EQUIPMOD" => "PC.EQUIPMENT.PART",
        "STAT" => "PC.STAT",
        "RACE" => "PC.RACE",
        "SIZE" => "PC.SIZE",
        "SKILL" => "PC.SKILL",
        "SAVE" => "PC.SAVE",
        _ => "PC",
    }
}

/// A variable declared at `decl_scope` is legal in `scope` when the scope is it or draws from it.
fn scope_reaches(scope: &str, decl_scope: &str) -> bool {
    decl_scope == "PC" || scope == decl_scope || scope.starts_with(&format!("{decl_scope}."))
}

/// Read the formula system of every registered book of `system` from `corpus_root`.
pub fn read_formula_system(system: GameSystem, corpus_root: &Path) -> Result<FormulaSystemReading, String> {
    let includes = resolve_book_includes(system, corpus_root)?;
    let mut files: Vec<(String, String, String)> = Vec::new(); // (rel, kind, text)
    for lst in &includes.lst_files {
        let rel = lst
            .path
            .strip_prefix(corpus_root)
            .map_err(|_| format!("{} is outside the corpus root", lst.path.display()))?
            .to_string_lossy()
            .replace('\\', "/");
        let bytes = std::fs::read(&lst.path).map_err(|e| format!("{}: {e}", lst.path.display()))?;
        files.push((rel, lst.kind.clone(), String::from_utf8_lossy(&bytes).into_owned()));
    }
    let mut declarations = Declarations::default();
    for (rel, kind, text) in &files {
        match kind.as_str() {
            "VARIABLE" => read_variables(rel, text, &mut declarations)?,
            "DATACONTROL" => read_datacontrols(rel, text, &mut declarations),
            "DYNAMIC" => read_dynamic(rel, text, &mut declarations),
            "DATATABLE" => read_datatables(rel, text, &mut declarations)?,
            _ => {}
        }
    }
    let mut tokens = Vec::new();
    for (rel, kind, text) in &files {
        for (i, raw) in text.lines().enumerate() {
            let line = raw.strip_suffix('\r').unwrap_or(raw);
            if line.starts_with('#') {
                continue;
            }
            for (j, field) in line.split('\t').enumerate() {
                let parsed = if let Some(body) = field.strip_prefix("MODIFY:") {
                    Some((ModifyKind::Modify, body))
                } else {
                    field.strip_prefix("MODIFYOTHER:").map(|body| (ModifyKind::ModifyOther, body))
                };
                if let Some((mk, body)) = parsed {
                    tokens.push(read_token(&declarations, rel, i + 1, j + 1, mk, kind, body));
                }
            }
        }
    }
    Ok(FormulaSystemReading { system, files_read: files.into_iter().map(|f| f.0).collect(), declarations, tokens })
}

fn data_lines(text: &str) -> impl Iterator<Item = (usize, &str)> {
    text.lines().enumerate().filter_map(|(i, raw)| {
        let line = raw.strip_suffix('\r').unwrap_or(raw);
        let t = line.trim();
        (!t.is_empty() && !t.starts_with('#')).then_some((i + 1, line))
    })
}

/// `GLOBAL:[FORMAT=]Name`, `LOCAL:Scope|[FORMAT=]Name`, `CHANNEL:Scope|FORMAT=Name`
/// (`plugin/lsttokens/variable/{Global,Local,Channel}Token.java`).
fn read_variables(file: &str, text: &str, d: &mut Declarations) -> Result<(), String> {
    for (line, row) in data_lines(text) {
        let first = row.split('\t').next().unwrap_or("").trim();
        let split_format = |s: &str| -> (String, String) {
            match s.split_once('=') {
                Some((f, n)) => (f.trim().to_string(), n.trim().to_string()),
                None => ("NUMBER".to_string(), s.trim().to_string()),
            }
        };
        if let Some(rest) = first.strip_prefix("GLOBAL:") {
            let (format, name) = split_format(rest);
            d.variables.push(VariableDecl { name, scope: "PC".into(), format, file: file.into(), line });
        } else if let Some(rest) = first.strip_prefix("LOCAL:") {
            let (scope, fv) = rest.split_once('|').ok_or_else(|| format!("{file}:{line}: LOCAL without a scope"))?;
            let (format, name) = split_format(fv);
            d.variables.push(VariableDecl { name, scope: scope.trim().into(), format, file: file.into(), line });
        } else if let Some(rest) = first.strip_prefix("CHANNEL:") {
            let (scope, fv) = rest.split_once('|').ok_or_else(|| format!("{file}:{line}: CHANNEL without a scope"))?;
            let (format, name) = split_format(fv);
            d.channels.push(ChannelDecl { name, scope: scope.trim().into(), format, file: file.into(), line });
        } else {
            return Err(format!("{file}:{line}: unknown VARIABLE row {first:?}"));
        }
    }
    Ok(())
}

/// `FUNCTION:<name> … VALUE:<formula>` and `DYNAMICSCOPE:<category>`; the other data-control
/// rows (`FACTDEF:`, `DEFAULTVARIABLEVALUE:` …) are not formula-system vocabulary.
fn read_datacontrols(file: &str, text: &str, d: &mut Declarations) {
    for (line, row) in data_lines(text) {
        let mut fields = row.split('\t').map(str::trim).filter(|f| !f.is_empty());
        let first = fields.next().unwrap_or("");
        if let Some(name) = first.strip_prefix("FUNCTION:") {
            let value = fields.find_map(|f| f.strip_prefix("VALUE:")).unwrap_or("").to_string();
            d.functions.push(FunctionDecl { name: name.into(), value, file: file.into(), line });
        } else if let Some(cat) = first.strip_prefix("DYNAMICSCOPE:") {
            d.dynamic_scopes.push(cat.into());
        }
    }
}

/// `<CATEGORY>:<name>` rows.
fn read_dynamic(_file: &str, text: &str, d: &mut Declarations) {
    for (_, row) in data_lines(text) {
        let first = row.split('\t').next().unwrap_or("").trim();
        if let Some((cat, name)) = first.split_once(':') {
            d.dynamic_objects.push((cat.trim().into(), name.trim().into()));
        }
    }
}

/// `STARTTABLE:<name>,` / column-name row / format row / data rows / `ENDTABLE:<name>,`.
fn read_datatables(file: &str, text: &str, d: &mut Declarations) -> Result<(), String> {
    let cells = |s: &str| -> Vec<String> {
        let mut v: Vec<String> = s.split(',').map(|c| c.trim().to_string()).collect();
        while v.last().is_some_and(String::is_empty) {
            v.pop();
        }
        v
    };
    let mut open: Option<(DataTable, usize)> = None; // (table, data rows seen incl. header rows)
    for (line, row) in data_lines(text) {
        let row = row.trim();
        if let Some(name) = row.strip_prefix("STARTTABLE:") {
            if let Some((t, _)) = &open {
                return Err(format!("{file}:{line}: STARTTABLE inside table {:?}", t.name));
            }
            let name = name.trim_end_matches(',').trim().to_string();
            open = Some((
                DataTable { name, columns: vec![], formats: vec![], rows: vec![], file: file.into(), line },
                0,
            ));
        } else if let Some(name) = row.strip_prefix("ENDTABLE:") {
            let name = name.trim_end_matches(',').trim();
            let (t, _) = open.take().ok_or_else(|| format!("{file}:{line}: ENDTABLE with no open table"))?;
            if t.name != name {
                return Err(format!("{file}:{line}: ENDTABLE {name:?} closes table {:?}", t.name));
            }
            d.datatables.push(t);
        } else if let Some((t, n)) = open.as_mut() {
            let c = cells(row);
            match *n {
                0 => t.columns = c,
                1 => {
                    if c.len() != t.columns.len() {
                        return Err(format!("{file}:{line}: table {:?} has {} formats for {} columns", t.name, c.len(), t.columns.len()));
                    }
                    t.formats = c;
                }
                _ => {
                    if c.len() != t.columns.len() {
                        return Err(format!("{file}:{line}: table {:?} row has {} cells for {} columns", t.name, c.len(), t.columns.len()));
                    }
                    for (cell, fmt) in c.iter().zip(&t.formats) {
                        if fmt == "NUMBER" && cell.parse::<f64>().is_err() {
                            return Err(format!("{file}:{line}: table {:?} NUMBER cell {cell:?}", t.name));
                        }
                    }
                    t.rows.push(c);
                }
            }
            *n += 1;
        } else {
            return Err(format!("{file}:{line}: DATATABLE row outside a table: {row:?}"));
        }
    }
    if let Some((t, _)) = open {
        return Err(format!("{file}: table {:?} is never closed", t.name));
    }
    Ok(())
}

/// Split on `sep` outside `(...)`, `[...]` and `"..."` (`ParsingSeparator` with grouping pairs).
fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    let mut out = Vec::new();
    let (mut depth, mut in_str, mut start) = (0i32, false, 0usize);
    for (i, c) in s.char_indices() {
        match c {
            '"' => in_str = !in_str,
            '(' | '[' if !in_str => depth += 1,
            ')' | ']' if !in_str => depth -= 1,
            c if c == sep && depth == 0 && !in_str => {
                out.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}

fn read_token(
    d: &Declarations,
    file: &str,
    line: usize,
    field: usize,
    kind: ModifyKind,
    record_kind: &str,
    body: &str,
) -> ModifyToken {
    let parts = split_top_level(body, '|');
    let mut token = ModifyToken {
        file: file.into(),
        line,
        field,
        kind,
        record_kind: record_kind.into(),
        scope: scope_of_record_kind(record_kind).into(),
        grouping: None,
        var: String::new(),
        op: String::new(),
        value: String::new(),
        priority: None,
        disposition: Disposition::Refused { reason: String::new() },
    };
    let rest: &[&str] = match kind {
        ModifyKind::Modify => &parts,
        ModifyKind::ModifyOther => {
            if parts.len() < 2 {
                token.disposition = refuse("MODIFYOTHER needs a scope and a grouping");
                return token;
            }
            token.scope = parts[0].to_string();
            token.grouping = Some(parts[1].to_string());
            &parts[2..]
        }
    };
    if rest.len() < 3 {
        token.var = rest.first().map(|s| s.to_string()).unwrap_or_default();
        token.disposition = refuse(&format!("{} needs a variable, a modifier and a value", kind_name(kind)));
        return token;
    }
    token.var = rest[0].to_string();
    token.op = rest[1].to_string();
    token.value = rest[2].to_string();
    for assoc in &rest[3..] {
        match assoc.strip_prefix("PRIORITY=").map(str::parse::<i64>) {
            Some(Ok(p)) => token.priority = Some(p),
            _ => {
                token.disposition = refuse(&format!("unread association {assoc:?}"));
                return token;
            }
        }
    }
    token.disposition = match map_token(d, &token) {
        Ok(m) => m,
        Err(reason) => refuse(&reason),
    };
    token
}

fn kind_name(kind: ModifyKind) -> &'static str {
    match kind {
        ModifyKind::Modify => "MODIFY",
        ModifyKind::ModifyOther => "MODIFYOTHER",
    }
}

fn refuse(reason: &str) -> Disposition {
    Disposition::Refused { reason: reason.into() }
}

fn find_variable<'a>(d: &'a Declarations, scope: &str, name: &str) -> Result<&'a VariableDecl, String> {
    let named: Vec<&VariableDecl> = d.variables.iter().filter(|v| v.name.eq_ignore_ascii_case(name)).collect();
    if named.is_empty() {
        return Err(format!("undeclared variable {name:?}"));
    }
    named
        .into_iter()
        .find(|v| scope_reaches(scope, &v.scope))
        .ok_or_else(|| format!("variable {name:?} is not legal in scope {scope}"))
}

fn map_token(d: &Declarations, t: &ModifyToken) -> Result<Disposition, String> {
    if t.kind == ModifyKind::ModifyOther {
        let cat = t
            .scope
            .strip_prefix("PC.")
            .ok_or_else(|| format!("MODIFYOTHER scope {:?} is not a PC scope", t.scope))?;
        if !d.dynamic_scopes.iter().any(|s| s == cat) {
            return Err(format!("MODIFYOTHER scope {:?} is not a declared dynamic scope", t.scope));
        }
        let grouping = t.grouping.as_deref().unwrap_or("");
        if !d.dynamic_objects.iter().any(|(c, n)| c == cat && n == grouping) {
            return Err(format!("MODIFYOTHER grouping {grouping:?} is no {cat} object"));
        }
    }
    let decl = find_variable(d, &t.scope, &t.var)?;
    if !legal_ops(&decl.format).contains(&t.op.as_str()) {
        return Err(format!("modifier {:?} is not defined for {} variable {:?}", t.op, decl.format, decl.name));
    }
    let value = match decl.format.as_str() {
        "ORDEREDPAIR" => {
            let (x, y) = t.value.split_once(',').ok_or_else(|| format!("ORDEREDPAIR value {:?} is not x,y", t.value))?;
            let (x, y) = (x.trim(), y.trim());
            if x.parse::<f64>().is_err() || y.parse::<f64>().is_err() {
                return Err(format!("ORDEREDPAIR value {:?} is not a literal pair", t.value));
            }
            FsExpr::Pair(x.into(), y.into())
        }
        "NUMBER" | "BOOLEAN" => parse_formula(d, &t.scope, &t.value)?,
        other => return Err(format!("{other} values are not read")),
    };
    let (_, role, sheet) = VARIABLE_ROLES
        .iter()
        .find(|(n, _, _)| n.eq_ignore_ascii_case(&decl.name))
        .ok_or_else(|| format!("variable {:?} has no sheet role", decl.name))?;
    Ok(Disposition::Mapped {
        variable: decl.name.clone(),
        format: decl.format.clone(),
        role: (*role).into(),
        sheet: *sheet,
        value,
    })
}

// --- the formula grammar -------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Num(String),
    Ident(String),
    Str(String),
    Op(&'static str),
    LParen,
    RParen,
    Comma,
}

fn tokenize(s: &str) -> Result<Vec<Tok>, String> {
    let chars: Vec<char> = s.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let two: String = chars[i..chars.len().min(i + 2)].iter().collect();
        if c.is_whitespace() {
            i += 1;
        } else if c.is_ascii_digit() || (c == '.' && chars.get(i + 1).is_some_and(char::is_ascii_digit)) {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                i += 1;
            }
            let n: String = chars[start..i].iter().collect();
            n.parse::<f64>().map_err(|_| format!("malformed number {n:?}"))?;
            out.push(Tok::Num(n));
        } else if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_' || chars[i] == '.') {
                i += 1;
            }
            out.push(Tok::Ident(chars[start..i].iter().collect()));
        } else if c == '"' {
            let start = i + 1;
            i += 1;
            while i < chars.len() && chars[i] != '"' {
                i += 1;
            }
            if i == chars.len() {
                return Err("unterminated string".into());
            }
            out.push(Tok::Str(chars[start..i].iter().collect()));
            i += 1;
        } else if let Some(op) = ["<=", ">=", "==", "!=", "&&", "||"].into_iter().find(|o| *o == two) {
            out.push(Tok::Op(op));
            i += 2;
        } else {
            out.push(match c {
                '+' => Tok::Op("+"),
                '-' => Tok::Op("-"),
                '*' => Tok::Op("*"),
                '/' => Tok::Op("/"),
                '%' => Tok::Op("%"),
                '^' => Tok::Op("^"),
                '<' => Tok::Op("<"),
                '>' => Tok::Op(">"),
                '!' => Tok::Op("!"),
                '(' => Tok::LParen,
                ')' => Tok::RParen,
                ',' => Tok::Comma,
                other => return Err(format!("unexpected character {other:?}")),
            });
            i += 1;
        }
    }
    Ok(out)
}

struct Parser<'a> {
    toks: Vec<Tok>,
    pos: usize,
    d: &'a Declarations,
    scope: &'a str,
}

const BINARY_LEVELS: &[&[&str]] = &[&["||"], &["&&"], &["==", "!="], &["<", "<=", ">", ">="], &["+", "-"], &["*", "/", "%"], &["^"]];

impl Parser<'_> {
    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.pos)
    }

    fn binary(&mut self, level: usize) -> Result<FsExpr, String> {
        if level == BINARY_LEVELS.len() {
            return self.unary();
        }
        let mut lhs = self.binary(level + 1)?;
        while let Some(Tok::Op(op)) = self.peek() {
            let op = *op;
            if !BINARY_LEVELS[level].contains(&op) {
                break;
            }
            self.pos += 1;
            let rhs = self.binary(level + 1)?;
            lhs = FsExpr::Bin { op: op.into(), lhs: Box::new(lhs), rhs: Box::new(rhs) };
        }
        Ok(lhs)
    }

    fn unary(&mut self) -> Result<FsExpr, String> {
        match self.peek() {
            Some(Tok::Op("-")) => {
                self.pos += 1;
                Ok(FsExpr::Neg(Box::new(self.unary()?)))
            }
            Some(Tok::Op("!")) => {
                self.pos += 1;
                Ok(FsExpr::Not(Box::new(self.unary()?)))
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Result<FsExpr, String> {
        let tok = self.peek().cloned().ok_or("formula ends early")?;
        self.pos += 1;
        match tok {
            Tok::Num(n) => Ok(FsExpr::Num(n)),
            Tok::Str(s) => Ok(FsExpr::Str(s)),
            Tok::LParen => {
                let e = self.binary(0)?;
                self.expect_rparen()?;
                Ok(e)
            }
            Tok::Ident(name) if self.peek() == Some(&Tok::LParen) => {
                self.pos += 1;
                let mut args = Vec::new();
                if self.peek() != Some(&Tok::RParen) {
                    loop {
                        args.push(self.binary(0)?);
                        if self.peek() == Some(&Tok::Comma) {
                            self.pos += 1;
                        } else {
                            break;
                        }
                    }
                }
                self.expect_rparen()?;
                self.check_call(&name, &args)?;
                Ok(FsExpr::Call { name, args })
            }
            Tok::Ident(name) if name.eq_ignore_ascii_case("true") => Ok(FsExpr::Bool(true)),
            Tok::Ident(name) if name.eq_ignore_ascii_case("false") => Ok(FsExpr::Bool(false)),
            Tok::Ident(name) => Ok(FsExpr::Var(find_variable(self.d, self.scope, &name)?.name.clone())),
            other => Err(format!("unexpected {other:?}")),
        }
    }

    fn expect_rparen(&mut self) -> Result<(), String> {
        if self.peek() == Some(&Tok::RParen) {
            self.pos += 1;
            Ok(())
        } else {
            Err("missing )".into())
        }
    }

    fn check_call(&self, name: &str, args: &[FsExpr]) -> Result<(), String> {
        let lower = name.to_ascii_lowercase();
        let data_fn = self.d.functions.iter().any(|f| f.name.eq_ignore_ascii_case(name));
        if !data_fn && !KNOWN_FUNCTIONS.contains(&lower.as_str()) {
            return Err(format!("unknown function {name:?}"));
        }
        let first_str = || match args.first() {
            Some(FsExpr::Str(s)) => Ok(s.as_str()),
            _ => Err(format!("{name}() needs a quoted name as its first argument")),
        };
        match lower.as_str() {
            "if" if args.len() != 3 => Err(format!("if() takes 3 arguments, got {}", args.len())),
            "input" => {
                let ch = first_str()?;
                if self.d.channels.iter().any(|c| c.name.eq_ignore_ascii_case(ch)) {
                    Ok(())
                } else {
                    Err(format!("input({ch:?}) names no declared channel"))
                }
            }
            "lookup" => {
                let table = first_str()?;
                if self.d.datatables.iter().any(|t| t.name == table) {
                    Ok(())
                } else {
                    Err(format!("lookup({table:?}) names no declared datatable"))
                }
            }
            _ => Ok(()),
        }
    }
}

fn parse_formula(d: &Declarations, scope: &str, text: &str) -> Result<FsExpr, String> {
    let toks = tokenize(text).map_err(|e| format!("formula {text:?}: {e}"))?;
    if toks.is_empty() {
        return Err("empty value".into());
    }
    let mut p = Parser { toks, pos: 0, d, scope };
    let e = p.binary(0).map_err(|e| format!("formula {text:?}: {e}"))?;
    if p.pos != p.toks.len() {
        return Err(format!("formula {text:?}: trailing input"));
    }
    Ok(e)
}

// --- the census -----------------------------------------------------------------------------

/// The census `scripts/token_coverage.py --sf-formula` sums: per token its location, parts and
/// disposition, plus the declarations and the totals.
pub fn census_json(r: &FormulaSystemReading) -> Value {
    let mapped = r.tokens.iter().filter(|t| matches!(t.disposition, Disposition::Mapped { .. })).count();
    let mut refused_by_reason = BTreeMap::<String, usize>::new();
    for t in &r.tokens {
        if let Disposition::Refused { reason } = &t.disposition {
            *refused_by_reason.entry(reason.clone()).or_default() += 1;
        }
    }
    let mut by_role = BTreeMap::<String, usize>::new();
    for t in &r.tokens {
        if let Disposition::Mapped { role, .. } = &t.disposition {
            *by_role.entry(role.clone()).or_default() += 1;
        }
    }
    let d = &r.declarations;
    json!({
        "schema": "sf-formula-census.v1",
        "derived_by": "cargo run --locked -p codex-ingest --bin sf_formula_census",
        "system": r.system.id(),
        "predicate": "a tab field that starts with MODIFY: or MODIFYOTHER: on a line that does not start with # in a .lst the registered books include",
        "files_read": r.files_read,
        "totals": {
            "tokens": r.tokens.len(),
            "mapped": mapped,
            "refused": r.tokens.len() - mapped,
            "refused_by_reason": refused_by_reason,
            "mapped_by_role": by_role,
        },
        "declarations": {
            "variables": d.variables,
            "channels": d.channels,
            "functions": d.functions,
            "dynamic_scopes": d.dynamic_scopes,
            "dynamic_objects": d.dynamic_objects,
            "datatables": d.datatables.iter().map(|t| json!({
                "name": t.name, "columns": t.columns, "formats": t.formats, "rows": t.rows.len(),
                "file": t.file, "line": t.line,
            })).collect::<Vec<_>>(),
        },
        "tokens": r.tokens,
    })
}

/// The census as written to disk: two-space-indented sections, one token per line, a trailing
/// newline. Byte-identical for identical input.
pub fn render_census(v: &Value) -> String {
    let mut out = String::from("{\n");
    let obj = v.as_object().expect("census is an object");
    let n = obj.len();
    for (i, (k, val)) in obj.iter().enumerate() {
        out.push_str(&format!("  {}: ", serde_json::to_string(k).unwrap()));
        match (k.as_str(), val) {
            ("tokens" | "files_read", Value::Array(items)) => {
                out.push_str("[\n");
                for (j, item) in items.iter().enumerate() {
                    out.push_str("    ");
                    out.push_str(&serde_json::to_string(item).unwrap());
                    out.push_str(if j + 1 < items.len() { ",\n" } else { "\n" });
                }
                out.push_str("  ]");
            }
            _ => out.push_str(&serde_json::to_string(val).unwrap()),
        }
        out.push_str(if i + 1 < n { ",\n" } else { "\n" });
    }
    out.push_str("}\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decls() -> Declarations {
        let mut d = Declarations::default();
        read_variables(
            "v.lst",
            "CHANNEL:PC.STAT|NUMBER=StatScore\nLOCAL:PC.EQUIPMENT|ItemLevel\nLOCAL:PC.STAT|NUMBER=Score\nLOCAL:PC.STAT|NUMBER=Mod\nLOCAL:PC.MOVEMENT|NUMBER=Speed\nGLOBAL:BOOLEAN=RaceType_Humanoid\nGLOBAL:ORDEREDPAIR=Face\nGLOBAL:NUMBER=Race_Reach\nGLOBAL:NUMBER=Unrolled\n",
            &mut d,
        )
        .unwrap();
        read_datacontrols("c.lst", "FUNCTION:d20Mod\tVALUE:floor((arg(0)-10)/2)\nDYNAMICSCOPE:MOVEMENT\n", &mut d);
        read_dynamic("y.lst", "MOVEMENT:Walk\n", &mut d);
        read_datatables("t.lst", "STARTTABLE:Item Level Cost,\nItemLevel,Price\nNUMBER,NUMBER\n1,120\nENDTABLE:Item Level Cost,\n", &mut d)
            .unwrap();
        d
    }

    fn read(kind: ModifyKind, record_kind: &str, body: &str) -> Disposition {
        read_token(&decls(), "f.lst", 1, 1, kind, record_kind, body).disposition
    }

    fn reason(d: Disposition) -> String {
        match d {
            Disposition::Refused { reason } => reason,
            m => panic!("expected a refusal, got {m:?}"),
        }
    }

    #[test]
    fn mapped_shapes() {
        assert!(matches!(read(ModifyKind::Modify, "EQUIPMENT", "ItemLevel|SET|5"), Disposition::Mapped { .. }));
        assert!(matches!(read(ModifyKind::Modify, "EQUIPMOD", "ItemLevel|ADD|1"), Disposition::Mapped { .. }), "a part draws from its equipment");
        assert!(matches!(read(ModifyKind::Modify, "STAT", "Mod|SET|d20Mod(Score)"), Disposition::Mapped { .. }));
        assert!(matches!(read(ModifyKind::Modify, "STAT", "Score|SET|input(\"STATSCORE\")"), Disposition::Mapped { .. }));
        assert!(matches!(read(ModifyKind::Modify, "RACE", "race_reach|SET|if(RaceType_Humanoid,5,10)|PRIORITY=100"), Disposition::Mapped { .. }));
        assert!(matches!(read(ModifyKind::Modify, "RACE", "Face|SET|2.5,2.5"), Disposition::Mapped { .. }));
        assert!(matches!(read(ModifyKind::Modify, "RACE", "RaceType_Humanoid|SET|True"), Disposition::Mapped { .. }));
        assert!(matches!(read(ModifyKind::ModifyOther, "EQUIPMENT", "PC.MOVEMENT|Walk|Speed|ADD|-5"), Disposition::Mapped { .. }));
    }

    #[test]
    fn every_failed_check_refuses_by_name() {
        let cases: &[(ModifyKind, &str, &str, &str)] = &[
            (ModifyKind::Modify, "EQUIPMENT", "Nope|SET|1", "undeclared variable"),
            (ModifyKind::Modify, "RACE", "ItemLevel|SET|1", "not legal in scope PC.RACE"),
            (ModifyKind::Modify, "RACE", "RaceType_Humanoid|ADD|1", "not defined for BOOLEAN"),
            (ModifyKind::Modify, "STAT", "Mod|SET|nope(Score)", "unknown function"),
            (ModifyKind::Modify, "STAT", "Score|SET|input(\"Other\")", "names no declared channel"),
            (ModifyKind::Modify, "EQUIPMENT", "ItemLevel|SET|lookup(\"Missing\",1,\"Price\")", "names no declared datatable"),
            (ModifyKind::Modify, "EQUIPMENT", "ItemLevel|SET|1|PREVARGT:x,1", "unread association"),
            (ModifyKind::Modify, "EQUIPMENT", "ItemLevel|SET", "needs a variable, a modifier and a value"),
            (ModifyKind::Modify, "EQUIPMENT", "ItemLevel|SET|(1", "missing )"),
            (ModifyKind::Modify, "RACE", "Face|SET|five", "not x,y"),
            (ModifyKind::Modify, "RACE", "Unrolled|SET|1", "has no sheet role"),
            (ModifyKind::ModifyOther, "EQUIPMENT", "PC.VISION|Walk|Speed|ADD|1", "not a declared dynamic scope"),
            (ModifyKind::ModifyOther, "EQUIPMENT", "PC.MOVEMENT|Burrow|Speed|ADD|1", "no MOVEMENT object"),
            (ModifyKind::ModifyOther, "EQUIPMENT", "PC.MOVEMENT", "needs a scope and a grouping"),
        ];
        for (kind, rk, body, want) in cases {
            let got = reason(read(*kind, rk, body));
            assert!(got.contains(want), "{body}: {got:?} lacks {want:?}");
        }
    }

    #[test]
    fn formulas_keep_precedence_and_spelling() {
        let d = decls();
        let e = parse_formula(&d, "PC.STAT", "1+Score*2-0").unwrap();
        let s = serde_json::to_string(&e).unwrap();
        assert_eq!(
            s,
            r#"{"Bin":{"op":"-","lhs":{"Bin":{"op":"+","lhs":{"Num":"1"},"rhs":{"Bin":{"op":"*","lhs":{"Var":"Score"},"rhs":{"Num":"2"}}}}},"rhs":{"Num":"0"}}}"#
        );
        assert_eq!(parse_formula(&d, "PC", "-0").unwrap(), FsExpr::Neg(Box::new(FsExpr::Num("0".into()))));
    }

    #[test]
    fn datatables_reject_ragged_rows() {
        let mut d = Declarations::default();
        let err = read_datatables("t.lst", "STARTTABLE:T,\na,b\nNUMBER,NUMBER\n1\nENDTABLE:T,\n", &mut d).unwrap_err();
        assert!(err.contains("1 cells for 2 columns"), "{err}");
        let err = read_datatables("t.lst", "STARTTABLE:T,\na\nNUMBER\nx\nENDTABLE:T,\n", &mut d).unwrap_err();
        assert!(err.contains("NUMBER cell"), "{err}");
        let err = read_datatables("t.lst", "STARTTABLE:T,\na\nNUMBER\n", &mut d).unwrap_err();
        assert!(err.contains("never closed"), "{err}");
    }

    #[test]
    fn split_respects_grouping() {
        assert_eq!(split_top_level("a|if(x,1|2)|\"p|q\"|[r|s]", '|'), vec!["a", "if(x,1|2)", "\"p|q\"", "[r|s]"]);
    }
}
