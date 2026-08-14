//! `wiring_class` determination (GE-01, `docs/release/GE-01-legacy-corpus-and-conversion-matrix/artifacts/wiring-class-determination.md`).
//!
//! This is the single production port of the reference determinator
//! (`wiring-class-determination.py`, a documentary artifact, not
//! production code). Every consumer — `v06_work_inventory`'s classifier
//! and `rules_core::cache_gen`'s per-book generators — calls this module
//! rather than reimplementing the rules, so the two surfaces cannot drift
//! against each other.
//!
//! `MAGNITUDE_TOKENS` lives here as the single definition; nothing else
//! in the crate may declare a second copy (`wiring-class-determination.md`
//! "Magnitude-bearing fields").

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

/// Tab-field prefixes that carry a real numeric magnitude. The single
/// definition; `v06_work_inventory`'s own generator selection logic uses
/// this same constant rather than a forked copy.
pub const MAGNITUDE_TOKENS: &[&str] = &[
    "BONUS:",
    "TEMPBONUS:",
    "DEFINE:",
    "COST:",
    "WT:",
    "CR:",
    "AC:",
    "ACCHECK:",
    "DAMAGE:",
    "CRITMULT:",
    "CRITRANGE:",
    "RANGE:",
    "REACH:",
    "MOVE:",
    "HITDIE:",
    "LEVELADJUSTMENT:",
    "SR:",
    "DR:",
    "SPELLFAILURE:",
    "STAT:",
];

/// PCGen keyword ranges whose real value is a function of caster level.
const RANGE_KEYWORDS: &[&str] = &["Close", "Medium", "Long"];

/// Upstream PCGen's own admission that a record is not mechanically
/// implemented. Reported separately (`carries_upstream_not_implemented_marker`)
/// and MUST NOT feed `wiring_class` in either direction: it is not stripped
/// from scanned text, and it never short-circuits a classification.
pub const UPSTREAM_NOT_IMPLEMENTED: &str = "[Not Implemented]";

/// Detection tables for the two rules that are most likely to move as the
/// GE-01 spec is field-corrected: which prose fields get scanned for a
/// parenthesised expression or a scaling phrase, and which literal phrases
/// count as "scaling stated only in English". Kept as data — not
/// hardcoded inside `signals()` — so absorbing a revision (e.g. adding
/// `BENEFIT:` to the scanned fields, or a new phrase) is a table edit,
/// not a rewrite of the scan logic.
#[derive(Debug, Clone)]
pub struct SignalRules {
    /// Fields whose value is prose but which PCGen allows to carry a
    /// parenthesised expression the renderer substitutes, e.g.
    /// `(min(10,CASTERLEVEL))d6`.
    pub prose_fields: &'static [&'static str],
    /// Literal substrings that mark scaling stated only in English prose
    /// (checked case-insensitively), beyond the `per N levels` / `every N
    /// levels` numeric-pattern check `has_prose_scaling_phrase` always runs.
    pub prose_scaling_phrases: &'static [&'static str],
}

impl Default for SignalRules {
    fn default() -> Self {
        SignalRules {
            // `BENEFIT:` joins the scan (`docs/release/.../wiring-class-determination.py`
            // commit 9e9e6993): 2,087 corpus rows carry a record's mechanical
            // benefit text in it, and a `.MOD BENEFIT:` row is exactly where a
            // token-closure magnitude like "spell resistance equal to 5 + your
            // character level" lives.
            prose_fields: &[
                "DESC:",
                "DURATION:",
                "TARGETAREA:",
                "SPROP:",
                "RANGE:",
                "SPECIALS:",
                "BENEFIT:",
            ],
            prose_scaling_phrases: &[
                "per caster level",
                "per level",
                "per two levels",
                "per three levels",
                "per four levels",
                "per five levels",
                "x your caster level",
                "times your caster level",
                "caster level (max",
                // Added from the `ultimate_campaign` story feats' `.MOD
                // BENEFIT:` rows (commit 9e9e6993).
                "your character level",
                "your class level",
                "your total level",
                "per hit die",
                "per hit dice",
                "per hd",
                // Ability-score/modifier/bonus phrases are NOT here: commit
                // 2e2ba619 found a bare literal match over-flags -- PF1's
                // flat-footed idiom ("you don't lose your Dexterity bonus
                // to AC") is a REFERENCE to an existing rule, not a new
                // magnitude, and appears throughout the corpus granting
                // nothing. `ability_scaling` (below) is the grant-vs-refer
                // discriminator that replaces a bare phrase match for
                // those six abilities.
            ],
        }
    }
}

/// An ability-score phrase is a scaling magnitude ONLY when a granting
/// construction introduces it. Bare mention is overwhelmingly a
/// cross-reference to an existing rule, not a new magnitude this record
/// computes (`wiring-class-determination.py` commit 2e2ba619, "D4b
/// over-flagging").
const ABILITY_NAMES: &[&str] =
    &["strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"];
const ABILITY_SUFFIXES: &[&str] = &["score", "modifier", "bonus"];

const ABILITY_GRANT_WORDS: &[&str] = &[
    "add",
    "adds",
    "adding",
    "gain",
    "gains",
    "gaining",
    "equal to",
    "plus",
    "minus",
    "times",
    "increase by",
    "increased by",
    "increases by",
    "bonus of",
];

const ABILITY_REFER_WORDS: &[&str] = &[
    "lose",
    "loses",
    "losing",
    "lost",
    "retain",
    "retains",
    "retaining",
    "deny",
    "denies",
    "denied",
    "deprived of",
    "instead of",
    "rather than",
    "in place of",
    "whichever is",
];

fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Byte-slice `starts_with`, operating on raw bytes rather than `&str` so a
/// scan position that lands inside a multi-byte UTF-8 character (real
/// corpus text carries the odd `°`/curly-quote) never panics on a slice
/// that isn't a char boundary.
fn bytes_start_with(haystack: &[u8], at: usize, needle: &str) -> bool {
    let n = needle.as_bytes();
    at + n.len() <= haystack.len() && &haystack[at..at + n.len()] == n
}

