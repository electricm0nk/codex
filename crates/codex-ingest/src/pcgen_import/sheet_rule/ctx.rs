//! Conversion context: the corpus-wide indexes every family reads (rule ids by category+key,
//! class/skill/race/template ids, fact declarers, variable ids) and the per-record state
//! (own rows, own variable contributions, the choice the record offers, the owning class,
//! refusals and defects collected while converting).

use std::collections::{BTreeMap, BTreeSet};

use super::closure::{Closure, PinnedTree, RowRef};
use codex::rules_core::sheet_rule::{Applies, BonusType, ClassId, Expr, RuleId, SkillId, StackMode, VarId};

/// The token shapes that refuse the whole RECORD, so no `SheetRule` is written for it at all.
///
/// This is the deliberate REFUSE-by-shape set of `decisions.md` §15 ruling R2: the record's
/// own value is the redacted field, so there is no licensed remainder to print. Every other
/// unlowerable term is a degradation (see [`RecordCtx::refuse_under`]) — the record still
/// reaches the sheet, printing its words.
///
/// `no_corpus_record` / `no_source_row` are handled before conversion begins
/// (`super::run`): a record with no source row has nothing to convert, degraded or not.
pub const RECORD_REFUSAL_SHAPES: &[&str] = &["BONUS:VAR ([redacted PI] value)"];

pub fn is_record_refusal(shape: &str) -> bool {
    RECORD_REFUSAL_SHAPES.contains(&shape)
}

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
    codex::rules_core::sheet_rule::var_id(name)
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
    /// SD-36 F1c-3 (D3): the base class KEY, when the corpus also files this record's source
    /// row as a CLASS (`data/corpus/<book>/class/`, a `base_class_key`) that no inventory unit
    /// joined, because the oracle declares no `CLASS:` object for it -- a `CATEGORY:CLASS`
    /// selection ability taken on its base class (Pathfinder Unchained's four). The converter
    /// writes the class principal it stands for ([`super::class_selection_principal`]).
    pub class_selection_of: Option<String>,
    /// The inventory unit joined to a shipped corpus record.
    pub joined: bool,
}

/// Corpus-wide lookup tables.
#[derive(Default)]
pub struct CorpusIndex {
    pub records: Vec<RecordRef>,
    /// `(CATEGORY upper, KEY upper)` -> rule id (first wins; ambiguity recorded in
    /// [`CorpusIndex::ambiguous_cat_key`]).
    pub by_cat_key: BTreeMap<(String, String), RuleId>,
    /// `(CATEGORY upper, NAME upper)` -> rule id (first wins; ambiguity recorded in
    /// [`CorpusIndex::ambiguous_cat_name`]).
    pub by_cat_name: BTreeMap<(String, String), RuleId>,
    /// Every `(CATEGORY upper, KEY upper)` pair `build_index` saw declared by more than one
    /// DIFFERENT record id -- `by_cat_key` itself still holds the first one (first-wins, so a
    /// direct-category hit that happens to be unambiguous elsewhere keeps working), but
    /// [`resolve_rule_in_checked`]'s parent retry consults this set so it can refuse to guess
    /// among them (F1 adversarial finding 4).
    pub ambiguous_cat_key: BTreeSet<(String, String)>,
    /// The `by_cat_name` twin of [`CorpusIndex::ambiguous_cat_key`].
    pub ambiguous_cat_name: BTreeSet<(String, String)>,
    /// SD-36 F3b2b: every candidate record id of each [`CorpusIndex::ambiguous_cat_key`] pair,
    /// in index order.
    pub cat_key_candidates: BTreeMap<(String, String), Vec<RuleId>>,
    /// The `by_cat_name` twin of [`CorpusIndex::cat_key_candidates`].
    pub cat_name_candidates: BTreeMap<(String, String), Vec<RuleId>>,
    /// SD-36 F3b2b, the standing supersession ruling (operator 2026-08-16, `decisions.md` §12):
    /// an ambiguous pair whose candidates are printings of ONE object (`super::reprint`) ->
    /// the newest printing. A pair missing here stays ambiguous.
    pub reprint_newest_key: BTreeMap<(String, String), RuleId>,
    /// The `by_cat_name` twin of [`CorpusIndex::reprint_newest_key`].
    pub reprint_newest_name: BTreeMap<(String, String), RuleId>,
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
    /// rule id -> the record's ACCUMULATED `(CATEGORY, TYPE tags)` over its whole closure (base,
    /// `.COPY=` base, level lines, `.MOD` rows), folded exactly as the converter folds them into
    /// the converted rule's `pool`/`tags` (`convert::accumulated_facets`). What an
    /// `ABILITY:<category>|...|TYPE=<tag>` grant selects on (SD-36 Epic F1c-1): PCGen checks the
    /// ability's accumulated TYPE, including tags a `.MOD` row adds.
    pub facets: BTreeMap<RuleId, (String, Vec<String>)>,
    /// SD-36 F1c-5 (D8): every variable pool a record of the index fills, by pool variable
    /// (upper), with its members (`pool_pick::filled_pools`).
    pub filled_pools: BTreeMap<String, (super::pool_pick::VariablePool, super::pool_pick::PoolMembers)>,
}

