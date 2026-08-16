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
    // `SPELLS:<label>|TIMES=...|CASTERLEVEL=<scalar-or-int>|<spell name>[,<DC formula>]`
    // -- a spell-like-ability grant. The `CASTERLEVEL=` segment and any
    // trailing comma-delimited DC formula are real numeric magnitude, not
    // prose (`OPEN-ISSUES.md` row 16, Finding D, `SD31-E2-F1-002`):
    // `bestiary_4:monster_ability:winter_hag_ice_staff`'s
    // `SPELLS:Ice Staff|CASTERLEVEL=10|Cone of Cold,15+CHA` states a
    // CHA-scalar save DC nowhere else on the row. Previously unscanned --
    // neither a `prose_fields` entry (it is a structured pipe-delimited
    // token, not English text) nor a `MAGNITUDE_TOKENS` entry -- so a
    // record whose only magnitude lived here fell to
    // `display:no_magnitude_token` (no chassis found at all) instead of
    // being scanned at all.
    "SPELLS:",
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

/// Is `s` a bare (optionally signed) integer literal, with nothing else —
/// `"10"`, `"+2"`, `"-4"` are; `"Cold Iron"`, `"EidolonDR"`,
/// `"MutagenStatBonus"`, `""`, `"-"` are not. Used to distinguish a
/// genuinely flat `MAGNITUDE_TOKENS` amount from a named PCGen variable
/// wearing the same slash/selector shape (`SD31-W2-INTEGRATE-001`,
/// Finding 1's D4 repair).
fn is_integer_literal(s: &str) -> bool {
    let digits = s.strip_prefix(['+', '-']).unwrap_or(s);
    !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit())
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
    has_arith_scoped(value, true)
}

/// As [`has_arith`], but `allow_slash` toggles whether a bare `/` counts as
/// division. Two `MAGNITUDE_TOKENS` reserve `/` as a literal notation
/// separator in PCGen's own grammar, never an operator (`OPEN-ISSUES.md`
/// row 2(b), SD31-E2-F2-001-wiringfix):
///   - `DR:` — `<amount>/<bypass-descriptor>` (`DR:10/Cold Iron`, `DR:1/-`).
///   - `CR:` — `<int>/<int>` sub-1 challenge-rating fraction (`CR:1/3`).
///
/// [`has_scalar_or_arith_for_token`] passes `false` for exactly those two
/// tokens; every other caller (including [`has_arith`] itself, used by
/// prose-field `%N`-substitution scanning where `/` is never PCGen's DR or
/// CR notation) keeps the original unscoped behaviour. Corpus-wide
/// re-derivation this cycle (38 known book directories) found every `DR:`
/// value carrying a `/` matches the bypass shape (none is an ambiguous
/// bare `<int>/<int>` that could plausibly be division) and every `CR:`
/// value carrying a `/` is the canonical fraction set (`1/2`, `1/3`,
/// `1/4`, `1/6`, `1/8`); a genuinely dynamic DR still signals via its own
/// `*` (`DR:1*ArmoredDefenseMult/-`) or a `min(`/`max(` call, so scoping
/// the `/` exclusion to these two tokens cannot hide a real DR/CR formula.
///
/// **D4 repair (`SD31-W2-INTEGRATE-001`, Finding 1).** The `/`-as-division
/// exclusion applies ONLY when the segment before the first `/` is itself
/// a bare integer literal (`10/Cold Iron`, `1/3`) — never when it names a
/// variable (`DR:EidolonDR/evil`, `DR:DamageReductionLVL/-`). The original
/// D3 fix disabled the whole slash-as-division arm for `CR:`/`DR:`
/// unconditionally, which over-shot: a `DR:`/`CR:` amount that is itself a
/// named, PCGen-`DEFINE:`d variable is not a literal magnitude at all, and
/// must keep signalling `derived` rather than falling to
/// `literal_magnitudes_only`, which the record would contradict.
fn has_arith_scoped(value: &str, allow_slash: bool) -> bool {
    if let Some(idx) = value.find('/')
        && (allow_slash || !is_integer_literal(&value[..idx]))
    {
        return true;
    }
    has_arith_no_slash(value)
}

