//! Conversion context: the corpus-wide indexes every family reads (rule ids by category+key,
//! class/skill/race/template ids, fact declarers, variable ids) and the per-record state
//! (own rows, own variable contributions, the choice the record offers, the owning class,
//! refusals and defects collected while converting).

use std::collections::{BTreeMap, BTreeSet};

use sha2::{Digest, Sha256};

use super::closure::{Closure, PinnedTree, RowRef};
use crate::rules_core::sheet_rule::{Applies, BonusType, ClassId, Expr, RuleId, SkillId, StackMode, VarId};

/// `"Fast Movement"` -> `"fast_movement"` -- `v06_work_inventory::slug`'s rule, reproduced so
/// rule ids here equal the inventory's unit ids.
pub fn slug(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    let mut last_underscore = true;
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
            last_underscore = false;
        } else if !last_underscore {
            out.push('_');
            last_underscore = true;
        }
    }
    while out.ends_with('_') {
        out.pop();
    }
    out
}

/// `VarId` for a source variable name: `"v"` + 16 hex of SHA-256 over the upper-cased name
/// (`SYNTHESIS.md` A2).
pub fn var_id(name: &str) -> VarId {
    let upper = name.trim().to_ascii_uppercase();
    let digest = Sha256::digest(upper.as_bytes());
    let hex: String = digest.iter().take(8).map(|b| format!("{b:02x}")).collect();
    format!("v{hex}")
}

/// One corpus record as the converter sees it: the inventory unit joined to its shipped record.
#[derive(Debug, Clone)]
pub struct RecordRef {
    pub id: RuleId,
    pub book: String,
    pub kind: String,
    pub name: String,
    pub key: String,
    pub category: String,
    pub type_facet: String,
    pub rel_path: String,
    pub line: usize,
    pub shipped_tokens: Option<Vec<(String, String)>>,
    pub prerequisites: Vec<String>,
    pub copy_base_key: Option<String>,
    pub license_pi: bool,
    pub pi_fields: Vec<String>,
    pub description: Option<String>,
    pub class_name: Option<String>,
    /// The inventory unit joined to a shipped corpus record.
    pub joined: bool,
}

/// Corpus-wide lookup tables.
#[derive(Default)]
pub struct CorpusIndex {
    pub records: Vec<RecordRef>,
    /// `(CATEGORY upper, KEY upper)` -> rule id (first wins; ambiguity recorded).
    pub by_cat_key: BTreeMap<(String, String), RuleId>,
    /// `(CATEGORY upper, NAME upper)` -> rule id.
    pub by_cat_name: BTreeMap<(String, String), RuleId>,
    /// `(kind, KEY-or-NAME upper)` -> rule id.
    pub by_kind_name: BTreeMap<(String, String), RuleId>,
    /// class NAME upper -> (rule id, class id).
    pub classes: BTreeMap<String, (RuleId, ClassId)>,
    /// skill NAME upper -> skill id.
    pub skills: BTreeMap<String, SkillId>,
    /// `(FACT NAME upper, value upper)` -> declaring rule ids.
    pub fact_declarers: BTreeMap<(String, String), Vec<RuleId>>,
    /// source row -> the rule id whose own rows include it.
    pub row_owner: BTreeMap<RowRef, RuleId>,
    /// rule id -> its closure's own rows.
    pub own_rows: BTreeMap<RuleId, BTreeSet<RowRef>>,
    /// rule id -> the record's own `BONUS:VAR` contributions by NAME upper: (formula, type, pre-tail tokens).
    pub own_var_contribs: BTreeMap<RuleId, BTreeMap<String, Vec<OwnContribution>>>,
    /// rule id -> names the record's own rows DEFINE.
    pub own_defines: BTreeMap<RuleId, BTreeSet<String>>,
}

#[derive(Debug, Clone)]
pub struct OwnContribution {
    pub formula: String,
    pub bonus_type: Option<BonusType>,
    pub gates: Vec<String>,
    pub level_gate: Option<u8>,
    pub owning_class: Option<ClassId>,
}

/// Parse `TYPE=Racial.STACK` into a typed bonus.
pub fn parse_bonus_type(field: &str) -> Option<BonusType> {
    let rest = field.strip_prefix("TYPE=").or_else(|| field.strip_prefix("Type="))?;
    let mut parts = rest.split('.');
    let name = parts.next().unwrap_or("").trim().to_string();
    if name.is_empty() {
        return None;
    }
    let mut mode = StackMode::Plain;
    for p in parts {
        match p.trim().to_ascii_uppercase().as_str() {
            "STACK" => mode = StackMode::Stack,
            "REPLACE" => mode = StackMode::Replace,
            _ => {}
        }
    }
    Some(BonusType { name, mode })
}

