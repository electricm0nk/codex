//! Mechanical detector for the PCGen corpus shapes that have repeatedly
//! produced wrong counts during book ingestion.
//!
//! # Why this module exists
//!
//! Four of twenty-three PCGen books are ingested. Every ingestion cycle so
//! far has hit the *same* corpus traps, rediscovered by hand, by a
//! different agent, every time — and nearly every count reported from
//! those rediscoveries was wrong on the first pass: 396 missing feats
//! (real: 301), 207 bonus-bearing feats (real: 166), 180 `BONUS:VAR`
//! records (real: 86). This module turns that rediscovery into something
//! an agent runs against a book *before* writing any ingest code.
//!
//! # The distinction the whole design turns on
//!
//! A `.MOD` record is **legitimate data**, not a defect. So is a
//! `#`-disabled row, an archetype-qualified `KEY:`, and a namespaced key.
//! The defect is *counting a `.MOD` as a declaration*, or reading the
//! disabled twin's class list, or crediting a base class with an
//! archetype's feature.
//!
//! Accordingly this module has two surfaces with two severities:
//!
//! * [`scan_book`] / [`scan_lst`] read upstream corpus files and emit
//!   [`Severity::Trap`] findings only. Upstream data is never called
//!   defective; the findings say *"this is a shape you must handle"*.
//! * [`audit_ingested_cache`] reads the JSON caches under `data/corpus/`
//!   and cross-checks each record's citation against the corpus line it
//!   claims. Contradictions there are [`Severity::Defect`], because they
//!   are our mistake, not upstream's.
//!
//! # The traps, and how each was confirmed against the real corpus
//!
//! Each entry below names the corpus evidence, because several of the
//! incident-report descriptions were slightly wrong and were corrected
//! here rather than encoded as received.
//!
//! 1. **[`Trap::ModRecord`]** — `.MOD` modifies a base record rather than
//!    declaring one. `apg_feats.lst` (37) and `acg_feats.lst` (39) carry
//!    76 live `.MOD` rows between them. **Correction:** only the first
//!    tab-separated field decides this. `cr_feats.lst`'s `Spell Mastery`
//!    contains `SELECT:var("STAT.3.MOD.NOEQUIP.NOTEMP")`, so a naive
//!    `grep -c '\.MOD'` reports 4 where the file has 3 `.MOD` rows (2
//!    live).
//!
//! 2. **[`Trap::DisabledLine`]** — `#`-prefixed rows are disabled
//!    duplicates that look entirely real. `apg_feats.lst:53`'s
//!    `#Elemental Fist` carries a live-looking `TYPE:Combat` plus full
//!    `BONUS:VAR` tokens. `apg_spells.lst:72`'s disabled
//!    `#Corruption Resistance` lists `Inquisitor=2`, which the live row
//!    at line 8 does not. **Correction:** `grep -c '^#'` badly overcounts.
//!    `cr_feats.lst` has 18 `#` lines but only **one** disabled record;
//!    the rest are file headers, `###Block:` separators, tab-aligned
//!    column legends, and two disabled *fragments* whose first field is
//!    empty or a bare token.
//!
//! 3. **[`Trap::KeyDiffersFromName`]** — field 0 is the display name and
//!    `KEY:` is the identity; they can differ. Confirmed at exactly the
//!    reported scale: 9 in `apg_spells.lst` + 9 in `acg_spells.lst` = 18,
//!    and all 9 ACG ones are the `Naturalist Summon Nature's Ally N`
//!    collision with the Core spells of the same display name (they carry
//!    their own `DURATION:` formula referencing
//!    `ConjurationNaturalistsCharmBonus`).
//!
//! 4. **[`Trap::ArchetypeScoped`]** — archetype records pose as
//!    base-class content. Confirmed precisely: the only Bloodrager
//!    `DEFINE:RagePowersLVL|0` in the corpus sits on
//!    `acg_abilities_class.lst:2706`,
//!    `KEY:Bloodrager Archetype ~ Primalist`, `CATEGORY:Archetype`,
//!    `PRECLASS:1,Bloodrager=1`.
//!
//! 5. **[`Trap::SharedNameDistinctRecords`]** — a shared name never
//!    implies a shared thing. `KEY:Bard ~ Lore Master`
//!    (`cr_abilities_class.lst:509`) and `KEY:Skald ~ Lore Master`
//!    (`acg_abilities_class.lst:1732`) are distinct records. So are the
//!    32 `Sorcerer Bloodline ~` and 11 `Bloodrager Bloodline ~` keys
//!    despite their overlapping names.
//!
//! 6. **[`Trap::DefineZeroValueElsewhere`]** — a variable `DEFINE`s to 0
//!    while its real value arrives from an unconditional `BONUS:VAR` on a
//!    different record. Confirmed: `WeaponFocusToHit` is `DEFINE`d to 0
//!    on the `Weapon Focus` feat (`cr_feats.lst:184`) and on
//!    `cr_abilities.lst:11`; the *only* `BONUS:VAR|WeaponFocusToHit|1|TYPE=Base`
//!    in the entire corpus is on that same `cr_abilities.lst:11`
//!    `CATEGORY=Internal|Default.MOD` row, ungated by any `PRE` token.
//!    Reading the feat alone shows a 0.
//!
//! 7. **[`Trap::NamespacedKey`]** — `KEY:<Namespace> ~ <Leaf>` makes a
//!    bare-leaf grep return zero. Corpus scale: 484 `Warpriest Bonus Feat ~`,
//!    251 `Special Ability ~`, 154 `Magus Spellblend ~`, 108 `Rage Power ~`.
//!
//! 8. **[`Trap::TokenDenseRecord`]** — one record carries many tokens, so
//!    a token count is not a record count. **Correction:** the reported
//!    "a single feat held 66 `BONUS:VAR` tokens" is not a feat and not 66.
//!    The corpus maximum is 69 `BONUS:VAR` tokens on
//!    `apg_abilities_class.lst:1149`,
//!    `CATEGORY=Internal|Druid Domain ~ Base.MOD`. The densest feat-file
//!    row is `Improved Channel` with 5. Across `cr_feats.lst` +
//!    `apg_feats.lst`, 60 records carry 85 `BONUS:VAR` tokens.
//!
//! 9. **[`concept_census`]** — book-scoped counts presented as
//!    corpus-wide. Confirmed by shape: `WitchHex`-typed records run APG
//!    28, UM 28, ACG 10, UW 5+4, HA 4+2, ARG 3, MA 2, MC 1, UC 1 — so an
//!    APG-only figure understates the corpus by roughly two-thirds. (The
//!    specific "27 vs 53" figures in the incident report did not
//!    reproduce under any single definition of "hex" tried here; the
//!    mechanism is real, those two numbers are not reliable.)
//!
//! 10. **[`Trap::GoverningTokenHiddenByFilter`]** — a grep narrowed to
//!     `BONUS:`/`PRE:` hides `MULT`/`STACK`/`CHOOSE`/`SELECT` and the
//!     other tokens that govern how the bonus applies. `cr_feats.lst`
//!     carries 21 `MULT:YES` and 6 `STACK:YES`; `acg_feats.lst` carries
//!     30 and 20.
//!
//! 11. **[`Trap::CopyRecord`]** — *not on the original list.* `.COPY=` is
//!     the mirror image of `.MOD`: it **declares** a new record derived
//!     from a base. `apg_spells.lst` carries 17 of them (e.g.
//!     `Planar Binding.COPY=Planar Binding (Demons Only)` at line 1039),
//!     and 13 already-ingested cache records cite a `.COPY=` line as
//!     their source. Excluding `.COPY=` from a declaration count
//!     undercounts by exactly as much as including `.MOD` overcounts.
//!
//! # Scope discipline (trap 9, structurally)
//!
//! No API here returns a bare corpus-wide total. [`FileScan`] is
//! per-file, [`BookScan`] shows its per-file parts, and [`concept_census`]
//! always answers with the per-book breakdown alongside the sum. A number
//! you cannot attribute to a scope is a number you cannot trust.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::Path;