/// Resolve `(category, name)` to a rule id: KEY-exact join first, then display-name-exact join,
/// both under `category`. If both miss AND `category` is a CHILD `ABILITYCATEGORY` (per
/// `PinnedTree::ability_category_parent`, built from the tree's own `ABILITYCATEGORY:` rows —
/// never a hardcoded list), retry the same two EXACT lookups under the category's PARENT. This
/// closes SD-36 Epic F mechanism A (`epic-f-class-completion.md` §1/§3.1 item 1): a reference
/// naming a child ability category (e.g. `Wizard Class Feature`) whose target record is indexed
/// under the parent category the corpus actually uses (`Special Ability`).
///
/// The retry never falls back to a name-similarity guess — it is the same two exact indices the
/// direct lookup already uses, just against a different category string, so a parent hit is
/// always the exact same record PCGen's own tool would resolve to.
///
/// **Ambiguity rule (parent category):** a child category name that `PinnedTree::build_indexes`
/// found declared under two DIFFERENT parents anywhere in the tree is left OUT of
/// `ability_category_parent` entirely. Such a name therefore has no parent to retry here, and a
/// miss under it stays a miss — exactly like an unmapped category — so the caller's own defect
/// path (`resolve_holdable_rule`) records it as an unresolved reference rather than the resolver
/// guessing which of several candidate parents is the right one.
///
/// **Ambiguity rule (parent-retry TARGET, F1 adversarial finding 4):** the parent category
/// itself may be unambiguous while the `(parent, key-or-name)` pair the retry looks up under it
/// still names more than one converted record (two unrelated records that merely share a display
/// name or key under the same broad category, e.g. `Special Ability`). Unlike the direct lookup
/// (whose own ambiguity is vanishingly rare and, where it exists, is left as today's first-wins
/// behavior to avoid widening this fix's blast radius), the retry NEVER guesses here: see
/// [`resolve_rule_in_checked`].
pub fn resolve_rule_in(tree: &PinnedTree, index: &CorpusIndex, category: &str, name: &str) -> Option<RuleId> {
    match resolve_rule_in_checked(tree, index, category, name) {
        RuleLookup::Found(id) => Some(id),
        RuleLookup::Ambiguous | RuleLookup::Missing => None,
    }
}

/// The three ways [`resolve_rule_in_checked`] can end: a real, unambiguous target; a target that
/// exists but names more than one candidate record (never resolved, never guessed); or no target
/// at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleLookup {
    Found(RuleId),
    Ambiguous,
    Missing,
}