/// Split a token value on `|`, peeling trailing `PRE...` / `!PRE...` segments into gates.
/// Returns `(value fields, gate tokens)`.
pub fn split_gates(value: &str) -> (Vec<String>, Vec<String>) {
    let mut fields: Vec<String> = Vec::new();
    let mut gates: Vec<String> = Vec::new();
    for seg in split_top_level(value, '|') {
        let s = seg.trim();
        let head = s.trim_start_matches('!');
        let pre_head = head.starts_with("PRE") && head.chars().skip(3).take_while(|c| c.is_ascii_uppercase()).count() > 0 && head.contains(':');
        if pre_head || s == "PRE:.CLEAR" {
            gates.push(s.to_string());
        } else {
            fields.push(s.to_string());
        }
    }
    (fields, gates)
}

/// Split on `sep` outside `[...]` and `(...)` and `"..."`.
pub fn split_top_level(s: &str, sep: char) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth_sq = 0i32;
    let mut depth_par = 0i32;
    let mut in_str = false;
    let mut cur = String::new();
    for c in s.chars() {
        match c {
            '"' => {
                in_str = !in_str;
                cur.push(c);
            }
            '[' if !in_str => {
                depth_sq += 1;
                cur.push(c);
            }
            ']' if !in_str => {
                depth_sq -= 1;
                cur.push(c);
            }
            '(' if !in_str => {
                depth_par += 1;
                cur.push(c);
            }
            ')' if !in_str => {
                depth_par -= 1;
                cur.push(c);
            }
            c if c == sep && !in_str && depth_sq <= 0 && depth_par <= 0 => {
                out.push(std::mem::take(&mut cur));
            }
            _ => cur.push(c),
        }
    }
    out.push(cur);
    out
}

/// The per-record conversion state.
pub struct RecordCtx<'a> {
    pub tree: &'a PinnedTree,
    pub index: &'a CorpusIndex,
    pub record: &'a RecordRef,
    pub closure: &'a Closure,
    pub choice_id: Option<String>,
    pub owning_class: Option<ClassId>,
    /// Token types that refused this record (per token SHAPE, never per unit).
    pub refusals: BTreeSet<String>,
    /// The token census (SD-35 AT-35-E2-004): the mapping-table row key of every token this
    /// record's closure carried -- `unmapped:<HEAD>` / `BONUS:<SUB>` when the table has no row.
    pub tokens: BTreeSet<String>,
    /// Refusal shape -> the token type(s) it arose under, so `token_coverage.py` counts
    /// "units refused because of this token" from the converter's own reading.
    pub refusal_under: BTreeMap<String, BTreeSet<String>>,
    /// Defect-list lines (`_defects/`), keyed by defect kind.
    pub defects: BTreeMap<String, Vec<String>>,
    /// Variable ids this record referenced through `Expr::Var`, with their source names.
    pub var_names: BTreeMap<VarId, String>,
    /// Names currently being inlined (cycle guard).
    inlining: Vec<String>,
    /// Fields omitted for product identity: declared / term hits.
    pub pi_declared: Vec<String>,
    pub pi_term_hits: Vec<String>,
}

impl<'a> RecordCtx<'a> {
    pub fn new(tree: &'a PinnedTree, index: &'a CorpusIndex, record: &'a RecordRef, closure: &'a Closure) -> Self {
        let owning_class = match record.kind.as_str() {
            "class" => Some(slug(&record.key)),
            _ => record.class_name.as_deref().and_then(|c| index.classes.get(&c.to_ascii_uppercase()).map(|(_, id)| id.clone()).or_else(|| Some(slug(c)))),
        };
        RecordCtx {
            tree,
            index,
            record,
            closure,
            choice_id: None,
            owning_class,
            refusals: BTreeSet::new(),
            tokens: BTreeSet::new(),
            refusal_under: BTreeMap::new(),
            defects: BTreeMap::new(),
            var_names: BTreeMap::new(),
            inlining: Vec::new(),
            pi_declared: Vec::new(),
            pi_term_hits: Vec::new(),
        }
    }

    pub fn refuse(&mut self, token_type: impl Into<String>) {
        self.refusals.insert(token_type.into());
    }

    /// Record that this record's closure carries a token of type `token_type` (census only).
    pub fn carry(&mut self, token_type: impl Into<String>) {
        self.tokens.insert(token_type.into());
    }

    /// Refuse under `shape`, recording the token type (`under`) the refusal arose under.
    pub fn refuse_under(&mut self, under: &str, shape: impl Into<String>) {
        let shape = shape.into();
        self.refusal_under.entry(shape.clone()).or_default().insert(under.to_string());
        self.refusals.insert(shape);
    }

