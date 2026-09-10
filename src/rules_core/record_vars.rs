//! Converted record variables -- the live side's own read of a corpus record's variable chain.
//!
//! SD-35 `AT-35-E6-001` (`decisions.md` §11: *"when we are done, there should be nothing left of
//! pcgen"*). Before this module, `pilot_compute::class_feature_grant_consumer` carried a
//! record's variable chain as **source formula strings** and ran them, at run time, through the
//! ingest-format formula interpreter and the ingest-format bonus-row reader (both now
//! converter-side only, `crate::pcgen_import`) -- an ingest-format engine sitting in the middle
//! of live code.
//!
//! The chain is now converted **at ingest**, by
//! [`crate::pcgen_import::class_feature_vars`], into [`Expr`] -- the same converted arithmetic
//! `sheet_rule::evaluate` already applies to every other magnitude on the sheet -- and shipped as
//! `data/converted/record_vars.json` ([`RecordVarPackage`]). This module reads that artifact and
//! evaluates it. Nothing here parses a token, names a token, or knows what a `.lst` file is; the
//! only source-format text that survives is a variable's own NAME, which is the key the record's
//! own description arguments are matched by and which leaves with the prose renderer
//! (`AT-35-E6-002`/`003`), not with this criterion.
//!
//! **The evaluation contract is unchanged from the interpreter it replaces**, deliberately, so
//! that the corpus-wide before/after comparison in
//! `artifacts/epic-6-pcgen-exit/AT-35-E6-001_cycle4_varchain-{before,after}.json` is a real
//! proof and not a re-specification:
//!
//! 1. Seed the environment with the two facts a caller knows about a character -- their level in
//!    the granting class (bound to the class's own level-variable name, and to the
//!    `classlevel(<that class>)` key) and their six ability modifiers.
//! 2. Substitute, to a fixed point, every reference the environment can reach.
//! 3. Then, and only then, default a reference the corpus binds NOWHERE to its declared
//!    baseline (or 0). A reference the corpus binds somewhere, under some condition, is never
//!    guessed -- the target it appears in stays unresolved and its consumer prints words.
//! 4. Evaluate what is closed. Arithmetic is exact and truncates once, toward zero, at the
//!    boundary -- `Rat`'s own contract, and the same one the replaced interpreter carried
//!    (`f64` end to end, truncated only at the public boundary).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use super::sheet_rule::{Ability, CharacterFacts, Expr, VarId, evaluate_expr_from_facts, var_id};

/// The relative path of the converted artifact, from the repo root.
pub const RECORD_VARS_PATH: &str = "data/converted/record_vars.json";

/// One converted variable: what it evaluates to, and which other variable names its source form
/// referenced before conversion inlined nothing (`refs` is what a consumer uses to tell a
/// terminal target from an intermediate one, and it is recorded at convert time because the
/// converted [`Expr`] carries opaque ids, not names).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConvertedVar {
    pub expr: Expr,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub refs: Vec<String>,
}

/// One record's converted chain: variable name -> what it evaluates to.
pub type ConvertedChain = BTreeMap<String, ConvertedVar>;

/// `data/converted/record_vars.json` -- every chain the class-feature consumers read, converted.
/// Records carrying no chain at all are absent: an absent key is an empty chain, and every
/// consumer already refuses cleanly on one.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RecordVarPackage {
    /// `class_feature` records admitted by the description gate, keyed by corpus `KEY:`.
    pub class_feature_described: BTreeMap<String, ConvertedChain>,
    /// Every `class_feature` record's chain, keyed by corpus `KEY:`, description or not.
    pub class_feature_any: BTreeMap<String, ConvertedChain>,
    /// Every `class` record's chain, keyed by class id.
    pub class_records: BTreeMap<String, ConvertedChain>,
    /// Every `domain` record's chain, keyed by the domain's bare `KEY:`.
    pub domain_records: BTreeMap<String, ConvertedChain>,
    /// A `class_feature` record's `%N` description arguments, keyed by the exact argument text
    /// the renderer matches on -- the OTHER family that used to be evaluated at request time.
    #[serde(default)]
    pub desc_arguments: BTreeMap<String, ConvertedChain>,
    /// A wildblooded bloodline variant's pool group -> its declared parent's pool group.
    pub wildblooded_parents: BTreeMap<String, String>,
    /// A referenced variable name the corpus binds nowhere -> its declared baseline (step 3).
    pub var_defaults: BTreeMap<String, i64>,
}