/// `\byour (Strength|Dexterity|...) (score|modifier|bonus)` — every start
/// byte offset (into `field`) of an occurrence, case-insensitive.
fn ability_phrase_occurrences(field: &str) -> Vec<usize> {
    let lower = field.to_ascii_lowercase();
    let bytes = lower.as_bytes();
    let mut out = Vec::new();
    for start in 0..bytes.len() {
        let left_ok = start == 0 || !is_word_byte(bytes[start - 1]);
        if !left_ok || !bytes_start_with(bytes, start, "your ") {
            continue;
        }
        let after_your = start + "your ".len();
        for name in ABILITY_NAMES {
            if !bytes_start_with(bytes, after_your, name) {
                continue;
            }
            let after_name = after_your + name.len();
            if after_name >= bytes.len() || bytes[after_name] != b' ' {
                continue;
            }
            let after_space = after_name + 1;
            for suffix in ABILITY_SUFFIXES {
                if !bytes_start_with(bytes, after_space, suffix) {
                    continue;
                }
                let end = after_space + suffix.len();
                let right_ok = end >= bytes.len() || !is_word_byte(bytes[end]);
                if right_ok {
                    out.push(start);
                }
            }
        }
    }
    out
}

/// `\b(?:word1|word2|...)\b[^.;|]{0,30}$` against `lead`: the LEFTMOST
/// (earliest-starting) construction word whose suffix, up to `lead`'s end,
/// is <=30 characters and carries none of `.`, `;`, `|`. Returns that
/// word's start byte offset within `lead`.
fn find_construction(lead: &str, words: &[&str]) -> Option<usize> {
    let lower = lead.to_ascii_lowercase();
    let bytes = lower.as_bytes();
    for start in 0..bytes.len() {
        let left_ok = start == 0 || !is_word_byte(bytes[start - 1]);
        if !left_ok {
            continue;
        }
        for w in words {
            if !bytes_start_with(bytes, start, w) {
                continue;
            }
            let end = start + w.len();
            let right_ok = end >= bytes.len() || !is_word_byte(bytes[end]);
            if !right_ok {
                continue;
            }
            let suffix = &bytes[end..];
            if suffix.len() <= 30 && !suffix.iter().any(|b| matches!(b, b'.' | b';' | b'|')) {
                return Some(start);
            }
        }
    }
    None
}

/// True if `field` GRANTS a magnitude derived from an ability score, as
/// opposed to merely referencing one. Decided per occurrence: a field may
/// both grant one magnitude and reference another, so for each occurrence
/// whichever construction sits nearest before it (highest start offset in
/// the 45-char lookback window) wins.
fn ability_scaling(field: &str) -> bool {
    let bytes = field.as_bytes();
    for start in ability_phrase_occurrences(field) {
        let lead_start = start.saturating_sub(45);
        let end = start.min(bytes.len());
        // `String::from_utf8_lossy` rather than a `&str` slice: `start`
        // came from a byte scan and may fall inside a multi-byte UTF-8
        // character (real corpus text carries the odd `°`/curly-quote), so
        // an exact `&field[..]` slice could panic on a non-char-boundary.
        // A lossy conversion is always safe and the ASCII grant/refer
        // words this scans for are unaffected by an occasional replaced
        // byte at a slice edge.
        let lead = String::from_utf8_lossy(&bytes[lead_start..end]).into_owned();
        let g = find_construction(&lead, ABILITY_GRANT_WORDS);
        let r = find_construction(&lead, ABILITY_REFER_WORDS);
        match (g, r) {
            (Some(g), Some(r)) if g > r => return true,
            (Some(_), None) => return true,
            _ => {}
        }
    }
    false
}

/// The determined class. Ordered `Display < Static < Derived < Computed`
/// — the strict evidence lattice `wiring-class-determination.md`
/// "Ordering" defines. `Ambiguous` is not part of the lattice: it is a
/// determination failure, not a class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WiringClass {
    Display,
    Static,
    Derived,
    Computed,
    Ambiguous,
}

impl WiringClass {
    pub fn id(self) -> &'static str {
        match self {
            WiringClass::Display => "display",
            WiringClass::Static => "static",
            WiringClass::Derived => "derived",
            WiringClass::Computed => "computed",
            WiringClass::Ambiguous => "ambiguous",
        }
    }
}

/// Split one raw `.lst` line into its tab fields, trimmed of surrounding
/// whitespace, matching how PCGen itself reads them.
fn tab_fields(line: &str) -> Vec<String> {
    line.trim_end_matches(['\n', '\r'])
        .split('\t')
        .map(|f| f.trim().to_string())
        .filter(|f| !f.is_empty())
        .collect()
}

/// Scalar names the reference `SCALARS` regex matches as a bare substring
/// (no `\b` in the Python pattern).
const SCALARS_SUBSTRING: &[&str] =
    &["CASTERLEVEL", "CLASSLEVEL", "TOTALLEVELS", "TOTALLEVEL", "PLUSTOTAL", "SPELLLEVEL"];

/// Scalar names the reference `SCALARS` regex wraps in `\b...\b` — short
/// enough (`HD`, `CL`, `STR`, ...) that an unbounded substring match would
/// false-positive on ordinary words (`CRITMULT` contains `TL` fine, but
/// `SCALE` would fabricate a `CL` hit without a boundary check).
const SCALARS_WORD: &[&str] =
    &["BAB", "HD", "STR", "DEX", "CON", "INT", "WIS", "CHA", "TL", "CL", "RACESIZE"];

/// `\b<needle>\b` — ASCII word boundary either side, matching Python's
/// `\b` for the short scalar names.
fn contains_word(haystack: &str, needle: &str) -> bool {
    let is_word_byte = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    let hb = haystack.as_bytes();
    let nb = needle.as_bytes();
    if nb.is_empty() || nb.len() > hb.len() {
        return false;
    }
    for start in 0..=(hb.len() - nb.len()) {
        if &hb[start..start + nb.len()] != nb {
            continue;
        }
        let left_ok = start == 0 || !is_word_byte(hb[start - 1]);
        let right = start + nb.len();
        let right_ok = right == hb.len() || !is_word_byte(hb[right]);
        if left_ok && right_ok {
            return true;
        }
    }
    false
}