// ===========================================================================
// Line classification
// ===========================================================================

/// The structural shape of one physical LST line.
///
/// Shape is orthogonal to [`LstLine::disabled`]: `#Acid Splash.MOD` is a
/// [`RecordShape::Modification`] that happens to be suppressed. Keeping
/// the two separate is what lets the scanner say "13 of the 611 `.MOD`
/// rows in this file are disabled" instead of conflating them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RecordShape {
    /// Field 0 is a plain name: this line *declares* a record.
    Declaration,
    /// Field 0 ends in `.MOD`: this line *modifies* a record declared
    /// elsewhere. It declares nothing.
    Modification,
    /// Field 0 contains `.COPY=`: this line declares a *new* record
    /// derived from the base named before the `.COPY=`.
    Copy,
    /// Field 0 is a body token repeated from a record above — PCGen's
    /// wrapped-row form, e.g. `CLASSES:Paladin=4|Inquisitor=5\tCLASSES:...`.
    Continuation,
    /// Field 0 is a file-level tag such as `SOURCELONG:` or
    /// `ABILITYCATEGORY:`. Declares no named record of the kind this
    /// scanner counts.
    Directive,
    /// Prose: a `#` header, a `###Block:` separator, or a tab-aligned
    /// column legend. Carries no tokens.
    Comment,
    /// Empty line.
    Blank,
}

impl RecordShape {
    /// Does this shape put a *new* named record into the book?
    ///
    /// `Declaration` and `Copy` do. `Modification` emphatically does not
    /// — that is trap 1.
    pub fn declares_a_record(self) -> bool {
        matches!(self, RecordShape::Declaration | RecordShape::Copy)
    }

    /// Does this shape carry a record name in field 0 at all?
    pub fn is_named_record(self) -> bool {
        matches!(
            self,
            RecordShape::Declaration | RecordShape::Modification | RecordShape::Copy
        )
    }
}

/// File-level tags that legitimately occupy field 0.
///
/// Derived by census over every `.lst` under
/// `pathfinder/paizo/roleplaying_game`: these are the tokens that appear
/// in field 0 across the corpus. Anything else token-shaped in field 0 is
/// a wrapped continuation row.
const DIRECTIVE_TOKENS: &[&str] = &[
    "ABILITY",
    "ABILITYCATEGORY",
    "ALIGN",
    "CAMPAIGN",
    "CLASS",
    "DATACONTROL",
    "DEFAULTVARIABLEVALUE",
    "DEITY",
    "FACTDEF",
    "FACTSETDEF",
    "FOLLOWER",
    "FUNDS",
    "GEAR",
    "GENDER",
    "GLOBAL",
    "KIT",
    "LANGBONUS",
    "LOCAL",
    "MASTERBONUSRACE",
    "NAME",
    "RACE",
    "SELECT",
    "SKILL",
    "SOURCEDATE",
    "SOURCELONG",
    "SOURCESHORT",
    "SOURCEWEB",
    "SPELLS",
    "STARTPACK",
    "STAT",
    "SUBCLASS",
    "SUBCLASSLEVEL",
    "SUBSTITUTIONCLASS",
    "SUBSTITUTIONLEVEL",
    "TEMPLATE",
    "VARIABLE",
];

/// Tokens that govern *how* a bonus or prerequisite applies, and that a
/// `BONUS:`/`PRE:`-narrowed grep therefore silently drops (trap 10).
///
/// Deliberately excludes `VISIBLE:`, which governs whether a record is
/// *shown*, not how its bonus applies. Including it fired on 778 of
/// Ultimate Combat's records and buried the `MULT`/`STACK`/`CHOOSE`
/// findings that actually change a computation.
const GOVERNING_TOKENS: &[&str] = &[
    "MULT",
    "STACK",
    "SELECT",
    "CHOOSE",
    "SERVESAS",
    "ASPECT",
    "PRERULE",
    "TEMPBONUS",
];

/// A record carrying at least this many `BONUS:VAR` tokens is reported,
/// because at that density a token count and a record count diverge
/// enough to change a scoping decision.
const TOKEN_DENSITY_THRESHOLD: usize = 10;

/// One classified physical line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LstLine {
    /// 1-based line number, matching what an editor and a `source.line`
    /// citation both use.
    pub line_no: u32,
    pub shape: RecordShape,
    /// `true` when the line is `#`-prefixed. Independent of `shape`.
    pub disabled: bool,
    /// Field 0 exactly as written, minus any leading `#` and any
    /// `CATEGORY=X|` qualifier: the record's display name for
    /// `Declaration`, the base name for `Modification`, the *new* name
    /// for `Copy`, and empty for everything else.
    pub name: String,
    /// For `Modification` and `Copy`, the base record being modified or
    /// copied from.
    pub base_name: String,
    /// The `CATEGORY=X|` qualifier on field 0, if present. Note this is
    /// the field-0 qualifier form, distinct from a `CATEGORY:X` token.
    pub field_qualifier: Option<String>,
    /// Value of the `KEY:` token, if the line carries one.
    pub key: Option<String>,
    /// Value of the `CATEGORY:` token, if the line carries one.
    pub category: Option<String>,
    /// Dot-separated components of the `TYPE:` token.
    pub types: Vec<String>,
    /// Every token on the line as `(name, value)`, in file order, for
    /// fields 1..N. Field 0 is not a token unless the shape is
    /// `Directive` or `Continuation`.
    pub tokens: Vec<(String, String)>,
}

impl LstLine {
    /// The record's identity: its `KEY:` when it has one, else its
    /// display name. This is the only correct join column between two
    /// corpus records — see traps 3, 5 and 7.
    pub fn identity(&self) -> &str {
        self.key.as_deref().unwrap_or(&self.name)
    }