impl RecordVarPackage {
    pub fn read(path: &Path) -> Result<RecordVarPackage, String> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| format!("reading {}: {e}", path.display()))?;
        serde_json::from_str(&text).map_err(|e| format!("parsing {}: {e}", path.display()))
    }
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The shipped converted artifact. An absent or unreadable artifact is an EMPTY package, never a
/// panic and never a fabricated chain: every consumer of a chain already refuses cleanly when a
/// record has no chain, which is exactly the behaviour a missing artifact must produce.
pub fn package() -> &'static RecordVarPackage {
    static PACKAGE: OnceLock<RecordVarPackage> = OnceLock::new();
    PACKAGE.get_or_init(|| {
        RecordVarPackage::read(&repo_root().join(RECORD_VARS_PATH)).unwrap_or_default()
    })
}

/// The six ability abbreviations a source formula may name bare, and the converted term each
/// one is seeded with. Held here rather than in the converter because it is a statement about
/// what the LIVE character has, not about what the source format wrote.
pub const ABILITY_SEED_NAMES: [(&str, Ability); 6] = [
    ("STR", Ability::Str),
    ("DEX", Ability::Dex),
    ("CON", Ability::Con),
    ("INT", Ability::Int),
    ("WIS", Ability::Wis),
    ("CHA", Ability::Cha),
];

/// The environment key a `classlevel(<class>)` term resolves through. The converter mints the
/// same key, so a term naming the record's OWN class resolves and a term naming any other class
/// stays unbound and refuses -- never silently answers with the wrong class's level.
pub fn class_level_call_key(class_name: &str) -> String {
    format!("CLASSLEVEL::{class_name}")
}

/// The class's own level-variable name: the class's display name with whitespace removed, plus
/// `LVL`. A source-format convention, reproduced here because it is the key a caller must seed.
pub fn class_level_variable_name(class: &str) -> String {
    let mut out: String = class.chars().filter(|c| !c.is_whitespace()).collect();
    out.push_str("LVL");
    out
}

/// Substitute `Var` references from `env`, once, everywhere in `e`.
fn substitute(e: &Expr, env: &BTreeMap<VarId, Expr>) -> Expr {
    match e {
        Expr::Var(id) => env.get(id).cloned().unwrap_or_else(|| e.clone()),
        Expr::Sum(parts) => Expr::Sum(parts.iter().map(|p| substitute(p, env)).collect()),
        Expr::Mul(a, b) => {
            Expr::Mul(Box::new(substitute(a, env)), Box::new(substitute(b, env)))
        }
        Expr::Div(a, b) => {
            Expr::Div(Box::new(substitute(a, env)), Box::new(substitute(b, env)))
        }
        Expr::Min(a, b) => {
            Expr::Min(Box::new(substitute(a, env)), Box::new(substitute(b, env)))
        }
        Expr::Max(a, b) => {
            Expr::Max(Box::new(substitute(a, env)), Box::new(substitute(b, env)))
        }
        Expr::Floor(a) => Expr::Floor(Box::new(substitute(a, env))),
        Expr::Ceil(a) => Expr::Ceil(Box::new(substitute(a, env))),
        _ => e.clone(),
    }
}

fn is_closed(e: &Expr) -> bool {
    let mut ids = Vec::new();
    e.var_ids(&mut ids);
    ids.is_empty()
}

