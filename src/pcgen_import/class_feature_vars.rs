//! Converter side: a corpus record's own variable chain, read out of the source tokens once, at
//! ingest, and written as converted [`Expr`] into `data/converted/record_vars.json`.
//!
//! SD-35 `AT-35-E6-001` cycle 4 (`decisions.md` §11). Everything in this module used to live in
//! `rules_core::pilot_compute::class_feature_grant_consumer` and run at REQUEST time: the source
//! row reader (`bonus_stack_reader`), the formula parser and the formula evaluator
//! (`PcgenFormulaEvaluator`). That is an ingest-format engine in the middle of live code, which
//! the operator ruled out. The reading and the conversion happen here now; the live side reads
//! the converted artifact and evaluates our own `Expr`
//! (`crate::rules_core::record_vars`).
//!
//! **Nothing about WHICH rows are read or HOW a target is chosen changed in the move.** The row
//! selection below (multi-row `PREVARGTEQ` summation via [`bonus_stack_reader`], the `TYPE=`
//! strip, the sole-ungated-row fallback) is the same code, and the lowering in [`lower_formula`]
//! is a term-for-term translation of the interpreter's own evaluation rules -- `floor` toward
//! negative infinity, `min`/`max` variadic, a comparison as a numeric 0/1, both sides of `&&`
//! always evaluated, one truncation at the boundary. The corpus-wide before/after comparison in
//! `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/` is what proves it.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::pcgen_import::bonus_stack_reader;
use crate::pcgen_import::formula_interpreter::{self, CmpOp};
use crate::rules_core::record_vars::{
    ABILITY_SEED_NAMES, ConvertedVar, RecordVarPackage, class_level_call_key,
};
use crate::rules_core::sheet_rule::{Cmp, Expr, var_id};

// ---------------------------------------------------------------------------------------------
// 1. Formula lowering: source formula text -> our converted `Expr`.
// ---------------------------------------------------------------------------------------------

/// Every free identifier the lowered form of `formula` still references, by NAME. Recorded
/// because the converted `Expr` carries opaque ids and a consumer still needs to know which
/// other target a target depends on (terminal detection) and which name to default.
#[derive(Debug, Clone)]
pub struct LoweredFormula {
    pub expr: Expr,
    pub refs: BTreeSet<String>,
}

fn literal(n: f64) -> Result<Expr, String> {
    if !n.is_finite() {
        return Err(format!("non-finite literal {n}"));
    }
    if n.fract() == 0.0 && n.abs() <= f64::from(i32::MAX) {
        return Ok(Expr::Const(n as i32));
    }
    // A decimal literal is an exact rational: scale it and divide. `1e6` covers every decimal
    // place this corpus's formulas carry and keeps both terms inside `i32`.
    for scale in [10i64, 100, 1_000, 10_000, 100_000, 1_000_000] {
        let scaled = n * scale as f64;
        if scaled.fract() == 0.0
            && let (Ok(num), Ok(den)) = (i32::try_from(scaled as i64), i32::try_from(scale))
        {
            return Ok(Expr::Div(Box::new(Expr::Const(num)), Box::new(Expr::Const(den))));
        }
    }
    Err(format!("literal {n} is not an exact decimal"))
}

fn neg(e: Expr) -> Expr {
    Expr::Mul(Box::new(Expr::Const(-1)), Box::new(e))
}

fn cmp_ge(lhs: Expr, rhs: Expr) -> Expr {
    Expr::Min(
        Box::new(Expr::Const(1)),
        Box::new(Expr::Max(
            Box::new(Expr::Const(0)),
            Box::new(Expr::Sum(vec![lhs, neg(rhs), Expr::Const(1)])),
        )),
    )
}

fn one_minus(e: Expr) -> Expr {
    Expr::Sum(vec![Expr::Const(1), neg(e)])
}

fn cmp(op: Cmp, lhs: Expr, rhs: Expr) -> Expr {
    match op {
        Cmp::Gte => cmp_ge(lhs, rhs),
        Cmp::Gt => cmp_ge(lhs, Expr::Sum(vec![rhs, Expr::Const(1)])),
        Cmp::Lt => one_minus(cmp_ge(lhs, rhs)),
        Cmp::Lte => one_minus(cmp_ge(lhs, Expr::Sum(vec![rhs, Expr::Const(1)]))),
        Cmp::Eq => Expr::Min(
            Box::new(cmp_ge(lhs.clone(), rhs.clone())),
            Box::new(one_minus(cmp_ge(lhs, Expr::Sum(vec![rhs, Expr::Const(1)])))),
        ),
        Cmp::Ne => one_minus(cmp(Cmp::Eq, lhs, rhs)),
    }
}