/// `SCALARS.search(value)` — is `value` a function of a character/item
/// scalar, at all (used alone by the `derived:prose_expr` rule, which does
/// NOT also accept bare arithmetic).
fn has_scalar(value: &str) -> bool {
    SCALARS_SUBSTRING.iter().any(|s| value.contains(s))
        || SCALARS_WORD.iter().any(|s| contains_word(value, s))
}

/// `ARITH.search(value)`: `[*/]|\+\s*\w*[A-Z]{2,}|MIN\(|MAX\(|min\(|max\(`.
fn has_arith(value: &str) -> bool {
    if value.contains('*') || value.contains('/') {
        return true;
    }
    if value.to_ascii_lowercase().contains("min(") || value.to_ascii_lowercase().contains("max(")
    {
        return true;
    }
    // `+\s*\w*[A-Z]{2,}`: a `+` followed by optional whitespace, then a
    // MAXIMAL run of word characters (any case), somewhere within which
    // two-or-more consecutive uppercase letters appear -- not necessarily
    // at the very end. `\w*` backtracks to let `[A-Z]{2,}` match at the
    // rightmost qualifying point, so the check is "does this word run
    // contain a 2+ consecutive uppercase substring anywhere", e.g.
    // `Sorcerer_Arcane_BloodlineLVL+BloodlinePower1LVLBonus`: the word run
    // is `BloodlinePower1LVLBonus`, which contains `LV`/`VL`/`LB` runs of
    // 2+ uppercase letters in its middle, not at its tail.
    let bytes = value.as_bytes();
    for (i, b) in bytes.iter().enumerate() {
        if *b != b'+' {
            continue;
        }
        let mut j = i + 1;
        while j < bytes.len() && bytes[j].is_ascii_whitespace() {
            j += 1;
        }
        let word_start = j;
        while j < bytes.len() && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') {
            j += 1;
        }
        let word = &bytes[word_start..j];
        let mut run = 0usize;
        for wb in word {
            if wb.is_ascii_uppercase() {
                run += 1;
                if run >= 2 {
                    return true;
                }
            } else {
                run = 0;
            }
        }
    }
    false
}

/// `SCALARS.search(value) or ARITH.search(value)` — the `derived:<token>`
/// rule for a `MAGNITUDE_TOKENS` field's own value.
fn has_scalar_or_arith(value: &str) -> bool {
    has_scalar(value) || has_arith(value)
}

/// `(^|\|)!?PRE(?!RULE)[A-Z]+:` — a conditional guard, excluding
/// `PRERULE` (a renderer directive, not a rules guard;
/// `wiring-class-determination.md` "PRERULE is excluded...").
fn has_guard(field: &str) -> bool {
    let mut starts: Vec<&str> = vec![field];
    for (i, _) in field.match_indices('|') {
        starts.push(&field[i + 1..]);
    }
    starts.iter().any(|s| {
        let s = s.strip_prefix('!').unwrap_or(s);
        let Some(rest) = s.strip_prefix("PRE") else { return false };
        if rest.starts_with("RULE") {
            return false;
        }
        // Must be `[A-Z]+:` immediately after `PRE`.
        let upto_colon: String = rest.chars().take_while(|c| c.is_ascii_uppercase()).collect();
        !upto_colon.is_empty() && rest.as_bytes().get(upto_colon.len()) == Some(&b':')
    })
}

/// Extract the top-level parenthesised groups of a field's value (after
/// the `TOKEN:` prefix), same shape as the reference `PAREN` regex:
/// balanced groups, not nested capture.
fn paren_groups(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '(' {
            let mut depth = 1;
            let mut j = i + 1;
            while j < chars.len() && depth > 0 {
                match chars[j] {
                    '(' => depth += 1,
                    ')' => depth -= 1,
                    _ => {}
                }
                j += 1;
            }
            if depth == 0 {
                out.push(chars[i + 1..j - 1].iter().collect());
                i = j;
                continue;
            }
        }
        i += 1;
    }
    out
}

/// True if `field` contains a PCGen `%N` render-time substitution
/// placeholder (`%1`, `%2`, ...).
fn field_has_percent_placeholder(field: &str) -> bool {
    let b = field.as_bytes();
    (0..b.len()).any(|i| b[i] == b'%' && b.get(i + 1).is_some_and(u8::is_ascii_digit))
}

/// A `%N` placeholder in a prose field is a genuine magnitude when the SAME
/// field later carries, pipe-delimited, the formula PCGen substitutes for
/// it -- e.g. `DESC:...a DC %1 Strength or Escape Artist check.|CON+18`
/// (`bestiary_4/b4_abilities_race.lst:1490`, "Zomok Breath Weapon": the DC
/// is `10+(HD/2)+CON`, stated only via this substitution, not a
/// `MAGNITUDE_TOKENS` field and not a parenthesised expression
/// `paren_groups` scans for). Every pipe-delimited segment after the
/// field's own text is a formula candidate; a `PRE*` guard segment is
/// excluded (that is a condition, never the substituted value), and a
/// candidate counts only if it itself carries a scalar or arithmetic
/// construction (`STR*1.5`, `10+HD/2+CON`) -- a bare cross-reference like
/// `SpecialArrowDC` (`bestiary/b1_abilities_race.lst:1039`, "Pixie Charm")
/// names a value defined elsewhere and is left undetermined rather than
/// guessed.
fn has_prose_formula_segment(field: &str) -> bool {
    if !field_has_percent_placeholder(field) {
        return false;
    }
    let mut segments = field.split('|');
    segments.next(); // the field's own tag/text, never itself the formula
    segments.any(|seg| {
        let trimmed = seg.trim();
        !trimmed.is_empty() && !has_guard(trimmed) && has_scalar_or_arith(trimmed)
    })
}