/// [`resolve_rule_in`]'s full-detail twin: distinguishes an ambiguous parent-retry target from a
/// plain miss, so the caller (`resolve_holdable_rule`) can record the two as separate, correctly
/// named defect kinds instead of folding an ambiguous match into the generic
/// `unresolved-references` bucket (F1 adversarial finding 4).
pub fn resolve_rule_in_checked(tree: &PinnedTree, index: &CorpusIndex, category: &str, name: &str) -> RuleLookup {
    let cat = category.trim().to_ascii_uppercase();
    let n = name.trim().to_ascii_uppercase();
    if let Some(id) = lookup_exact(index, &cat, &n) {
        return RuleLookup::Found(id);
    }
    let Some(parent) = tree.ability_category_parent.get(&cat) else {
        return RuleLookup::Missing;
    };
    // KEY takes priority over NAME, the same order `lookup_exact` uses -- an ambiguous KEY hit
    // must never silently fall through to the NAME map, which could return an entirely
    // different (and equally unchosen) candidate.
    let key_pair = (parent.clone(), n.clone());
    // An ambiguous pair whose candidates are printings of one object resolves to the newest
    // printing (SD-36 F3b2b, `decisions.md` §12); any other ambiguous pair stays ambiguous.
    if index.by_cat_key.contains_key(&key_pair) {
        return if index.ambiguous_cat_key.contains(&key_pair) {
            index.reprint_newest_key.get(&key_pair).map_or(RuleLookup::Ambiguous, |id| RuleLookup::Found(id.clone()))
        } else {
            RuleLookup::Found(index.by_cat_key[&key_pair].clone())
        };
    }
    if index.by_cat_name.contains_key(&key_pair) {
        return if index.ambiguous_cat_name.contains(&key_pair) {
            index.reprint_newest_name.get(&key_pair).map_or(RuleLookup::Ambiguous, |id| RuleLookup::Found(id.clone()))
        } else {
            RuleLookup::Found(index.by_cat_name[&key_pair].clone())
        };
    }
    RuleLookup::Missing
}

fn lookup_exact(index: &CorpusIndex, category: &str, name: &str) -> Option<RuleId> {
    if let Some(id) = index.by_cat_key.get(&(category.to_string(), name.to_string())) {
        return Some(id.clone());
    }
    index.by_cat_name.get(&(category.to_string(), name.to_string())).cloned()
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
    /// Token types that refused this RECORD outright (per token SHAPE, never per unit).
    /// Only [`RECORD_REFUSAL_SHAPES`] land here; every other unlowerable term degrades.
    pub refusals: BTreeSet<String>,
    /// Token shapes whose term the converter could not lower. The record still converts and
    /// prints its words (`decisions.md` §1 form 3: a term the character does not settle stays
    /// as words) instead of vanishing from the sheet entirely.
    pub degradations: BTreeSet<String>,
    /// Degradation shape -> the token type(s) it arose under, the census twin of
    /// [`RecordCtx::refusal_under`].
    pub degraded_under: BTreeMap<String, BTreeSet<String>>,
    /// The token census (SD-35 AT-35-E2-004): the mapping-table row key of every token this
    /// record's closure carried -- `unmapped:<HEAD>` / `BONUS:<SUB>` when the table has no row.
    pub tokens: BTreeSet<String>,
    /// Refusal shape -> the token type(s) it arose under, so `token_coverage.py` counts
    /// "units refused because of this token" from the converter's own reading.
    pub refusal_under: BTreeMap<String, BTreeSet<String>>,
    /// The census key of the row being converted right now, so a term that degrades deep
    /// inside the row (a prose argument, SD-35 `AT-35-E6-003` cycle 6) records the same
    /// `under` the row-level `Err` path recorded.
    pub current_under: Option<String>,
    /// SD-36 Epic E CONV-05: a per-occurrence sequence number for the row being converted
    /// right now, distinct from `current_under` -- two DIFFERENT rows on the same record often
    /// share one generic census key (every `TEMPBONUS` row's `under` is the literal string
    /// `"TEMPBONUS"` regardless of its own sub-shape), so `under` alone cannot tell which of
    /// several same-headed rows actually degraded. `Line::seq` tags a pushed line with this
    /// value; [`RecordCtx::current_seq_degraded`] tells the row loop whether THIS occurrence
    /// degraded, however deep the degrading call was (a direct `Err`, or a nested prose-slot
    /// fallback that still returns `Ok`).
    pub current_seq: Option<usize>,
    /// Set by [`RecordCtx::refuse_under`] whenever the current occurrence (`current_seq`)
    /// degrades, so only the lines THAT occurrence pushed are wiped at assembly, never a
    /// sibling occurrence's already-successful line.
    pub current_seq_degraded: bool,
    /// Defect-list lines (`_defects/`), keyed by defect kind.
    pub defects: BTreeMap<String, Vec<String>>,
    /// Variable ids this record referenced through `Expr::Var`, with their source names.
    pub var_names: BTreeMap<VarId, String>,
    /// The same ids with the source name's ORIGINAL case, which is what
    /// `sheet_rule::display_label` spaces into the printed label. `var_names` is upper-cased
    /// because every index keys on the folded name; a label needs the case back.
    pub var_labels: BTreeMap<VarId, String>,
    /// Names currently being inlined (cycle guard).
    inlining: Vec<String>,
    /// Fields omitted for product identity: declared / term hits.
    pub pi_declared: Vec<String>,
    pub pi_term_hits: Vec<String>,
    /// SD-36 F3b2b: names read as 0 under oracle semantics (declared nowhere in the pinned
    /// tree, not a built-in term; [`super::oracle_terms`]), sorted, deduplicated.
    pub undeclared_in_pinned_tree: BTreeSet<String>,
}