    /// Namespace and leaf of a `KEY:<Namespace> ~ <Leaf>` (trap 7).
    pub fn key_namespace(&self) -> Option<(&str, &str)> {
        self.key.as_deref()?.split_once(" ~ ")
    }

    /// Values of every token with the given name, in file order.
    pub fn token_values<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a str> + 'a {
        self.tokens
            .iter()
            .filter(move |(n, _)| n == name)
            .map(|(_, v)| v.as_str())
    }

    fn has_token(&self, name: &str) -> bool {
        self.tokens.iter().any(|(n, _)| n == name)
    }

    /// `DEFINE:<Var>|0` variables declared on this line.
    fn defines_zero(&self) -> Vec<String> {
        self.token_values("DEFINE")
            .filter_map(|v| {
                let (var, init) = v.split_once('|')?;
                (init.trim() == "0").then(|| var.trim().to_string())
            })
            .collect()
    }

    /// Variables this line grants a `BONUS:VAR` to.
    fn bonus_vars(&self) -> Vec<String> {
        self.token_values("BONUS")
            .filter_map(|v| v.strip_prefix("VAR|"))
            .filter_map(|rest| {
                let var = rest.split('|').next()?;
                (!var.is_empty()).then(|| var.trim().to_string())
            })
            .collect()
    }

    fn bonus_var_token_count(&self) -> usize {
        self.token_values("BONUS")
            .filter(|v| v.starts_with("VAR|"))
            .count()
    }

    /// Is this record scoped to an archetype rather than a base class
    /// (trap 4)?
    fn archetype_scope(&self) -> Option<String> {
        let mut reasons = Vec::new();
        if self.category.as_deref() == Some("Archetype") {
            reasons.push("CATEGORY:Archetype".to_string());
        }
        if let Some(t) = self.types.iter().find(|t| t.contains("Archetype")) {
            reasons.push(format!("TYPE component `{t}`"));
        }
        if let Some((ns, _)) = self.key_namespace()
            && ns.ends_with("Archetype")
        {
            reasons.push(format!("KEY namespace `{ns}`"));
        }
        if reasons.is_empty() {
            return None;
        }
        let gated: Vec<String> = self
            .token_values("PRECLASS")
            .map(|v| v.to_string())
            .collect();
        if !gated.is_empty() {
            reasons.push(format!("PRECLASS:{}", gated.join(", ")));
        } else if let Some((ns, _)) = self.key_namespace() {
            reasons.push(format!("owner `{}`", ns.trim_end_matches(" Archetype")));
        }
        Some(reasons.join("; "))
    }

    fn governing_tokens(&self) -> Vec<&'static str> {
        GOVERNING_TOKENS
            .iter()
            .copied()
            .filter(|t| self.has_token(t))
            .collect()
    }

    fn has_bonus_or_pre(&self) -> bool {
        self.tokens
            .iter()
            .any(|(n, _)| n == "BONUS" || n.starts_with("PRE"))
    }
}

fn split_token(field: &str) -> Option<(String, String)> {
    let (name, value) = field.split_once(':')?;
    if name.is_empty() || !name.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()) {
        return None;
    }
    Some((name.to_string(), value.to_string()))
}

fn is_token_shaped(field: &str) -> bool {
    split_token(field).is_some()
}

/// Classify one physical line. `line_no` is 1-based.
pub fn parse_line(line_no: u32, raw: &str) -> LstLine {
    let raw = raw.trim_end_matches(['\r', '\n']);
    let fields: Vec<&str> = raw.split('\t').collect();

    let mut line = LstLine {
        line_no,
        shape: RecordShape::Blank,
        disabled: false,
        name: String::new(),
        base_name: String::new(),
        field_qualifier: None,
        key: None,
        category: None,
        types: Vec::new(),
        tokens: Vec::new(),
    };

    if raw.trim().is_empty() {
        return line;
    }

    let mut head = fields[0].trim();
    if let Some(stripped) = head.strip_prefix('#') {
        line.disabled = true;
        head = stripped.trim_start_matches('#').trim();
    }

    // Tokens live in fields 1..N. Empty fields are padding — PCGen
    // tab-aligns its columns, so a real record can carry dozens of them.
    for field in fields.iter().skip(1) {
        let field = field.trim();
        if field.is_empty() {
            continue;
        }
        if let Some(tok) = split_token(field) {
            line.tokens.push(tok);
        }
    }

    let carries_tokens = !line.tokens.is_empty();

    // A `#` line with an empty or token-shaped field 0 is a suppressed
    // *fragment*, not a suppressed record: there is no name to suppress.
    // Two of `cr_feats.lst`'s 18 `#` lines are exactly this.
    if head.is_empty() {
        line.shape = RecordShape::Comment;
        return line;
    }

    if is_token_shaped(head) {
        let (name, value) = split_token(head).expect("token-shaped head splits");
        let shape = if DIRECTIVE_TOKENS.contains(&name.as_str()) {
            RecordShape::Directive
        } else {
            RecordShape::Continuation
        };
        line.shape = shape;
        line.tokens.insert(0, (name, value));
        finish_derived_fields(&mut line);
        return line;
    }

    // `CATEGORY=FEAT|Arcane Strike.MOD` — the qualifier precedes the name.
    let mut name_part = head;
    if let Some((qualifier, rest)) = head.split_once('|')
        && qualifier.contains('=')
    {
        line.field_qualifier = Some(qualifier.to_string());
        name_part = rest;
    }

    if let Some((base, new_name)) = name_part.split_once(".COPY=") {
        line.shape = RecordShape::Copy;
        line.base_name = base.to_string();
        line.name = new_name.to_string();
    } else if let Some(base) = name_part.strip_suffix(".MOD") {
        line.shape = RecordShape::Modification;
        line.base_name = base.to_string();
        line.name = base.to_string();
    } else if carries_tokens {
        line.shape = RecordShape::Declaration;
        line.name = name_part.to_string();
    } else {
        // A bare name with no tokens anywhere is a column legend
        // (`# Ability Name\tOutput Name\t...`), not a record. The
        // `.MOD`/`.COPY=` suffixes are checked *first* because they are
        // self-proving: `apg_spells.lst:1052` is the single-field row
        // `Call Lightning Storm.COPY=Call Lightning Storm (Starsoul)`,
        // which declares a real spell that five ingested records depend
        // on, and carries no tokens at all.
        line.shape = RecordShape::Comment;
        return line;
    }

    finish_derived_fields(&mut line);
    line
}

fn finish_derived_fields(line: &mut LstLine) {
    let first = |name: &str| -> Option<String> {
        line.tokens
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, v)| v.clone())
    };
    let key = first("KEY");
    let category = first("CATEGORY");
    let types = first("TYPE")
        .map(|t| t.split('.').map(str::to_string).collect())
        .unwrap_or_default();
    line.key = key;
    line.category = category;
    line.types = types;
}

// ===========================================================================
// Traps and findings
// ===========================================================================