fn has_prose_scaling_phrase(field: &str, rules: &SignalRules) -> bool {
    let lower = field.to_ascii_lowercase();
    if rules.prose_scaling_phrases.iter().any(|p| lower.contains(p)) {
        return true;
    }
    // `per \d+ (caster )?levels?` and `every \d+ levels`: the digit MUST be
    // followed by `level(s)` (optionally through a `caster` word for
    // `per`), not just any digit-led word -- `struck once every 10
    // minutes` must not match.
    let words: Vec<&str> = lower.split_whitespace().collect();
    let starts_with_digit = |s: &str| s.chars().next().is_some_and(|c| c.is_ascii_digit());
    let is_level_word = |s: &str| {
        let s = s.trim_end_matches(|c: char| c.is_ascii_punctuation());
        s == "level" || s == "levels"
    };
    for i in 0..words.len() {
        if words[i] == "per" && words.get(i + 1).is_some_and(|w| starts_with_digit(w)) {
            let after = &words[i + 2..];
            if after.first().is_some_and(|w| is_level_word(w))
                || (after.first() == Some(&"caster")
                    && after.get(1).is_some_and(|w| is_level_word(w)))
            {
                return true;
            }
        }
        if words[i] == "every"
            && words.get(i + 1).is_some_and(|w| starts_with_digit(w))
            && words.get(i + 2).is_some_and(|w| is_level_word(w))
        {
            return true;
        }
    }
    false
}

/// Return the SET of signals one raw `.lst` row carries. A row may carry
/// several; `classify` collapses the set to one class. Uses the default
/// [`SignalRules`]; call [`signals_with_rules`] to scan against a
/// different prose-field / phrase table.
pub fn signals(raw: &str) -> BTreeSet<String> {
    signals_with_rules(raw, &SignalRules::default())
}

/// [`signals`], parameterised on the prose-field and scaling-phrase
/// tables, so a spec revision to either is a data change here rather
/// than a rewrite of the scan itself.
pub fn signals_with_rules(raw: &str, rules: &SignalRules) -> BTreeSet<String> {
    let fields = tab_fields(raw);
    let mags: Vec<&String> =
        fields.iter().filter(|f| MAGNITUDE_TOKENS.iter().any(|t| f.starts_with(t))).collect();
    let mut out: BTreeSet<String> = BTreeSet::new();

    // A `computed:` signal only matters if there is a magnitude for it to
    // govern. A guard or a choice on a row with no magnitude token gates
    // TEXT, and text is `display` work under the standing ruling.
    if !mags.is_empty() {
        for f in &fields {
            if f.starts_with("TEMPBONUS:") {
                out.insert("computed:tempbonus".to_string());
            }
            if f.contains("%CHOICE") || f.starts_with("CHOOSE:") {
                out.insert("computed:choice".to_string());
            }
            if has_guard(f) {
                out.insert("computed:pre_guard".to_string());
            }
        }
    }

    for f in &mags {
        let (token, value) = f.split_once(':').unwrap_or((f.as_str(), ""));
        if token == "RANGE" && RANGE_KEYWORDS.contains(&value.trim()) {
            out.insert("derived:range_keyword".to_string());
        }
        if has_scalar_or_arith(value) {
            out.insert(format!("derived:{}", token.to_ascii_lowercase()));
        }
    }

    for f in &fields {
        if rules.prose_fields.iter().any(|p| f.starts_with(p)) {
            for group in paren_groups(f) {
                if has_scalar(&group) {
                    out.insert("derived:prose_expr".to_string());
                }
            }
            if has_prose_formula_segment(f) {
                out.insert("derived:prose_formula_segment".to_string());
            }
            if has_prose_scaling_phrase(f, rules) {
                out.insert("ambiguous:prose_scaling_phrase".to_string());
            } else if ability_scaling(f) {
                out.insert("ambiguous:prose_ability_scaling".to_string());
            }
        }
    }

    if mags.is_empty() {
        out.insert("display:no_magnitude_token".to_string());
    } else if !out.iter().any(|s| s.starts_with("computed:") || s.starts_with("derived:")) {
        out.insert("static:literal_magnitudes_only".to_string());
    }
    out
}

/// The signal set for a unit with no resolvable corpus line at all
/// (D0 — a synthetic generator target with no corpus provenance).
pub fn no_corpus_line_signals() -> BTreeSet<String> {
    let mut s = BTreeSet::new();
    s.insert("no_corpus_line".to_string());
    s
}

/// Union the signals over a unit's TOKEN CLOSURE: its base corpus row plus
/// every `.MOD` row that targets it (commit 9e9e6993). A unit's real
/// magnitude can live on a `.MOD` row rather than its own base row — the
/// caller resolves which `.MOD` rows target this unit (mirroring the
/// generator's own base-name resolution) and passes every row it found,
/// `None` for a row that did not resolve.
///
/// `display` survives only if NO row in the closure carries a magnitude
/// token: a magnitude on a `.MOD` row must not leave the base unit looking
/// like a text-only record.
pub fn closure_signals(rows: &[Option<&str>]) -> BTreeSet<String> {
    closure_signals_with_rules(rows, &SignalRules::default())
}

/// [`closure_signals`], parameterised on [`SignalRules`].
pub fn closure_signals_with_rules(rows: &[Option<&str>], rules: &SignalRules) -> BTreeSet<String> {
    let real: Vec<&str> = rows.iter().filter_map(|r| *r).collect();
    if real.is_empty() {
        return no_corpus_line_signals();
    }
    let mut out: BTreeSet<String> = BTreeSet::new();
    for r in &real {
        out.extend(signals_with_rules(r, rules));
    }
    let any_row_carries_magnitude = real.iter().any(|r| {
        tab_fields(r).iter().any(|f| MAGNITUDE_TOKENS.iter().any(|t| f.starts_with(t)))
    });
    if real.len() > 1 && any_row_carries_magnitude {
        out.remove("display:no_magnitude_token");
    }
    out.remove("no_corpus_line");
    if out.is_empty() { no_corpus_line_signals() } else { out }
}

/// Whether any row in a unit's token closure carries upstream PCGen's own
/// `[Not Implemented]` admission. Reporting-only: this MUST NEVER be used
/// to gate or suppress a `wiring_class` determination in either direction.
pub fn carries_upstream_not_implemented_marker(rows: &[Option<&str>]) -> bool {
    rows.iter().any(|r| r.is_some_and(|line| line.contains(UPSTREAM_NOT_IMPLEMENTED)))
}