/// Whether any division in `e` has a denominator that evaluates to zero for `facts`. The
/// interpreter this replaces refused such a formula outright rather than answering; `Rat`
/// answers 0. Checked explicitly so the refusal survives the swap.
fn divides_by_zero(e: &Expr, facts: &CharacterFacts) -> bool {
    match e {
        Expr::Div(a, b) => {
            evaluate_expr_from_facts(b, facts).is_zero()
                || divides_by_zero(a, facts)
                || divides_by_zero(b, facts)
        }
        Expr::Sum(parts) => parts.iter().any(|p| divides_by_zero(p, facts)),
        Expr::Mul(a, b) | Expr::Min(a, b) | Expr::Max(a, b) => {
            divides_by_zero(a, facts) || divides_by_zero(b, facts)
        }
        Expr::Floor(a) | Expr::Ceil(a) => divides_by_zero(a, facts),
        _ => false,
    }
}

/// The six ability modifiers a caller has already computed for a character, in the order
/// [`ABILITY_SEED_NAMES`] names them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SeedAbilityMods {
    pub values: [i64; 6],
}

/// Resolve a merged converted chain for one character.
///
/// `chain` is the record's own converted variables merged with whatever header/class/domain
/// chains the caller decided contribute (the merge policy is the caller's, unchanged).
/// `class_level_var` is the granting class's own level-variable name and `level` the
/// character's level in it. The returned map carries the seeds as well as the resolved targets,
/// because a record's own description arguments may name a seed directly.
///
/// A target that does not close -- an unreachable reference the corpus binds somewhere under
/// some condition, a shape the converter refused, a division whose denominator is zero at this
/// level -- is simply ABSENT. Never guessed, never defaulted.
pub fn resolve_chain(
    chain: &ConvertedChain,
    class_level_var: &str,
    level: u8,
    ability_mods: SeedAbilityMods,
    defaults: &BTreeMap<String, i64>,
) -> BTreeMap<String, i64> {
    let mut out: BTreeMap<String, i64> = BTreeMap::new();
    let mut env: BTreeMap<VarId, Expr> = BTreeMap::new();
    let mut facts = CharacterFacts::default();

    for (i, (abbr, ability)) in ABILITY_SEED_NAMES.iter().enumerate() {
        env.insert(var_id(abbr), Expr::AbilityMod(*ability));
        facts.ability_mods[ability_seed_index(*ability)] = ability_mods.values[i];
        out.insert((*abbr).to_string(), ability_mods.values[i]);
    }
    // One class, one level: the only class fact this resolver's callers ever hold.
    const OWNER: &str = "__granting_class__";
    facts.class_levels = vec![(OWNER.to_string(), i64::from(level))];
    facts.level = i64::from(level);
    env.insert(var_id(class_level_var), Expr::ClassLevel(OWNER.to_string()));
    out.insert(class_level_var.to_string(), i64::from(level));
    if let Some(class_name) = class_level_var.strip_suffix("LVL") {
        env.insert(var_id(&class_level_call_key(class_name)), Expr::ClassLevel(OWNER.to_string()));
    }

    // Pass 1: substitute only what the chain itself binds, to a fixed point.
    let mut bound: BTreeMap<String, Expr> = BTreeMap::new();
    let mut progressed = true;
    let mut guard = 0;
    while progressed && guard < 16 {
        progressed = false;
        guard += 1;
        for (name, var) in chain {
            if bound.contains_key(name) || out.contains_key(name) {
                continue;
            }
            let folded = substitute(&var.expr, &env);
            if is_closed(&folded) {
                env.insert(var_id(name), folded.clone());
                bound.insert(name.clone(), folded);
                progressed = true;
            }
        }
    }

    // Pass 2: a reference the corpus binds NOWHERE takes its declared baseline.
    let mut defaulted: BTreeSet<VarId> = BTreeSet::new();
    let mut progressed = true;
    let mut guard = 0;
    while progressed && guard < 16 {
        progressed = false;
        guard += 1;
        for (name, var) in chain {
            if bound.contains_key(name) || out.contains_key(name) {
                continue;
            }
            let folded = substitute(&var.expr, &env);
            if is_closed(&folded) {
                env.insert(var_id(name), folded.clone());
                bound.insert(name.clone(), folded);
                progressed = true;
                continue;
            }
            for referenced in &var.refs {
                let id = var_id(referenced);
                if env.contains_key(&id) || defaulted.contains(&id) {
                    continue;
                }
                let Some(default_value) = defaults.get(referenced) else { continue };
                let Ok(as_i32) = i32::try_from(*default_value) else { continue };
                env.insert(id.clone(), Expr::Const(as_i32));
                defaulted.insert(id);
                progressed = true;
            }
        }
    }

    for (name, expr) in &bound {
        if divides_by_zero(expr, &facts) {
            continue;
        }
        out.insert(name.clone(), evaluate_expr_from_facts(expr, &facts).trunc());
    }
    out
}