fn reference(name: &str, refs: &mut BTreeSet<String>) -> Expr {
    refs.insert(name.to_string());
    Expr::Var(var_id(name))
}

fn lower_ast(
    ast: &formula_interpreter::Expr,
    refs: &mut BTreeSet<String>,
) -> Result<Expr, String> {
    use formula_interpreter::Expr as A;
    Ok(match ast {
        A::Num(n) => literal(*n)?,
        A::Var(name) => reference(name, refs),
        A::Neg(a) => neg(lower_ast(a, refs)?),
        A::Add(a, b) => Expr::Sum(vec![lower_ast(a, refs)?, lower_ast(b, refs)?]),
        A::Sub(a, b) => Expr::Sum(vec![lower_ast(a, refs)?, neg(lower_ast(b, refs)?)]),
        A::Mul(a, b) => Expr::Mul(Box::new(lower_ast(a, refs)?), Box::new(lower_ast(b, refs)?)),
        A::Div(a, b) => Expr::Div(Box::new(lower_ast(a, refs)?), Box::new(lower_ast(b, refs)?)),
        // A level in a NAMED class. Bound only when the name is the record's own granting class,
        // exactly as the evaluator this replaces bound it -- a genuinely different class stays
        // unbound and the target refuses rather than answering with the wrong class's level.
        A::ClassLevel(class_name) => reference(&class_level_call_key(class_name), refs),
        // Never bound by any caller of this chain, in the interpreter or here: refuses.
        A::SkillInfoTotalRank(skill) => {
            reference(&format!("SKILLINFO_TOTALRANK::{skill}"), refs)
        }
        A::Cmp(a, op, b) => {
            let lhs = lower_ast(a, refs)?;
            let rhs = lower_ast(b, refs)?;
            cmp(
                match op {
                    CmpOp::Ge => Cmp::Gte,
                    CmpOp::Le => Cmp::Lte,
                    CmpOp::Eq => Cmp::Eq,
                    CmpOp::Ne => Cmp::Ne,
                    CmpOp::Gt => Cmp::Gt,
                    CmpOp::Lt => Cmp::Lt,
                },
                lhs,
                rhs,
            )
        }
        // Both operands always evaluated, "nonzero is true": with 0/1 operands that is a product.
        A::And(a, b) => {
            Expr::Mul(Box::new(lower_ast(a, refs)?), Box::new(lower_ast(b, refs)?))
        }
        // `cond` is a comparison or an `&&` of them (the parser enforces it), so it is already
        // 0/1: `cond*then + (1-cond)*else`.
        A::If(cond, then, other) => {
            let c = lower_ast(cond, refs)?;
            let t = lower_ast(then, refs)?;
            let e = lower_ast(other, refs)?;
            Expr::Sum(vec![
                Expr::Mul(Box::new(c.clone()), Box::new(t)),
                Expr::Mul(Box::new(one_minus(c)), Box::new(e)),
            ])
        }
        A::Call(name, args) => {
            let mut lowered = Vec::with_capacity(args.len());
            for a in args {
                lowered.push(lower_ast(a, refs)?);
            }
            match name.as_str() {
                "min" | "max" => {
                    let mut it = lowered.into_iter();
                    let first = it.next().ok_or_else(|| format!("{name}() with no argument"))?;
                    it.fold(first, |acc, next| {
                        if name == "min" {
                            Expr::Min(Box::new(acc), Box::new(next))
                        } else {
                            Expr::Max(Box::new(acc), Box::new(next))
                        }
                    })
                }
                "floor" => Expr::Floor(Box::new(
                    lowered.into_iter().next().ok_or("floor() with no argument")?,
                )),
                "ceil" => Expr::Ceil(Box::new(
                    lowered.into_iter().next().ok_or("ceil() with no argument")?,
                )),
                // `|x|` is `max(x, -x)`; our vocabulary has no absolute value of its own.
                "abs" => {
                    let x = lowered.into_iter().next().ok_or("abs() with no argument")?;
                    Expr::Max(Box::new(x.clone()), Box::new(neg(x)))
                }
                other => return Err(format!("unknown function {other:?}")),
            }
        }
    })
}