/// [`determine`], over a unit's token closure rather than a single row.
/// This is the entry point real callers (the work-inventory classifier,
/// the cache generators) should use — a single-row `determine` silently
/// reproduces the pre-9e9e6993 defect of missing a magnitude that lives on
/// a `.MOD` row.
pub fn determine_closure(rows: &[Option<&str>]) -> (WiringClass, String, BTreeSet<String>) {
    let sigs = closure_signals(rows);
    let (class, reason) = classify(&sigs);
    (class, reason, sigs)
}

// ---------------------------------------------------------------------------
// Corpus-wide token-closure machinery
// ---------------------------------------------------------------------------
//
// Shared by every real caller (`v06_work_inventory`'s classifier,
// `rules_core::cache_gen`'s per-book generators) so a `.MOD` row's base
// name is resolved exactly once, the same way everywhere. A unit's real
// magnitude can live on a `.MOD` row rather than its own base row
// (`wiring-class-determination.py` commit 9e9e6993/2e2ba619), so any
// caller emitting `wiring_class` needs this closure, not just a single
// corpus line.

/// Resolve a `.MOD` row's base record name from the text of field 0 before
/// `.MOD` (`"Foo.MOD"` -> `"Foo"`, already stripped of the `.MOD` suffix by
/// the caller). The same resolution `v06_work_inventory`'s own
/// `mod_only_rescue` path performs and `wiring-class-determination.py`'s
/// `mod_index()` performs, so all three always agree about which record a
/// `.MOD` row belongs to.
pub fn mod_base_name(before_mod: &str) -> String {
    let mut base = before_mod.to_string();
    if let Some(rest) = base
        .strip_prefix("CATEGORY=")
        .and_then(|r| r.split_once('|'))
        .map(|(_, rest)| rest.to_string())
    {
        // `CATEGORY=Special Ability|Foo.MOD` -> `Foo`
        base = rest;
    }
    // `CLASS:Bard.MOD` names the base class `Bard`, not a record called
    // `CLASS:Bard`. Without this, the name never matches the declared set
    // and a naive rescue would invent a second Bard in every book that
    // merely modifies the Core Rulebook's one.
    if let Some(rest) = base.strip_prefix("CLASS:") {
        base = rest.to_string();
    }
    base.trim().to_string()
}

/// `(book, base record name) -> every raw `.MOD` row targeting it`, built
/// once over every `.lst` file in every known book directory. Independent
/// of any per-record `file_kind` recognition or book-scope filtering --
/// mirroring the reference determinator's own `mod_index()`, which walks
/// the whole corpus tree rather than reusing a generator's own
/// (kind-scoped, ingestion-scoped) enumeration.
pub fn build_mod_index(
    book_paths: &BTreeMap<String, PathBuf>,
) -> BTreeMap<(String, String), Vec<String>> {
    let mut index: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    for (book, dir) in book_paths {
        let mut stack = vec![dir.clone()];
        let mut files: Vec<PathBuf> = Vec::new();
        while let Some(d) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&d) else { continue };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().and_then(|e| e.to_str()) == Some("lst") {
                    files.push(path);
                }
            }
        }
        // Sorted, so the `Vec<String>` of `.MOD` rows this index hands back is
        // in corpus-path order rather than in the filesystem's `read_dir`
        // order. Today `closure_signals_with_rules` unions its rows into a
        // `BTreeSet` and asks `.any()`, so `determine_closure` cannot see this
        // order and no `wiring_class` moves either way -- verified, and the
        // reason this is hardening rather than a bug fix. But `read_dir` order
        // is stable only for one directory on one machine, so leaving it
        // unsorted leaves a caller free to become order-sensitive later and
        // acquire a nondeterminism no test would attribute to it.
        // `v06_work_inventory::enumerate_book` sorts its own identical walk for
        // exactly this reason.
        files.sort();
        for path in files {
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            for raw in text.split('\n') {
                let trimmed = raw.trim_end_matches(['\r']);
                let first = trimmed.trim_start();
                if first.is_empty() || first.starts_with('#') {
                    continue;
                }
                let head = trimmed.split('\t').next().unwrap_or("").trim();
                let Some(mod_at) = head.find(".MOD") else { continue };
                let base = mod_base_name(&head[..mod_at]);
                index.entry((book.clone(), base)).or_default().push(trimmed.to_string());
            }
        }
    }
    index
}

/// Read+cache raw `.lst` text so a unit's base corpus row can be fetched by
/// `(book, source_file, source_line)` without re-reading the file per unit.
pub struct CorpusLines<'a> {
    book_paths: &'a BTreeMap<String, PathBuf>,
    cache: BTreeMap<(String, String), Vec<String>>,
}

impl<'a> CorpusLines<'a> {
    pub fn new(book_paths: &'a BTreeMap<String, PathBuf>) -> Self {
        CorpusLines { book_paths, cache: BTreeMap::new() }
    }

    /// The 1-based `line`'s raw text in `book`'s `file`, or `None` if the
    /// book/file/line does not resolve (D0 -- a synthetic generator target
    /// with no real corpus provenance, e.g. a `core_essentials` race file
    /// nested two directories deeper than this single-level join reaches).
    pub fn line(&mut self, book: &str, file: &str, line: usize) -> Option<String> {
        let key = (book.to_string(), file.to_string());
        if !self.cache.contains_key(&key) {
            let Some(dir) = self.book_paths.get(book) else {
                self.cache.insert(key.clone(), Vec::new());
                return None;
            };
            let text = std::fs::read_to_string(dir.join(file)).unwrap_or_default();
            self.cache.insert(key.clone(), text.split('\n').map(|s| s.to_string()).collect());
        }
        let buf = &self.cache[&key];
        if line == 0 || line > buf.len() {
            return None;
        }
        Some(buf[line - 1].clone())
    }
}

/// The full token closure for one record: its base corpus row plus every
/// `.MOD` row targeting its name or corpus key, as owned strings ready to
/// pass (via `.iter().map(|r| r.as_deref())`) to [`determine_closure`].
pub fn token_closure_rows(
    lines: &mut CorpusLines,
    mod_index: &BTreeMap<(String, String), Vec<String>>,
    book: &str,
    file: &str,
    line: usize,
    name: &str,
    key: &str,
) -> Vec<Option<String>> {
    let mut rows = vec![lines.line(book, file, line)];
    let mut seen_names: BTreeSet<&str> = BTreeSet::new();
    for n in [name, key] {
        if !seen_names.insert(n) {
            continue;
        }
        if let Some(mods) = mod_index.get(&(book.to_string(), n.to_string())) {
            rows.extend(mods.iter().cloned().map(Some));
        }
    }
    rows
}