/// Evaluate ONE converted variable against a flat `name -> value` environment.
///
/// The shape a `%N` description argument needs: its converted form is a standalone expression,
/// and the caller has already resolved the seeds and whatever header chain it merged in. Returns
/// `None` -- never a guess -- when a reference is unbound or a denominator is zero, which is
/// exactly what the interpreter this replaces did with the same input.
pub fn evaluate_with_bindings(var: &ConvertedVar, bindings: &BTreeMap<String, i64>) -> Option<i64> {
    let mut env: BTreeMap<VarId, Expr> = BTreeMap::new();
    for (name, value) in bindings {
        let as_i32 = i32::try_from(*value).ok()?;
        env.insert(var_id(name), Expr::Const(as_i32));
    }
    let folded = substitute(&var.expr, &env);
    if !is_closed(&folded) {
        return None;
    }
    let facts = CharacterFacts::default();
    if divides_by_zero(&folded, &facts) {
        return None;
    }
    Some(evaluate_expr_from_facts(&folded, &facts).trunc())
}

fn ability_seed_index(a: Ability) -> usize {
    match a {
        Ability::Str => 0,
        Ability::Dex => 1,
        Ability::Con => 2,
        Ability::Int => 3,
        Ability::Wis => 4,
        Ability::Cha => 5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn var(expr: Expr, refs: &[&str]) -> ConvertedVar {
        ConvertedVar { expr, refs: refs.iter().map(|s| (*s).to_string()).collect() }
    }

    #[test]
    fn a_single_hop_division_chain_resolves_to_the_level_scaled_number() {
        // `AssassinDeathAttackDC | 10+(AssassinLVL/2)+INT`
        let mut chain = BTreeMap::new();
        chain.insert(
            "AssassinDeathAttackDC".to_string(),
            var(
                Expr::Sum(vec![
                    Expr::Const(10),
                    Expr::Div(
                        Box::new(Expr::Var(var_id("AssassinLVL"))),
                        Box::new(Expr::Const(2)),
                    ),
                    Expr::Var(var_id("INT")),
                ]),
                &["AssassinLVL", "INT"],
            ),
        );
        let mods = SeedAbilityMods { values: [0, 0, 0, 3, 0, 0] };
        let out = resolve_chain(&chain, "AssassinLVL", 7, mods, &BTreeMap::new());
        assert_eq!(out.get("AssassinDeathAttackDC"), Some(&16), "10 + 7/2 + 3, truncated once");
        assert_eq!(out.get("AssassinLVL"), Some(&7), "the seed itself is reported");
        assert_eq!(out.get("INT"), Some(&3));
    }

    #[test]
    fn a_two_hop_chain_resolves_through_its_intermediate() {
        let mut chain = BTreeMap::new();
        chain.insert(
            "PoolLVL".to_string(),
            var(Expr::Var(var_id("RogueLVL")), &["RogueLVL"]),
        );
        chain.insert(
            "PoolDC".to_string(),
            var(
                Expr::Sum(vec![
                    Expr::Const(10),
                    Expr::Div(Box::new(Expr::Var(var_id("PoolLVL"))), Box::new(Expr::Const(2))),
                ]),
                &["PoolLVL"],
            ),
        );
        let out =
            resolve_chain(&chain, "RogueLVL", 11, SeedAbilityMods::default(), &BTreeMap::new());
        assert_eq!(out.get("PoolLVL"), Some(&11));
        assert_eq!(out.get("PoolDC"), Some(&15));
    }

    #[test]
    fn a_reference_the_corpus_binds_somewhere_is_refused_not_defaulted() {
        let mut chain = BTreeMap::new();
        chain.insert(
            "Target".to_string(),
            var(Expr::Var(var_id("BoundElsewhere")), &["BoundElsewhere"]),
        );
        // `defaults` carries only names the corpus binds NOWHERE; this one is absent from it.
        let out =
            resolve_chain(&chain, "RogueLVL", 10, SeedAbilityMods::default(), &BTreeMap::new());
        assert!(!out.contains_key("Target"), "an unreachable reference is never guessed");
    }

    #[test]
    fn a_reference_the_corpus_binds_nowhere_takes_its_declared_baseline() {
        let mut chain = BTreeMap::new();
        chain.insert(
            "Target".to_string(),
            var(
                Expr::Sum(vec![Expr::Const(1), Expr::Var(var_id("NeverBound"))]),
                &["NeverBound"],
            ),
        );
        let mut defaults = BTreeMap::new();
        defaults.insert("NeverBound".to_string(), 0i64);
        let out = resolve_chain(&chain, "RogueLVL", 10, SeedAbilityMods::default(), &defaults);
        assert_eq!(out.get("Target"), Some(&1));
    }

    #[test]
    fn a_division_by_zero_refuses_rather_than_answering_zero() {
        let mut chain = BTreeMap::new();
        chain.insert(
            "Target".to_string(),
            var(
                Expr::Div(Box::new(Expr::Const(4)), Box::new(Expr::Var(var_id("Zeroed")))),
                &["Zeroed"],
            ),
        );
        let mut defaults = BTreeMap::new();
        defaults.insert("Zeroed".to_string(), 0i64);
        let out = resolve_chain(&chain, "RogueLVL", 10, SeedAbilityMods::default(), &defaults);
        assert!(!out.contains_key("Target"), "the interpreter refused this; so does the fold");
    }

    #[test]
    fn a_reference_that_differs_only_in_case_binds_the_same_variable() {
        // SD-35 `AT-35-E6-001` cycle 4's own finding. The interpreter this fold replaces looked a
        // name up by exact bytes, so a corpus row writing `RogueLvl` where the class declares
        // `RogueLVL` silently answered 0. Variable names are case-insensitive in the rule source
        // and the converted package mints one id per case-folded name, so they bind.
        let mut chain = BTreeMap::new();
        chain.insert(
            "HiddenBladeBonus".to_string(),
            var(
                Expr::Div(Box::new(Expr::Var(var_id("RogueLvl"))), Box::new(Expr::Const(2))),
                &["RogueLvl"],
            ),
        );
        let out =
            resolve_chain(&chain, "RogueLVL", 10, SeedAbilityMods::default(), &BTreeMap::new());
        assert_eq!(out.get("HiddenBladeBonus"), Some(&5), "RogueLvl IS RogueLVL");
    }

    #[test]
    fn a_class_level_call_naming_another_class_stays_unbound() {
        let mut chain = BTreeMap::new();
        chain.insert(
            "Same".to_string(),
            var(Expr::Var(var_id(&class_level_call_key("Summoner"))), &[]),
        );
        chain.insert(
            "Other".to_string(),
            var(Expr::Var(var_id(&class_level_call_key("Wizard"))), &[]),
        );
        let out =
            resolve_chain(&chain, "SummonerLVL", 9, SeedAbilityMods::default(), &BTreeMap::new());
        assert_eq!(out.get("Same"), Some(&9));
        assert!(!out.contains_key("Other"));
    }
}
