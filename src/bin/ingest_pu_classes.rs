//! SD-27 — ingests **Pathfinder Unchained's four Unchained classes** and
//! their class features from
//! `pathfinder_unchained/pu_abilities_class.lst` into Shape B v1 records
//! under `data/corpus/pathfinder_unchained/{class,class_feature}/`.
//!
//! Run with `cargo run --bin ingest_pu_classes`. `PCGEN_CORPUS_ROOT` may
//! point at a local PCGen `data/` checkout; it defaults to
//! `$HOME/workspace/repos/pcgen/data`. `CODEX_INGESTED_AT` pins the
//! `ingested_at` stamp so a run can be reproduced byte-for-byte.
//!
//! # Scope finding: PU declares no `CLASS` object at all
//!
//! `_pathfinder_unchained.pcc` carries **no `CLASS:` line** — verified by
//! reading the `.pcc`, and by `grep -rn '^CLASS:' | grep -i unchained`
//! over the whole PCGen data tree returning nothing. The only `CLASS:`
//! mention anywhere in the book is a **commented-out** line in
//! `_pfs/_.pcc` naming a `pfs_pu_classes.lst` that does not exist on
//! disk.
//!
//! Each "Unchained class" is instead an `ABILITY` in `CATEGORY:CLASS`
//! that plugs into the *base* class's own selection pool. The pool is
//! declared by `core_rulebook/cr_abilitycategories.lst`
//! (`ABILITYCATEGORY:Monk Class Selection ... POOL:Pool_Monk_Class_Selection`),
//! the base class seeds it (`cr_abilities_class.lst`'s `Monk` row
//! `DEFINE:Pool_Monk_Class_Selection|0`), and PU opens it with a one-line
//! `.MOD` (`CATEGORY=Class|Monk.MOD  BONUS:VAR|Pool_Monk_Class_Selection|1`).
//! The variant then swaps the base class's features for its own via
//! `CATEGORY=Class|<Class> ~ Unchained Class.MOD  ABILITY:...` grant rows.
//!
//! **This is the ARG-races shape, with the opposite verdict.** For ARG,
//! the chassis lived in `core_essentials`, which is out of project scope,
//! so 19 of its 37 races could not be ingested without inventing
//! provenance (`decisions.md §25`). Here the chassis lives in
//! `core_rulebook` (Barbarian, Monk, Rogue) and `advanced_players_guide`
//! (Summoner) — **both already ingested in this repo**
//! (`data/corpus/core_rulebook/class/{barbarian,monk,rogue}.json`,
//! `data/corpus/advanced_players_guide/class/summoner.json`). Nothing here
//! depends on an unregistered book, so the ingest is in scope. The base
//! class each variant attaches to is recorded on the record
//! (`base_class_key` / `base_class_book`) rather than left implicit, and a
//! variant whose base class is *not* already ingested is refused rather
//! than written — the same rule §25.3 applies to races.
//!
//! # What is deliberately NOT written
//!
//! PU also declares large *selectable-option* pools these classes draw
//! from — Unchained Rage Powers, Ki Powers, Style Strikes, Rogue Talents
//! and Advanced Talents, Monk Bonus Feats, and the whole Unchained Eidolon
//! tree. Those are options a feature *offers*, not features the class
//! *has*, and each needs its own content-kind directory. This binary
//! counts them and prints the tally; it writes none of them. Inventing a
//! home for them under `class_feature/` would misfile them permanently.
//!
//! # Provenance honesty
//!
//! `SOURCEPAGE:` is mapped to `None` whenever the value is PCGen's `p.xx`
//! placeholder (`decisions.md §27.2`), so a populated `source_page`
//! always means a real page. The raw token is preserved verbatim in
//! `raw_tokens` either way. (This file happens to carry zero `p.xx` —
//! verified, and the guard still runs so the property is *checked*
//! rather than assumed.)
//!
//! # `decisions.md §24` compliance
//!
//! No formula is interpreted. Three same-row literal reads happen, each
//! a transcription of a constant written on the row being read:
//!
//! 1. `PREVARGTEQ:<Class>_CFP_Level,7` → grant level `7`.
//! 2. `HITDIE:10|CLASS=Monk` on the applied `TEMPLATE:` → hit die `10`.
//! 3. `DESC:` `%N` substitution against same-row `DEFINE:`/`BONUS:VAR|`
//!    integer literals, ported unchanged from `ingest_races.rs`.
//!
//! The BAB/save columns are a fourth, narrower case: the clause value must
//! match `classlevel("<BaseClass>",...)` followed by an arithmetic tail,
//! and the only edit is substituting the literal token `level` for that
//! call, leaving the tail byte-identical. Any other shape yields `None`.
//! That is a single named substitution with a test pinning it, not an
//! evaluator.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use codex::rules_core::cache_gen::WiringClassIndex;
use codex::rules_core::pi_screening;
use codex::rules_core::shape_b_v1::{
    ClassFeatureCacheData, ClassFeatureGrant, ClassVariantCacheData, Completeness, CorpusRecordV1, CorpusSource,
    License, Population, RawBonusChain, RawToken,
};

/// The one source file this binary ingests, relative to the PCGen `data/`
/// root. The same string is written into every record's `source.path`.
const LST_RELATIVE: &str = "pathfinder/paizo/roleplaying_game/pathfinder_unchained/pu_abilities_class.lst";

/// PU's template file, read only for the one `HITDIE:` override Unchained
/// Monk applies.
const TEMPLATES_RELATIVE: &str = "pathfinder/paizo/roleplaying_game/pathfinder_unchained/pu_templates.lst";

/// The literal placeholder PCGen leaves where a real page citation
/// belongs (`decisions.md §27.2`). Never stored as a `source_page`.
const PLACEHOLDER_SOURCE_PAGE: &str = "p.xx";

/// One in-scope Unchained class: the base class it layers over, and the
/// corpus book that base class is already ingested under.
#[derive(Clone, Copy)]
struct VariantSpec {
    /// `TYPE:<base> Class Selection` component, and the base class's key.
    base_class_key: &'static str,
    /// Corpus book directory holding `class/<base>.json`.
    base_class_book: &'static str,
}

/// The four Unchained classes, each keyed by its `CATEGORY:CLASS` record
/// key. Barbarian/Monk/Rogue are Core Rulebook classes; Summoner is an
/// Advanced Player's Guide class, which is why PU's `.pcc` carries
/// `PRECAMPAIGN:1,INCLUDES=Advanced Player's Guide`.
const VARIANTS: &[(&str, VariantSpec)] = &[
    ("Barbarian ~ Unchained Class", VariantSpec { base_class_key: "Barbarian", base_class_book: "core_rulebook" }),
    ("Monk ~ Unchained Class", VariantSpec { base_class_key: "Monk", base_class_book: "core_rulebook" }),
    ("Rogue ~ Unchained Class", VariantSpec { base_class_key: "Rogue", base_class_book: "core_rulebook" }),
    (
        "Summoner ~ Unchained Class",
        VariantSpec { base_class_key: "Summoner", base_class_book: "advanced_players_guide" },
    ),
];

/// Heuristic OGL/PI screen (`docs/governance/ogl-pi-blacklist.md`), the same
/// bounded substring scan `ingest_races.rs` and `gen_book_cache.rs`
/// apply. Class features are pure game mechanics, so every record here is
/// expected to classify `OGL`; the screen exists so that expectation is
/// checked rather than assumed, and a hit fails the run loudly.
const PI_BLACKLIST_TERMS: &[&str] = &[
    "Iomedae", "Sarenrae", "Asmodeus", "Cayden Cailean", "Abadar", "Calistria", "Desna", "Erastil", "Gorum", "Gozreh",
    "Irori", "Lamashtu", "Nethys", "Norgorber", "Pharasma", "Rovagug", "Shelyn", "Torag", "Urgathoa", "Zon-Kuthon",
    "Golarion", "Absalom", "Cheliax", "Varisia", "Andoran", "Taldor", "Osirion", "Katapesh", "Ustalav", "Numeria",
    "Mwangi", "Tian Xia", "Avistan", "Garund", "Sarkoris", "Worldwound", "Vudra", "Kyonin", "Molthune", "Nidal",
    "Nirmathas", "Qadira", "Razmiran", "Rahadoum", "Galt", "Isger", "Lastwall", "Brevoy", "Druma", "Irrisen",
    "Jalmeray", "Thuvia", "Geb", "Nex",
];

/// `DESC:` prose that is a **PCGen application instruction**, not game
/// content, and must never reach a player of *this* app: it tells the
/// reader to save, close and reopen the character in a program they are
/// not using. Excluded from `description` by exact match; the raw token is
/// still preserved verbatim in `raw_tokens`.
///
/// This is an explicit one-entry allowlist rather than a heuristic on
/// purpose — a heuristic that silently swallowed real rules text would be
/// exactly the failure this repo keeps getting burned by.
const PCGEN_APPLICATION_INSTRUCTION_DESCS: &[&str] = &[
    "IMPORTANT!!! After selecting the UNCHAINED SUMMONER. You Need to SAVE the character. Close the Character, and then Re-Open to get the correct Unchained Summoner Spell List.",
];