/// Collapse a signal set to one class + a named reason. Strictly
/// highest-bar-wins: `computed`/`derived` outrank everything, `ambiguous`
/// outranks `display`, and `display` is the LAST resort before the
/// `static` fallback — never a short circuit (commit 9e9e6993, "wiring_class
/// reads a record's token closure, not one row": a record with no magnitude
/// TOKEN can still state one in prose, e.g. a `.MOD BENEFIT:` row reading
/// "spell resistance equal to 5 + your character level"; letting `display`
/// win there marks a unit done the moment its text renders, which is
/// exactly the over-claim this axis exists to prevent).
pub fn classify(sigs: &BTreeSet<String>) -> (WiringClass, String) {
    if sigs.len() == 1 && sigs.contains("no_corpus_line") {
        return (WiringClass::Ambiguous, "no_corpus_line".to_string());
    }
    for (prefix, class) in [("computed:", WiringClass::Computed), ("derived:", WiringClass::Derived)]
    {
        if let Some(hit) = sigs.iter().filter(|s| s.starts_with(prefix)).min() {
            return (class, hit[prefix.len()..].to_string());
        }
    }
    if let Some(hit) = sigs.iter().filter(|s| s.starts_with("ambiguous:")).min() {
        return (WiringClass::Ambiguous, hit["ambiguous:".len()..].to_string());
    }
    if sigs.iter().any(|s| s.starts_with("display:")) {
        return (WiringClass::Display, "no_magnitude_token".to_string());
    }
    (WiringClass::Static, "literal_magnitudes_only".to_string())
}