/// The corpus shapes this module detects.
///
/// Each variant is a *shape you must handle*, not a defect, except where
/// [`audit_ingested_cache`] raises the same variant to
/// [`Severity::Defect`] because our own ingested data contradicts the
/// corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Trap {
    /// 1 — `.MOD` modifies; it does not declare.
    ModRecord,
    /// 2 — `#`-prefixed rows are disabled duplicates that look real.
    DisabledLine,
    /// 3 — `KEY:` differs from the display name in field 0.
    KeyDiffersFromName,
    /// 4 — the record is archetype-scoped, not base-class content.
    ArchetypeScoped,
    /// 5 — two records share a display name but are different things.
    SharedNameDistinctRecords,
    /// 6 — `DEFINE`d to 0 here; the real value arrives elsewhere.
    DefineZeroValueElsewhere,
    /// 7 — `KEY:<Namespace> ~ <Leaf>`; a bare-leaf grep returns zero.
    NamespacedKey,
    /// 8 — one record carrying many tokens; tokens are not records.
    TokenDenseRecord,
    /// 10 — governing tokens a `BONUS:`/`PRE:` grep drops.
    GoverningTokenHiddenByFilter,
    /// 11 — `.COPY=` declares a new record.
    CopyRecord,
    /// Audit-only: an ingested record cites a line that does not resolve.
    UnresolvableCitation,
}

impl Trap {
    /// Stable short identifier, for report output and dashboards.
    pub fn id(self) -> &'static str {
        match self {
            Trap::ModRecord => "mod-record",
            Trap::DisabledLine => "disabled-line",
            Trap::KeyDiffersFromName => "key-differs-from-name",
            Trap::ArchetypeScoped => "archetype-scoped",
            Trap::SharedNameDistinctRecords => "shared-name-distinct-records",
            Trap::DefineZeroValueElsewhere => "define-zero-value-elsewhere",
            Trap::NamespacedKey => "namespaced-key",
            Trap::TokenDenseRecord => "token-dense-record",
            Trap::GoverningTokenHiddenByFilter => "governing-token-hidden-by-filter",
            Trap::CopyRecord => "copy-record",
            Trap::UnresolvableCitation => "unresolvable-citation",
        }
    }

    /// The miscount this trap produces if handled naively — the sentence
    /// an agent needs in order to act on the finding.
    pub fn miscount_risk(self) -> &'static str {
        match self {
            Trap::ModRecord => {
                "Counting these as declarations inflates a record estimate. Only field 0 \
                 decides: `.MOD` inside a token value (`var(\"STAT.3.MOD...\")`) is not one."
            }
            Trap::DisabledLine => {
                "These are suppressed and must not be ingested, but they look live — one \
                 carries a `TYPE:Combat`, another a different class list than its live twin."
            }
            Trap::KeyDiffersFromName => {
                "Joining on display name merges records that are not the same record. \
                 `KEY:` is the identity; field 0 is only the label."
            }
            Trap::ArchetypeScoped => {
                "Crediting the base class with this feature is wrong: it is reachable only \
                 through the archetype."
            }
            Trap::SharedNameDistinctRecords => {
                "A bare name grep makes one of these look already covered by the other."
            }
            Trap::DefineZeroValueElsewhere => {
                "Reading this record alone shows 0. The real value is granted by a \
                 `BONUS:VAR` on a different record."
            }
            Trap::NamespacedKey => {
                "A bare-leaf `KEY:` grep returns zero and looks like the records do not exist."
            }
            Trap::TokenDenseRecord => {
                "A token count is not a record count. Report both, and say which you mean."
            }
            Trap::GoverningTokenHiddenByFilter => {
                "A grep narrowed to `BONUS:`/`PRE:` drops the tokens that govern how the \
                 bonus applies. Read the whole record."
            }
            Trap::CopyRecord => {
                "`.COPY=` declares a *new* record. Excluding it undercounts by as much as \
                 including `.MOD` overcounts."
            }
            Trap::UnresolvableCitation => {
                "The provenance chain is broken: the cited line does not exist or is blank."
            }
        }
    }
}

/// Whether a finding describes legitimate corpus shape or our own error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    /// Legitimate upstream data that an ingest must handle correctly.
    /// Never an error.
    Trap,
    /// A contradiction inside content we already ingested.
    Defect,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Trap => write!(f, "TRAP"),
            Severity::Defect => write!(f, "DEFECT"),
        }
    }
}

/// One reported occurrence: where it is, which trap, and enough context
/// to act without reopening the file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub file: String,
    pub line: u32,
    pub trap: Trap,
    pub severity: Severity,
    /// The record this finding is about.
    pub record: String,
    /// Trap-specific context: the base record, the colliding key, the
    /// line the value actually comes from, and so on.
    pub detail: String,
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{} [{}] {} — {} — {}",
            self.file,
            self.line,
            self.severity,
            self.trap.id(),
            self.record,
            self.detail
        )
    }
}

// ===========================================================================
// File scan
// ===========================================================================

/// Every classified line of one `.lst` file, plus its findings.
#[derive(Debug, Clone)]
pub struct FileScan {
    pub path: String,
    pub lines: Vec<LstLine>,
    pub declarations: usize,
    pub modifications: usize,
    pub copies: usize,
    /// `#`-prefixed lines that are *records* (declaration, modification
    /// or copy). Suppressed fragments and prose are not counted here.
    pub disabled_records: usize,
    pub continuations: usize,
    pub directives: usize,
    pub comments: usize,
    pub blanks: usize,
    /// Total `BONUS:VAR` *tokens* across live records.
    pub bonus_var_tokens: usize,
    /// Number of live records carrying at least one `BONUS:VAR`.
    pub bonus_var_records: usize,
    findings: Vec<Finding>,
}

impl FileScan {
    /// Lines that put a new named record into the book: declarations plus
    /// `.COPY=` rows. This — not the raw line count, and not
    /// declarations-plus-modifications — is the number an inventory wants.
    pub fn declaring_lines(&self) -> usize {
        self.declarations + self.copies
    }

    pub fn findings(&self) -> impl Iterator<Item = &Finding> {
        self.findings.iter()
    }

    pub fn findings_for(&self, trap: Trap) -> impl Iterator<Item = &Finding> {
        self.findings.iter().filter(move |f| f.trap == trap)
    }

    pub fn count_for(&self, trap: Trap) -> usize {
        self.findings_for(trap).count()
    }

    pub fn counts_by_trap(&self) -> BTreeMap<Trap, usize> {
        let mut out = BTreeMap::new();
        for f in &self.findings {
            *out.entry(f.trap).or_insert(0) += 1;
        }
        out
    }