// ---------------------------------------------------------------------
// LST parsing primitives (same shape as `ingest_races.rs`)
// ---------------------------------------------------------------------

/// One real (non-comment, non-blank) LST row: its 1-indexed line number
/// and its non-empty tab-delimited fields. PrettyLST pads columns with
/// runs of tabs, so empty fields are structural padding carrying no
/// content.
#[derive(Debug, Clone)]
struct LstRow {
    line_no: u32,
    fields: Vec<String>,
}

impl LstRow {
    /// The unkeyed first column — the record's display name (or, on a
    /// `.MOD` row, the `CATEGORY=...|<target>.MOD` selector).
    fn name(&self) -> &str {
        self.fields.first().map(String::as_str).unwrap_or_default()
    }

    /// Every keyed token after the name column, as `(key, value)` split on
    /// the *first* colon (values routinely contain further colons, e.g.
    /// `ABILITY:Class|AUTOMATIC|X|PREVAREQ:Foo,0`).
    fn tokens(&self) -> impl Iterator<Item = (&str, &str)> {
        self.fields.iter().skip(1).map(|f| split_token(f))
    }

    fn first(&self, key: &str) -> Option<&str> {
        self.tokens().find(|(k, _)| *k == key).map(|(_, v)| v)
    }

    /// The record key: `KEY:` when present, otherwise the display name.
    fn record_key(&self) -> &str {
        self.first("KEY").unwrap_or_else(|| self.name())
    }

    /// `Some("<target>")` when this row is a `.MOD` of the named ability
    /// category, e.g. `CATEGORY=Class|Monk ~ Unchained Class.MOD` →
    /// `Some("Monk ~ Unchained Class")`.
    fn mod_target(&self, category: &str) -> Option<&str> {
        let prefix = format!("CATEGORY={category}|");
        self.name().strip_prefix(&prefix)?.strip_suffix(".MOD")
    }
}

fn split_token(field: &str) -> (&str, &str) {
    match field.find(':') {
        Some(i) => (&field[..i], &field[i + 1..]),
        None => (field, ""),
    }
}

/// Parses an LST file body into its real rows, skipping blank lines and
/// `#` comment/header lines (PrettyLST emits both `# ...` legends and
/// `###Block: ...` separators).
fn parse_rows(text: &str) -> Vec<LstRow> {
    let mut out = Vec::new();
    for (idx, raw) in text.lines().enumerate() {
        let line = raw.trim_end_matches('\r');
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }
        let fields: Vec<String> =
            line.split('\t').map(str::trim).filter(|f| !f.is_empty()).map(String::from).collect();
        if fields.is_empty() {
            continue;
        }
        out.push(LstRow { line_no: (idx + 1) as u32, fields });
    }
    out
}

fn type_tokens(row: &LstRow) -> Vec<String> {
    row.first("TYPE").map(|v| v.split('.').map(String::from).collect()).unwrap_or_default()
}

/// Every non-`BONUS:` token, preserved verbatim, unioned across every row in
/// `rows` -- the record's own base row PLUS any `.MOD` row targeting the
/// same identity ([`closure_lst_rows`]'s job to gather). `BONUS:` clauses are
/// carried separately as [`RawBonusChain`]s, matching every pre-existing
/// Shape B v1 record on disk.
///
/// SD-32 T12 row 21 cycle 2: this used to take a single `&LstRow` (the base
/// row alone), inheriting the exact `.MOD`-appended-row-loss defect row 21
/// fixed for the generic `class_feature.rs` path -- `class_feature::
/// generate()`'s own post-fix blast-radius re-scan found exactly 9 records
/// still missing real `.MOD` content, ALL in this book, all owned by THIS
/// generator (its `foreign_citations` guard correctly refuses to overwrite a
/// PU-owned coordinate, so the generic fix never reached them). Confirmed
/// live: `Unchained Summoner ~ Eidolon` alone carries 14 real `.MOD` rows
/// (`CATEGORY=Special Ability|Unchained Summoner ~ Eidolon.MOD`, e.g. the
/// `EidolonEvolution` progression formula and 10 `EidolonSubtype_*`
/// selection bonuses) that the single-row read silently dropped -- 8 of the
/// 9 records lose real `BONUS:` content this way, not merely inert tokens.
fn raw_tokens_excluding_bonus(rows: &[LstRow]) -> Vec<RawToken> {
    rows.iter()
        .flat_map(LstRow::tokens)
        .filter(|(k, _)| *k != "BONUS")
        .map(|(k, v)| RawToken { key: k.to_string(), value: v.to_string() })
        .collect()
}

/// Same closure-union widening as [`raw_tokens_excluding_bonus`], for the
/// `BONUS:` half.
fn raw_bonus_chains(rows: &[LstRow]) -> Vec<RawBonusChain> {
    rows.iter()
        .flat_map(LstRow::tokens)
        .filter(|(k, _)| *k == "BONUS")
        .map(|(_, v)| RawBonusChain { qualifiers: v.split('|').map(String::from).collect() })
        .collect()
}

/// This record's own base row, plus every `.MOD` row targeting the SAME
/// identity within this book -- the identical closure
/// `WiringClassIndex::closure_rows` already resolves for `wiring_class`
/// classification (`wiring_index.wiring_class_for`'s own call, a few lines
/// below every call site of this function), reused here so `raw_tokens`/
/// `raw_bonus_chains` see exactly the rows the wiring-class read already
/// "sees" -- row 21's own fix for `class_feature.rs`, applied here rather
/// than a fourth mechanism (`decisions.md §17`). A row's own text is
/// re-tokenized through [`parse_rows`]' identical per-line split so every
/// caller downstream (`LstRow::tokens`/`::first`/`::name`) behaves exactly
/// as it does for the base row it has always read.
fn closure_lst_rows(
    wiring_index: &WiringClassIndex,
    lines: &mut codex::rules_core::wiring_class::CorpusLines,
    lst_basename: &str,
    base: &LstRow,
    name: &str,
    key: &str,
) -> Vec<LstRow> {
    wiring_index
        .closure_rows(lines, lst_basename, base.line_no, name, key)
        .into_iter()
        .filter_map(|row_text| row_text.map(|text| lst_row_from_text(base.line_no, &text)))
        .collect()
}

/// One raw `.lst` line's own `LstRow`, split identically to [`parse_rows`]'
/// per-line logic -- reused here for a `.MOD`/`.COPY=` closure row's text,
/// which `WiringClassIndex::closure_rows` returns as a bare `String`
/// (already resolved to a real line via [`CorpusLines`], never re-read from
/// this file's own `.lst` handle). `line_no` is carried through from the
/// CALLER's own base row -- the true origin line of a closure row is not
/// needed by any consumer here (`LstRow::tokens`/`::first`/`::name` never
/// read it), so reusing the base row's number keeps this a total function.
fn lst_row_from_text(line_no: u32, text: &str) -> LstRow {
    let fields: Vec<String> =
        text.split('\t').map(str::trim).filter(|f| !f.is_empty()).map(String::from).collect();
    LstRow { line_no, fields }
}

/// `SOURCEPAGE:p.14` → `Some("p.14")`; `SOURCEPAGE:p.xx` → `None`
/// (`decisions.md §27.2`). Absent → `None`.
fn source_page(row: &LstRow) -> Option<String> {
    row.first("SOURCEPAGE").filter(|v| *v != PLACEHOLDER_SOURCE_PAGE).map(str::to_string)
}

/// [`pi_screening::declared_product_identity`] over one parsed row's own
/// tokens (`row.tokens()`, not a re-parse) -- the shared reader
/// `ingest_race_traits.rs::declared_product_identity_of` already uses,
/// applied here rather than forked (`decisions.md §39.4`: "the same reader
/// `ingest_race_traits` already uses, not a new implementation"). PCGen
/// declares Product Identity per record via `NAMEISPI:YES` /
/// `DESCISPI:YES`; this binary's own 54-term `PI_BLACKLIST_TERMS` heuristic
/// (`pi_hits`, below) is a sibling check, not a substitute -- reading a
/// declaration and scanning for undeclared terms are different questions
/// (`decisions.md §39.4`/`§53.1`, "the two are now a union").
fn declared_product_identity_of(row: &LstRow) -> pi_screening::DeclaredProductIdentity {
    pi_screening::declared_product_identity(row.tokens())
}

/// `CSKILL:Acrobatics|Climb|TYPE=Craft` → the `|`-split list, verbatim.
/// `TYPE=Craft` entries are PCGen skill-type selectors and are kept as
/// written; expanding them would require the skill corpus and would be
/// interpretation, not transcription.
fn cskills(row: &LstRow) -> Vec<String> {
    row.first("CSKILL").map(|v| v.split('|').map(str::to_string).collect()).unwrap_or_default()
}

// ---------------------------------------------------------------------
// `DESC:` rendering — ported unchanged from `ingest_race_traits.rs`
// ---------------------------------------------------------------------