    pub fn defect(&mut self, kind: &str, line: String) {
        self.defects.entry(kind.to_string()).or_default().push(line);
    }

    pub fn class_id(&self, name: &str) -> ClassId {
        let key = name.trim().to_ascii_uppercase();
        self.index.classes.get(&key).map(|(_, id)| id.clone()).unwrap_or_else(|| slug(name))
    }

    pub fn skill_id(&self, name: &str) -> SkillId {
        let key = name.trim().to_ascii_uppercase();
        self.index.skills.get(&key).cloned().unwrap_or_else(|| slug(name))
    }

    /// Resolve `(category, name)` to a rule id: KEY join first, then display name, then any
    /// kind's key/name for the category-less callers.
    pub fn resolve_rule(&self, category: &str, name: &str) -> Option<RuleId> {
        let cat = category.trim().to_ascii_uppercase();
        let n = name.trim().to_ascii_uppercase();
        if let Some(id) = self.index.by_cat_key.get(&(cat.clone(), n.clone())) {
            return Some(id.clone());
        }
        if let Some(id) = self.index.by_cat_name.get(&(cat, n)) {
            return Some(id.clone());
        }
        None
    }

    pub fn resolve_kind(&self, kind: &str, name: &str) -> Option<RuleId> {
        let n = name.trim().to_ascii_uppercase();
        self.index.by_kind_name.get(&(kind.to_string(), n)).cloned()
    }

    /// The variable's disposition: inline fold (same-record contributors only, all ungated or
    /// compare-gated), a `Var` over the cross-record table, or the C1 split for names declared
    /// nowhere in the corpus.
    pub fn resolve_variable(&mut self, name: &str, strict: bool) -> Result<Expr, String> {
        let upper = name.trim().to_ascii_uppercase();
        // Builtin master-level alias: companion records read the master's level.
        if upper == "MASTERLEVEL" {
            return Ok(Expr::MasterLevel);
        }
        let rows = self.tree.variable_rows(name);
        let declared = self.tree.is_declared(name);
        if rows.is_empty() {
            // C1 (c): DEFINEd nowhere. Const(0) + defect where the use is bare / additive;
            // REFUSE per name inside a function or under * /.
            if strict {
                return Err(format!("FORMULA:identifier DEFINEd nowhere ({name} inside a function or under * /)"));
            }
            self.defect("undefined-variables", format!("{}: {name}", self.record.id));
            return Ok(Expr::Const(0));
        }
        let own = &self.closure.own_rows;
        let same_record_only = rows.iter().all(|r| own.contains(r));
        if same_record_only
            && !self.inlining.iter().any(|n| n == &upper)
            && let Some(folded) = self.inline_same_record(&upper)?
        {
            return Ok(folded);
        }
        if !declared {
            // Contributions exist somewhere but no DEFINE anywhere: PCGen drops them (0).
            self.defect("undeclared-contributed-variables", format!("{}: {name}", self.record.id));
        }
        let id = var_id(name);
        self.var_names.insert(id.clone(), upper);
        Ok(Expr::Var(id))
    }

    /// Row 27 (`FORMULA:corpus variable, SAME-record contributors only`): fold the record's own
    /// contributions at convert time. `None` when a contribution is gated by something the
    /// 0/1 arithmetic cannot express (then the name goes through the `Var` table instead).
    fn inline_same_record(&mut self, upper: &str) -> Result<Option<Expr>, String> {
        let contribs = self
            .index
            .own_var_contribs
            .get(&self.record.id)
            .and_then(|m| m.get(upper))
            .cloned()
            .unwrap_or_default();
        if contribs.is_empty() {
            // DEFINE only: the seed is 0.
            return Ok(Some(Expr::Const(0)));
        }
        self.inlining.push(upper.to_string());
        let mut typed: Vec<(Option<BonusType>, Expr)> = Vec::new();
        let result = (|| {
            for c in &contribs {
                let saved_class = self.owning_class.clone();
                if let Some(cls) = &c.owning_class {
                    self.owning_class = Some(cls.clone());
                }
                let mut expr = super::formula::convert_formula(self, &c.formula)?;
                self.owning_class = saved_class;
                let mut gate_terms: Vec<Applies> = Vec::new();
                for g in &c.gates {
                    gate_terms.push(super::prereq::convert_pre_token(self, g)?);
                }
                if let Some(l) = c.level_gate
                    && let Some(cls) = &self.owning_class
                {
                    gate_terms.push(Applies::Compare { lhs: Expr::ClassLevel(cls.clone()), op: crate::rules_core::sheet_rule::Cmp::Gte, rhs: Expr::Const(l as i32) });
                }
                let gate = Applies::all(gate_terms);
                match gate {
                    Applies::Always => {}
                    Applies::Never => continue,
                    other => match super::prereq::applies_as_01(&other) {
                        Some(g01) => expr = Expr::mul(g01, expr),
                        None => return Ok(None),
                    },
                }
                typed.push((c.bonus_type.clone(), expr));
            }
            Ok(Some(fold_typed(typed)))
        })();
        self.inlining.pop();
        result
    }
}