/// A class record's own class id: its inventory unit's slug (`adventurers_guide:class:
/// golden_legionnaire` -> `golden_legionnaire`), the id the census and every seed name the class
/// by. Never `slug(key)`: a class whose name is product identity ships a codex-named placeholder
/// KEY (`Codex-Named Unit (class_..._lst_136)`), and its own level-line grants and class-level
/// expressions keyed under that placeholder's slug were reachable by no seed (SD-36 F1c-3; 21 of
/// 185 class units). For every other class the two are identical (185 of 185 inventory units:
/// the unit slug equals the slug of its corpus key).
pub fn own_class_id(record: &RecordRef) -> ClassId {
    record.id.rsplit(':').next().filter(|s| !s.is_empty()).map(str::to_string).unwrap_or_else(|| slug(&record.key))
}

impl<'a> RecordCtx<'a> {
    pub fn new(tree: &'a PinnedTree, index: &'a CorpusIndex, record: &'a RecordRef, closure: &'a Closure) -> Self {
        let owning_class = match record.kind.as_str() {
            "class" => Some(own_class_id(record)),
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
            degradations: BTreeSet::new(),
            degraded_under: BTreeMap::new(),
            tokens: BTreeSet::new(),
            refusal_under: BTreeMap::new(),
            current_under: None,
            current_seq: None,
            current_seq_degraded: false,
            defects: BTreeMap::new(),
            var_names: BTreeMap::new(),
            var_labels: BTreeMap::new(),
            inlining: Vec::new(),
            pi_declared: Vec::new(),
            pi_term_hits: Vec::new(),
            undeclared_in_pinned_tree: BTreeSet::new(),
        }
    }

    pub fn refuse(&mut self, token_type: impl Into<String>) {
        let tt = token_type.into();
        if is_record_refusal(&tt) {
            self.refusals.insert(tt);
        } else {
            self.degradations.insert(tt);
        }
    }

    /// Record that this record's closure carries a token of type `token_type` (census only).
    pub fn carry(&mut self, token_type: impl Into<String>) {
        self.tokens.insert(token_type.into());
    }