/// Every variable this row defines *and finishes* on its own, with its
/// resolved integer value — or `None` where the row names the variable but
/// its value depends on something the row does not itself state.
///
/// The instant any contribution stops being a same-row literal — a formula
/// (`BONUS:VAR|X|OtherVar`), a conditional bonus (a trailing `PRE...`
/// qualifier), or a base declared elsewhere — the variable is marked
/// unresolvable and **no value is guessed** (`decisions.md §24`).
fn same_row_vars(row: &LstRow) -> BTreeMap<String, Option<i64>> {
    let mut vars: BTreeMap<String, Option<i64>> = BTreeMap::new();

    for (_, value) in row.tokens().filter(|(k, _)| *k == "DEFINE") {
        let Some((name, base)) = value.split_once('|') else { continue };
        vars.insert(name.trim().to_string(), base.trim().parse::<i64>().ok());
    }

    for (_, value) in row.tokens().filter(|(k, _)| *k == "BONUS") {
        let quals: Vec<&str> = value.split('|').collect();
        if !quals.first().map(|q| q.eq_ignore_ascii_case("VAR")).unwrap_or(false) {
            continue;
        }
        let (Some(names), Some(amount)) = (quals.get(1), quals.get(2)) else { continue };
        let conditional = quals[3..].iter().any(|q| q.starts_with("PRE") || q.starts_with("!PRE"));
        let amount = if conditional { None } else { amount.trim().parse::<i64>().ok() };
        for name in names.split(',') {
            let name = name.trim().to_string();
            match vars.get_mut(&name) {
                None => {
                    vars.insert(name, None);
                }
                Some(slot) => {
                    *slot = match (*slot, amount) {
                        (Some(current), Some(add)) => Some(current + add),
                        _ => None,
                    };
                }
            }
        }
    }

    vars
}

/// True when a `DESC:` argument is a prerequisite gate rather than a
/// substitution argument.
fn is_prerequisite_arg(arg: &str) -> bool {
    arg.contains(':') && (arg.starts_with("PRE") || arg.starts_with("!PRE"))
}

/// Evaluates one `PREVAR<CMP>:<lhs>,<rhs>...` gate against the row's own
/// variable table. Anything undecidable is an `Err`, never a coin flip.
fn eval_prevar_gate(token: &str, vars: &BTreeMap<String, Option<i64>>) -> Result<bool, String> {
    let (negated, body) = match token.strip_prefix('!') {
        Some(rest) => (true, rest),
        None => (false, token),
    };
    let (head, args) = body.split_once(':').ok_or_else(|| format!("malformed DESC gate {token:?}"))?;
    let cmp = head.strip_prefix("PREVAR").ok_or_else(|| format!("unmodelled DESC gate kind {token:?}"))?;

    let operand = |raw: &str| -> Result<i64, String> {
        let raw = raw.trim();
        if let Ok(n) = raw.parse::<i64>() {
            return Ok(n);
        }
        vars.get(raw)
            .copied()
            .flatten()
            .ok_or_else(|| format!("DESC gate {token:?}: {raw:?} is not a same-row literal"))
    };

    let parts: Vec<&str> = args.split(',').collect();
    if parts.is_empty() || !parts.len().is_multiple_of(2) {
        return Err(format!("DESC gate {token:?} is not a list of <operand>,<value> pairs"));
    }

    let mut all = true;
    for pair in parts.chunks(2) {
        let (lhs, rhs) = (operand(pair[0])?, operand(pair[1])?);
        all &= match cmp {
            "EQ" => lhs == rhs,
            "NEQ" => lhs != rhs,
            "LT" => lhs < rhs,
            "LTEQ" => lhs <= rhs,
            "GT" => lhs > rhs,
            "GTEQ" => lhs >= rhs,
            other => return Err(format!("DESC gate {token:?}: unmodelled comparison {other:?}")),
        };
    }
    Ok(negated != all)
}

fn collapse_whitespace(text: &str) -> String {
    let mut out = String::new();
    let mut last_was_space = false;
    for ch in text.chars() {
        if ch.is_whitespace() {
            if !last_was_space {
                out.push(' ');
            }
            last_was_space = true;
        } else {
            out.push(ch);
            last_was_space = false;
        }
    }
    out.trim().to_string()
}

/// Renders one `DESC:` segment's prose: `%%` becomes a literal `%`, and
/// every `%N` becomes argument N's resolved literal. An unresolvable
/// argument is **dropped, never guessed**.
fn substitute_placeholders(prose: &str, args: &[&str], vars: &BTreeMap<String, Option<i64>>) -> (String, Vec<String>) {
    let chars: Vec<char> = prose.chars().collect();
    let mut out = String::new();
    let mut unresolved: Vec<String> = Vec::new();
    let mut dropped_any = false;
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '%' && chars.get(i + 1) == Some(&'%') {
            out.push('%');
            i += 2;
            continue;
        }
        if chars[i] == '%'
            && let Some(digit) = chars.get(i + 1).and_then(|c| c.to_digit(10))
            && digit >= 1
        {
            let arg = args.get(digit as usize - 1).copied();
            let value = arg.and_then(|name| {
                let name = name.trim();
                name.parse::<i64>().ok().or_else(|| vars.get(name).copied().flatten())
            });
            match value {
                Some(v) => out.push_str(&v.to_string()),
                None => {
                    if let Some(name) = arg {
                        unresolved.push(name.to_string());
                    }
                    while out.ends_with('+') || out.ends_with('-') {
                        out.pop();
                    }
                    dropped_any = true;
                }
            }
            i += 2;
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }

    let text = if dropped_any { collapse_whitespace(&out) } else { out };
    (text, unresolved)
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct RenderedDescription {
    text: Option<String>,
    unresolved_args: Vec<String>,
    /// `DESC:` segments dropped because they are PCGen application
    /// instructions rather than game content.
    dropped_app_instructions: Vec<String>,
    /// Why no description could be stated, when [`text`](Self::text) is
    /// `None` because a gate could not be decided rather than because the
    /// row carries no `DESC:` at all.
    undecidable: Option<String>,
}

/// Turns a row's `DESC:` tokens into the prose the player actually sees.
///
/// **A row whose segments are gated on something this row does not state
/// gets no description at all.** `Unchained Rogue ~ Rogues Edge` writes its
/// prose as three mutually exclusive segments gated on `RoguesEdgeLVL`,
/// which the row seeds at 0 and then fills with the formula
/// `BONUS:VAR|RoguesEdgeLVL|RogueLVL/5` — a character-level expression, not
/// a constant. Picking a branch would assert a specific character level;
/// dropping the gated segments and keeping their neighbours would emit the
/// mangled sentence *"You have mastered gaining results that others can
/// only dream about."* Both are worse than saying nothing, so the row
/// yields `None` and the run reports it. The raw `DESC:` tokens are still
/// preserved verbatim in `raw_tokens`.
fn render_description(row: &LstRow) -> RenderedDescription {
    let vars = same_row_vars(row);
    let mut segments: Vec<String> = Vec::new();
    let mut unresolved_args: Vec<String> = Vec::new();
    let mut dropped_app_instructions: Vec<String> = Vec::new();
    let mut saw_desc = false;

    for (_, value) in row.tokens().filter(|(k, _)| *k == "DESC") {
        saw_desc = true;
        let mut parts = value.split('|');
        let prose = parts.next().unwrap_or_default();
        if PCGEN_APPLICATION_INSTRUCTION_DESCS.contains(&prose.trim()) {
            dropped_app_instructions.push(prose.trim().to_string());
            continue;
        }
        let (gates, args): (Vec<&str>, Vec<&str>) = parts.partition(|p| is_prerequisite_arg(p));

        let mut applies = true;
        for gate in &gates {
            // `!PREABILITY`/`PREABILITY` guards are not variable
            // comparisons; they never suppress the segment for ingest
            // purposes and are preserved verbatim in `raw_tokens`. Only
            // `PREVAR` gates are evaluated.
            if !gate.trim_start_matches('!').starts_with("PREVAR") {
                continue;
            }
            match eval_prevar_gate(gate, &vars) {
                Ok(decided) => applies &= decided,
                Err(why) => {
                    return RenderedDescription {
                        text: None,
                        unresolved_args,
                        dropped_app_instructions,
                        undecidable: Some(why),
                    };
                }
            }
        }
        if !applies {
            continue;
        }

        let (text, mut unresolved) = substitute_placeholders(prose.trim(), &args, &vars);
        unresolved_args.append(&mut unresolved);
        if !text.is_empty() {
            segments.push(text);
        }
    }

    let joined = segments.join(" ");
    let text = if !saw_desc || joined.is_empty() { None } else { Some(joined) };
    RenderedDescription { text, unresolved_args, dropped_app_instructions, undecidable: None }
}

/// The PCGen syntax that must never reach a player. Production guard on
/// every description this binary writes.
fn leaked_pcgen_syntax(text: &str) -> Option<&'static str> {
    if text.contains('|') {
        return Some("raw '|' argument tail");
    }
    if text.contains("%%") {
        return Some("unescaped '%%' literal-percent escape");
    }
    let chars: Vec<char> = text.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        if *c == '%' && chars.get(i + 1).is_some_and(char::is_ascii_digit) {
            return Some("unsubstituted '%N' argument reference");
        }
    }
    None
}

// ---------------------------------------------------------------------
// Grant rows (the level progression)
// ---------------------------------------------------------------------