    /// `KEY:` namespaces used in this file and how many leaves each has
    /// (trap 7). An agent grepping for a leaf name needs this list to
    /// know what prefix to search under.
    pub fn namespaces(&self) -> BTreeMap<String, usize> {
        let mut out = BTreeMap::new();
        for line in self.lines.iter().filter(|l| !l.disabled) {
            if let Some((ns, _)) = line.key_namespace() {
                *out.entry(ns.to_string()).or_insert(0) += 1;
            }
        }
        out
    }
}

/// Scan one `.lst` file's text. Pure: no I/O, so it is directly testable
/// against hand-built lines.
pub fn scan_lst(path: &str, contents: &str) -> FileScan {
    let lines: Vec<LstLine> = contents
        .lines()
        .enumerate()
        .map(|(i, raw)| parse_line(i as u32 + 1, raw))
        .collect();

    let mut scan = FileScan {
        path: path.to_string(),
        declarations: 0,
        modifications: 0,
        copies: 0,
        disabled_records: 0,
        continuations: 0,
        directives: 0,
        comments: 0,
        blanks: 0,
        bonus_var_tokens: 0,
        bonus_var_records: 0,
        findings: Vec::new(),
        lines,
    };

    for line in &scan.lines {
        if line.disabled {
            if line.shape.is_named_record() {
                scan.disabled_records += 1;
            } else {
                scan.comments += 1;
            }
            continue;
        }
        match line.shape {
            RecordShape::Declaration => scan.declarations += 1,
            RecordShape::Modification => scan.modifications += 1,
            RecordShape::Copy => scan.copies += 1,
            RecordShape::Continuation => scan.continuations += 1,
            RecordShape::Directive => scan.directives += 1,
            RecordShape::Comment => scan.comments += 1,
            RecordShape::Blank => scan.blanks += 1,
        }
        let n = line.bonus_var_token_count();
        if n > 0 {
            scan.bonus_var_tokens += n;
            scan.bonus_var_records += 1;
        }
    }

    scan.findings = collect_findings(path, &scan.lines);
    scan
}