/// Lower one source formula. `Err` for any shape the parser or this lowering does not
/// recognise -- the target is then absent from the artifact and its consumer refuses, exactly as
/// the interpreter's own refusal did.
pub fn lower_formula(formula: &str) -> Result<LoweredFormula, String> {
    let ast = formula_interpreter::parse(formula).map_err(|e| e.0)?;
    let mut refs = BTreeSet::new();
    let expr = lower_ast(&ast, &mut refs)?;
    Ok(LoweredFormula { expr, refs })
}

// ---------------------------------------------------------------------------------------------
// 2. Row reading: which `BONUS:VAR` rows a record contributes for a target name.
//    Moved verbatim from `class_feature_grant_consumer::parse_bonus_var_tokens_pre_gate_safe`
//    (SD-31 wave 26 / SD-32 T12 row 18 cycle 6 / row 21 cycle 2 -- the three widenings named in
//    that function's own doc: `TYPE=` is a stacking classification and is stripped, never a
//    gate; a multi-row `PREVARGTEQ`-gated target is summed over its qualifying rows via
//    `bonus_stack_reader`; a target whose rows carry a shape the reader refuses falls back to
//    its own SOLE ungated row, if it has exactly one, rather than being dropped entirely).
// ---------------------------------------------------------------------------------------------

fn parse_bonus_var_tokens_pre_gate_safe(raw_tokens: &[Value]) -> BTreeMap<String, String> {
    let mut expanded: Vec<(String, String)> = Vec::new();
    let mut target_order: Vec<String> = Vec::new();
    let mut seen_targets: BTreeSet<String> = BTreeSet::new();
    let mut ungated_formulas: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for token in raw_tokens {
        if token["key"].as_str() != Some("BONUS") {
            continue;
        }
        let Some(value) = token["value"].as_str() else { continue };
        let Some(rest) = value.strip_prefix("VAR|") else { continue };
        let mut parts = rest.splitn(2, '|');
        let (Some(names), Some(formula_and_tail)) = (parts.next(), parts.next()) else {
            continue;
        };
        let mut tail_fields: Vec<&str> = formula_and_tail.split('|').collect();
        let formula = tail_fields.remove(0);
        tail_fields.retain(|field| !field.starts_with("TYPE="));
        let rebuilt = if tail_fields.is_empty() {
            formula.to_string()
        } else {
            format!("{formula}|{}", tail_fields.join("|"))
        };
        for name in names.split(',') {
            let name = name.trim();
            if name.is_empty() {
                continue;
            }
            if seen_targets.insert(name.to_string()) {
                target_order.push(name.to_string());
            }
            if tail_fields.is_empty() {
                ungated_formulas.entry(name.to_string()).or_default().insert(formula.to_string());
            }
            expanded.push(("BONUS".to_string(), format!("VAR|{name}|{rebuilt}")));
        }
    }
    let borrowed: Vec<(&str, &str)> =
        expanded.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
    let mut out = BTreeMap::new();
    for name in target_order {
        let Ok(addends) = bonus_stack_reader::extract_addends(&name, borrowed.iter().copied())
        else {
            if let Some(formulas) = ungated_formulas.get(&name)
                && let [only] = formulas.iter().collect::<Vec<_>>().as_slice()
            {
                out.insert(name, (*only).clone());
            }
            continue;
        };
        match addends.as_slice() {
            [] => {}
            [only] if only.gate.is_none() => {
                out.insert(name, only.formula.clone());
            }
            _ => {
                let synthesized = addends
                    .iter()
                    .map(|addend| match &addend.gate {
                        None => format!("({})", addend.formula),
                        Some(gate) => format!(
                            "if({}>={},({}),0)",
                            gate.variable, gate.threshold, addend.formula
                        ),
                    })
                    .collect::<Vec<_>>()
                    .join("+");
                out.insert(name, synthesized);
            }
        }
    }
    out
}

/// The one per-target merge policy every cross-book table here shares: a target already bound by
/// an earlier book is never overwritten, but a target that book never carried does merge in.
fn merge_never_overwriting<V>(into: &mut BTreeMap<String, V>, from: BTreeMap<String, V>) {
    for (target, value) in from {
        into.entry(target).or_insert(value);
    }
}

fn lower_map(formulas: BTreeMap<String, String>) -> BTreeMap<String, ConvertedVar> {
    let mut out = BTreeMap::new();
    for (name, formula) in formulas {
        let Ok(lowered) = lower_formula(&formula) else { continue };
        out.insert(
            name,
            ConvertedVar { expr: lowered.expr, refs: lowered.refs.into_iter().collect() },
        );
    }
    out
}