/// Determine `(class, reason, signals)` for one raw `.lst` line in one
/// call — the shape both the work-inventory classifier and the
/// cache-generators need.
pub fn determine(raw: Option<&str>) -> (WiringClass, String, BTreeSet<String>) {
    let sigs = match raw {
        Some(line) => signals(line),
        None => no_corpus_line_signals(),
    };
    let (class, reason) = classify(&sigs);
    (class, reason, sigs)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Verbatim from `wiring-class-determination.py`'s `ABILITY_CASES`
    // (`--selftest`, commit 2e2ba619): the grant-vs-reference discriminator
    // ported here must agree with the reference on every one of these real
    // corpus phrases.
    const ABILITY_CASES: &[(bool, &str)] = &[
        (
            true,
            "BENEFIT:You add your Dexterity bonus to your base attack bonus and \
             Strength modifier when determining CMB",
        ),
        (
            true,
            "DESC:you gain a bonus on electricity damage rolls equal to your Wisdom bonus (%1)",
        ),
        (true, "DESC:deal 1d6 points of bludgeoning damage plus your Strength modifier"),
        (
            true,
            "BENEFIT:choose a number of spells that you already know equal to your \
             Intelligence modifier",
        ),
        (true, "DESC:you can add twice your Intelligence modifier in damage (minimum 2)"),
        (true, "DESC:move up to 5 feet times your Intelligence modifier (minimum 1)"),
        (
            true,
            "DESC:you recover additional hit points equal to half your Constitution \
             modifier (minimum +1)",
        ),
        (
            false,
            "DESC:you don't lose your Dexterity bonus to Armor Class, and the attacker \
             doesn't get the +2 bonus",
        ),
        (false, "DESC:While running, you retain your Dexterity bonus to your Armor Class."),
        (false, "DESC:You retain your Dexterity bonus to AC even when flat-footed"),
        (
            false,
            "DESC:you may use your Dexterity modifier instead of your Strength modifier \
             on attack rolls",
        ),
        (false, "DESC:While denied your Dexterity bonus to AC you are also denied this resistance"),
        (
            false,
            "DESC:A condition that makes you lose your Dexterity bonus to Armor Class \
             also makes you lose dodge bonuses",
        ),
        (
            false,
            "DESC:use the higher of your caster level or your Strength modifier, \
             whichever is your Charisma modifier",
        ),
    ];

    #[test]
    fn ability_scaling_discriminator_matches_reference_selftest() {
        let mut bad = Vec::new();
        for (want, text) in ABILITY_CASES {
            let got = ability_scaling(text);
            if got != *want {
                bad.push((want, got, text));
            }
        }
        assert!(bad.is_empty(), "ability_scaling disagreements: {bad:#?}");
    }

    fn cls(raw: &str) -> (WiringClass, String) {
        classify(&signals(raw))
    }

    // D0
    #[test]
    fn d0_no_corpus_line_is_ambiguous() {
        let (class, reason, sigs) = determine(None);
        assert_eq!(class, WiringClass::Ambiguous);
        assert_eq!(reason, "no_corpus_line");
        assert!(sigs.contains("no_corpus_line"));
    }

    // D1
    #[test]
    fn d1_no_magnitude_token_is_display() {
        let (class, reason) = cls("Skill Focus\tTYPE:General\tDESC:You are skilled.");
        assert_eq!(class, WiringClass::Display);
        assert_eq!(reason, "no_magnitude_token");
    }

    #[test]
    fn d1_pre_guard_on_no_magnitude_row_stays_display() {
        // A PRE* guard on a row with no magnitude gates text, not a
        // magnitude, and must not promote the row.
        let (class, _) = cls("Some Feat\tTYPE:General\tPREBAB:MIN=1\tDESC:Some text.");
        assert_eq!(class, WiringClass::Display);
    }

    // D2 — computed:tempbonus
    #[test]
    fn d2_tempbonus_is_computed() {
        let (class, reason) = cls(
            "Lead Blades\tSCHOOL:Transmutation\tTEMPBONUS:EQ|Weapon,Melee|COMBAT|DAMAGESIZE|1|TYPE=Temporary",
        );
        assert_eq!(class, WiringClass::Computed);
        assert_eq!(reason, "tempbonus");
    }

    // D2 — computed:choice
    #[test]
    fn d2_choice_is_computed() {
        let (class, reason) =
            cls("Composite Bow Strength Rating\tBONUS:WEAPON|DAMAGE|MIN(%CHOICE,STR)");
        assert_eq!(class, WiringClass::Computed);
        assert_eq!(reason, "choice");
    }

    // D2 — computed:pre_guard, excluding PRERULE
    #[test]
    fn d2_pre_guard_is_computed() {
        let (class, reason) = cls(
            "Amulet of the Spirits\tBONUS:SKILL|TYPE.Charisma|max(0,WIS)|TYPE=WisdomBonus|PREABILITY:1,Mystery",
        );
        assert_eq!(class, WiringClass::Computed);
        assert_eq!(reason, "pre_guard");
    }

    #[test]
    fn d2_prerule_guard_is_excluded_load_bearing() {
        // `!PRERULE:1,DisplayFullSpell` must NOT count as a guard, or every
        // spell row becomes `computed` (the 23x overstatement the spec
        // documents).
        let (class, _) = cls(
            "Fireball\tSCHOOL:Evocation\tRANGE:Long\tDESC:deals (min(10,CASTERLEVEL))d6 points of fire damage\t!PRERULE:1,DisplayFullSpell",
        );
        assert_eq!(class, WiringClass::Derived);
    }

    // D3 — derived:<token>
    #[test]
    fn d3_scalar_field_is_derived() {
        let (class, reason) =
            cls("Amulet of Mighty Fists\tCOST:4000*PLUSTOTAL*PLUSTOTAL");
        assert_eq!(class, WiringClass::Derived);
        assert_eq!(reason, "cost");
    }

    // Regression: `Arcane Bloodline` (`core_rulebook/cr_abilities_class.lst:2369`)
    // -- an earlier port of `+\s*\w*[A-Z]{2,}` only checked for a trailing
    // uppercase run, and missed a 2+-uppercase run in the MIDDLE of the
    // word (`BloodlinePower1LVLBonus` carries `LV`/`VL` mid-word, not at
    // its tail). The reference regex's `\w*` backtracks to find a match
    // anywhere in the run, not just at the end.
    #[test]
    fn d3_arith_uppercase_run_matches_mid_word_not_only_trailing() {
        let (class, reason) = cls(
            "Arcane Bloodline\tBONUS:VAR|Sorcerer_Arcane_BloodlinePower1LVL|Sorcerer_Arcane_BloodlineLVL+BloodlinePower1LVLBonus",
        );
        assert_eq!(class, WiringClass::Derived);
        assert_eq!(reason, "bonus");
    }

    // D3 — derived:prose_expr
    #[test]
    fn d3_prose_expr_is_derived() {
        let (class, reason) = cls(
            "Fireball\tSCHOOL:Evocation\tRANGE:Long\tDESC:deals (min(10,CASTERLEVEL))d6 points of fire damage",
        );
        assert_eq!(class, WiringClass::Derived);
        // RANGE:Long is also a range_keyword signal but prose_expr and
        // range_keyword both sort under `derived:`; either is a legitimate
        // `derived` collapse. Assert on the class, not the exact reason,
        // since both signals are real.
        assert!(reason == "prose_expr" || reason == "range_keyword");
    }

    // D3 — derived:range_keyword
    #[test]
    fn d3_range_keyword_is_derived() {
        let (class, reason) = cls("Burning Hands\tSCHOOL:Evocation\tRANGE:Close");
        assert_eq!(class, WiringClass::Derived);
        assert_eq!(reason, "range_keyword");
    }

    // Real classifier miss found auditing the `display`+`grounded`
    // contradiction set (2026-08): PCGen's `%N` render-time substitution
    // syntax states a genuine formula that neither `MAGNITUDE_TOKENS` nor
    // `paren_groups` sees, so the record fell to `display:no_magnitude_token`
    // despite carrying a real per-creature magnitude.
    #[test]
    fn d3_percent_placeholder_with_trailing_formula_is_derived() {
        // `bestiary_4/b4_abilities_race.lst:1490` "Zomok Breath Weapon".
        let (class, reason) = cls(
            "Breath Weapon\tKEY:Zomok ~ Breath Weapon\tCATEGORY:Special Ability\tTYPE:SpecialAttack.Supernatural\tDESC:A zomok's breath weapon is a cone of flying dirt, bark, stones, and moss. A creature can break free with a DC %1 Strength or Escape Artist check.|CON+18",
        );
        assert_eq!(class, WiringClass::Derived);
        assert_eq!(reason, "prose_formula_segment");
    }

    #[test]
    fn d3_percent_placeholder_formula_survives_a_trailing_pre_guard() {
        // `ultimate_wilderness/uw_abilities_companion.lst`-shaped row (the
        // corpus pattern in `assassin_bug_giant_poison` /
        // `spitting_cobra_poison`): the formula segment is followed by a
        // `PREVARLT:` guard segment, which must not be mistaken for the
        // formula or suppress detection of the real one before it.
        let (class, reason) = cls(
            "Poison\tKEY:Spitting Cobra ~ Poison\tCATEGORY:Special Ability\tTYPE:SpecialAttack.Extraordinary\tDESC:Fort DC %1|10+HD/2+CON|PREVARLT:CompanionAdvancement,1",
        );
        assert_eq!(class, WiringClass::Derived);
        assert_eq!(reason, "prose_formula_segment");
    }

    #[test]
    fn percent_placeholder_without_a_resolvable_formula_segment_stays_display() {
        // `bestiary/b1_abilities_race.lst:1039` "Pixie Charm": the `%1` is
        // substituted by `SpecialArrowDC`, a cross-reference to a value
        // defined elsewhere in the corpus, not a scalar/arithmetic
        // construction this classifier can itself resolve. Must NOT be
        // guessed into `derived` -- an unresolvable reference stays
        // `display`, same standing rule as bare ability-score mentions.
        let (class, reason) = cls(
            "Special Charm Arrow\tKEY:Pixie ~ Charm\tCATEGORY:Special Ability\tTYPE:SpecialQuality.Supernatural\tDESC:Charm; The target must succeed on a DC %1 Will save or be affected as though by a Charm Monster spell for 10 minutes.|SpecialArrowDC",
        );
        assert_eq!(class, WiringClass::Display);
        assert_eq!(reason, "no_magnitude_token");
    }

    // D4 — ambiguous:prose_scaling_phrase
    #[test]
    fn d4_prose_scaling_phrase_is_ambiguous() {
        let (class, reason) = cls(
            "Air Geyser\tSCHOOL:Evocation\tRANGE:Personal\tDESC:deals 2d6 points of bludgeoning damage and hurls the target upward a number of feet equal to 5 x your caster level.",
        );
        assert_eq!(class, WiringClass::Ambiguous);
        assert_eq!(reason, "prose_scaling_phrase");
    }

    // D5 — static
    #[test]
    fn d5_literal_magnitude_is_static() {
        let (class, reason) = cls("Longsword\tCOST:15\tWT:4");
        assert_eq!(class, WiringClass::Static);
        assert_eq!(reason, "literal_magnitudes_only");
    }

    // Lattice collapse: a dual-signal unit resolves to `computed` while
    // both signals remain visible in the full signal set.
    #[test]
    fn dual_signal_collapses_to_computed_but_retains_both_signals() {
        let raw = "Amulet of the Spirits (Heavens)\tBONUS:SKILL|TYPE.Charisma|max(0,WIS)|TYPE=WisdomBonus\tSPROP:increase effective level of mystery or spirit powers by 2|PREABILITY:1,Mystery";
        let sigs = signals(raw);
        assert!(sigs.iter().any(|s| s.starts_with("derived:")));
        assert!(sigs.iter().any(|s| s.starts_with("computed:")));
        let (class, _) = classify(&sigs);
        assert_eq!(class, WiringClass::Computed);
    }

    #[test]
    fn wiring_class_lattice_ordering() {
        assert!(WiringClass::Display < WiringClass::Static);
        assert!(WiringClass::Static < WiringClass::Derived);
        assert!(WiringClass::Derived < WiringClass::Computed);
    }

    // Mandatory per commit 9e9e6993 review: a base row with NO magnitude
    // token, plus a `.MOD` row carrying a formulaic magnitude, must resolve
    // to `derived` for the unit -- not `display`. Getting the base/`.MOD`
    // association wrong either hides a magnitude (`display`, wrong) or
    // bleeds one record's magnitude onto another.
    #[test]
    fn closure_promotes_display_base_to_derived_via_mod_row() {
        let base = "Accursed\tTYPE:General\tDESC:You are marked by a curse.";
        let modification = "Accursed.MOD\tBONUS:SAVE|Fortitude|CASTERLEVEL/2";
        let rows = [Some(base), Some(modification)];
        let (class, _, sigs) = determine_closure(&rows);
        assert_eq!(class, WiringClass::Derived);
        assert!(sigs.iter().any(|s| s.starts_with("derived:")));
        // The base row alone would have been `display` -- confirms the
        // promotion is coming from the `.MOD` row, not a fluke of the base.
        let (base_only, _) = classify(&signals(base));
        assert_eq!(base_only, WiringClass::Display);
    }

    // The worked case from the spec: prose-only scaling on a `.MOD
    // BENEFIT:` row lands `ambiguous`, not `derived` (no machine-readable
    // expression) and not `display` (a magnitude IS stated, just not
    // mechanically). Guessing which is exactly what `ambiguous` prevents.
    #[test]
    fn accursed_worked_case_is_ambiguous_not_display_or_derived() {
        let base = "Accursed\tTYPE:General\tDESC:You are marked by a curse.";
        let modification = "Accursed.MOD\tBENEFIT:spell resistance equal to 5 + your character level.";
        let rows = [Some(base), Some(modification)];
        let (class, reason, _) = determine_closure(&rows);
        assert_eq!(class, WiringClass::Ambiguous);
        assert_eq!(reason, "prose_scaling_phrase");
    }

    // `[Not Implemented]` is reported separately and must never suppress a
    // real magnitude found elsewhere in the closure.
    #[test]
    fn upstream_not_implemented_marker_does_not_suppress_a_real_magnitude() {
        let base = "Ability Focus\tTYPE:General\tDESC:[Not Implemented]";
        let modification = "Ability Focus.MOD\tBONUS:SAVE|Fortitude|CASTERLEVEL/2";
        let rows = [Some(base), Some(modification)];
        assert!(carries_upstream_not_implemented_marker(&rows));
        let (class, _, _) = determine_closure(&rows);
        assert_eq!(class, WiringClass::Derived);
    }

    #[test]
    fn classify_ordering_puts_ambiguous_above_display() {
        // A signal set with only an `ambiguous:` and a `display:` signal
        // (as `closure_signals` can produce when one closure row has no
        // magnitude and another states one only in prose) must resolve
        // `ambiguous`, not `display`.
        let mut sigs = BTreeSet::new();
        sigs.insert("display:no_magnitude_token".to_string());
        sigs.insert("ambiguous:prose_scaling_phrase".to_string());
        let (class, _) = classify(&sigs);
        assert_eq!(class, WiringClass::Ambiguous);
    }

    // Proves the detection tables are real data, not hardcoded scan logic:
    // a field the default rules never scan (`BENEFIT:`) is picked up once
    // it's added to a custom `SignalRules.prose_fields`, with no change to
    // `signals_with_rules` itself.
    #[test]
    fn signal_rules_are_swappable_without_touching_scan_logic() {
        // `FLUFF:` is not in the default `prose_fields` table, so its
        // scaling phrase is invisible to the default rules...
        let raw = "Some Story Feat\tRANGE:Personal\tFLUFF:power scales at 5 x your caster level";
        assert!(!signals(raw).contains("ambiguous:prose_scaling_phrase"));

        // ...and visible once a caller supplies a rules table that scans it,
        // with no change to `signals_with_rules` itself.
        let custom = SignalRules {
            prose_fields: &["FLUFF:", "RANGE:"],
            prose_scaling_phrases: &["x your caster level"],
        };
        let sigs = signals_with_rules(raw, &custom);
        assert!(sigs.contains("ambiguous:prose_scaling_phrase"));
    }
}