fn collect_findings(path: &str, lines: &[LstLine]) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Names declared live in this file, for resolving `.MOD` bases.
    let live_declared: BTreeSet<&str> = lines
        .iter()
        .filter(|l| !l.disabled && l.shape.declares_a_record())
        .map(|l| l.name.as_str())
        .collect();

    // Every variable granted a `BONUS:VAR` anywhere in this file, and by
    // which line — for trap 6.
    let mut bonus_sites: BTreeMap<String, Vec<u32>> = BTreeMap::new();
    for line in lines.iter().filter(|l| !l.disabled) {
        for var in line.bonus_vars() {
            bonus_sites.entry(var).or_default().push(line.line_no);
        }
    }

    // Live records grouped by display name, for trap 5.
    let mut by_name: BTreeMap<&str, Vec<&LstLine>> = BTreeMap::new();
    for line in lines
        .iter()
        .filter(|l| !l.disabled && l.shape.declares_a_record())
    {
        by_name.entry(line.name.as_str()).or_default().push(line);
    }

    let push = |findings: &mut Vec<Finding>, line: &LstLine, trap: Trap, detail: String| {
        findings.push(Finding {
            file: path.to_string(),
            line: line.line_no,
            trap,
            severity: Severity::Trap,
            record: line.name.clone(),
            detail,
        });
    };

    for line in lines {
        // ------------------------------------------------------- trap 2
        if line.disabled {
            if !line.shape.is_named_record() {
                continue;
            }
            let mut detail = format!("suppressed {:?}", line.shape);
            if let Some(twin) = lines.iter().find(|l| {
                !l.disabled && l.shape.declares_a_record() && l.name == line.name
            }) {
                detail.push_str(&format!(
                    "; a live record of the same name is declared at line {}",
                    twin.line_no
                ));
                let diverging = diverging_tokens(line, twin);
                if !diverging.is_empty() {
                    detail.push_str(&format!(
                        "; the two rows disagree on {} — read the live one",
                        diverging.join(", ")
                    ));
                }
            } else {
                detail.push_str("; no live record of this name in this file");
            }
            push(&mut findings, line, Trap::DisabledLine, detail);
            // A disabled row is reported once, as disabled. Reporting its
            // other shapes too would bury the live findings in noise.
            continue;
        }

        // ------------------------------------------------------- trap 1
        if line.shape == RecordShape::Modification {
            let known = live_declared.contains(line.base_name.as_str());
            push(
                &mut findings,
                line,
                Trap::ModRecord,
                format!(
                    "modifies `{}`, which is {}declared in this file; this row declares nothing",
                    line.base_name,
                    if known { "" } else { "not " }
                ),
            );
        }

        // ------------------------------------------------------ trap 11
        if line.shape == RecordShape::Copy {
            push(
                &mut findings,
                line,
                Trap::CopyRecord,
                format!(
                    "declares a new record copied from `{}`; count it as a declaration",
                    line.base_name
                ),
            );
        }

        if !line.shape.is_named_record() {
            continue;
        }

        // ------------------------------------------------------- trap 3
        if let Some(key) = &line.key
            && key != &line.name
        {
            // The two shapes of this trap need different handling, so
            // the detail says which one it is. A namespaced KEY is
            // *visibly* different from the display name — you notice it.
            // A flat KEY that merely renames the record is the dangerous
            // one: `Abjuration School` keyed as `Abjuration Savant School`
            // (`acg_abilities_class.lst:2494`) looks like the Wizard's
            // school to any name-based join, and the nine ACG
            // `Naturalist Summon Nature's Ally N` rows are the same shape.
            let namespaced = line.key_namespace().is_some();
            push(
                &mut findings,
                line,
                Trap::KeyDiffersFromName,
                format!(
                    "display name `{}` but KEY `{}`; join on the KEY ({})",
                    line.name,
                    key,
                    if namespaced {
                        "namespaced — the prefix makes the difference visible"
                    } else {
                        "flat rename — a bare-name grep will silently merge this with any \
                         other record of the same display name"
                    }
                ),
            );
        }

        // ------------------------------------------------------- trap 7
        if let Some((ns, leaf)) = line.key_namespace() {
            push(
                &mut findings,
                line,
                Trap::NamespacedKey,
                format!("namespace `{ns}`; leaf `{leaf}`"),
            );
        }

        // ------------------------------------------------------- trap 4
        if let Some(reason) = line.archetype_scope() {
            push(
                &mut findings,
                line,
                Trap::ArchetypeScoped,
                format!("archetype-scoped ({reason}); do not credit the base class"),
            );
        }

        // ------------------------------------------------------- trap 6
        let own_bonuses = line.bonus_vars();
        for var in line.defines_zero() {
            if own_bonuses.contains(&var) {
                continue;
            }
            let elsewhere: Vec<u32> = bonus_sites
                .get(&var)
                .map(|v| v.iter().copied().filter(|n| *n != line.line_no).collect())
                .unwrap_or_default();
            let detail = if elsewhere.is_empty() {
                format!(
                    "`{var}` is DEFINEd to 0 here and has no `BONUS:VAR` in this file; \
                     its value arrives from another file — widen the search before \
                     concluding it is 0"
                )
            } else {
                format!(
                    "`{var}` is DEFINEd to 0 here; its value is granted at {}",
                    elsewhere
                        .iter()
                        .map(|n| format!("{path}:{n}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };
            push(&mut findings, line, Trap::DefineZeroValueElsewhere, detail);
        }

        // ------------------------------------------------------- trap 8
        let dense = line.bonus_var_token_count();
        if dense >= TOKEN_DENSITY_THRESHOLD {
            push(
                &mut findings,
                line,
                Trap::TokenDenseRecord,
                format!(
                    "{dense} `BONUS:VAR` tokens on this single record; \
                     a token count here is {dense}x a record count"
                ),
            );
        }

        // ------------------------------------------------------ trap 10
        let governing = line.governing_tokens();
        if !governing.is_empty() && line.has_bonus_or_pre() {
            push(
                &mut findings,
                line,
                Trap::GoverningTokenHiddenByFilter,
                format!(
                    "carries {} alongside its BONUS/PRE tokens; a filtered grep drops them",
                    governing
                        .iter()
                        .map(|t| format!("`{t}`"))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            );
        }
    }

    // ----------------------------------------------------------- trap 5
    for (name, group) in &by_name {
        let identities: BTreeSet<&str> = group.iter().map(|l| l.identity()).collect();
        if identities.len() < 2 {
            continue;
        }
        for line in group {
            let others: Vec<String> = group
                .iter()
                .filter(|o| o.line_no != line.line_no)
                .map(|o| format!("`{}` at line {}", o.identity(), o.line_no))
                .collect();
            findings.push(Finding {
                file: path.to_string(),
                line: line.line_no,
                trap: Trap::SharedNameDistinctRecords,
                severity: Severity::Trap,
                record: (*name).to_string(),
                detail: format!(
                    "`{}` here, but the same display name is also {}; \
                     these are different records",
                    line.identity(),
                    others.join(" and ")
                ),
            });
        }
    }

    findings.sort_by_key(|f| (f.line, f.trap));
    findings
}

/// Tokens whose values differ between a disabled row and its live twin.
/// Restricted to the tokens that decide what a record *does*, because a
/// `SOURCEPAGE:` difference tells the reader nothing.
fn diverging_tokens(disabled: &LstLine, live: &LstLine) -> Vec<String> {
    const DECISIVE: &[&str] = &[
        "CLASSES", "TYPE", "CATEGORY", "SCHOOL", "BONUS", "PRECLASS", "KEY",
    ];
    let mut out = Vec::new();
    for name in DECISIVE {
        let a: Vec<&str> = disabled.token_values(name).collect();
        let b: Vec<&str> = live.token_values(name).collect();
        if a != b && !(a.is_empty() && b.is_empty()) {
            out.push((*name).to_string());
        }
    }
    out
}

// ===========================================================================
// Book scan
// ===========================================================================

/// Every `.lst` file in one book directory.
///
/// Deliberately keeps its per-file parts rather than only a total: a
/// number without a scope is trap 9.
#[derive(Debug, Clone)]
pub struct BookScan {
    pub book: String,
    pub files: Vec<FileScan>,
}

impl BookScan {
    pub fn declaring_lines(&self) -> usize {
        self.files.iter().map(FileScan::declaring_lines).sum()
    }

    pub fn declarations(&self) -> usize {
        self.files.iter().map(|f| f.declarations).sum()
    }

    pub fn modifications(&self) -> usize {
        self.files.iter().map(|f| f.modifications).sum()
    }

    pub fn copies(&self) -> usize {
        self.files.iter().map(|f| f.copies).sum()
    }

    pub fn disabled_records(&self) -> usize {
        self.files.iter().map(|f| f.disabled_records).sum()
    }

    pub fn findings(&self) -> impl Iterator<Item = &Finding> {
        self.files.iter().flat_map(FileScan::findings)
    }

    pub fn findings_for(&self, trap: Trap) -> impl Iterator<Item = &Finding> {
        self.findings().filter(move |f| f.trap == trap)
    }

    pub fn count_for(&self, trap: Trap) -> usize {
        self.findings_for(trap).count()
    }

    pub fn counts_by_trap(&self) -> BTreeMap<Trap, usize> {
        let mut out = BTreeMap::new();
        for f in self.findings() {
            *out.entry(f.trap).or_insert(0) += 1;
        }
        out
    }

    /// `KEY:` namespaces across the whole book, with leaf counts.
    pub fn namespaces(&self) -> BTreeMap<String, usize> {
        let mut out: BTreeMap<String, usize> = BTreeMap::new();
        for file in &self.files {
            for (ns, n) in file.namespaces() {
                *out.entry(ns).or_insert(0) += n;
            }
        }
        out
    }
}

/// Scan every `.lst` file directly inside `book_dir` (and its immediate
/// `support/` subdirectory, which several books use for cross-book
/// patches).
pub fn scan_book(book_dir: &Path) -> std::io::Result<BookScan> {
    let book = book_dir
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| book_dir.display().to_string());

    let mut paths = Vec::new();
    collect_lst_paths(book_dir, &mut paths)?;
    if let Ok(support) = std::fs::read_dir(book_dir.join("support")) {
        for entry in support.flatten() {
            let p = entry.path();
            if p.extension().is_some_and(|e| e == "lst") {
                paths.push(p);
            }
        }
    }
    paths.sort();

    let mut files = Vec::with_capacity(paths.len());
    for path in paths {
        // The corpus is upstream data with occasional non-UTF-8 bytes;
        // lossy decoding keeps the scan honest about line numbers rather
        // than aborting the whole book over one byte.
        let bytes = std::fs::read(&path)?;
        let text = String::from_utf8_lossy(&bytes);
        files.push(scan_lst(&path.display().to_string(), &text));
    }

    Ok(BookScan { book, files })
}

fn collect_lst_paths(dir: &Path, out: &mut Vec<std::path::PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_file() && path.extension().is_some_and(|e| e == "lst") {
            out.push(path);
        }
    }
    Ok(())
}

// ===========================================================================
// Concept census — trap 9, made structural
// ===========================================================================

/// A count that cannot be quoted without its scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConceptCensus {
    /// What was counted.
    pub needle: String,
    /// Book directory name → matching live lines in that book.
    pub per_book: BTreeMap<String, usize>,
    /// Sum of `per_book`. Always shown together with its parts.
    pub total: usize,
}

impl fmt::Display for ConceptCensus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "`{}` — {} live lines across {} books:", self.needle, self.total, self.per_book.len())?;
        for (book, n) in &self.per_book {
            let pct = if self.total == 0 { 0.0 } else { *n as f64 * 100.0 / self.total as f64 };
            writeln!(f, "  {n:>6}  ({pct:>5.1}%)  {book}")?;
        }
        Ok(())
    }
}