/// The slash-independent half of [`has_arith_scoped`]: `*`, `min(`/`max(`,
/// `classlevel(`, and the `+`-then-guard patterns — every arithmetic shape
/// this scanner recognizes EXCEPT the `/`-as-division check. Split out for
/// [`has_scalar_or_arith_in_spells_field`]'s D7 repair
/// (`SD31-W3-INTEGRATE-001`): a spell-NAME segment's own literal slash
/// (`Open/Close`) must never be read as arithmetic, but `has_arith_scoped`'s
/// `allow_slash=false` arm was written for `DR:`/`CR:` bypass notation and
/// deliberately still flags a slash whose left side is a NAMED VARIABLE
/// (`EidolonDR/evil`) — exactly the shape a spell name's `Open` half also
/// has (not an integer literal), so passing `allow_slash: false` to
/// `has_arith_scoped` does not suppress it for spell names. This helper
/// skips the slash check entirely rather than narrowing it further.
fn has_arith_no_slash(value: &str) -> bool {
    if value.contains('*') {
        return true;
    }
    let lower = value.to_ascii_lowercase();
    if lower.contains("min(") || lower.contains("max(") {
        return true;
    }
    // PCGen's `classlevel("<ClassName>")` function-call form is a genuine
    // class-level-scaling formula, same shape as the bare `CLASSLEVEL`
    // scalar keyword `has_scalar`/`SCALARS_SUBSTRING` already recognizes --
    // but that check is case-sensitive (`value.contains("CLASSLEVEL")`) and
    // real corpus rows spell the function call lowercase
    // (`OPEN-ISSUES.md` row 9(a), `SD31-E2-F2-001-wiringfix`):
    // `ultimate_magic:class_feature:dragon_shaman_totem_transformation`'s
    // `BONUS:VAR|TotemTransformationDuration|classlevel("Druid")` carries no
    // uppercase `CLASSLEVEL` anywhere and was misread `static`. Checked as
    // its own case-insensitive function-call form here rather than by
    // lower-casing the whole `SCALARS_SUBSTRING` scan, which would risk new
    // false positives on ordinary lowercase corpus prose the bare-keyword
    // scan was never exposed to.
    if lower.contains("classlevel(") {
        return true;
    }
    // A `+` immediately followed by a parenthesised sub-expression is
    // arithmetic even when nothing inside the parens starts with an
    // uppercase-letter run right after the `+` (`has_arith`'s existing
    // `+\s*\w*[A-Z]{2,}` check requires a WORD character after `+`, not `(`;
    // `OPEN-ISSUES.md` row 9(c)): `horror_adventures/support/ha_abilities_class_oa.lst:305`,
    // "Rapturous Rage" -- `BONUS:ABILITYPOOL|Rage Power|10+(SpiritualistLVL>=14)+(SpiritualistLVL>=18)`
    // is a real level-gated formula stated entirely inside parenthesised
    // comparisons after each `+`.
    {
        let bytes = value.as_bytes();
        for (i, b) in bytes.iter().enumerate() {
            if *b == b'+' {
                let mut j = i + 1;
                while j < bytes.len() && bytes[j].is_ascii_whitespace() {
                    j += 1;
                }
                if bytes.get(j) == Some(&b'(') {
                    return true;
                }
            }
        }
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

/// `BONUS:STAT|<selector>|<magnitude>[|<tag>=<val>...]` (and the
/// structurally identical `TEMPBONUS:STAT|...`) puts an ability-score
/// SELECTOR — not a magnitude — in the field's second pipe segment.
/// Scanning the whole value for a scalar word makes any such field
/// misclassify `derived` purely because the selector happens to spell a
/// scalar name (`BONUS:STAT|DEX|2|TYPE=Racial` is a flat literal +2, not a
/// DEX-dependent formula; `OPEN-ISSUES.md` row 2(a),
/// SD31-E2-F2-001-wiringfix). This strips exactly that one segment before
/// the scalar/arith scan runs; every other pipe segment — the real
/// magnitude and any `TYPE=`/tag segments — is scanned in full, unchanged.
/// A value that does not start with the literal `STAT|` sub-token (i.e.
/// every `MAGNITUDE_TOKENS` field except `BONUS:STAT`/`TEMPBONUS:STAT`) is
/// returned untouched.
fn strip_stat_selector(value: &str) -> std::borrow::Cow<'_, str> {
    let mut parts = value.splitn(3, '|');
    match (parts.next(), parts.next(), parts.next()) {
        (Some("STAT"), Some(_selector), Some(rest)) => rest.to_string().into(),
        // `STAT|<selector>` with nothing after it (no magnitude segment at
        // all) has no selector-collision magnitude to false-positive on.
        (Some("STAT"), Some(_selector), None) => "".into(),
        _ => value.into(),
    }
}

/// [`has_scalar_or_arith`], scoped per `MAGNITUDE_TOKENS` field: strips a
/// `BONUS:STAT`/`TEMPBONUS:STAT` selector segment before the scalar scan
/// ([`strip_stat_selector`]), and disables the `/`-as-division arm of
/// [`has_arith_scoped`] for `CR:`/`DR:` specifically, where PCGen's own
/// grammar reserves `/` as a literal notation separator. The single call
/// site is the `MAGNITUDE_TOKENS` loop in [`signals_with_rules`]; prose-field
/// scanning (`has_prose_formula_segment`) keeps calling the unscoped
/// [`has_scalar_or_arith`] because CR/DR bypass notation never appears in
/// a prose field.
///
/// **D4 repair (`SD31-W2-INTEGRATE-001`, Finding 1).** Stripping the
/// `STAT` selector segment can leave behind a magnitude that is itself a
/// named PCGen variable rather than a literal integer
/// (`BONUS:STAT|STR|MutagenStatBonus`) — the selector was never the only
/// possible false-positive source; a non-literal magnitude left behind by
/// the strip must not be allowed to fall through to
/// `literal_magnitudes_only` just because it fails the (deliberately
/// narrow) `SCALARS`/`ARITH` word lists.
fn has_scalar_or_arith_for_token(token: &str, value: &str) -> bool {
    if token == "SPELLS" {
        return has_scalar_or_arith_in_spells_field(value);
    }
    let scan_value = strip_stat_selector(value);
    let allow_slash = token != "CR" && token != "DR";
    if has_scalar(&scan_value) || has_arith_scoped(&scan_value, allow_slash) {
        return true;
    }
    if value.starts_with("STAT|") {
        let magnitude = scan_value.split('|').next().unwrap_or("");
        if !is_integer_literal(magnitude) {
            return true;
        }
    }
    false
}

/// Scoped scalar/arithmetic scan for a `SPELLS:` field value
/// (`<label>|TIMES=<n>/DAY-or-ATWILL|CASTERLEVEL=<int-or-scalar>|<spell
/// name>[,<DC formula>]`). A whole-value scan the way every other
/// `MAGNITUDE_TOKENS` field is scanned false-positives on two of this
/// token's own STRUCTURAL tags, the same selector-vs-magnitude collision
/// shape already fixed for `BONUS:STAT`/`STAT:` (`OPEN-ISSUES.md` row 16,
/// Finding D, discovered while adding this test coverage):
///
/// - The literal tag text `CASTERLEVEL=` always contains the substring
///   `CASTERLEVEL`, so a whole-value scan calls EVERY `SPELLS:` field
///   `derived` regardless of whether the level after `=` is a real scalar
///   (`CASTERLEVEL=TL`) or a flat literal (`CASTERLEVEL=10`) — the tag
///   NAME, not the value, was triggering the signal.
/// - `TIMES=<n>/DAY` is PCGen's literal "N times per day" notation, not
///   division; a whole-value scan's unscoped `/` check would call every
///   limited-use spell-like ability `derived` purely for using this
///   near-universal notation.
///
/// Scanned per pipe segment instead: `CASTERLEVEL=`'s value counts only
/// when it is not a bare integer literal (same `is_integer_literal` rule
/// as the `BONUS:STAT`/`CR:`/`DR:` fixes); `TIMES=` is skipped outright
/// (its `/DAY` suffix is never a formula); every other segment — the spell
/// name and any trailing comma-delimited DC formula — is scanned in full
/// with the ordinary unscoped rules, since a genuine DC formula
/// (`Cone of Cold,15+CHA`) carries no tag prefix to collide with.
fn has_scalar_or_arith_in_spells_field(value: &str) -> bool {
    for seg in value.split('|') {
        if let Some(level) = seg.strip_prefix("CASTERLEVEL=") {
            if !is_integer_literal(level) {
                return true;
            }
            continue;
        }
        if seg.starts_with("TIMES=") {
            continue;
        }
        // D7 repair (`SD31-W3-INTEGRATE-001`, adversarial-review finding):
        // this segment is `<spell name>[,<DC formula>]`, and canonical PF1
        // spell names routinely carry a literal slash of their own
        // (`Open/Close`, `Blindness/Deafness`, `Clairaudience/Clairvoyance`)
        // that is not division. Scan the whole segment with the slash-as-
        // division arm DISABLED (still catches `*`, `min(`/`max(`,
        // `classlevel(`, and the `+<UPPER-run>` guard, none of which a real
        // spell name collides with), then separately re-scan only the
        // comma-delimited DC-formula tail (if any) with the slash arm
        // enabled -- a genuine formula never needs a slash in the name half,
        // and the tail is where PCGen's own `,15+CHA`-shaped DCs live.
        if has_scalar(seg) || has_arith_no_slash(seg) {
            return true;
        }
        if let Some(comma_idx) = seg.rfind(',') {
            let tail = &seg[comma_idx + 1..];
            if has_arith_scoped(tail, true) {
                return true;
            }
        }
    }
    false
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
        if has_scalar_or_arith_for_token(token, value) {
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

/// Maximum subdirectory depth (below the book root) [`resolve_corpus_file`]
/// searches when the direct `dir.join(file)` join misses. Corpus-derived
/// (`OPEN-ISSUES.md` row 1, SD31-E2-F2-001-wiringfix's receipt): a full
/// walk of all 38 known book directories found the deepest real `.lst`
/// nesting today is 2 subdirectory levels
/// (`core_essentials/races/<race>/*.lst`); this constant carries one
/// level of headroom above that measured maximum without becoming an
/// unbounded walk of the whole book tree.
const MAX_NESTED_LST_DEPTH: usize = 3;

/// Resolve `file`'s real path under a book's `dir`.
///
/// Checks the direct single-level `dir.join(file)` first — the fast,
/// common case, and (proven below) the ONLY match any already-resolving
/// unit has, so no currently-correct resolution changes which file it
/// reads. If that misses, searches `dir` up to [`MAX_NESTED_LST_DEPTH`]
/// levels deep for a file with that exact basename. Several books nest
/// their `.lst` files (`core_essentials/races/<race>/`,
/// `ultimate_combat/support/`, `horror_adventures/support/`,
/// `inner_sea_world_guide/_pfs/`, `advanced_race_guide/_pfs/`,
/// `adventurers_guide/support/`, ...); the prior single-level join
/// silently missed all of them and fell to D0 `ambiguous:no_corpus_line`
/// for ~1,707 corpus-real units (`OPEN-ISSUES.md` row 1).
///
/// A bounded walk, not an unbounded recursive glob of the whole tree —
/// and the resolution bar is the *correct* file, not merely *a* file with
/// the right name: a same-named `.lst` file in a different subdirectory
/// of the same book, or of a different book, would silently corrupt the
/// wiring-class read if picked by accident (this program's repeat
/// identifier-scope-collision hazard). Enumeration across the full
/// 38-book corpus (this cycle's receipt) found **zero** basenames
/// duplicated within any one book's tree at any depth — INCLUDING
/// root-vs-nested (`SD31-W2-INTEGRATE-001`, Finding 4: the direct-join
/// candidate is now collected alongside the nested-search matches below,
/// not returned early, so a root-shadowing-nested duplicate refuses to
/// guess too, not just a nested-vs-nested one) — so an exact basename
/// match under a book's own directory is unambiguous today, but this
/// function still refuses to guess if a future corpus revision
/// introduces one: more than one match resolves to `None`, the same
/// outcome as no match, never a silently-wrong pick. Cross-book
/// collisions cannot occur by construction: the search is confined to the
/// single `dir` the caller's own `book` key maps to.
fn resolve_corpus_file(dir: &std::path::Path, file: &str) -> Option<PathBuf> {
    let mut matches: Vec<PathBuf> = Vec::new();
    let direct = dir.join(file);
    if direct.is_file() {
        matches.push(direct);
    }
    let mut stack = vec![(dir.to_path_buf(), 0usize)];
    while let Some((d, depth)) = stack.pop() {
        if depth > MAX_NESTED_LST_DEPTH {
            continue;
        }
        let Ok(entries) = std::fs::read_dir(&d) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push((path, depth + 1));
            } else if depth > 0 && path.file_name().and_then(|n| n.to_str()) == Some(file) {
                // `depth > 0`: the book-root's own direct entries were
                // already checked above via `direct` -- only NESTED
                // matches are collected here, so a root-level file is
                // never pushed into `matches` twice under two different
                // `PathBuf` values for the same real path.
                matches.push(path);
            }
        }
    }
    match matches.len() {
        1 => matches.pop(),
        _ => None,
    }
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
    /// with no real corpus provenance, or a same-book basename collision
    /// [`resolve_corpus_file`] refuses to guess between).
    pub fn line(&mut self, book: &str, file: &str, line: usize) -> Option<String> {
        let key = (book.to_string(), file.to_string());
        if !self.cache.contains_key(&key) {
            let Some(dir) = self.book_paths.get(book) else {
                self.cache.insert(key.clone(), Vec::new());
                return None;
            };
            let text = resolve_corpus_file(dir, file)
                .and_then(|p| std::fs::read_to_string(p).ok())
                .unwrap_or_default();
            self.cache.insert(key.clone(), text.split('\n').map(|s| s.to_string()).collect());
        }
        let buf = &self.cache[&key];
        if line == 0 || line > buf.len() {
            return None;
        }
        Some(buf[line - 1].clone())
    }
}

/// The identity a `.COPY=<name>` row's first tab-separated field names as
/// its base — the string before `.COPY=`. `None` for a plain row. This is
/// the SAME split `gen_equipment_gap_tables.rs`'s own inheritance walk and
/// `corpus_literal_sweep.rs`'s `copy_base_identity`/`copy_base_row` use for
/// the identical relationship — wave-8 adversarial review (SD31-W8-
/// INTEGRATE-001, confirmed GAMED verdict) found this was the ONE call site
/// among those that still had no `.COPY=` awareness at all: `equipment_
/// modifier`'s `.COPY=` rows carried real inherited `BONUS:` chains that
/// the generator, the sweep and the raw-token enricher all resolved, but
/// this classifier read only the `.COPY=` row's own (magnitude-free) text
/// and stamped `wiring_class: display` on units whose shipped
/// `raw_bonus_chains` already proved them magnitude-bearing.
pub fn copy_base_identity(row: &str) -> Option<&str> {
    let first = row.split('\t').next().unwrap_or("");
    first.split_once(".COPY=").map(|(base, _)| base)
}

/// `(book, base identity a `.COPY=` row names) -> the PLAIN (non-`.COPY=`)
/// row that declares it`, built once over every `.lst` file in every known
/// book directory — the same shape as [`build_mod_index`], and resolved by
/// the identical `KEY:`-token-or-bare-name rule `corpus_literal_sweep.rs`'s
/// `Sweep::copy_base_row` and `gen_equipment_gap_tables.rs`'s own
/// inheritance walk both already use for this relationship, so all of them
/// (and now this classifier) agree on what "the base" means for the same
/// corpus row. At most one hop: a `.COPY=` row is never itself matched as
/// someone else's base (mirrors the generator's own "at most one hop"
/// rule), so a chain of `.COPY=` rows resolves only its own immediate
/// declared base, never transitively.
pub fn build_copy_base_index(
    book_paths: &BTreeMap<String, PathBuf>,
) -> BTreeMap<(String, String), String> {
    let mut index: BTreeMap<(String, String), String> = BTreeMap::new();
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
        // Sorted for the same determinism reason `build_mod_index` sorts:
        // `read_dir` order is stable only for one directory on one
        // machine, and a first-plain-declaration-wins policy needs a
        // stable "first" to be meaningful.
        files.sort();
        for path in files {
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            for raw in text.split('\n') {
                let trimmed = raw.trim_end_matches(['\r']);
                let first_field = trimmed.split('\t').next().unwrap_or("").trim();
                if first_field.is_empty() || first_field.contains(".COPY=") {
                    continue;
                }
                let key_token =
                    trimmed.split('\t').find_map(|f| f.trim().strip_prefix("KEY:"));
                let identity = key_token.unwrap_or(first_field);
                index
                    .entry((book.clone(), identity.to_string()))
                    .or_insert_with(|| trimmed.to_string());
            }
        }
    }
    index
}