    /// Refuse under `shape`, recording the token type (`under`) it arose under.
    ///
    /// SD-35 `AT-35-E3-001`: a token the table does not map, or whose formula/gate the
    /// converter cannot lower, no longer deletes the whole record from the sheet. Only the
    /// deliberate REFUSE-by-shape set ([`RECORD_REFUSAL_SHAPES`], `decisions.md` §15 R2)
    /// refuses the record; every other shape is a **term-level degradation** — the token
    /// contributes no number, the record converts, and its principal value becomes
    /// `SheetValue::Text` so the sheet prints the rule's own words rather than a number the
    /// converter only partly read (`decisions.md` §1 form 3, and the standing no-carve-outs
    /// ruling: "the engine cannot model X" is a number to report, never an exemption).
    pub fn refuse_under(&mut self, under: &str, shape: impl Into<String>) {
        let shape = shape.into();
        if is_record_refusal(&shape) {
            self.refusal_under.entry(shape.clone()).or_default().insert(under.to_string());
            self.refusals.insert(shape);
        } else {
            self.degraded_under.entry(shape.clone()).or_default().insert(under.to_string());
            self.degradations.insert(shape);
            self.current_seq_degraded = true;
        }
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

    /// Resolve `(category, name)` to a rule id. Delegates to [`resolve_rule_in`] (a free
    /// function so the parent-category retry has a unit-test surface that does not need a whole
    /// `RecordCtx`).
    pub fn resolve_rule(&self, category: &str, name: &str) -> Option<RuleId> {
        resolve_rule_in(self.tree, self.index, category, name)
    }

    /// [`RecordCtx::resolve_rule`]'s full-detail twin (F1 adversarial finding 4): distinguishes
    /// an ambiguous parent-retry target from a plain miss, so `resolve_holdable_rule` can record
    /// the two as separate, correctly named defect kinds.
    pub fn resolve_rule_checked(&self, category: &str, name: &str) -> RuleLookup {
        resolve_rule_in_checked(self.tree, self.index, category, name)
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
            // SD-36 F3b2b: a name the oracle cannot read as a built-in term evaluates as 0 there
            // (`oracle_terms.rs`), so `Const(0)` IS the oracle's reading and the closure is
            // complete: an informational row, and a provenance note. A name the oracle might
            // read as a built-in term, or a token that is not one plain identifier, keeps the
            // closure defect (its oracle value is not proved to be 0).
            if super::oracle_terms::undeclared_reads_as_zero(name) {
                self.defect("undeclared-in-pinned-tree", format!("{}: {name}", self.record.id));
                self.undeclared_in_pinned_tree.insert(name.trim().to_string());
            } else {
                self.defect("undefined-variables", format!("{}: {name}", self.record.id));
            }
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
        self.var_labels.entry(id.clone()).or_insert_with(|| name.trim().to_string());
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
                    gate_terms.push(Applies::Compare { lhs: Expr::ClassLevel(cls.clone()), op: codex::rules_core::sheet_rule::Cmp::Gte, rhs: Expr::Const(l as i32) });
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
    use codex::rules_core::sheet_rule::STACKING_TYPES;
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

    /// A minimal `PinnedTree` for `resolve_rule_in` unit tests: no files on disk, only the
    /// `ability_category_parent` map under test (everything else `resolve_rule_in` never reads).
    fn tree_with_parent(pairs: &[(&str, &str)]) -> PinnedTree {
        PinnedTree {
            root: std::path::PathBuf::new(),
            book_paths: BTreeMap::new(),
            source_dates: BTreeMap::new(),
            files: Vec::new(),
            mod_index: BTreeMap::new(),
            base_index: BTreeMap::new(),
            keyed_index: BTreeMap::new(),
            define_index: BTreeMap::new(),
            bonus_var_index: BTreeMap::new(),
            class_rows: BTreeMap::new(),
            level_lines: BTreeMap::new(),
            fact_index: BTreeMap::new(),
            pfs_base_keys: BTreeSet::new(),
            ability_category_parent: pairs.iter().map(|(c, p)| (c.to_string(), p.to_string())).collect(),
            ability_category_type: BTreeMap::new(),
            ability_category_pool: BTreeMap::new(),
        }
    }

    /// SD-36 Epic F1-1, mechanism A (`epic-f-class-completion.md` §1/§3.1 item 1): a reference
    /// naming a CHILD ability category resolves through its declared PARENT when the exact
    /// `(category, key)` join misses under the child name itself.
    #[test]
    fn a_child_category_reference_resolves_through_its_parent() {
        let tree = tree_with_parent(&[("WIZARD CLASS FEATURE", "SPECIAL ABILITY")]);
        let mut index = CorpusIndex::default();
        index.by_cat_key.insert(("SPECIAL ABILITY".into(), "WIZARD ~ WEAPON AND ARMOR PROFICIENCY".into()), "core_rulebook:class_feature:wizard_weapon_and_armor_proficiency".into());
        let id = resolve_rule_in(&tree, &index, "Wizard Class Feature", "Wizard ~ Weapon and Armor Proficiency");
        assert_eq!(id.as_deref(), Some("core_rulebook:class_feature:wizard_weapon_and_armor_proficiency"));
    }

    /// A hit under the reference's OWN category must win before any parent retry is even
    /// attempted -- the retry is a fallback for a miss, never a second candidate to choose among.
    #[test]
    fn a_direct_match_never_needs_the_parent_retry() {
        let tree = tree_with_parent(&[("CHILD", "PARENT")]);
        let mut index = CorpusIndex::default();
        index.by_cat_key.insert(("CHILD".into(), "X".into()), "book:kind:direct".into());
        index.by_cat_key.insert(("PARENT".into(), "X".into()), "book:kind:wrong".into());
        let id = resolve_rule_in(&tree, &index, "Child", "X");
        assert_eq!(id.as_deref(), Some("book:kind:direct"), "the direct hit wins; the parent retry must not even be consulted");
    }

    /// An ambiguous child category (one `build_indexes` left OUT of `ability_category_parent`
    /// because its declarations disagreed on the parent -- proven directly in closure.rs's own
    /// tests) has no parent to retry here: the miss stays a miss, which is what lets the caller's
    /// normal defect path (`resolve_holdable_rule`) record it, rather than the resolver guessing
    /// between candidate parents.
    #[test]
    fn an_ambiguous_parent_match_stays_a_defect() {
        let tree = tree_with_parent(&[]); // no entry at all -- exactly what an ambiguous name leaves behind
        let mut index = CorpusIndex::default();
        index.by_cat_key.insert(("SPECIAL ABILITY".into(), "SOME TARGET".into()), "book:kind:some_target".into());
        let id = resolve_rule_in(&tree, &index, "Ambiguous Category", "Some Target");
        assert!(id.is_none(), "an unmapped (including formerly-ambiguous) category must miss, never guess a parent");
    }

    /// F1 adversarial finding 4: unlike an ambiguous CATEGORY (above), an ambiguous parent-retry
    /// TARGET -- two DIFFERENT records both claiming the same `(parent, key-or-name)` pair -- is
    /// not caught by `ability_category_parent` at all; `by_cat_key`'s own `.or_insert` silently
    /// kept only the first. `resolve_rule_in` must miss (never guess), and
    /// `resolve_rule_in_checked` must report `Ambiguous`, not `Missing`, so the caller can name it
    /// as its own defect kind rather than folding it into a plain unresolved reference.
    #[test]
    fn an_ambiguous_parent_retry_target_is_reported_distinctly_from_a_plain_miss() {
        let tree = tree_with_parent(&[("CHILD", "PARENT")]);
        let mut index = CorpusIndex::default();
        // Two different record ids both loaded under the SAME (parent, key) pair -- `build_index`
        // keeps the first in `by_cat_key` (unchanged, first-wins) but must ALSO mark the pair
        // ambiguous.
        index.by_cat_key.insert(("PARENT".into(), "SMITE GOOD".into()), "book:kind:first_loaded".into());
        index.ambiguous_cat_key.insert(("PARENT".into(), "SMITE GOOD".into()));

        assert!(
            resolve_rule_in(&tree, &index, "Child", "Smite Good").is_none(),
            "resolve_rule_in must never silently resolve to the first-loaded candidate of an ambiguous target"
        );
        assert_eq!(
            resolve_rule_in_checked(&tree, &index, "Child", "Smite Good"),
            RuleLookup::Ambiguous,
            "resolve_rule_in_checked must distinguish this from a plain Missing"
        );
    }

    /// A direct-category hit is unaffected by an ambiguous entry that exists only under some
    /// OTHER category -- ambiguity is scoped to the exact `(category, key/name)` pair, not the
    /// key/name alone.
    #[test]
    fn ambiguity_under_one_category_does_not_leak_into_a_different_categorys_lookup() {
        let tree = tree_with_parent(&[]);
        let mut index = CorpusIndex::default();
        index.by_cat_key.insert(("OTHER CATEGORY".into(), "X".into()), "book:kind:direct".into());
        index.ambiguous_cat_key.insert(("SOME OTHER PAIR".into(), "X".into()));

        let id = resolve_rule_in(&tree, &index, "Other Category", "X");
        assert_eq!(id.as_deref(), Some("book:kind:direct"));
    }

    #[test]
    fn a_category_with_no_known_parent_at_all_simply_misses() {
        let tree = tree_with_parent(&[]);
        let index = CorpusIndex::default();
        assert!(resolve_rule_in(&tree, &index, "Nothing Here", "X").is_none());
    }

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