/// Count live lines containing `needle` in every book under `books_dir`.
///
/// This exists because a book-scoped subtotal quoted as a corpus total is
/// how a "27 hexes" figure became a claim about the whole corpus. The
/// return type makes that impossible: there is no way to get the total
/// without also getting the breakdown that produced it.
///
/// Disabled and blank lines are excluded, so the census never counts a
/// suppressed row.
pub fn concept_census(books_dir: &Path, needle: &str) -> std::io::Result<ConceptCensus> {
    let mut per_book = BTreeMap::new();

    let mut books: Vec<std::path::PathBuf> = std::fs::read_dir(books_dir)?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    books.sort();

    for book_dir in books {
        let scan = scan_book(&book_dir)?;
        let n: usize = scan
            .files
            .iter()
            .map(|f| {
                f.lines
                    .iter()
                    .filter(|l| {
                        !l.disabled
                            && l.shape != RecordShape::Blank
                            && (l.name.contains(needle)
                                || l.tokens.iter().any(|(tn, tv)| {
                                    tn.contains(needle) || tv.contains(needle)
                                }))
                    })
                    .count()
            })
            .sum();
        if n > 0 {
            per_book.insert(scan.book, n);
        }
    }

    let total = per_book.values().sum();
    Ok(ConceptCensus {
        needle: needle.to_string(),
        per_book,
        total,
    })
}

// ===========================================================================
// Ingested-cache audit — the ratchet surface
// ===========================================================================