/// Parses one `ABILITY:<category>|AUTOMATIC|<feature>[|PRE...]` grant
/// clause off a `.MOD` row.
///
/// Every number read here is a literal written on the clause itself:
/// `PREVARGTEQ:Monk_CFP_Level,3` states `3`. No formula is evaluated.
fn parse_grant(mod_target: &str, clause: &str) -> Option<ClassFeatureGrant> {
    let mut parts = clause.split('|');
    let category = parts.next()?.trim().to_string();
    let nature = parts.next()?.trim();
    if nature != "AUTOMATIC" {
        return None;
    }
    let feature_key = parts.next()?.trim().to_string();

    let mut min_level = None;
    let mut suppressed_by_var = None;
    for qual in parts {
        let qual = qual.trim();
        if let Some(args) = qual.strip_prefix("PREVARGTEQ:")
            && let Some((var, level)) = args.rsplit_once(',')
            && var.ends_with("_CFP_Level")
        {
            min_level = level.trim().parse::<u8>().ok();
        }
        if let Some(args) = qual.strip_prefix("PREVAREQ:")
            && let Some((var, value)) = args.rsplit_once(',')
            && value.trim() == "0"
            && var.contains("_CF_")
        {
            suppressed_by_var = Some(var.trim().to_string());
        }
    }

    Some(ClassFeatureGrant {
        feature_key,
        feature_category: category,
        min_level,
        granted_by_key: mod_target.to_string(),
        suppressed_by_var,
    })
}

/// Drops any grant whose `feature_key` names a feature row PCGen itself
/// declares `NAMEISPI:YES` on. Those rows are dropped by the per-feature
/// loop and never shipped as their own `class_feature` record, so a
/// class-variant chassis that still lists them as granted would ship a
/// dangling reference the corpus does not provide (code review finding
/// SD30-E8-F2). Mirrors `ingest_race_traits.rs`'s ordering, which drops a
/// PI-declared row before any chassis-level list derives from it.
fn drop_pi_named_grants(variant_grants: Vec<ClassFeatureGrant>, feature_rows: &[&LstRow]) -> Vec<ClassFeatureGrant> {
    let pi_dropped_keys: BTreeSet<&str> = feature_rows
        .iter()
        .filter(|frow| declared_product_identity_of(frow).name)
        .map(|frow| frow.record_key())
        .collect();
    variant_grants.into_iter().filter(|g| !pi_dropped_keys.contains(g.feature_key.as_str())).collect()
}

// ---------------------------------------------------------------------
// Chassis overrides (Unchained Monk only, in practice)
// ---------------------------------------------------------------------

/// Rewrites a PCGen class-level progression clause into the `level`-relative
/// notation the existing class records already use (`"level*3/4"`), by
/// substituting the literal token `level` for a
/// `classlevel("<BaseClass>", ...)` call and keeping the arithmetic tail
/// byte-identical.
///
/// This is one named substitution, not an evaluator: a value whose shape is
/// anything other than `classlevel("<BaseClass>"...)<tail>` yields `None`
/// rather than a guess (`decisions.md §24`).
fn level_progression(value: &str, base_class_key: &str) -> Option<String> {
    let value = value.trim();
    let prefix = format!("classlevel(\"{base_class_key}\"");
    let rest = value.strip_prefix(&prefix)?;
    // Skip the remaining call arguments up to the closing paren. The call
    // takes no nested parens in this corpus; a nested one would not match
    // and would fall through to `None`.
    let close = rest.find(')')?;
    if rest[..close].contains('(') {
        return None;
    }
    Some(format!("level{}", &rest[close + 1..]))
}

/// The chassis columns a variant row overrides, read off its `BONUS:`
/// clauses. All-`None` for a variant that overrides nothing.
#[derive(Debug, Default, PartialEq, Eq)]
struct ChassisOverrides {
    bab: Option<String>,
    bab_replaces_base: bool,
    save_fort: Option<String>,
    save_ref: Option<String>,
    save_will: Option<String>,
}