// ---------------------------------------------------------------------------------------------
// 3. The corpus walk. Every table below is the same walk, over the same directories, with the
//    same admission rule its `rules_core` original carried; only the value type changed.
// ---------------------------------------------------------------------------------------------

fn walk_json_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            walk_json_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "json") {
            out.push(path);
        }
    }
}

fn book_dirs(corpus_root: &Path) -> Vec<PathBuf> {
    let Ok(books) = std::fs::read_dir(corpus_root) else { return Vec::new() };
    let mut dirs: Vec<_> = books.flatten().collect();
    dirs.sort_by_key(|e| e.file_name());
    dirs.into_iter().map(|e| e.path()).collect()
}

fn read_doc(file: &Path) -> Option<Value> {
    let text = std::fs::read_to_string(file).ok()?;
    serde_json::from_str(&text).ok()
}

fn is_real_description_value(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }
    let lower = trimmed.to_ascii_lowercase();
    !matches!(lower.as_str(), ".clear" | ".clearall" | "[redacted pi]")
}

/// Build the whole converted package from a corpus tree.
pub fn build(repo: &Path) -> RecordVarPackage {
    let corpus_root = repo.join("data/corpus");
    let mut pkg = RecordVarPackage::default();
    // The corpus-wide "bound anywhere" set is derived from exactly what was INGESTED, not from
    // what lowered: a target whose formula the lowering refuses is still a target the corpus
    // binds, and defaulting it would fabricate a value.
    let mut bound_anywhere: BTreeSet<String> = BTreeSet::new();
    let mut define_defaults: BTreeMap<String, i64> = BTreeMap::new();

    for book in book_dirs(&corpus_root) {
        let cf_dir = book.join("class_feature");
        if cf_dir.is_dir() {
            let mut files = Vec::new();
            walk_json_files(&cf_dir, &mut files);
            for file in &files {
                let Some(doc) = read_doc(file) else { continue };
                let data = &doc["data"];
                let Some(key) = data["key"].as_str() else { continue };
                if data["name"].as_str().is_none() {
                    continue;
                }
                let raw_tokens = data["raw_tokens"].as_array().cloned().unwrap_or_default();
                let formulas = parse_bonus_var_tokens_pre_gate_safe(&raw_tokens);
                bound_anywhere.extend(formulas.keys().cloned());
                for token in &raw_tokens {
                    collect_define_default(token, &mut define_defaults);
                }
                // `%N` description arguments are their own family: a record whose `%N` is a
                // formula EXPRESSION rather than a chain target is resolved by
                // `resolved_description_for_formula_only_desc_argument`, which used to run the
                // interpreter over that raw argument text at request time. Lowered here instead,
                // keyed by the exact argument text the renderer matches on.
                if let Some(desc) = data["description"].as_str()
                    && is_real_description_value(desc)
                {
                    let mut args: BTreeMap<String, ConvertedVar> = BTreeMap::new();
                    for arg in crate::rules_core::pcgen_desc::desc_token_arguments(desc) {
                        let trimmed = arg.trim().to_string();
                        if trimmed.is_empty() || args.contains_key(&trimmed) {
                            continue;
                        }
                        let Ok(lowered) = lower_formula(&trimmed) else { continue };
                        args.insert(
                            trimmed,
                            ConvertedVar {
                                expr: lowered.expr,
                                refs: lowered.refs.into_iter().collect(),
                            },
                        );
                    }
                    if !args.is_empty() {
                        let entry = pkg.desc_arguments.entry(key.to_string()).or_default();
                        merge_never_overwriting(entry, args);
                    }
                }
                if formulas.is_empty() {
                    continue;
                }
                let lowered = lower_map(formulas);
                if lowered.is_empty() {
                    continue;
                }
                // Table A -- the description-gated member table. Same admission rule as its
                // `rules_core` original: a class-less record is out, and a description that is
                // present but not a real value (`.CLEAR`, a PI marker) is out; an ABSENT
                // description is in, carrying an empty one.
                let admitted_by_description = data["class"].as_str().is_some()
                    && match data["description"].as_str() {
                        Some(s) => is_real_description_value(s),
                        None => true,
                    };
                if admitted_by_description {
                    let entry = pkg.class_feature_described.entry(key.to_string()).or_default();
                    merge_never_overwriting(entry, lowered.clone());
                }
                // Table B -- every class_feature record, description or not.
                let entry = pkg.class_feature_any.entry(key.to_string()).or_default();
                merge_never_overwriting(entry, lowered);
            }

            // Wildblooded bloodline variants: the parent pool group each one declares.
            let wb_dir = cf_dir.join("wildblooded");
            if wb_dir.is_dir() {
                let mut wb_files = Vec::new();
                walk_json_files(&wb_dir, &mut wb_files);
                for file in &wb_files {
                    let Some(doc) = read_doc(file) else { continue };
                    let data = &doc["data"];
                    let Some(name) = data["name"].as_str() else { continue };
                    let Some(tokens) = data["raw_tokens"].as_array() else { continue };
                    let Some(preability) = tokens
                        .iter()
                        .find(|t| t["key"].as_str() == Some("PREABILITY"))
                        .and_then(|t| t["value"].as_str())
                    else {
                        continue;
                    };
                    let Some(last_segment) = preability.rsplit(',').next() else { continue };
                    let Some(parent) = last_segment.rsplit(" ~ ").next() else { continue };
                    if parent == last_segment {
                        continue;
                    }
                    pkg.wildblooded_parents
                        .entry(format!("{name} Bloodline"))
                        .or_insert_with(|| format!("{parent} Bloodline"));
                }
            }
        }

        // Class records.
        let class_dir = book.join("class");
        if class_dir.is_dir() {
            let mut files = Vec::new();
            walk_json_files(&class_dir, &mut files);
            for file in &files {
                let Some(doc) = read_doc(file) else { continue };
                let data = &doc["data"];
                let Some(class_id) = data["class_id"].as_str() else { continue };
                let raw_tokens = data["raw_tokens"].as_array().cloned().unwrap_or_default();
                let formulas = parse_bonus_var_tokens_pre_gate_safe(&raw_tokens);
                bound_anywhere.extend(formulas.keys().cloned());
                for token in &raw_tokens {
                    collect_define_default(token, &mut define_defaults);
                }
                if formulas.is_empty() {
                    continue;
                }
                let entry = pkg.class_records.entry(class_id.to_string()).or_default();
                merge_never_overwriting(entry, lower_map(formulas));
            }
        }

        // Domain records.
        let domain_dir = book.join("domain");
        if domain_dir.is_dir() {
            let mut files = Vec::new();
            walk_json_files(&domain_dir, &mut files);
            for file in &files {
                let Some(doc) = read_doc(file) else { continue };
                let data = &doc["data"];
                let Some(key) = data["key"].as_str() else { continue };
                let raw_tokens = data["raw_tokens"].as_array().cloned().unwrap_or_default();
                let formulas = parse_bonus_var_tokens_pre_gate_safe(&raw_tokens);
                if formulas.is_empty() {
                    continue;
                }
                let entry = pkg.domain_records.entry(key.to_string()).or_default();
                merge_never_overwriting(entry, lower_map(formulas));
            }
        }
    }

    // Every name any converted chain still references, minus the ones the corpus binds
    // somewhere: those, and only those, carry a declared baseline the live fold may substitute.
    let mut referenced: BTreeSet<String> = BTreeSet::new();
    for chain in pkg
        .class_feature_described
        .values()
        .chain(pkg.class_feature_any.values())
        .chain(pkg.class_records.values())
        .chain(pkg.domain_records.values())
        .chain(pkg.desc_arguments.values())
    {
        for var in chain.values() {
            referenced.extend(var.refs.iter().cloned());
        }
    }
    let seeds: BTreeSet<&str> = ABILITY_SEED_NAMES.iter().map(|(abbr, _)| *abbr).collect();
    for name in referenced {
        if bound_anywhere.contains(&name) || seeds.contains(name.as_str()) {
            continue;
        }
        // A `classlevel(<other class>)` key and a `skillinfo` key are deliberately NOT
        // defaultable: no caller binds them and guessing one answers with another class's level.
        if name.starts_with("CLASSLEVEL::") || name.starts_with("SKILLINFO_TOTALRANK::") {
            continue;
        }
        let value = define_defaults.get(&name).copied().unwrap_or(0);
        pkg.var_defaults.insert(name, value);
    }
    pkg
}