/// Cross-check every JSON record under `cache_dir` against the corpus
/// line its `source` citation claims.
///
/// Unlike [`scan_book`], findings here can be [`Severity::Defect`]: a
/// contradiction between an ingested record and the line it cites is our
/// error, not upstream's. `.MOD`-sourced records are the one deliberate
/// exception — PCGen genuinely splits a record across a declaring row and
/// `.MOD` rows carrying `DESC:`/`ITEM:`, and citing the row a field
/// actually lives on is correct. Those are reported at
/// [`Severity::Trap`], and only become defects when the `.MOD` has no
/// live base declaration in its file, which would mean the ingest
/// manufactured a record out of a modification.
pub fn audit_ingested_cache(cache_dir: &Path, corpus_root: &Path) -> std::io::Result<Vec<Finding>> {
    let mut findings = Vec::new();
    let mut file_cache: BTreeMap<String, FileScan> = BTreeMap::new();
    // (book, kind, record_key) -> cache file, for collision detection.
    let mut seen_keys: BTreeMap<(String, String, String), String> = BTreeMap::new();

    let mut books: Vec<std::path::PathBuf> = std::fs::read_dir(cache_dir)?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    books.sort();

    for book_dir in books {
        let book = book_dir.file_name().unwrap_or_default().to_string_lossy().to_string();
        let mut kinds: Vec<std::path::PathBuf> = std::fs::read_dir(&book_dir)?
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .collect();
        kinds.sort();

        for kind_dir in kinds {
            let kind = kind_dir.file_name().unwrap_or_default().to_string_lossy().to_string();
            let mut records: Vec<std::path::PathBuf> = std::fs::read_dir(&kind_dir)?
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|e| e == "json"))
                .collect();
            records.sort();

            for record_path in records {
                let text = std::fs::read_to_string(&record_path)?;
                let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else {
                    findings.push(Finding {
                        file: record_path.display().to_string(),
                        line: 0,
                        trap: Trap::UnresolvableCitation,
                        severity: Severity::Defect,
                        record: String::new(),
                        detail: "cache record is not valid JSON".to_string(),
                    });
                    continue;
                };
                let source = &json["source"];
                // Records sourced from a second source (web errata, a
                // same-book fallback, a corrected ingest) have no corpus
                // line to check against.
                if source["kind"].as_str() != Some("lst_token") {
                    continue;
                }
                let (Some(rel), Some(line_no)) =
                    (source["path"].as_str(), source["line"].as_u64())
                else {
                    findings.push(Finding {
                        file: record_path.display().to_string(),
                        line: 0,
                        trap: Trap::UnresolvableCitation,
                        severity: Severity::Defect,
                        record: String::new(),
                        detail: "lst_token source is missing `path` or `line`".to_string(),
                    });
                    continue;
                };
                let record_key = source["record_key"].as_str().unwrap_or_default().to_string();
                let cache_file = record_path.display().to_string();

                if let Some(prev) = seen_keys.insert(
                    (book.clone(), kind.clone(), record_key.clone()),
                    cache_file.clone(),
                ) {
                    findings.push(Finding {
                        file: cache_file.clone(),
                        line: line_no as u32,
                        trap: Trap::SharedNameDistinctRecords,
                        severity: Severity::Defect,
                        record: record_key.clone(),
                        detail: format!(
                            "record_key `{record_key}` is already used by {prev} in \
                             {book}/{kind}; two records cannot share one identity"
                        ),
                    });
                }

                let scan = match file_cache.get(rel) {
                    Some(s) => s,
                    None => {
                        let abs = corpus_root.join(rel);
                        let Ok(bytes) = std::fs::read(&abs) else {
                            findings.push(Finding {
                                file: cache_file.clone(),
                                line: line_no as u32,
                                trap: Trap::UnresolvableCitation,
                                severity: Severity::Defect,
                                record: record_key.clone(),
                                detail: format!("cited corpus file is unreadable: {}", abs.display()),
                            });
                            continue;
                        };
                        let text = String::from_utf8_lossy(&bytes);
                        file_cache.insert(rel.to_string(), scan_lst(rel, &text));
                        &file_cache[rel]
                    }
                };

                let Some(cited) = scan.lines.get(line_no as usize - 1) else {
                    findings.push(Finding {
                        file: cache_file.clone(),
                        line: line_no as u32,
                        trap: Trap::UnresolvableCitation,
                        severity: Severity::Defect,
                        record: record_key.clone(),
                        detail: format!("{rel} has fewer than {line_no} lines"),
                    });
                    continue;
                };

                if cited.shape == RecordShape::Blank {
                    findings.push(Finding {
                        file: cache_file.clone(),
                        line: line_no as u32,
                        trap: Trap::UnresolvableCitation,
                        severity: Severity::Defect,
                        record: record_key.clone(),
                        detail: format!("{rel}:{line_no} is blank"),
                    });
                    continue;
                }

                if cited.disabled {
                    findings.push(Finding {
                        file: cache_file.clone(),
                        line: line_no as u32,
                        trap: Trap::DisabledLine,
                        severity: Severity::Defect,
                        record: record_key.clone(),
                        detail: format!(
                            "{rel}:{line_no} is `#`-disabled; a suppressed row must never \
                             be the source of an ingested record"
                        ),
                    });
                    continue;
                }

                if cited.shape == RecordShape::Modification {
                    let has_base = scan.lines.iter().any(|l| {
                        !l.disabled && l.shape.declares_a_record() && l.name == cited.base_name
                    });
                    findings.push(Finding {
                        file: cache_file.clone(),
                        line: line_no as u32,
                        trap: Trap::ModRecord,
                        severity: if has_base { Severity::Trap } else { Severity::Defect },
                        record: record_key.clone(),
                        detail: if has_base {
                            format!(
                                "{rel}:{line_no} is a `.MOD` on `{}`, which is declared \
                                 live in the same file — citing the row a field lives on \
                                 is correct, but this row must not be counted as a \
                                 declaration",
                                cited.base_name
                            )
                        } else {
                            format!(
                                "{rel}:{line_no} is a `.MOD` on `{}` with no live base \
                                 declaration in that file: this record was manufactured \
                                 out of a modification",
                                cited.base_name
                            )
                        },
                    });
                }

                if let Some(key) = &cited.key
                    && !record_key.is_empty()
                    && key != &record_key
                {
                    findings.push(Finding {
                        file: cache_file.clone(),
                        line: line_no as u32,
                        trap: Trap::KeyDiffersFromName,
                        severity: Severity::Defect,
                        record: record_key.clone(),
                        detail: format!(
                            "ingested as `{record_key}`, but {rel}:{line_no} declares \
                             KEY `{key}`; the ingest filed an archetype-qualified record \
                             under a different record's identity"
                        ),
                    });
                }
            }
        }
    }

    Ok(findings)
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn category_qualifier_on_field_zero_is_stripped_from_the_name() {
        // Real shape from `cr_feats.lst`: the qualifier selects which
        // category's `Arcane Strike` is being modified.
        let line = parse_line(1, "CATEGORY=FEAT|Arcane Strike.MOD\tBONUS:VAR|ArcaneStrikeLVL|1");
        assert_eq!(line.shape, RecordShape::Modification);
        assert_eq!(line.name, "Arcane Strike");
        assert_eq!(line.base_name, "Arcane Strike");
        assert_eq!(line.field_qualifier.as_deref(), Some("CATEGORY=FEAT"));
    }

    #[test]
    fn identity_prefers_key_over_display_name() {
        let keyed = parse_line(1, "Cackle\tKEY:Witch Hex ~ Cackle\tCATEGORY:Special Ability");
        assert_eq!(keyed.identity(), "Witch Hex ~ Cackle");
        let unkeyed = parse_line(1, "Cackle\tCATEGORY:Special Ability");
        assert_eq!(unkeyed.identity(), "Cackle");
    }

    #[test]
    fn empty_padding_fields_do_not_become_tokens() {
        // PCGen tab-aligns its columns; a real record can carry dozens of
        // empty fields between tokens.
        let line = parse_line(1, "Toughness\t\t\t\tCATEGORY:FEAT\t\t\tTYPE:General");
        assert_eq!(line.shape, RecordShape::Declaration);
        assert_eq!(line.tokens.len(), 2);
        assert_eq!(line.category.as_deref(), Some("FEAT"));
    }

    #[test]
    fn bonus_var_parsing_reads_the_variable_not_the_formula() {
        let line = parse_line(
            1,
            "X\tBONUS:VAR|ArcaneStrikeLVL|min((1+(charbonusto(\"CASTERLEVEL\",\"Wizard\"))/5),5)",
        );
        assert_eq!(line.bonus_vars(), vec!["ArcaneStrikeLVL".to_string()]);
        assert_eq!(line.bonus_var_token_count(), 1);
    }

    #[test]
    fn define_with_a_nonzero_initial_value_is_not_a_zero_define() {
        let line = parse_line(1, "X\tDEFINE:A|0\tDEFINE:B|3");
        assert_eq!(line.defines_zero(), vec!["A".to_string()]);
    }

    #[test]
    fn non_bonus_var_bonuses_are_not_counted_as_var_tokens() {
        let line = parse_line(1, "Toughness\tBONUS:HP|CURRENTMAX|3\tBONUS:VAR|X|1");
        assert_eq!(line.bonus_var_token_count(), 1);
    }

    #[test]
    fn key_namespace_splits_on_the_spaced_tilde_only() {
        let ns = parse_line(1, "X\tKEY:Rage Power ~ Animal Fury");
        assert_eq!(ns.key_namespace(), Some(("Rage Power", "Animal Fury")));
        // A tilde without the surrounding spaces is part of the name.
        let not_ns = parse_line(1, "X\tKEY:Blindness/Deafness~Only");
        assert_eq!(not_ns.key_namespace(), None);
    }

    #[test]
    fn every_trap_has_a_distinct_id_and_a_miscount_sentence() {
        let traps = [
            Trap::ModRecord,
            Trap::DisabledLine,
            Trap::KeyDiffersFromName,
            Trap::ArchetypeScoped,
            Trap::SharedNameDistinctRecords,
            Trap::DefineZeroValueElsewhere,
            Trap::NamespacedKey,
            Trap::TokenDenseRecord,
            Trap::GoverningTokenHiddenByFilter,
            Trap::CopyRecord,
            Trap::UnresolvableCitation,
        ];
        let ids: BTreeSet<&str> = traps.iter().map(|t| t.id()).collect();
        assert_eq!(ids.len(), traps.len(), "trap ids must be unique");
        for t in traps {
            assert!(!t.miscount_risk().is_empty(), "{t:?} needs a miscount sentence");
        }
    }

    #[test]
    fn declaring_lines_counts_declarations_and_copies_but_never_mods() {
        let scan = scan_lst(
            "t.lst",
            "A\tTYPE:x\nA.MOD\tDESC:y\nA.COPY=B\tDESC:z\n#C\tTYPE:x\n",
        );
        assert_eq!(scan.declarations, 1);
        assert_eq!(scan.modifications, 1);
        assert_eq!(scan.copies, 1);
        assert_eq!(scan.disabled_records, 1);
        assert_eq!(scan.declaring_lines(), 2);
    }

    #[test]
    fn a_disabled_row_is_reported_once_as_disabled_not_under_every_other_trap() {
        // The disabled row below would otherwise also trip the namespaced
        // key and governing-token detectors, burying the live findings.
        let scan = scan_lst(
            "t.lst",
            "#Cackle\tKEY:Witch Hex ~ Cackle\tMULT:YES\tBONUS:VAR|X|1\n",
        );
        assert_eq!(scan.findings().count(), 1);
        assert_eq!(scan.count_for(Trap::DisabledLine), 1);
        assert_eq!(scan.count_for(Trap::NamespacedKey), 0);
    }

    #[test]
    fn findings_are_ordered_by_line_so_a_report_reads_like_the_file() {
        let scan = scan_lst(
            "t.lst",
            "A\tKEY:NS ~ A\nB.MOD\tDESC:x\nC\tKEY:NS ~ C\n",
        );
        let lines: Vec<u32> = scan.findings().map(|f| f.line).collect();
        let mut sorted = lines.clone();
        sorted.sort_unstable();
        assert_eq!(lines, sorted);
    }
}