/// Fold typed contributions the way the character-level fold does: untyped, `.STACK`, and
/// stacking-listed types sum; other same-type contributions take the max; `.REPLACE` =
/// `max(plain + stack, replace)`.
pub fn fold_typed(contribs: Vec<(Option<BonusType>, Expr)>) -> Expr {
    use crate::rules_core::sheet_rule::STACKING_TYPES;
    let mut sum_terms: Vec<Expr> = Vec::new();
    // (plain, stack, replace) contributions of one type.
    type Buckets = (Vec<Expr>, Vec<Expr>, Vec<Expr>);
    let mut by_type: BTreeMap<String, Buckets> = BTreeMap::new();
    for (bt, expr) in contribs {
        match bt {
            None => sum_terms.push(expr),
            Some(bt) => {
                let stacking = STACKING_TYPES.iter().any(|t| t.eq_ignore_ascii_case(&bt.name));
                let entry = by_type.entry(bt.name.clone()).or_default();
                match bt.mode {
                    StackMode::Stack => entry.1.push(expr),
                    StackMode::Replace => entry.2.push(expr),
                    StackMode::Plain if stacking => entry.1.push(expr),
                    StackMode::Plain => entry.0.push(expr),
                }
            }
        }
    }
    for (_, (plain, stack, replace)) in by_type {
        let plain_max = plain.into_iter().reduce(Expr::max);
        let stack_sum = if stack.is_empty() { None } else { Some(Expr::sum(stack)) };
        let plain_plus_stack = match (plain_max, stack_sum) {
            (Some(p), Some(s)) => Some(Expr::sum(vec![p, s])),
            (Some(p), None) => Some(p),
            (None, Some(s)) => Some(s),
            (None, None) => None,
        };
        let replace_max = replace.into_iter().reduce(Expr::max);
        match (plain_plus_stack, replace_max) {
            (Some(a), Some(r)) => sum_terms.push(Expr::max(a, r)),
            (Some(a), None) => sum_terms.push(a),
            (None, Some(r)) => sum_terms.push(r),
            (None, None) => {}
        }
    }
    Expr::sum(sum_terms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn var_id_is_case_folded_and_opaque() {
        assert_eq!(var_id("FighterLvl"), var_id("FIGHTERLVL"));
        assert!(var_id("X").starts_with('v'));
        assert_eq!(var_id("X").len(), 17);
    }

    #[test]
    fn split_gates_peels_trailing_pre_segments() {
        let (f, g) = split_gates("VAR|X|1|TYPE=Base|PREVARGTEQ:Y,8|!PREABILITY:1,CATEGORY=FEAT,Z");
        assert_eq!(f, vec!["VAR", "X", "1", "TYPE=Base"]);
        assert_eq!(g, vec!["PREVARGTEQ:Y,8", "!PREABILITY:1,CATEGORY=FEAT,Z"]);
    }

    #[test]
    fn split_top_level_respects_brackets() {
        let v = split_top_level("1,[PREABILITY:1,CATEGORY=FEAT,A],[PREVAREQ:B,1]", ',');
        assert_eq!(v, vec!["1", "[PREABILITY:1,CATEGORY=FEAT,A]", "[PREVAREQ:B,1]"]);
    }

    #[test]
    fn typed_fold_takes_max_for_non_stacking_and_sums_stacking() {
        let e = fold_typed(vec![
            (Some(BonusType { name: "Resistance".into(), mode: StackMode::Plain }), Expr::Const(10)),
            (Some(BonusType { name: "Resistance".into(), mode: StackMode::Plain }), Expr::Const(15)),
        ]);
        assert_eq!(e, Expr::Max(Box::new(Expr::Const(10)), Box::new(Expr::Const(15))));
        let e = fold_typed(vec![
            (Some(BonusType { name: "Racial".into(), mode: StackMode::Plain }), Expr::Const(2)),
            (Some(BonusType { name: "Racial".into(), mode: StackMode::Plain }), Expr::Const(2)),
            (None, Expr::Level),
        ]);
        assert_eq!(e, Expr::Sum(vec![Expr::Level, Expr::Const(2), Expr::Const(2)]), "stacking types sum term by term (no constant folding: the shape reads as the table states it)");
    }
}