fn chassis_overrides(row: &LstRow, base_class_key: &str) -> ChassisOverrides {
    let mut out = ChassisOverrides::default();
    for (_, value) in row.tokens().filter(|(k, _)| *k == "BONUS") {
        let quals: Vec<&str> = value.split('|').collect();
        let [kind, target, formula, tail @ ..] = quals.as_slice() else { continue };
        match (*kind, *target) {
            ("COMBAT", "BASEAB") => {
                out.bab = level_progression(formula, base_class_key);
                out.bab_replaces_base = tail.iter().any(|t| t.contains("REPLACE"));
            }
            ("SAVE", targets) => {
                let Some(progression) = level_progression(formula, base_class_key) else { continue };
                for t in targets.split(',') {
                    match t.trim() {
                        "BASE.Fortitude" => out.save_fort = Some(progression.clone()),
                        "BASE.Reflex" => out.save_ref = Some(progression.clone()),
                        "BASE.Will" => out.save_will = Some(progression.clone()),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    out
}

/// `HITDIE:10|CLASS=Monk` → `10`. A same-row literal read.
fn template_hit_die(row: &LstRow) -> Option<u32> {
    let value = row.first("HITDIE")?;
    value.split('|').next()?.trim().parse::<u32>().ok()
}

/// The templates a variant row applies, in source order.
fn applied_templates(row: &LstRow) -> Vec<String> {
    row.tokens().filter(|(k, _)| *k == "TEMPLATE").map(|(_, v)| v.trim().to_string()).collect()
}

/// The `Class Skills ~ <name>` internal abilities a variant row grants.
fn granted_internal_class_skill_keys(row: &LstRow) -> Vec<String> {
    row.tokens()
        .filter(|(k, _)| *k == "ABILITY")
        .filter_map(|(_, v)| {
            let mut parts = v.split('|');
            if parts.next()?.trim() != "Internal" {
                return None;
            }
            if parts.next()?.trim() != "AUTOMATIC" {
                return None;
            }
            let key = parts.next()?.trim();
            key.starts_with("Class Skills ~ ").then(|| key.to_string())
        })
        .collect()
}

// ---------------------------------------------------------------------
// Output
// ---------------------------------------------------------------------

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

/// Same slug rule every other content kind already uses.
fn slugify(raw: &str) -> String {
    let mut out = String::new();
    let mut last_was_sep = false;
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            last_was_sep = false;
        } else if !last_was_sep {
            out.push('_');
            last_was_sep = true;
        }
    }
    let trimmed = out.trim_matches('_').to_string();
    if trimmed.is_empty() { "record".to_string() } else { trimmed }
}

fn write_record<T: serde::Serialize>(path: &Path, record: &CorpusRecordV1<T>) {
    fs::create_dir_all(path.parent().expect("record path must have a parent dir")).expect("failed to create output dir");
    let json = serde_json::to_string_pretty(record).expect("record must serialize");
    fs::write(path, json).unwrap_or_else(|e| panic!("failed to write {path:?}: {e}"));
}

fn pi_hits(texts: &[&str]) -> Vec<String> {
    let mut hits = Vec::new();
    for text in texts {
        for term in PI_BLACKLIST_TERMS {
            if text.contains(term) {
                hits.push((*term).to_string());
            }
        }
    }
    hits
}

fn ingested_at() -> String {
    if let Ok(v) = std::env::var("CODEX_INGESTED_AT") {
        return v;
    }
    let output = std::process::Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .expect("`date -u` must be available to stamp ingested_at");
    String::from_utf8(output.stdout).expect("date output is valid UTF-8").trim().to_string()
}

fn pcgen_data_root() -> PathBuf {
    if let Ok(v) = std::env::var("PCGEN_CORPUS_ROOT") {
        return PathBuf::from(v);
    }
    let home = std::env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
    PathBuf::from(home).join("workspace/repos/pcgen/data")
}

fn main() {
    let data_root = pcgen_data_root();
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let lst_path = data_root.join(LST_RELATIVE);
    let bytes = fs::read(&lst_path).unwrap_or_else(|e| panic!("failed to read the PU class-ability corpus {lst_path:?}: {e}"));
    let sha256 = sha256_hex(&bytes);
    let text = String::from_utf8_lossy(&bytes).to_string();
    let rows = parse_rows(&text);

    let templates_path = data_root.join(TEMPLATES_RELATIVE);
    let template_bytes =
        fs::read(&templates_path).unwrap_or_else(|e| panic!("failed to read the PU template corpus {templates_path:?}: {e}"));
    let template_rows = parse_rows(&String::from_utf8_lossy(&template_bytes));

    let ingested_at = ingested_at();
    let mut errors: Vec<String> = Vec::new();

    // --- Refuse to write a variant whose base class is not ingested ----
    // (`decisions.md §25.3`'s rule, applied to classes.)
    for (key, spec) in VARIANTS {
        let base = repo_root
            .join("data/corpus")
            .join(spec.base_class_book)
            .join("class")
            .join(format!("{}.json", slugify(spec.base_class_key)));
        if !base.exists() {
            errors.push(format!(
                "{key}: base class {} is not ingested at {base:?} — refusing to write a variant over an unregistered chassis",
                spec.base_class_key
            ));
        }
    }

    // --- Index: variant rows, grant rows, feature rows -----------------
    let variant_specs: BTreeMap<&str, VariantSpec> = VARIANTS.iter().copied().collect();

    let mut variant_rows: BTreeMap<String, LstRow> = BTreeMap::new();
    for row in &rows {
        if row.first("CATEGORY").map(|c| c.eq_ignore_ascii_case("CLASS")).unwrap_or(false)
            && variant_specs.contains_key(row.record_key())
            && let Some(prev) = variant_rows.insert(row.record_key().to_string(), row.clone())
        {
            errors.push(format!(
                "duplicate variant row for {:?} (lines {} and {})",
                row.record_key(),
                prev.line_no,
                row.line_no
            ));
        }
    }
    for (key, _) in VARIANTS {
        if !variant_rows.contains_key(*key) {
            errors.push(format!("{key}: no CATEGORY:CLASS row found in {LST_RELATIVE}"));
        }
    }

    // Internal `Class Skills ~ ...` rows, by key.
    let class_skill_rows: BTreeMap<&str, &LstRow> = rows
        .iter()
        .filter(|r| r.record_key().starts_with("Class Skills ~ "))
        .map(|r| (r.record_key(), r))
        .collect();

    // Grants: every `CATEGORY=Class|<target>.MOD  ABILITY:...` row, keyed
    // by the variant it ultimately belongs to.
    //
    // Unchained Barbarian routes its progression through two
    // sub-selections it grants itself (`... Full` / `... Ex-Class`), so the
    // owning variant is resolved by walking that one indirection rather
    // than by string-matching the target name.
    let mut sub_selection_owner: BTreeMap<String, String> = BTreeMap::new();
    for (key, row) in &variant_rows {
        for (_, value) in row.tokens().filter(|(k, _)| *k == "ABILITY") {
            let mut parts = value.split('|');
            if parts.next().map(str::trim) != Some("Class") {
                continue;
            }
            if parts.next().map(str::trim) != Some("AUTOMATIC") {
                continue;
            }
            if let Some(sub) = parts.next() {
                sub_selection_owner.insert(sub.trim().to_string(), key.clone());
            }
        }
    }

    let mut grants: BTreeMap<String, Vec<ClassFeatureGrant>> = BTreeMap::new();
    let mut orphan_grant_targets: BTreeSet<String> = BTreeSet::new();
    for row in &rows {
        let Some(target) = row.mod_target("Class") else { continue };
        let owner = if variant_specs.contains_key(target) {
            Some(target.to_string())
        } else {
            sub_selection_owner.get(target).cloned()
        };
        for (_, value) in row.tokens().filter(|(k, _)| *k == "ABILITY") {
            let Some(grant) = parse_grant(target, value) else { continue };
            if !grant.feature_category.starts_with("Unchained ") || !grant.feature_category.ends_with(" Class Feature") {
                continue;
            }
            match &owner {
                Some(owner) => grants.entry(owner.clone()).or_default().push(grant),
                None => {
                    orphan_grant_targets.insert(target.to_string());
                }
            }
        }
    }

    // Feature rows: `TYPE:Unchained <Base> Class Feature`, non-`.MOD`.
    let mut feature_rows: BTreeMap<String, Vec<&LstRow>> = BTreeMap::new();
    let mut selectable_option_types: BTreeMap<String, usize> = BTreeMap::new();
    for row in &rows {
        if row.mod_target("Class").is_some() || row.mod_target("Special Ability").is_some() {
            continue;
        }
        let types = type_tokens(row);
        // Tally the selectable-option pools this phase deliberately does
        // not write, so the gap is a printed number rather than silence.
        for t in &types {
            if t.starts_with("Unchained ") && !t.ends_with(" Class Feature") {
                *selectable_option_types.entry(t.clone()).or_default() += 1;
            }
        }
        let Some(feature_type) = types.iter().find(|t| t.starts_with("Unchained ") && t.ends_with(" Class Feature"))
        else {
            continue;
        };
        let base = feature_type
            .trim_start_matches("Unchained ")
            .trim_end_matches(" Class Feature")
            .to_string();
        let Some((variant_key, _)) = VARIANTS.iter().find(|(_, s)| s.base_class_key == base) else {
            errors.push(format!("{LST_RELATIVE}:{}: feature TYPE {feature_type:?} names no in-scope class", row.line_no));
            continue;
        };
        feature_rows.entry((*variant_key).to_string()).or_default().push(row);
    }

    // --- Rebuild the output trees -------------------------------------
    let class_root = repo_root.join("data/corpus/pathfinder_unchained/class");
    let feature_root = repo_root.join("data/corpus/pathfinder_unchained/class_feature");
    for root in [&class_root, &feature_root] {
        if root.exists() {
            fs::remove_dir_all(root).unwrap_or_else(|e| panic!("failed to clear {root:?}: {e}"));
        }
    }

    let source = |line: u32, key: &str| CorpusSource::LstToken {
        path: LST_RELATIVE.to_string(),
        sha256: sha256.clone(),
        line,
        record_key: key.to_string(),
    };
    let pu_book_dir = data_root.join("pathfinder/paizo/roleplaying_game/pathfinder_unchained");
    let wiring_index = WiringClassIndex::build("pathfinder_unchained", &pu_book_dir);
    let mut wiring_lines = wiring_index.lines();
    let lst_basename = LST_RELATIVE.rsplit('/').next().unwrap_or(LST_RELATIVE);

    let mut unresolved_desc_args: Vec<String> = Vec::new();
    let mut dropped_app_instructions: Vec<String> = Vec::new();
    let mut undecidable_descriptions: Vec<String> = Vec::new();
    let mut classes_written = 0usize;
    let mut features_per_class: BTreeMap<String, usize> = BTreeMap::new();
    let mut grants_per_class: BTreeMap<String, usize> = BTreeMap::new();
    let mut ungranted_features: Vec<String> = Vec::new();
    let mut real_pages = 0usize;
    let mut missing_pages = 0usize;
    // `class_feature` rows PCGen itself declares `NAMEISPI:YES` on, dropped
    // before any other processing -- reported, never silent
    // (`decisions.md §39.4`, mirrors `ingest_race_traits.rs`'s
    // `dropped, NAMEISPI:YES` line).
    let mut pi_dropped: Vec<String> = Vec::new();
    // `class_feature` descriptions PCGen declares `DESCISPI:YES` on,
    // redacted through the shared reader (mirrors `ingest_race_traits.rs`'s
    // `descriptions redacted by DESCISPI:YES` line).
    let mut pi_declared_descriptions = 0usize;

    for (variant_key, spec) in VARIANTS {
        let Some(row) = variant_rows.get(*variant_key) else { continue };
        let variant_grants = grants.get(*variant_key).cloned().unwrap_or_default();
        // A grant naming a feature row PCGen itself declares `NAMEISPI:YES`
        // on must never reach the class-variant record: that row is
        // DROPPED (never emitted as its own `class_feature` JSON) by the
        // per-feature loop below, which runs *after* this point. Without
        // this filter the variant's own `feature_grants` would still list
        // the dropped key, shipping a reference the corpus does not
        // provide -- code review finding SD30-E8-F2, `decisions.md §51`
        // scope note. Computed here, before `feature_grants` is captured
        // into `ClassVariantCacheData` and written to disk, so the
        // record on disk can never disagree with what the per-feature loop
        // actually emits. Mirrors `ingest_race_traits.rs`'s ordering, which
        // drops a PI-declared row before any chassis-level list is derived
        // from it. See `drop_pi_named_grants_test` below for the proof.
        let variant_grants = drop_pi_named_grants(variant_grants, feature_rows.get(*variant_key).map(Vec::as_slice).unwrap_or_default());
        grants_per_class.insert((*variant_key).to_string(), variant_grants.len());

        // Hit-die override: resolve the applied template to its `HITDIE:`.
        let mut hit_die = None;
        let mut hit_die_template = None;
        for tpl in applied_templates(row) {
            if let Some(t) = template_rows.iter().find(|r| r.record_key() == tpl)
                && let Some(hd) = template_hit_die(t)
            {
                hit_die = Some(hd);
                hit_die_template = Some(tpl.clone());
            }
        }

        let overrides = chassis_overrides(row, spec.base_class_key);
        let variant_closure_rows =
            closure_lst_rows(&wiring_index, &mut wiring_lines, lst_basename, row, variant_key, variant_key);

        let mut class_skills: Vec<String> = Vec::new();
        for key in granted_internal_class_skill_keys(row) {
            match class_skill_rows.get(key.as_str()) {
                Some(skill_row) => class_skills.extend(cskills(skill_row)),
                None => errors.push(format!("{variant_key}: grants {key:?} but no such internal row exists")),
            }
        }

        let rendered = render_description(row);
        if let Some(why) = &rendered.undecidable {
            undecidable_descriptions.push(format!("{variant_key} -> {why}"));
        }
        for arg in &rendered.unresolved_args {
            unresolved_desc_args.push(format!("{variant_key} -> DESC arg {arg:?} is not a same-row literal (dropped, not guessed)"));
        }
        for d in &rendered.dropped_app_instructions {
            dropped_app_instructions.push(format!("{variant_key} -> {d}"));
        }
        if let Some(desc) = rendered.text.as_deref()
            && let Some(leak) = leaked_pcgen_syntax(desc)
        {
            errors.push(format!("{LST_RELATIVE}:{}: {variant_key} would ship a {leak}: {desc}", row.line_no));
        }

        let data = ClassVariantCacheData {
            key: (*variant_key).to_string(),
            name: row.name().to_string(),
            base_class_key: spec.base_class_key.to_string(),
            base_class_book: spec.base_class_book.to_string(),
            category: row.first("CATEGORY").map(str::to_string),
            type_tokens: type_tokens(row),
            hit_die,
            hit_die_template,
            bab: overrides.bab,
            bab_replaces_base: overrides.bab_replaces_base,
            save_fort: overrides.save_fort,
            save_ref: overrides.save_ref,
            save_will: overrides.save_will,
            class_skills,
            feature_grants: variant_grants.clone(),
            description: rendered.text,
            source_page: source_page(row),
            raw_tokens: raw_tokens_excluding_bonus(&variant_closure_rows),
            raw_bonus_chains: raw_bonus_chains(&variant_closure_rows),
        };

        let desc = data.description.clone().unwrap_or_default();
        let hits = pi_hits(&[&data.key, &data.name, &desc]);
        if !hits.is_empty() {
            errors.push(format!("PI-blacklist hit on class {}: {hits:?}", data.key));
        }

        let path = class_root.join(format!("{}.json", slugify(variant_key)));
        let (wiring_class, wiring_class_signals) = wiring_index.wiring_class_for(
            &mut wiring_lines,
            lst_basename,
            row.line_no,
            variant_key,
            variant_key,
        );
        write_record(
            &path,
            &CorpusRecordV1 {
                population: Population::InScope,
                completeness: Completeness::Full,
                ingested_at: ingested_at.clone(),
                data,
                source: source(row.line_no, variant_key),
                license: Some(License::Ogl),
                pi_field: None,
                pi_marker: None,
                wiring_class,
                wiring_class_signals,
                description_source: None,
            },
        );
        classes_written += 1;

        // --- Features ------------------------------------------------
        // A feature's `min_level` comes from its **first** grant in source
        // order. Unchained Barbarian grants six of its features twice —
        // once via `Barbarian ~ Unchained Class Full` and again via
        // `Barbarian ~ Unchained Ex-Class`, the reduced progression an
        // ex-barbarian keeps — and the two disagree (Ex-Class states
        // level 1 for Weapon and Armor Proficiency where Full states no
        // level at all). First-in-source-order pins the primary
        // progression; the complete set, Ex-Class rows included, is on the
        // class record's `feature_grants`.
        let mut grant_by_feature: BTreeMap<&str, &ClassFeatureGrant> = BTreeMap::new();
        for g in &variant_grants {
            grant_by_feature.entry(g.feature_key.as_str()).or_insert(g);
        }

        let class_dir = feature_root.join(slugify(variant_key));
        let mut seen_slugs: BTreeMap<String, String> = BTreeMap::new();
        let mut written = 0usize;

        for frow in feature_rows.get(*variant_key).map(Vec::as_slice).unwrap_or_default() {
            let key = frow.record_key().to_string();

            // PCGen's own per-record Product Identity declaration, read
            // before any other processing (mirrors `ingest_race_traits.rs`,
            // where the same check runs before the race scope filter). A
            // NAME cannot be redacted -- it is the record's identity on
            // every screen and half of its key -- so a row declaring
            // `NAMEISPI:YES` is DROPPED, never screened
            // (`decisions.md §39.4`, `§50.3`/`§53.2`).
            let declared = declared_product_identity_of(frow);
            if declared.name {
                pi_dropped.push(format!("{LST_RELATIVE}:{}: {key}", frow.line_no));
                continue;
            }

            let grant = grant_by_feature.get(key.as_str());

            let rendered = render_description(frow);
            if let Some(why) = &rendered.undecidable {
                undecidable_descriptions.push(format!("{key} -> {why}"));
            }
            for arg in &rendered.unresolved_args {
                unresolved_desc_args
                    .push(format!("{key} -> DESC arg {arg:?} is not a same-row literal (dropped, not guessed)"));
            }
            for d in &rendered.dropped_app_instructions {
                dropped_app_instructions.push(format!("{key} -> {d}"));
            }
            if let Some(desc) = rendered.text.as_deref()
                && let Some(leak) = leaked_pcgen_syntax(desc)
            {
                errors.push(format!("{LST_RELATIVE}:{}: {key} would ship a {leak}: {desc}", frow.line_no));
            }

            let page = source_page(frow);
            if page.is_some() {
                real_pages += 1;
            } else {
                missing_pages += 1;
            }
            if grant.is_none() {
                ungranted_features.push(key.clone());
            }

            // `DESCISPI:YES` is PCGen stating that this description is
            // Product Identity, redacted through the shared reader
            // whatever the 54-term blacklist below says (`decisions.md
            // §39.4`/`§53.1`, "the two are now a union" -- an undeclared
            // description still runs `pi_hits` unchanged, below). A row
            // that declares nothing keeps its rendered text exactly as
            // before this change.
            let (feature_license, feature_pi_field, feature_pi_marker, description) = if declared.description {
                pi_declared_descriptions += 1;
                pi_screening::classify_optional_field_declared("description", rendered.text.as_deref(), true)
            } else {
                (License::Ogl, None, None, rendered.text.clone())
            };

            let feature_closure_rows =
                closure_lst_rows(&wiring_index, &mut wiring_lines, lst_basename, frow, &key, &key);
            let data = ClassFeatureCacheData {
                key: key.clone(),
                name: frow.name().to_string(),
                class_key: (*variant_key).to_string(),
                base_class_key: spec.base_class_key.to_string(),
                category: frow.first("CATEGORY").map(str::to_string),
                type_tokens: type_tokens(frow),
                min_level: grant.and_then(|g| g.min_level),
                is_granted: grant.is_some(),
                visible: frow.first("VISIBLE").map(str::to_string),
                class_skills: cskills(frow),
                description,
                source_page: page,
                raw_tokens: raw_tokens_excluding_bonus(&feature_closure_rows),
                raw_bonus_chains: raw_bonus_chains(&feature_closure_rows),
            };

            let desc = data.description.clone().unwrap_or_default();
            let hits = pi_hits(&[&data.key, &data.name, &desc]);
            if !hits.is_empty() {
                errors.push(format!("PI-blacklist hit on feature {}: {hits:?}", data.key));
            }

            let slug = slugify(&key);
            if let Some(prev) = seen_slugs.insert(slug.clone(), key.clone()) {
                errors.push(format!("slug collision {slug:?} in {variant_key}: {prev:?} and {key:?}"));
                continue;
            }

            let (wiring_class, wiring_class_signals) =
                wiring_index.wiring_class_for(&mut wiring_lines, lst_basename, frow.line_no, &key, &key);
            write_record(
                &class_dir.join(format!("{slug}.json")),
                &CorpusRecordV1 {
                    population: Population::InScope,
                    completeness: Completeness::Full,
                    ingested_at: ingested_at.clone(),
                    data,
                    source: source(frow.line_no, &key),
                    license: Some(feature_license),
                    pi_field: feature_pi_field,
                    pi_marker: feature_pi_marker,
                    wiring_class,
                    wiring_class_signals,
                    description_source: None,
                },
            );
            written += 1;
        }
        features_per_class.insert((*variant_key).to_string(), written);

        // Every grant must name a feature row that exists, or the record
        // would promise a feature the corpus cannot produce.
        let declared: BTreeSet<&str> =
            feature_rows.get(*variant_key).map(Vec::as_slice).unwrap_or_default().iter().map(|r| r.record_key()).collect();
        for g in &variant_grants {
            if !declared.contains(g.feature_key.as_str()) {
                errors.push(format!("{variant_key}: grants {:?}, which no feature row declares", g.feature_key));
            }
        }
    }

    // --- Report --------------------------------------------------------
    println!("PU Unchained class ingest ({LST_RELATIVE})");
    println!("  sha256                : {sha256}");
    println!("  real (non-comment) rows: {}", rows.len());
    println!("  classes written        : {classes_written}");
    for (key, spec) in VARIANTS {
        println!(
            "    {key:<30} base={} ({})  features={}  grants={}",
            spec.base_class_key,
            spec.base_class_book,
            features_per_class.get(*key).copied().unwrap_or(0),
            grants_per_class.get(*key).copied().unwrap_or(0),
        );
    }
    println!("  features written       : {}", features_per_class.values().sum::<usize>());
    println!("  source_page real / absent: {real_pages} / {missing_pages}");
    println!("  dropped, NAMEISPI:YES  : {}", pi_dropped.len());
    for line in &pi_dropped {
        println!("    {line}");
    }
    println!("  descriptions redacted by DESCISPI:YES : {pi_declared_descriptions}");

    if !ungranted_features.is_empty() {
        println!("\n  declared but never granted by any progression row ({}):", ungranted_features.len());
        for f in &ungranted_features {
            println!("    {f}");
        }
    }
    if !orphan_grant_targets.is_empty() {
        println!("\n  grant rows whose .MOD target is not an in-scope variant ({}):", orphan_grant_targets.len());
        for t in &orphan_grant_targets {
            println!("    {t}");
        }
    }
    if !selectable_option_types.is_empty() {
        println!("\n  selectable-option pools NOT written this phase (need their own content kinds):");
        for (t, n) in &selectable_option_types {
            println!("    {t:<40} {n}");
        }
    }
    if !dropped_app_instructions.is_empty() {
        println!("\n  DESC segments dropped as PCGen application instructions ({}):", dropped_app_instructions.len());
        for d in &dropped_app_instructions {
            println!("    {d}");
        }
    }
    if !undecidable_descriptions.is_empty() {
        println!(
            "\n  rows left with NO description because a DESC gate is not statically decidable ({}):",
            undecidable_descriptions.len()
        );
        for d in &undecidable_descriptions {
            println!("    {d}");
        }
    }
    if !unresolved_desc_args.is_empty() {
        println!("\n  DESC args not resolvable from their own row ({}), dropped not guessed:", unresolved_desc_args.len());
        for a in &unresolved_desc_args {
            println!("    {a}");
        }
    }

    if !errors.is_empty() {
        eprintln!("\nFAILED with {} error(s):", errors.len());
        for e in &errors {
            eprintln!("  {e}");
        }
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(line: &str) -> LstRow {
        parse_rows(line).pop().expect("test row must parse")
    }

    #[test]
    fn parse_rows_drops_comments_blanks_and_padding_tabs() {
        let text = "# legend\n\n###Block: x\nName\t\t\tKEY:K\t\tCATEGORY:CLASS\n";
        let rows = parse_rows(text);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].line_no, 4);
        assert_eq!(rows[0].fields, vec!["Name", "KEY:K", "CATEGORY:CLASS"]);
    }

    #[test]
    fn record_key_prefers_key_token_over_display_name() {
        let r = row("Unchained Monk\tKEY:Monk ~ Unchained Class\tCATEGORY:CLASS");
        assert_eq!(r.record_key(), "Monk ~ Unchained Class");
        assert_eq!(r.name(), "Unchained Monk");
    }

    #[test]
    fn mod_target_reads_the_category_selector() {
        let r = row("CATEGORY=Class|Monk ~ Unchained Class.MOD\tABILITY:X|AUTOMATIC|Y");
        assert_eq!(r.mod_target("Class"), Some("Monk ~ Unchained Class"));
        assert_eq!(r.mod_target("Special Ability"), None);
    }

    // --- grant parsing -------------------------------------------------

    #[test]
    fn parse_grant_reads_the_level_literal_off_the_clause() {
        let g = parse_grant(
            "Monk ~ Unchained Class",
            "Unchained Monk Class Feature|AUTOMATIC|Unchained Monk ~ Ki Pool|PREVAREQ:Monk_CF_KiPool,0|PREVARGTEQ:Monk_CFP_Level,3",
        )
        .expect("grant must parse");
        assert_eq!(g.feature_key, "Unchained Monk ~ Ki Pool");
        assert_eq!(g.feature_category, "Unchained Monk Class Feature");
        assert_eq!(g.min_level, Some(3));
        assert_eq!(g.suppressed_by_var.as_deref(), Some("Monk_CF_KiPool"));
        assert_eq!(g.granted_by_key, "Monk ~ Unchained Class");
    }

    #[test]
    fn parse_grant_leaves_min_level_none_when_the_row_states_none() {
        // Unchained Barbarian's Weapon and Armor Proficiency genuinely
        // carries no `PREVARGTEQ:..._CFP_Level` upstream. The book says
        // 1st level; the corpus does not, and inventing it here would be
        // fabricating corpus data.
        let g = parse_grant(
            "Barbarian ~ Unchained Class Full",
            "Unchained Barbarian Class Feature|AUTOMATIC|Unchained Barbarian ~ Weapon and Armor Proficiency|PREVAREQ:Barbarian_CF_ArmorProficiencies,0|PREVAREQ:Barbarian_CF_WeaponProficiencies,0",
        )
        .expect("grant must parse");
        assert_eq!(g.min_level, None);
    }

    #[test]
    fn parse_grant_ignores_non_automatic_clauses() {
        assert!(parse_grant("X", "Special Ability|VIRTUAL|Something").is_none());
    }

    // --- PI-named grant filtering (code review finding SD30-E8-F2) -----
    //
    // Proves `drop_pi_named_grants` actually filters a dangling reference,
    // not merely that it compiles: one clean row survives, one
    // `NAMEISPI:YES` row's grant is dropped, and a grant naming a feature
    // key that doesn't appear in `feature_rows` at all (an orphan grant,
    // unrelated to PI) is left untouched -- the function must not over-drop.

    #[test]
    fn drop_pi_named_grants_removes_only_the_grant_naming_a_nameispi_row() {
        let clean = row("Ki Pool\tKEY:Unchained Monk ~ Ki Pool\tTYPE:Unchained Monk Class Feature");
        let secret = row(
            "Secret Feature\tKEY:Unchained Monk ~ Secret Feature\tNAMEISPI:YES\tTYPE:Unchained Monk Class Feature",
        );
        let feature_rows: Vec<&LstRow> = vec![&clean, &secret];

        let grants = vec![
            ClassFeatureGrant {
                feature_key: "Unchained Monk ~ Ki Pool".to_string(),
                feature_category: "Unchained Monk Class Feature".to_string(),
                min_level: None,
                granted_by_key: "Monk ~ Unchained Class".to_string(),
                suppressed_by_var: None,
            },
            ClassFeatureGrant {
                feature_key: "Unchained Monk ~ Secret Feature".to_string(),
                feature_category: "Unchained Monk Class Feature".to_string(),
                min_level: None,
                granted_by_key: "Monk ~ Unchained Class".to_string(),
                suppressed_by_var: None,
            },
            ClassFeatureGrant {
                feature_key: "Unchained Monk ~ Orphan (no feature row)".to_string(),
                feature_category: "Unchained Monk Class Feature".to_string(),
                min_level: None,
                granted_by_key: "Monk ~ Unchained Class".to_string(),
                suppressed_by_var: None,
            },
        ];

        let filtered = drop_pi_named_grants(grants, &feature_rows);
        let kept: Vec<&str> = filtered.iter().map(|g| g.feature_key.as_str()).collect();

        // The PI-declared grant is gone -- the class-variant record can no
        // longer ship a dangling reference to a dropped feature row.
        assert!(!kept.contains(&"Unchained Monk ~ Secret Feature"));
        // The clean grant survives untouched.
        assert!(kept.contains(&"Unchained Monk ~ Ki Pool"));
        // A grant naming no feature row at all (a different, pre-existing
        // defect class the later `declared: BTreeSet` sanity check catches)
        // is not this function's concern and must not be silently dropped
        // here too -- that would hide the orphan-grant check's own finding.
        assert!(kept.contains(&"Unchained Monk ~ Orphan (no feature row)"));
        assert_eq!(filtered.len(), 2);
    }

    // --- chassis overrides ---------------------------------------------

    #[test]
    fn level_progression_substitutes_level_and_keeps_the_tail_byte_identical() {
        assert_eq!(level_progression("classlevel(\"Monk\",\"APPLIEDAS=NONEPIC\")/2+2", "Monk").as_deref(), Some("level/2+2"));
        assert_eq!(level_progression("classlevel(\"Monk\",\"APPLIEDAS=NONEPIC\")", "Monk").as_deref(), Some("level"));
        assert_eq!(level_progression("classlevel(\"Monk\",\"APPLIEDAS=NONEPIC\")/3", "Monk").as_deref(), Some("level/3"));
    }

    #[test]
    fn level_progression_refuses_any_shape_it_does_not_model() {
        // Wrong class, a bare variable, and an arithmetic expression that
        // is not a classlevel() call all yield None rather than a guess.
        assert_eq!(level_progression("classlevel(\"Rogue\",\"APPLIEDAS=NONEPIC\")/2", "Monk"), None);
        assert_eq!(level_progression("TL/2+2", "Monk"), None);
        assert_eq!(level_progression("min(classlevel(\"Monk\"),5)", "Monk"), None);
    }

    #[test]
    fn chassis_overrides_reads_the_unchained_monk_columns() {
        let r = row(concat!(
            "Unchained Monk\tKEY:Monk ~ Unchained Class\tCATEGORY:CLASS\t",
            "BONUS:COMBAT|BASEAB|classlevel(\"Monk\",\"APPLIEDAS=NONEPIC\")|TYPE=Base.REPLACE|PREVAREQ:UseAlternateBABProgression,0\t",
            "BONUS:SAVE|BASE.Fortitude,BASE.Reflex|classlevel(\"Monk\",\"APPLIEDAS=NONEPIC\")/2+2|PREVAREQ:UseAlternateSaveProgression,0\t",
            "BONUS:SAVE|BASE.Will|classlevel(\"Monk\",\"APPLIEDAS=NONEPIC\")/3|PREVAREQ:UseAlternateSaveProgression,0"
        ));
        let o = chassis_overrides(&r, "Monk");
        assert_eq!(o.bab.as_deref(), Some("level"));
        assert!(o.bab_replaces_base);
        assert_eq!(o.save_fort.as_deref(), Some("level/2+2"));
        assert_eq!(o.save_ref.as_deref(), Some("level/2+2"));
        assert_eq!(o.save_will.as_deref(), Some("level/3"));
    }

    #[test]
    fn chassis_overrides_is_all_none_for_a_variant_that_overrides_nothing() {
        let r = row("Unchained Rogue\tKEY:Rogue ~ Unchained Class\tCATEGORY:CLASS\tCOST:1");
        assert_eq!(chassis_overrides(&r, "Rogue"), ChassisOverrides::default());
    }

    #[test]
    fn template_hit_die_reads_the_same_row_literal() {
        let r = row("Monk ~ Unchained HD\tHITDIE:10|CLASS=Monk\tVISIBLE:NO");
        assert_eq!(template_hit_die(&r), Some(10));
    }

    // --- provenance -----------------------------------------------------

    #[test]
    fn source_page_maps_the_placeholder_to_none_and_keeps_a_real_page() {
        assert_eq!(source_page(&row("X\tSOURCEPAGE:p.14")).as_deref(), Some("p.14"));
        assert_eq!(source_page(&row("X\tSOURCEPAGE:p.xx")), None);
        assert_eq!(source_page(&row("X\tCATEGORY:CLASS")), None);
    }

    #[test]
    fn raw_tokens_keep_the_placeholder_verbatim_even_though_source_page_drops_it() {
        let r = row("X\tSOURCEPAGE:p.xx");
        assert!(raw_tokens_excluding_bonus(std::slice::from_ref(&r)).iter().any(|t| t.key == "SOURCEPAGE" && t.value == "p.xx"));
    }

    // --- declared Product Identity (`NAMEISPI`/`DESCISPI`) ---------------
    //
    // `pu_abilities_class.lst` itself carries zero `NAMEISPI:YES` /
    // `DESCISPI:YES` tokens today (re-derived this cycle:
    // `grep -o 'NAMEISPI:[A-Za-z]*\|DESCISPI:[A-Za-z]*'
    //   ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/
    //   pathfinder_unchained/pu_abilities_class.lst` → no hits), so the
    // production path this card wires in has nothing live to redact/drop
    // and the real ingest run's own `dropped, NAMEISPI:YES`/
    // `descriptions redacted by DESCISPI:YES` counters both print `0`
    // (correct, not a proof the mechanism works). These tests build a row
    // in the exact shape a future PCGen source addition would carry and
    // replay it through the real production functions
    // (`declared_product_identity_of`, `pi_screening::
    // classify_optional_field_declared`) this binary now calls, mirroring
    // `ingest_race_traits.rs`'s own `declared_product_identity_of` tests.

    #[test]
    fn declared_product_identity_of_reads_nameispi_and_descispi_off_the_row() {
        let neither = row("Ordinary Feature\tKEY:Ordinary\tDESC:Mechanics only.");
        let declared = declared_product_identity_of(&neither);
        assert!(!declared.name && !declared.description);

        let name_declared = row("Secret Feature\tKEY:Secret\tNAMEISPI:YES\tDESC:Whatever.");
        assert!(declared_product_identity_of(&name_declared).name);

        let desc_declared = row("Public Feature\tKEY:Public\tDESCISPI:YES\tDESC:Whatever.");
        assert!(declared_product_identity_of(&desc_declared).description);

        // PCGen writes `NAMEISPI:NO` explicitly on OGL rows; that is not a
        // declaration (same rule `pi_screening::declared_product_identity`
        // itself pins).
        let explicit_no = row("Explicit No\tKEY:ExplicitNo\tNAMEISPI:NO\tDESCISPI:NO");
        let declared = declared_product_identity_of(&explicit_no);
        assert!(!declared.name && !declared.description);
    }

    #[test]
    fn a_descispi_row_is_redacted_through_the_shared_reader_even_with_no_blacklist_term() {
        // Real shape from `decisions.md §39.2`'s finding: a description
        // PCGen declares Product Identity that names nothing the 54-term
        // blacklist below knows, so only the declared-PI reader catches it.
        let r = row(
            "Unchained Something\tKEY:Unchained Something\tDESCISPI:YES\tDESC:You channel a rite passed down among the Ekujae, granting a +2 bonus.",
        );
        let declared = declared_product_identity_of(&r);
        assert!(declared.description);

        let rendered = render_description(&r);
        assert!(pi_hits(&[rendered.text.as_deref().unwrap_or_default()]).is_empty(), "no blacklist term in this prose");

        let (license, pi_field, pi_marker, stored) =
            pi_screening::classify_optional_field_declared("description", rendered.text.as_deref(), true);
        assert_eq!(license, License::PiRedacted);
        assert_eq!(pi_field.as_deref(), Some("description"));
        assert!(pi_marker.is_some());
        assert_ne!(stored.as_deref(), rendered.text.as_deref(), "the declared row must not ship its real prose");
    }

    // --- descriptions ---------------------------------------------------

    #[test]
    fn description_substitutes_a_same_row_literal() {
        let r = row("Danger Sense\tDEFINE:TrapSenseBonus|0\tBONUS:VAR|TrapSenseBonus|2\tDESC:You gain a +%1 bonus on Reflex saves.|TrapSenseBonus");
        let rendered = render_description(&r);
        assert_eq!(rendered.text.as_deref(), Some("You gain a +2 bonus on Reflex saves."));
        assert!(rendered.unresolved_args.is_empty());
    }

    #[test]
    fn description_drops_an_unresolvable_argument_rather_than_guessing() {
        let r = row("X\tDESC:You gain a +%1 bonus.|SomeVarDefinedElsewhere");
        let rendered = render_description(&r);
        assert_eq!(rendered.text.as_deref(), Some("You gain a bonus."));
        assert_eq!(rendered.unresolved_args, vec!["SomeVarDefinedElsewhere".to_string()]);
    }

    #[test]
    fn an_undecidable_gate_yields_no_description_rather_than_a_mangled_one() {
        // `Unchained Rogue ~ Rogues Edge`, reduced to the shape that
        // matters: three segments, two of them gated on a variable this
        // row fills with a character-level formula. Neither branch may be
        // asserted, and dropping both while keeping their neighbours would
        // emit "You have mastered gaining results...".
        let r = row(concat!(
            "Rogue's Edge\tKEY:Unchained Rogue ~ Rogues Edge\tDEFINE:RoguesEdgeLVL|0\t",
            "BONUS:VAR|RoguesEdgeLVL|RogueLVL/5\t",
            "DESC:You have mastered\t",
            "DESC:a single skill beyond that skill's normal boundaries,|PREVAREQ:RoguesEdgeLVL,1\t",
            "DESC:%1 skills beyond those skill's normal boundaries,|RoguesEdgeLVL|PREVARGT:RoguesEdgeLVL,1\t",
            "DESC:gaining results that others can only dream about."
        ));
        let rendered = render_description(&r);
        assert_eq!(rendered.text, None);
        assert!(rendered.undecidable.is_some(), "the reason must be reported, not swallowed");
        // The raw prose survives for a later hand-modelled feature to use.
        assert_eq!(raw_tokens_excluding_bonus(std::slice::from_ref(&r)).iter().filter(|t| t.key == "DESC").count(), 4);
    }

    #[test]
    fn description_excludes_the_pcgen_application_instruction() {
        let instruction = PCGEN_APPLICATION_INSTRUCTION_DESCS[0];
        let r = row(&format!("Unchained Summoner\tKEY:Summoner ~ Unchained Class\tDESC:{instruction}"));
        let rendered = render_description(&r);
        assert_eq!(rendered.text, None);
        assert_eq!(rendered.dropped_app_instructions, vec![instruction.to_string()]);
        // ...and the raw token survives untouched.
        assert!(raw_tokens_excluding_bonus(std::slice::from_ref(&r)).iter().any(|t| t.key == "DESC" && t.value == instruction));
    }

    #[test]
    fn leak_guard_catches_every_shape_of_pcgen_substitution_syntax() {
        assert!(leaked_pcgen_syntax("a +%1 bonus").is_some());
        assert!(leaked_pcgen_syntax("prose|ArgTail").is_some());
        assert!(leaked_pcgen_syntax("reduced by 20%%").is_some());
        assert_eq!(leaked_pcgen_syntax("You gain a +2 bonus."), None);
    }

    // --- misc -----------------------------------------------------------

    #[test]
    fn cskill_list_is_transcribed_verbatim_including_type_selectors() {
        let r = row("Skills\tCSKILL:Acrobatics|Climb|TYPE=Craft");
        assert_eq!(cskills(&r), vec!["Acrobatics", "Climb", "TYPE=Craft"]);
    }

    #[test]
    fn granted_internal_class_skill_keys_finds_the_indirection() {
        let r = row("Unchained Barbarian\tABILITY:Internal|AUTOMATIC|Class Skills ~ Unchained Barbarian|PREVAREQ:Barbarian_CF_ClassSkills,0\tABILITY:Class|AUTOMATIC|Barbarian ~ Unchained Class Full");
        assert_eq!(granted_internal_class_skill_keys(&r), vec!["Class Skills ~ Unchained Barbarian".to_string()]);
    }

    #[test]
    fn slugify_matches_the_convention_every_other_content_kind_uses() {
        assert_eq!(slugify("Monk ~ Unchained Class"), "monk_unchained_class");
        assert_eq!(slugify("Unchained Summoner ~ Maker's Call"), "unchained_summoner_maker_s_call");
    }
}