/// `DEFINE:<name>|<literal integer>` -- the source format's own idiom for a target's declared
/// baseline. First book wins; a non-literal value is skipped rather than guessed at.
fn collect_define_default(token: &Value, out: &mut BTreeMap<String, i64>) {
    if token["key"].as_str() != Some("DEFINE") {
        return;
    }
    let Some(value) = token["value"].as_str() else { return };
    let mut parts = value.splitn(2, '|');
    let (Some(name), Some(literal)) = (parts.next(), parts.next()) else { return };
    let name = name.trim();
    if name.is_empty() {
        return;
    }
    let Ok(parsed) = literal.trim().parse::<i64>() else { return };
    out.entry(name.to_string()).or_insert(parsed);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::record_vars::{SeedAbilityMods, resolve_chain};

    fn resolve_one(formula: &str, class_level_var: &str, level: u8) -> Option<i64> {
        let lowered = lower_formula(formula).ok()?;
        let mut chain = BTreeMap::new();
        chain.insert(
            "T".to_string(),
            ConvertedVar { expr: lowered.expr, refs: lowered.refs.into_iter().collect() },
        );
        resolve_chain(&chain, class_level_var, level, SeedAbilityMods::default(), &BTreeMap::new())
            .get("T")
            .copied()
    }

    #[test]
    fn a_division_truncates_once_at_the_boundary_not_at_each_step() {
        // `(7/2)*2` is 7, not 6: the fold is exact until the single boundary truncation, which
        // is what the interpreter's own `f64`-end-to-end contract did.
        assert_eq!(resolve_one("(RogueLVL/2)*2", "RogueLVL", 7), Some(7));
    }

    #[test]
    fn floor_rounds_toward_negative_infinity() {
        assert_eq!(resolve_one("floor(0-(RogueLVL/2))", "RogueLVL", 7), Some(-4));
    }

    #[test]
    fn a_bare_comparison_is_a_numeric_one_or_zero() {
        assert_eq!(resolve_one("1+(RogueLVL>=15)", "RogueLVL", 15), Some(2));
        assert_eq!(resolve_one("1+(RogueLVL>=15)", "RogueLVL", 14), Some(1));
    }

    #[test]
    fn an_if_selects_its_branch() {
        assert_eq!(resolve_one("if(RogueLVL<19,1+((RogueLVL/2)-5),5)", "RogueLVL", 10), Some(1));
        assert_eq!(resolve_one("if(RogueLVL<19,1+((RogueLVL/2)-5),5)", "RogueLVL", 20), Some(5));
    }

    #[test]
    fn min_and_max_are_variadic_and_abs_is_a_max() {
        assert_eq!(resolve_one("max(1,RogueLVL/2)", "RogueLVL", 1), Some(1));
        assert_eq!(resolve_one("min(3,RogueLVL,9)", "RogueLVL", 5), Some(3));
        assert_eq!(resolve_one("abs(0-RogueLVL)", "RogueLVL", 6), Some(6));
    }

    #[test]
    fn an_and_of_two_comparisons_is_their_product() {
        assert_eq!(resolve_one("if(RogueLVL>=5&&RogueLVL<=9,1,0)", "RogueLVL", 7), Some(1));
        assert_eq!(resolve_one("if(RogueLVL>=5&&RogueLVL<=9,1,0)", "RogueLVL", 12), Some(0));
    }

    #[test]
    fn a_shape_the_parser_refuses_is_refused_here_too_never_guessed() {
        assert!(lower_formula("skillinfo(\"RANK\", \"Stealth\")").is_err());
        assert!(lower_formula("!!!").is_err());
    }

    #[test]
    fn the_row_reader_sums_a_multi_row_gated_target_and_keeps_a_sole_ungated_row() {
        let tokens: Vec<Value> = vec![
            serde_json::json!({"key":"BONUS","value":"VAR|BarbarianDR|(BarbarianDRLVL-4)/3"}),
            serde_json::json!({"key":"BONUS","value":"VAR|BarbarianDR|1|PREVARGTEQ:Barbarian_CF_DamageReduction1,1|PREVAREQ:Barbarian_CF_DamageReduction1,1"}),
        ];
        let out = parse_bonus_var_tokens_pre_gate_safe(&tokens);
        assert_eq!(
            out.get("BarbarianDR").map(String::as_str),
            Some("(BarbarianDRLVL-4)/3"),
            "the reader refuses the two-PRE-kind row and keeps the record's sole ungated row"
        );
    }

    #[test]
    fn a_type_field_is_stripped_and_never_read_as_a_gate() {
        let tokens: Vec<Value> = vec![serde_json::json!(
            {"key":"BONUS","value":"VAR|DomainAirLVL|DomainLVL|TYPE=Domain"}
        )];
        let out = parse_bonus_var_tokens_pre_gate_safe(&tokens);
        assert_eq!(out.get("DomainAirLVL").map(String::as_str), Some("DomainLVL"));
    }
}