/// The two corpus-wide indexes [`token_closure_rows`] resolves a record's
/// closure against — bundled into one parameter so the function stays
/// under clippy's `too_many_arguments` threshold as this program adds more
/// closure-widening indexes over time (`.MOD` rows, then `.COPY=` bases;
/// see each field's own doc comment for what it resolves and why).
pub struct ClosureIndexes<'a> {
    pub mod_index: &'a BTreeMap<(String, String), Vec<String>>,
    pub copy_base_index: &'a BTreeMap<(String, String), String>,
}

/// The full token closure for one record: its base corpus row, the plain
/// base row a `.COPY=` row inherits from (if any — see
/// [`build_copy_base_index`]), and every `.MOD` row targeting its name or
/// corpus key — as owned strings ready to pass (via
/// `.iter().map(|r| r.as_deref())`) to [`determine_closure`].
pub fn token_closure_rows(
    lines: &mut CorpusLines,
    indexes: ClosureIndexes,
    book: &str,
    file: &str,
    line: usize,
    name: &str,
    key: &str,
) -> Vec<Option<String>> {
    let base_row = lines.line(book, file, line);
    let mut rows = vec![base_row.clone()];
    if let Some(row) = base_row.as_deref()
        && let Some(base_identity) = copy_base_identity(row)
        && let Some(copy_base_row) =
            indexes.copy_base_index.get(&(book.to_string(), base_identity.to_string()))
    {
        rows.push(Some(copy_base_row.clone()));
    }
    let mod_index = indexes.mod_index;
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

    // D3 — `BONUS:STAT` selector false positive (`OPEN-ISSUES.md` row 2(a),
    // SD31-E2-F2-001-wiringfix). `core_rulebook/cr_abilities_race.lst:149`,
    // "+2 Dexterity" (re-derived this cycle): the sole magnitude field is
    // `BONUS:STAT|DEX|2|TYPE=Racial`, a flat literal +2. A pre-fix scan read
    // the WHOLE field value (`STAT|DEX|2|TYPE=Racial`), where the
    // ability-SELECTOR `DEX` collides with `SCALARS_WORD` and wrongly
    // signals `derived`; the value itself (`2`) is not scalar-dependent.
    // True class is `static`, confirmed against the ground-truth sample's
    // hand label (`SD31-E2-F1-ground-truth-sample-v1.json`,
    // `core_rulebook:race_trait:2_dexterity`, `hand_wiring_class: static`).
    #[test]
    fn d3_bonus_stat_selector_flat_literal_is_static_not_derived() {
        let (class, reason) = cls(
            "+2 Dexterity\tCATEGORY:Special Ability\tTYPE:AbilityBonus\tVISIBLE:DISPLAY\tSTACK:NO\tMULT:NO\tBONUS:STAT|DEX|2|TYPE=Racial",
        );
        assert_eq!(class, WiringClass::Static);
        assert_eq!(reason, "literal_magnitudes_only");
    }

    // Same false positive, a second real row:
    // `ultimate_equipment/ue_equip_magic_items.lst:397`, "Staff of Mithral
    // Might" (re-derived this cycle) — `BONUS:STAT|INT|2|TYPE=Enhancement`
    // is the only field that would otherwise signal `derived`; every other
    // magnitude field on the row (COST/WT/CRITMULT/CRITRANGE/DAMAGE/
    // ALTDAMAGE) is a plain literal. Padding tab fields the real row also
    // carries (PROFICIENCY/TYPE/ALTTYPE/EQMOD/SPELLS/etc.) are omitted here
    // because `signals_with_rules` never scans them (not a `MAGNITUDE_TOKENS`
    // or `prose_fields` prefix) — dropping them changes nothing `tab_fields`
    // would see. Ground truth: `ultimate_equipment:equipment:staff_of_mithral_might`,
    // `hand_wiring_class: static`.
    #[test]
    fn d3_bonus_stat_selector_false_positive_second_real_row() {
        let (class, reason) = cls(
            "Staff of Mithral Might\tCOST:58000\tWT:7\tCRITMULT:x2\tCRITRANGE:1\tDAMAGE:1d6\tALTDAMAGE:1d6\tBONUS:STAT|INT|2|TYPE=Enhancement",
        );
        assert_eq!(class, WiringClass::Static);
        assert_eq!(reason, "literal_magnitudes_only");
    }

    // Regression guard: the STAT-selector strip must not blind the scan to
    // a GENUINE scalar/arithmetic magnitude living elsewhere in the same
    // `BONUS:` token family on the same row.
    // `core_essentials/ce_races_familiar_um.lst:28`, "Pig" (re-derived this
    // cycle) — `BONUS:WEAPONPROF=Bite|DAMAGE|max(0,(STR/2))` is a real
    // STR-dependent formula, entirely separate from the row's six
    // `BONUS:STAT|<ability>|<flat-int>` fields and its `CR:1/3` fraction.
    // Ground truth (`core_essentials:companion:pig`) confirms `derived` is
    // still the correct class after both false-positive fixes land.
    #[test]
    fn d3_bonus_stat_false_positive_does_not_hide_a_real_bonus_formula() {
        let (class, reason) = cls(
            "Pig\tSTARTFEATS:1\tSIZE:S\tMOVE:Walk,30\tREACH:5\t\
             BONUS:STAT|STR|0\tBONUS:STAT|DEX|2\tBONUS:STAT|CON|4\t\
             BONUS:STAT|INT|-8\tBONUS:STAT|WIS|2\tBONUS:STAT|CHA|-6\t\
             BONUS:VAR|AC_Natural_Armor|1|TYPE=Base\tBONUS:VAR|RaceSizeIsLong|1|TYPE=Base\t\
             BONUS:WEAPONPROF=Bite|DAMAGE|max(0,(STR/2))\tCR:1/3",
        );
        assert_eq!(class, WiringClass::Derived);
        assert_eq!(reason, "bonus");
    }

    // D3 — `CR:`/`DR:` `/` bypass-and-fraction notation is not arithmetic
    // (`OPEN-ISSUES.md` row 2(b)). `CR:1/3` is Pathfinder's literal sub-1
    // challenge-rating fraction, not a division; a pre-fix `has_arith`'s
    // unconditional `value.contains('/')` misread it as `derived:cr`.
    // Corpus-wide re-derivation this cycle (38 books) found every `CR:`
    // value carrying a `/` is exactly this `<int>/<int>` shape.
    #[test]
    fn d3_cr_fraction_slash_is_not_arithmetic() {
        let (class, reason) = cls("Fractional Threat\tCR:1/3\tMOVE:Walk,20");
        assert_eq!(class, WiringClass::Static);
        assert_eq!(reason, "literal_magnitudes_only");
    }

    // `DR:10/Cold Iron` is PCGen's `<amount>/<bypass-type>` notation, not
    // division. Corpus-wide re-derivation this cycle found all 267 `DR:`
    // values carrying a `/` follow this shape; none is an ambiguous bare
    // `<int>/<int>` that could plausibly be real division.
    #[test]
    fn d3_dr_bypass_slash_is_not_arithmetic() {
        let (class, reason) = cls("Iron Hide\tDR:10/Cold Iron\tMOVE:Walk,30");
        assert_eq!(class, WiringClass::Static);
        assert_eq!(reason, "literal_magnitudes_only");
    }

    // Regression guard: a genuinely dynamic DR must still signal `derived`
    // via its own `*`, even though the trailing `/` is no longer treated
    // as arithmetic for the `DR:` token. Real corpus shape (multiple rows),
    // `DR:1*ArmoredDefenseMult/-`.
    #[test]
    fn d3_dr_bypass_slash_fix_does_not_hide_a_real_multiplicative_dr() {
        let (class, reason) = cls("Armored Defense\tDR:1*ArmoredDefenseMult/-\tMOVE:Walk,30");
        assert_eq!(class, WiringClass::Derived);
        assert_eq!(reason, "dr");
    }

    // D4 (SD31-W2-INTEGRATE-001, Finding 1, over-shoot repair) — the D3 fix
    // above must not go so far that a NAMED-VARIABLE `DR:` amount is
    // silently swept into `literal_magnitudes_only`. Real row,
    // `pathfinder_unchained/pu_abilities_race.lst:101`, "Eidolon
    // Progession Lv.12" (`Agathion ~ Unchained Eidolon LVL12`):
    // `DR:EidolonDR/evil` where `EidolonDR` is a `DEFINE:`d/`BONUS:VAR`-set
    // variable, not a literal amount — the segment before the `/` is not a
    // bare integer, so the `/`-as-division exclusion must not apply and
    // this must still signal `derived`.
    #[test]
    fn d4_dr_variable_amount_slash_is_derived_not_static() {
        let (class, reason) = cls(
            "Eidolon Progession Lv.12\tDEFINE:EidolonDR|0\tBONUS:VAR|EidolonDR|5\tDR:EidolonDR/evil",
        );
        assert_eq!(class, WiringClass::Derived);
        assert_eq!(reason, "dr");
    }

    // Regression guard alongside D4: `DR:10/Cold Iron` (bare-integer
    // amount) must still resolve `static`, proving the fix is scoped to
    // non-numeric amounts and does not regress the D3 fix it repairs.
    #[test]
    fn d4_dr_literal_amount_slash_still_static() {
        let (class, reason) = cls("Iron Hide\tDR:10/Cold Iron\tMOVE:Walk,30");
        assert_eq!(class, WiringClass::Static);
        assert_eq!(reason, "literal_magnitudes_only");
    }

    // D4 — the `BONUS:STAT` selector-strip fix (D3, row 2(a)) must not go
    // so far that a NAMED-VARIABLE magnitude is silently swept into
    // `literal_magnitudes_only` either. Real row,
    // `advanced_class_guide/acg_abilities_class.lst:2876`, "Mutagen
    // Strength/Primary" (`Mutagenic Mauler Brawler ~ Mutagen Strength
    // (First)`): `BONUS:STAT|STR|MutagenicMaulerMutagenStatBonus|TYPE=Alchemical`
    // — the magnitude segment after the STR selector is the variable
    // `MutagenicMaulerMutagenStatBonus`, not a literal integer, so the
    // strip must not blind the scan to it.
    #[test]
    fn d4_bonus_stat_variable_magnitude_is_derived_not_static() {
        let (class, reason) = cls(
            "Mutagen Strength/Primary\tBONUS:STAT|STR|MutagenicMaulerMutagenStatBonus|TYPE=Alchemical",
        );
        assert_eq!(class, WiringClass::Derived);
        assert_eq!(reason, "bonus");
    }

    // Regression guard alongside D4: a bare-integer `BONUS:STAT` magnitude
    // (the D3 fix's own worked example) must still resolve `static`.
    #[test]
    fn d4_bonus_stat_literal_magnitude_still_static() {
        let (class, reason) = cls("+2 Dexterity\tBONUS:STAT|DEX|2|TYPE=Racial");
        assert_eq!(class, WiringClass::Static);
        assert_eq!(reason, "literal_magnitudes_only");
    }

    // Both `BONUS:STAT`/`DR:` false positives compounded in one record,
    // testing THOSE TWO FIXES IN ISOLATION: `bestiary/b1_races.lst:305`,
    // "Neothelid" — six `BONUS:STAT|<ability>|<int>` fields (selector
    // collision) plus `DR:10/Cold Iron` (slash bypass) are proven, on their
    // own, to carry no genuine scalar/arithmetic magnitude.
    //
    // **This is deliberately NOT the full real row.** Neothelid's real
    // corpus row also carries a `SPELLS:Innate|TIMES=ATWILL|CASTERLEVEL=20|
    // Charm Monster,14+CHA|...` field with genuine CHA-scalar save-DC
    // formulas — invisible before `SD31-E2-F3-001` added `SPELLS:` to
    // `MAGNITUDE_TOKENS` (D6, Finding D). The record's TRUE overall class
    // is `derived`, via that field (see
    // `d6_neothelid_full_row_is_derived_via_spells_not_static` below), not
    // `static` — the ground-truth sample's original `hand_wiring_class:
    // static` label was itself wrong (not just stale), because at label
    // time no scanner anywhere examined `SPELLS:` fields either;
    // corrected in `SD31-E2-F1-ground-truth-sample-v1.json` this cycle
    // (`token_evidence` carries the full correction trail). This
    // stripped-row test still earns its keep as a narrow regression guard
    // that the STAT-selector and DR-slash fixes specifically don't
    // false-positive on their own.
    #[test]
    fn d3_bonus_stat_and_dr_false_positives_alone_resolve_to_static() {
        let (class, reason) = cls(
            "Neothelid\tSTARTFEATS:1\tSIZE:G\tMOVE:Walk,30,Fly,60\tREACH:20\t\
             BONUS:STAT|STR|20\tBONUS:STAT|DEX|-4\tBONUS:STAT|CON|14\t\
             BONUS:STAT|INT|6\tBONUS:STAT|WIS|4\tBONUS:STAT|CHA|10\t\
             BONUS:VAR|AC_Natural_Armor|26|TYPE=Base\tBONUS:VAR|BlindsightRange|100|TYPE=Base\t\
             BONUS:VAR|Maneuverability|4\tBONUS:VAR|NoTypeTraits|1\t\
             DEFINE:Maneuverability|0\tDEFINE:NoTypeTraits|0\tSR:26\tDR:10/Cold Iron\tCR:15",
        );
        assert_eq!(class, WiringClass::Static);
        assert_eq!(reason, "literal_magnitudes_only");
    }

    // D6 (SD31-E2-F3-001, Finding D) — the FULL real Neothelid row,
    // `bestiary/b1_races.lst:305` verbatim, including the `SPELLS:` field
    // the test above deliberately omits. The BONUS:STAT/DR: fields are
    // still all false positives (proven above in isolation), but the row's
    // TRUE class is `derived` via a real signal neither the BONUS:STAT nor
    // the DR fix has anything to do with: `SPELLS:Innate|TIMES=ATWILL|
    // CASTERLEVEL=20|Charm Monster,14+CHA|...` states genuine CHA-scalar
    // save DCs for the creature's innate spell-like abilities.
    #[test]
    fn d6_neothelid_full_row_is_derived_via_spells_not_static() {
        let (class, reason) = cls(
            "Neothelid\tSTARTFEATS:1\tSIZE:G\tMOVE:Walk,30,Fly,60\tREACH:20\t\
             BONUS:STAT|STR|20\tBONUS:STAT|DEX|-4\tBONUS:STAT|CON|14\t\
             BONUS:STAT|INT|6\tBONUS:STAT|WIS|4\tBONUS:STAT|CHA|10\t\
             BONUS:VAR|AC_Natural_Armor|26|TYPE=Base\tBONUS:VAR|BlindsightRange|100|TYPE=Base\t\
             BONUS:VAR|Maneuverability|4\tBONUS:VAR|NoTypeTraits|1\t\
             DEFINE:Maneuverability|0\tDEFINE:NoTypeTraits|0\tSR:26\tDR:10/Cold Iron\tCR:15\t\
             SPELLS:Innate|TIMES=ATWILL|CASTERLEVEL=20|Charm Monster,14+CHA|Clairaudience/Clairvoyance|\
             Detect Thoughts,12+CHA|Poison,14+CHA|Suggestion,13+CHA|Telekinesis,15+CHA|Teleport\t\
             SPELLS:Neothelid|TIMES=3|CASTERLEVEL=20|Quickened Suggestion,13+CHA",
        );
        assert_eq!(class, WiringClass::Derived);
        assert_eq!(reason, "spells");
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

    // D6 (SD31-E2-F3-001, Finding D, `OPEN-ISSUES.md` row 16). Real row,
    // `bestiary_4/b4_abilities_race.lst:1460`, "Winter Hag ~ Ice Staff":
    // `SPELLS:Ice Staff|CASTERLEVEL=10|Cone of Cold,15+CHA` states a
    // CHA-scalar save DC nowhere else on the row -- the record's ONLY
    // magnitude field. Before `SPELLS:` joined `MAGNITUDE_TOKENS`, this
    // fell all the way to `display:no_magnitude_token` (`mags` was empty,
    // so the row was never even scanned as carrying a chassis at all).
    #[test]
    fn d6_spells_field_scalar_formula_is_derived() {
        let (class, reason) = cls(
            "Ice Staff\tKEY:Winter Hag ~ Ice Staff\tCATEGORY:Special Ability\tTYPE:SpecialQuality.Supernatural\tSPELLS:Ice Staff|CASTERLEVEL=10|Cone of Cold,15+CHA",
        );
        assert_eq!(class, WiringClass::Derived);
        assert_eq!(reason, "spells");
    }

    // A `SPELLS:` field whose `CASTERLEVEL=` value is itself a named
    // variable (not a literal int) is exactly the same shape --
    // `advanced_class_guide:feat:nature_magic`'s
    // `SPELLS:Innate|TIMES=ATWILL|CASTERLEVEL=TL` (`TL` -- total level, a
    // recognized `SCALARS_WORD` entry).
    #[test]
    fn d6_spells_field_casterlevel_scalar_keyword_is_derived() {
        let (class, reason) =
            cls("Nature Magic\tTYPE:General\tSPELLS:Innate|TIMES=ATWILL|CASTERLEVEL=TL|Know Direction");
        assert_eq!(class, WiringClass::Derived);
        assert_eq!(reason, "spells");
    }

    // Regression guard: a `SPELLS:` field with no scalar/arithmetic
    // anywhere in it (a flat at-will grant, no scaling DC or duration)
    // must still resolve `static`, not be swept into `derived` just for
    // existing.
    #[test]
    fn d6_spells_field_with_no_scalar_stays_static() {
        let (class, reason) =
            cls("Innate Light\tTYPE:General\tSPELLS:Innate|CASTERLEVEL=1|Light");
        assert_eq!(class, WiringClass::Static);
        assert_eq!(reason, "literal_magnitudes_only");
    }

    // Regression guard, discovered writing the test above: a whole-value
    // scan of a `SPELLS:` field false-positives on its own STRUCTURAL tag
    // text -- `CASTERLEVEL=` always contains the substring `CASTERLEVEL`
    // regardless of what follows `=`, and `TIMES=<n>/DAY` (PCGen's "N times
    // per day" notation, not division) trips the unscoped `/` check. A
    // flat, fully-literal `SPELLS:` field carrying both tags must still
    // resolve `static`.
    #[test]
    fn d6_spells_field_times_per_day_slash_is_not_arithmetic() {
        let (class, reason) = cls(
            "Innate Fireball\tTYPE:General\tSPELLS:Innate|TIMES=3/DAY|CASTERLEVEL=10|Fireball",
        );
        assert_eq!(class, WiringClass::Static);
        assert_eq!(reason, "literal_magnitudes_only");
    }

    // Regression guard: a `TIMES=N/DAY` field must not hide a REAL scalar
    // elsewhere in the same `SPELLS:` value.
    #[test]
    fn d6_spells_field_times_per_day_does_not_hide_a_real_scalar() {
        let (class, reason) = cls(
            "Innate Fireball\tTYPE:General\tSPELLS:Innate|TIMES=3/DAY|CASTERLEVEL=TL|Fireball",
        );
        assert_eq!(class, WiringClass::Derived);
        assert_eq!(reason, "spells");
    }

    // D7 repair (`SD31-W3-INTEGRATE-001`, adversarial-review finding on
    // `SD31-E2-F3-001`'s `SPELLS:` change): a spell-name segment's own
    // literal slash (`Open/Close`, `Blindness/Deafness`,
    // `Clairaudience/Clairvoyance`) is not division, and must not flip an
    // otherwise fully-literal `SPELLS:` field to `derived`. Real row,
    // `inner_sea_bestiary/isb_abilities_race.lst:54`,
    // "Cayhound ~ Spell-Like Abilities":
    // `SPELLS:Innate|TIMES=ATWILL|Freedom of Movement|Open/Close` carries no
    // `CASTERLEVEL=`, no DC formula, and no other scalar/arith token
    // anywhere in the record.
    #[test]
    fn d7_spells_field_slash_in_spell_name_is_not_arithmetic() {
        let (class, reason) = cls(
            "Cayhound Spell-Like Abilities\tTYPE:General\tSPELLS:Innate|TIMES=ATWILL|Freedom of Movement|Open/Close",
        );
        assert_eq!(class, WiringClass::Static);
        assert_eq!(reason, "literal_magnitudes_only");
    }

    // D7 — same shape, a second canonical PF1 slashed spell name
    // (`Blindness/Deafness`), and a third segment in the same field to
    // confirm the fix is not order-dependent.
    #[test]
    fn d7_spells_field_blindness_deafness_is_not_arithmetic() {
        let (class, reason) = cls(
            "Doll Spells\tTYPE:General\tSPELLS:Innate|CASTERLEVEL=3|Light|Mage Hand|Open/Close|Prestidigitation|Blindness/Deafness",
        );
        assert_eq!(class, WiringClass::Static);
        assert_eq!(reason, "literal_magnitudes_only");
    }

    // D7 — a slash in the spell-NAME half must not hide a REAL DC formula
    // in the same segment's comma-delimited tail.
    #[test]
    fn d7_spells_field_slash_in_name_does_not_hide_a_real_dc_formula() {
        let (class, reason) = cls(
            "Slashed Save Spell\tTYPE:General\tSPELLS:Innate|CASTERLEVEL=10|Open/Close,15+CHA",
        );
        assert_eq!(class, WiringClass::Derived);
        assert_eq!(reason, "spells");
    }

    // D6 — `classlevel(...)` function-call form, case-insensitive
    // (`OPEN-ISSUES.md` row 9(a)). Real row,
    // `ultimate_magic/um_abilities_class.lst:1101`, "Dragon Shaman ~ Totem
    // Transformation": `BONUS:VAR|TotemTransformationDuration|classlevel("Druid")`
    // carries no uppercase `CLASSLEVEL` anywhere, so the pre-fix
    // case-sensitive `SCALARS_SUBSTRING` check missed it and the record
    // read `static` despite stating a genuine class-level-scaling duration.
    #[test]
    fn d6_lowercase_classlevel_function_call_is_derived() {
        let (class, reason) = cls(
            "Totem Transformation\tKEY:Dragon Shaman ~ Totem Transformation\tCATEGORY:Special Ability\tTYPE:DruidClassFeatures.ArchetypeDruid.SpecialQuality.Supernatural\tDEFINE:TotemTransformationDuration|0\tBONUS:VAR|TotemTransformationDuration|classlevel(\"Druid\")",
        );
        assert_eq!(class, WiringClass::Derived);
        assert_eq!(reason, "bonus");
    }

    // D6 — `+` immediately followed by a parenthesised sub-expression is
    // arithmetic (`OPEN-ISSUES.md` row 9(c)). Real row,
    // `horror_adventures/support/ha_abilities_class_oa.lst:305`, "Exciter ~
    // Rapturous Rage": `BONUS:ABILITYPOOL|Rage Power|10+(SpiritualistLVL>=14)+(SpiritualistLVL>=18)`
    // -- the pre-fix `+\s*\w*[A-Z]{2,}` rule requires a WORD character
    // immediately after `+`, which a `(` is not, so this real level-gated
    // formula was invisible to `has_arith_scoped` and the record read
    // `static`.
    #[test]
    fn d6_plus_paren_subexpression_is_derived() {
        let (class, reason) = cls(
            "Rapturous Rage\tKEY:Exciter ~ Rapturous Rage\tCATEGORY:Special Ability\tTYPE:Spiritualist Class Feature.ExciterClassFeatures.SpecialQuality\tBONUS:ABILITYPOOL|Rage Power|10+(SpiritualistLVL>=14)+(SpiritualistLVL>=18)",
        );
        assert_eq!(class, WiringClass::Derived);
        assert_eq!(reason, "bonus");
    }

    // Regression guard: an ordinary `+` followed by whitespace then a
    // lowercase word (no parenthesis, no uppercase run) must NOT be swept
    // into arithmetic by the new `+(` check -- only a literal `(`
    // immediately (modulo whitespace) after `+` counts.
    #[test]
    fn d6_plus_then_lowercase_word_without_paren_stays_non_arith() {
        assert!(!has_arith_scoped("10+ this is not a formula", true));
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

    // --- D0 corpus-row resolution (`OPEN-ISSUES.md` row 1,
    // SD31-E2-F2-001-wiringfix) ------------------------------------------

    /// A scratch corpus directory, cleaned up on drop, so these tests
    /// never touch the real PCGen checkout `PCGEN_CORPUS_ROOT` would
    /// point at. Same pattern as `v06_work_inventory`'s own
    /// `wiring_class_wiring_tests::ScratchBook`, plus `write_nested` for
    /// the subdirectory shapes this deliverable is about.
    struct ScratchBook {
        root: PathBuf,
    }

    impl ScratchBook {
        fn new(name: &str) -> Self {
            let root = std::env::temp_dir()
                .join(format!("codex_wiring_class_resolve_test_{name}_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&root);
            std::fs::create_dir_all(&root).unwrap();
            ScratchBook { root }
        }

        fn write(&self, filename: &str, contents: &str) {
            std::fs::write(self.root.join(filename), contents).unwrap();
        }

        /// `relative` may contain `/` path separators; parent directories
        /// are created as needed.
        fn write_nested(&self, relative: &str, contents: &str) {
            let path = self.root.join(relative);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, contents).unwrap();
        }
    }

    impl Drop for ScratchBook {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    // Regression guard: the single-level `dir.join(file)` fast path must
    // resolve identically to before this deliverable for every book whose
    // `.lst` files sit directly in the book root (every currently-resolving
    // unit).
    #[test]
    fn corpus_lines_direct_join_unchanged_for_a_flat_book_layout() {
        let book = ScratchBook::new("directjoin");
        book.write("flat_file.lst", "Row One\tTYPE:General\n");
        let mut book_paths = BTreeMap::new();
        book_paths.insert("test_book".to_string(), book.root.clone());
        let mut lines = CorpusLines::new(&book_paths);
        assert_eq!(
            lines.line("test_book", "flat_file.lst", 1).as_deref(),
            Some("Row One\tTYPE:General")
        );
    }

    // The failing case this deliverable exists to fix: a real corpus row
    // that the pre-fix single-level join could not reach at all.
    // `core_essentials/races/android/android_races.lst:6`, "Android"
    // (re-derived this cycle) — `book_paths["core_essentials"]` is the
    // book ROOT, but the file lives two directories deeper
    // (`races/android/`). Before this fix, `CorpusLines::line()` returned
    // `None` here and the unit fell to D0 `ambiguous:no_corpus_line`
    // despite the row genuinely existing
    // (`docs/work-inventory.json`, `core_essentials:race:android`,
    // pre-fix `wiring_class_reason: no_corpus_line`).
    #[test]
    fn corpus_lines_resolves_a_nested_lst_file_the_direct_join_misses() {
        let book = ScratchBook::new("nested_android");
        book.write_nested(
            "races/android/android_races.lst",
            "\n\n\n\n\nAndroid\tSORTKEY:a_base_pc\tSTARTFEATS:1\tFACT:BaseSize|M\tMOVE:Walk,30\tABILITY:Internal|AUTOMATIC|Racial Traits ~ Android\tLEGS:2\tHANDS:2\tRACETYPE:Humanoid\tTYPE:Base.PC\tSOURCEPAGE:p.xx\tFACT:IsPC|True\n",
        );
        let mut book_paths = BTreeMap::new();
        book_paths.insert("core_essentials".to_string(), book.root.clone());
        let mut lines = CorpusLines::new(&book_paths);

        let row = lines.line("core_essentials", "android_races.lst", 6);
        assert_eq!(
            row.as_deref(),
            Some(
                "Android\tSORTKEY:a_base_pc\tSTARTFEATS:1\tFACT:BaseSize|M\tMOVE:Walk,30\tABILITY:Internal|AUTOMATIC|Racial Traits ~ Android\tLEGS:2\tHANDS:2\tRACETYPE:Humanoid\tTYPE:Base.PC\tSOURCEPAGE:p.xx\tFACT:IsPC|True"
            )
        );
        // The correctness bar is the whole point: the resolved row must
        // classify as the record genuinely implies (`static` — its only
        // magnitude field, `MOVE:Walk,30`, is a plain literal), not merely
        // resolve to SOME text.
        let (class, _, _) = determine(row.as_deref());
        assert_eq!(class, WiringClass::Static);
    }

    // Collision safety: a same-named `.lst` file nested under two
    // different subdirectories of the SAME book. Corpus-wide enumeration
    // (this cycle's receipt) found zero such collisions across all 38
    // known book directories today, so this is a defensive guard against
    // a future corpus revision, not a currently-triggered case — but it
    // proves the implementation never silently picks one arbitrary match
    // over another (which a naive `find`-style first-match glob would):
    // resolving to the WRONG file is worse than resolving to none, so an
    // ambiguous basename must resolve `None`, exactly like no match.
    #[test]
    fn corpus_lines_refuses_to_guess_when_a_nested_basename_collides_within_one_book() {
        let book = ScratchBook::new("collision");
        book.write_nested("races/alpha/shared.lst", "Alpha Row\tTYPE:General\n");
        book.write_nested("races/beta/shared.lst", "Beta Row\tTYPE:General\n");
        let mut book_paths = BTreeMap::new();
        book_paths.insert("test_book".to_string(), book.root.clone());
        let mut lines = CorpusLines::new(&book_paths);
        // Line 2, not 1: an unresolved file caches as a 1-element buffer
        // (`"".split('\n')` yields one empty string, not zero elements) --
        // a PRE-EXISTING quirk of the `unwrap_or_default()` + `split('\n')`
        // pattern this deliverable did not touch, under which `line == 1`
        // against ANY unresolved file returns `Some("")` rather than
        // `None`. Zero real corpus units carry `source_line == 1` in the
        // `no_corpus_line` population today (re-derived this cycle), so it
        // is out of this deliverable's bounded scope; logged to
        // `OPEN-ISSUES.md` as an informational finding rather than fixed
        // here.
        assert_eq!(lines.line("test_book", "shared.lst", 2), None);
    }

    // Collision safety, the ROOT-shadows-NESTED axis (`SD31-W2-INTEGRATE-001`,
    // Finding 4). Before this fix, `resolve_corpus_file`'s direct
    // `dir.join(file)` fast path returned EARLY, before the nested-search
    // collision scan ran at all — so a book carrying `shared.lst` at its
    // ROOT *and* `sub/shared.lst` nested would silently resolve every
    // caller against the root copy, never detecting the nested duplicate.
    // Corpus-wide enumeration found zero real instances of this shape
    // (`resolve_corpus_file`'s doc comment), so this is a defensive guard
    // against a future corpus revision, exactly like the nested-vs-nested
    // test above — but it must refuse to guess here too, not just there.
    #[test]
    fn corpus_lines_refuses_to_guess_when_a_root_file_shadows_a_nested_basename_collision() {
        let book = ScratchBook::new("root_shadow_collision");
        book.write("shared.lst", "Root Row\tTYPE:General\n");
        book.write_nested("sub/shared.lst", "Nested Row\tTYPE:General\n");
        let mut book_paths = BTreeMap::new();
        book_paths.insert("test_book".to_string(), book.root.clone());
        let mut lines = CorpusLines::new(&book_paths);
        // Line 2, not 1 -- same pre-existing empty-buffer quirk noted above.
        assert_eq!(lines.line("test_book", "shared.lst", 2), None);
    }

    // Regression guard alongside the root-shadow test: a book with ONLY a
    // root-level file (no nested duplicate) must still resolve it via the
    // fast path, proving the collision-safety fix does not regress the
    // ordinary single-match case.
    #[test]
    fn corpus_lines_direct_join_still_resolves_when_no_nested_duplicate_exists() {
        let book = ScratchBook::new("root_only_no_collision");
        book.write("solo.lst", "Solo Row\tTYPE:General\n");
        let mut book_paths = BTreeMap::new();
        book_paths.insert("test_book".to_string(), book.root.clone());
        let mut lines = CorpusLines::new(&book_paths);
        assert_eq!(lines.line("test_book", "solo.lst", 1).as_deref(), Some("Solo Row\tTYPE:General"));
    }

    // Collision safety, the other axis: a same-named nested file in TWO
    // DIFFERENT books must never cross-resolve — book A's lookup must
    // return book A's content even though book B carries an
    // identically-named file at the identical relative path. This is the
    // "a shared name does not mean a shared thing" hazard this program has
    // been bitten by repeatedly, proven directly rather than assumed from
    // `book_paths` scoping alone.
    #[test]
    fn corpus_lines_nested_resolution_stays_scoped_to_its_own_book_not_a_same_named_sibling() {
        let book_a = ScratchBook::new("crossbook_a");
        let book_b = ScratchBook::new("crossbook_b");
        book_a.write_nested("support/shared_name.lst", "From Book A\tTYPE:General\n");
        book_b.write_nested("support/shared_name.lst", "From Book B\tTYPE:General\n");
        let mut book_paths = BTreeMap::new();
        book_paths.insert("book_a".to_string(), book_a.root.clone());
        book_paths.insert("book_b".to_string(), book_b.root.clone());
        let mut lines = CorpusLines::new(&book_paths);
        assert_eq!(
            lines.line("book_a", "shared_name.lst", 1).as_deref(),
            Some("From Book A\tTYPE:General")
        );
        assert_eq!(
            lines.line("book_b", "shared_name.lst", 1).as_deref(),
            Some("From Book B\tTYPE:General")
        );
    }

    // A file genuinely absent from the book tree at any depth must still
    // resolve `None` — the bounded nested search must not manufacture a
    // match, and must not panic or loop on an ordinary empty book.
    #[test]
    fn corpus_lines_still_none_for_a_file_absent_at_every_depth() {
        let book = ScratchBook::new("absent");
        book.write_nested("races/oread/oread_races.lst", "Oread\tTYPE:Base.PC\n");
        let mut book_paths = BTreeMap::new();
        book_paths.insert("test_book".to_string(), book.root.clone());
        let mut lines = CorpusLines::new(&book_paths);
        // Line 2, see the comment in the collision test above for why 1
        // is excluded here.
        assert_eq!(lines.line("test_book", "does_not_exist.lst", 2), None);
    }
}
