//! Race-chassis + standard-racial-trait ingestion for SD-27
//! (`docs/release/SD-27-future-state-book-content-ingestion/decisions.md`
//! §25, §26).
//!
//! Run with:
//!
//! ```text
//! cargo run --bin ingest_races
//! ```
//!
//! Reads the 18 in-scope races out of PCGen's shared race storage at
//! `pathfinder/paizo/roleplaying_game/core_essentials/races/<dir>/` and
//! writes Shape B v1 records to
//! `data/corpus/{core_rulebook,beastiary}/{race,race_trait}/`.
//!
//! # Scope, and why `core_essentials` never appears in an output path
//!
//! `core_essentials/` is **PCGen's physical storage for race files shared
//! across books, not a book**, and it is out of project scope
//! (`decisions.md §1`, §25.2). This tool therefore *reads* from it but
//! never *attributes* to it: every record is filed under the race's true
//! source book, taken from `advanced_race_guide.pcc`'s own section
//! comments — Core Rulebook for the 7 core races, Bestiary 1 for its 11
//! (`decisions.md §25.2`'s table). `core_essentials` gets no corpus
//! directory, no `RuleSetId` variant, and no `data/stubs/` entry.
//!
//! The other 19 races ARG reprints (Bestiary 2/3/4, Inner Sea World
//! Guide) are deliberately **not** ingested here: their source books are
//! unregistered, and creating their first content would be inventing
//! provenance for a tome nobody has audited (`decisions.md §25.3`).
//!
//! # Provenance honesty
//!
//! Two placeholder hazards in the upstream corpus are handled explicitly
//! rather than transcribed:
//!
//! 1. **Chassis `SOURCEPAGE:p.xx`** (`decisions.md §26`, corpus-quality
//!    note). [`RaceCacheData`] has no page field at all, so the chassis
//!    placeholder cannot leak into a record as though it were a citation.
//!    It is still preserved verbatim inside `raw_tokens`, where it reads
//!    as raw source text, not as a resolved citation.
//! 2. **Trait `SOURCEPAGE:p.xx`.** §26 observed that Dwarf's trait rows
//!    carry a real `p.21`. That is true of Dwarf, but *not* of most
//!    races — the placeholder recurs on the trait rows too (see the
//!    per-race breakdown this binary prints). `source_page` is therefore
//!    set to `None` whenever the value is the literal placeholder, so a
//!    populated `source_page` always means a real page. The raw token is
//!    still preserved verbatim in `raw_tokens`.
//!
//! # Determinism
//!
//! Races, traits, and tokens are emitted in source order; output paths
//! are derived from the record key; JSON is pretty-printed by
//! `serde_json` in struct-declaration order. The only non-content input
//! is `ingested_at`, which honours `SD27_INGESTED_AT` when set so a run
//! can be reproduced byte-for-byte, and otherwise stamps real UTC now
//! (the convention every sibling generator in `src/bin/` already uses).

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use codex::rules_core::shape_b_v1::{
    Completeness, CorpusRecordV1, CorpusSource, License, Population, RaceCacheData, RaceTraitCacheData, RawBonusChain,
    RawToken,
};

/// PCGen-repo-relative prefix for the shared race storage. Matches the
/// `source.path` convention every existing on-disk record uses (relative
/// to the PCGen checkout's `data/` directory, e.g.
/// `pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst`).
const RACES_RELATIVE: &str = "pathfinder/paizo/roleplaying_game/core_essentials/races";

/// The literal placeholder PCGen leaves where a real page citation
/// belongs (`decisions.md §26`). Never stored as a `source_page`.
const PLACEHOLDER_SOURCE_PAGE: &str = "p.xx";

/// One in-scope race: the `core_essentials/races/` directory it is read
/// out of, and the corpus book directory it is attributed to.
#[derive(Clone, Copy)]
struct RaceSpec {
    /// Directory name under `core_essentials/races/`. Note the LST file
    /// basenames inside do *not* always match it (`half_elf/` holds
    /// `halfelf_races.lst`), so files are discovered by suffix, not by
    /// composing this name.
    dir: &'static str,
    /// Corpus book directory. Note the repo spells Bestiary 1's
    /// directory `beastiary` (pre-existing; `data/corpus/beastiary/`).
    book: &'static str,
}

/// The 18 races whose true source book is already ingested
/// (`decisions.md §25.3`). Core Rulebook's 7 and Bestiary 1's 11.
const IN_SCOPE_RACES: &[RaceSpec] = &[
    // `# Core Races` section of advanced_race_guide.pcc -> Core Rulebook.
    RaceSpec { dir: "dwarf", book: "core_rulebook" },
    RaceSpec { dir: "elf", book: "core_rulebook" },
    RaceSpec { dir: "gnome", book: "core_rulebook" },
    RaceSpec { dir: "half_elf", book: "core_rulebook" },
    RaceSpec { dir: "half_orc", book: "core_rulebook" },
    RaceSpec { dir: "halfling", book: "core_rulebook" },
    RaceSpec { dir: "human", book: "core_rulebook" },
    // `# B1 races` section of advanced_race_guide.pcc -> Bestiary 1.
    RaceSpec { dir: "aasimar", book: "beastiary" },
    RaceSpec { dir: "drow", book: "beastiary" },
    RaceSpec { dir: "duergar", book: "beastiary" },
    RaceSpec { dir: "goblin", book: "beastiary" },
    RaceSpec { dir: "hobgoblin", book: "beastiary" },
    RaceSpec { dir: "kobold", book: "beastiary" },
    RaceSpec { dir: "merfolk", book: "beastiary" },
    RaceSpec { dir: "orc", book: "beastiary" },
    RaceSpec { dir: "svirfneblin", book: "beastiary" },
    RaceSpec { dir: "tengu", book: "beastiary" },
    RaceSpec { dir: "tiefling", book: "beastiary" },
];

/// Heuristic OGL/PI screen (`docs/governance/ogl-pi-blacklist.md`) — the
/// same bounded substring scan `src/bin/sd27_gen_book_cache.rs` and
/// `scripts/sd27_apg_license_retrofit.py` already apply to the 4
/// previously-in-scope books. Racial traits are pure game mechanics
/// (ability-score adjustments, saves, speeds, weapon familiarity), so
/// every record here is expected to classify `OGL`; this screen exists so
/// that expectation is *checked* rather than assumed, and a hit fails the
/// run loudly instead of shipping unreviewed Product Identity.
const PI_BLACKLIST_TERMS: &[&str] = &[
    "Iomedae", "Sarenrae", "Asmodeus", "Cayden Cailean", "Abadar", "Calistria", "Desna", "Erastil", "Gorum", "Gozreh",
    "Irori", "Lamashtu", "Nethys", "Norgorber", "Pharasma", "Rovagug", "Shelyn", "Torag", "Urgathoa", "Zon-Kuthon",
    "Golarion", "Absalom", "Cheliax", "Varisia", "Andoran", "Taldor", "Osirion", "Katapesh", "Ustalav", "Numeria",
    "Mwangi", "Tian Xia", "Avistan", "Garund", "Sarkoris", "Worldwound", "Vudra", "Kyonin", "Molthune", "Nidal",
    "Nirmathas", "Qadira", "Razmiran", "Rahadoum", "Galt", "Isger", "Lastwall", "Brevoy", "Druma", "Irrisen",
    "Jalmeray", "Thuvia", "Geb", "Nex",
];

// ---------------------------------------------------------------------
// LST parsing primitives
// ---------------------------------------------------------------------

/// One real (non-comment, non-blank) LST row: its 1-indexed line number
/// and its tab-delimited fields with empties discarded.
///
/// PCGen's PrettyLST writer pads every row out to a fixed column grid
/// with runs of consecutive tabs, so "split on tab, drop empties" is the
/// format's actual field rule — a positional read would be wrong.
#[derive(Debug, Clone, PartialEq, Eq)]
struct LstRow {
    line_no: u32,
    fields: Vec<String>,
}

impl LstRow {
    /// The unkeyed first column — the record's display name.
    fn name(&self) -> &str {
        self.fields.first().map(String::as_str).unwrap_or_default()
    }

    /// Every keyed token after the name column, as `(key, value)` split
    /// on the *first* colon (values routinely contain further colons,
    /// e.g. `BONUS:SITUATION|...|PREVAREQ:Foo,0`).
    fn tokens(&self) -> impl Iterator<Item = (&str, &str)> {
        self.fields.iter().skip(1).map(|f| split_token(f))
    }

    /// First value for `key`, or `None`.
    fn first(&self, key: &str) -> Option<&str> {
        self.tokens().find(|(k, _)| *k == key).map(|(_, v)| v)
    }
}

/// Splits one LST field into `(key, value)` on the first colon. A field
/// with no colon yields `(field, "")`.
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
            line.split('\t').map(|f| f.trim_end_matches('\r')).filter(|f| !f.trim().is_empty()).map(String::from).collect();
        if fields.is_empty() {
            continue;
        }
        out.push(LstRow { line_no: (idx + 1) as u32, fields });
    }
    out
}

/// Splits a dotted `TYPE:` chain into its tokens.
fn type_tokens(row: &LstRow) -> Vec<String> {
    row.first("TYPE").map(|v| v.split('.').map(String::from).collect()).unwrap_or_default()
}

/// Every non-`BONUS:` token, preserved verbatim. `BONUS:` clauses are
/// carried separately as [`RawBonusChain`]s, matching the shape every
/// pre-existing Shape B v1 record on disk already uses (see
/// `data/corpus/core_rulebook/equipment/arms_armor/padded_armor_base.json`).
fn raw_tokens_excluding_bonus(row: &LstRow) -> Vec<RawToken> {
    row.tokens()
        .filter(|(k, _)| *k != "BONUS")
        .map(|(k, v)| RawToken { key: k.to_string(), value: v.to_string() })
        .collect()
}

/// Every token including `BONUS:` ones — used for the race chassis, whose
/// [`RaceCacheData`] payload has no separate bonus-chain field, so "every
/// token" genuinely means every token (Halfling's chassis row carries a
/// `BONUS:SAVE|ALL|...` that would otherwise be dropped).
fn raw_tokens_all(row: &LstRow) -> Vec<RawToken> {
    row.tokens().map(|(k, v)| RawToken { key: k.to_string(), value: v.to_string() }).collect()
}

/// Every `BONUS:` clause's pipe-delimited qualifiers, in source order.
fn raw_bonus_chains(row: &LstRow) -> Vec<RawBonusChain> {
    row.tokens()
        .filter(|(k, _)| *k == "BONUS")
        .map(|(_, v)| RawBonusChain { qualifiers: v.split('|').map(String::from).collect() })
        .collect()
}

// ---------------------------------------------------------------------
// `DESC:` rendering
// ---------------------------------------------------------------------

/// Every variable this row defines *and finishes* on its own, with its
/// resolved integer value — or `None` where the row names the variable
/// but its value depends on something the row does not itself state.
///
/// PCGen seeds a row-local variable with `DEFINE:<Var>|<base>` and adds
/// to it with `BONUS:VAR|<Var>|<value>`. Where both are integer literals
/// on the same row, the variable is a constant written across two tokens:
/// Dwarf's Defensive Training row carries `DEFINE:RacialDefensiveTrainingBonus|0`
/// and `BONUS:VAR|RacialDefensiveTrainingBonus|4`, so the value is 4 and
/// reading it is transcription, not evaluation. `decisions.md §24`'s ban
/// on a formula interpreter is therefore not engaged.
///
/// The instant any contribution stops being a same-row literal — a
/// formula (`BONUS:VAR|X|OtherVar`), a conditional bonus (a trailing
/// `PRE...` qualifier), or a base declared elsewhere in the corpus — the
/// variable is marked unresolvable and **no value is guessed**.
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
                // Never `DEFINE`d here: the base lives in another file,
                // so this row cannot resolve the variable by itself.
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
/// substitution argument. PCGen prerequisites are upper-case and always
/// carry a colon (`PREVARLTEQ:Orc_OrcFerocity_Times,1`); variable names
/// never contain one.
fn is_prerequisite_arg(arg: &str) -> bool {
    arg.contains(':') && (arg.starts_with("PRE") || arg.starts_with("!PRE"))
}

/// Evaluates one `PREVAR<CMP>:<lhs>,<rhs>[,<lhs>,<rhs>...]` gate against
/// the row's own variable table, honouring a leading `!` as negation and
/// requiring every pair to hold.
///
/// This compares two same-row constants; it is not formula evaluation.
/// An operand must be an integer literal or a variable [`same_row_vars`]
/// already resolved to one. Anything undecidable — an unknown comparison,
/// a prerequisite kind this does not model, an operand defined elsewhere —
/// is an `Err`, never a coin flip: a gate decides what the rules text
/// *says* ("Once" vs "Twice per day"), so guessing it would ship a false
/// statement rather than merely an incomplete one.
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

/// Collapses every whitespace run to a single space and trims the ends.
/// Applied only where a placeholder was removed, so prose that needed no
/// edit stays byte-identical to the source.
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

/// Replaces every `%N` in one `DESC:` segment with argument N's resolved
/// literal, returning the rendered text and the names of any arguments
/// that would not resolve.
///
/// An unresolvable argument is **dropped, never guessed**: the
/// placeholder goes, the `+`/`-` sign that introduced it goes with it,
/// and the whitespace is closed up so the sentence still reads. The raw
/// argument tail is not emitted under any branch — that is the whole
/// point of this function.
fn substitute_placeholders(prose: &str, args: &[&str], vars: &BTreeMap<String, Option<i64>>) -> (String, Vec<String>) {
    let chars: Vec<char> = prose.chars().collect();
    let mut out = String::new();
    let mut unresolved: Vec<String> = Vec::new();
    let mut dropped_any = false;
    let mut i = 0;

    while i < chars.len() {
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

/// One row's rendered player-facing description, plus the arguments that
/// could not be resolved (reported by the binary rather than swallowed).
#[derive(Debug, Clone, PartialEq, Eq)]
struct RenderedDescription {
    text: Option<String>,
    unresolved_args: Vec<String>,
}

/// Turns a row's `DESC:` tokens into the prose the player actually sees.
///
/// PCGen's format is `DESC:<prose>|<arg1>|<arg2>...`, where `%1` in the
/// prose stands for arg 1's resolved value, and where an argument that
/// looks like a prerequisite gates the whole segment instead. A row may
/// carry several `DESC:` tokens (Half-Orc's Orc Ferocity carries four);
/// the surviving segments concatenate, in source order, into one
/// description.
///
/// Storing the raw token instead — which is what this binary used to do —
/// put PCGen substitution syntax on screen verbatim, e.g. *"Dwarves get a
/// +%1 dodge bonus to AC against monsters of the giant
/// subtype.|RacialDefensiveTrainingBonus"*.
fn render_description(row: &LstRow) -> Result<RenderedDescription, String> {
    let vars = same_row_vars(row);
    let mut segments: Vec<String> = Vec::new();
    let mut unresolved_args: Vec<String> = Vec::new();
    let mut saw_desc = false;

    for (_, value) in row.tokens().filter(|(k, _)| *k == "DESC") {
        saw_desc = true;
        let mut parts = value.split('|');
        let prose = parts.next().unwrap_or_default();
        let (gates, args): (Vec<&str>, Vec<&str>) = parts.partition(|p| is_prerequisite_arg(p));

        let mut applies = true;
        for gate in &gates {
            // Every gate is evaluated even once one has failed, so an
            // undecidable gate is surfaced rather than masked by a
            // neighbour that happened to be decided first.
            applies &= eval_prevar_gate(gate, &vars)?;
        }
        if !applies {
            continue;
        }

        let (text, mut unresolved) = substitute_placeholders(prose, &args, &vars);
        unresolved_args.append(&mut unresolved);
        if !text.is_empty() {
            segments.push(text);
        }
    }

    let joined = segments.join(" ");
    let text = if !saw_desc || joined.is_empty() { None } else { Some(joined) };
    Ok(RenderedDescription { text, unresolved_args })
}

/// The PCGen substitution syntax that must never reach a player: an
/// unsubstituted `%<digit>` placeholder, or a raw `|` argument tail.
/// Used as a production guard on every description this binary writes.
fn leaked_pcgen_syntax(text: &str) -> Option<&'static str> {
    if text.contains('|') {
        return Some("raw '|' argument tail");
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
// Chassis (`<name>_races.lst`)
// ---------------------------------------------------------------------

/// Extracts `FACT:BaseSize|M` -> `"M"` from a chassis row.
fn base_size(row: &LstRow) -> Option<String> {
    row.tokens()
        .filter(|(k, _)| *k == "FACT")
        .find_map(|(_, v)| v.strip_prefix("BaseSize|"))
        .map(str::to_string)
}

/// Extracts the `Walk` leg of `MOVE:Walk,20` -> `20`. `MOVE:` is a
/// comma-separated list of `mode,rate` pairs, so this reads pairwise
/// rather than assuming `Walk` comes first.
///
/// Faithfulness note: Goblin and Hobgoblin chassis rows genuinely carry
/// `MOVE:Walk,0` upstream — their real 30 ft. speed is granted by their
/// `~ Speed` racial trait row's own `MOVE:Walk,30`. The `0` is recorded
/// as-is; inventing a 30 here would be fabricating chassis data.
fn base_move_walk(row: &LstRow) -> Option<i32> {
    let move_value = row.first("MOVE")?;
    let parts: Vec<&str> = move_value.split(',').collect();
    for pair in parts.chunks(2) {
        if let [mode, rate] = pair
            && mode.eq_ignore_ascii_case("Walk")
        {
            return rate.trim().parse::<i32>().ok();
        }
    }
    None
}

fn parse_chassis(row: &LstRow) -> RaceCacheData {
    RaceCacheData {
        key: row.name().to_string(),
        name: row.name().to_string(),
        base_size: base_size(row),
        base_move_walk: base_move_walk(row),
        race_type: row.first("RACETYPE").map(str::to_string),
        type_tokens: type_tokens(row),
        legs: row.first("LEGS").and_then(|v| v.trim().parse::<i32>().ok()),
        hands: row.first("HANDS").and_then(|v| v.trim().parse::<i32>().ok()),
        raw_tokens: raw_tokens_all(row),
    }
}

// ---------------------------------------------------------------------
// Standard racial traits (`<name>_abilities_race.lst`)
// ---------------------------------------------------------------------

/// True when a row is a *standard racial trait* row.
///
/// The selector is the leading `RacialTraits` token of the dotted `TYPE:`
/// chain. Every one of the 18 files mixes standard traits in with rows
/// that must not be ingested as traits: the `CATEGORY:Internal` grantor
/// row, `.MOD` support rows (`TYPE:Dwarf Racial Trait` alone),
/// Ranger favored-enemy entries (`TYPE:RangerClassFeatures...`),
/// `TYPE:AdoptiveRace` selectors, and per-race extras like
/// `TYPE:Aasimar Subrace` / `TYPE:Tiefling Language Choice` /
/// `TYPE:Gnome Obsessive Skill Bonus`. Only the standard-trait rows lead
/// with `RacialTraits`.
fn is_standard_racial_trait(row: &LstRow) -> bool {
    type_tokens(row).first().map(|t| t == "RacialTraits").unwrap_or(false)
}

/// True when the dotted `TYPE:` chain carries the `<Race> Racial Default`
/// marker (`decisions.md §26`: standard traits are self-identifying, so
/// the default roster is read from the corpus rather than assumed).
///
/// The comparison is ASCII-case-insensitive because one upstream row —
/// Drow's `Drow ~ Poison Use` — spells it `Drow Racial default`. It is
/// unambiguously the same marker in the same position in the same chain;
/// a case-sensitive read would silently drop a genuine default trait.
fn is_racial_default(type_tokens: &[String], race_key: &str) -> bool {
    let marker = format!("{race_key} Racial Default");
    type_tokens.iter().any(|t| t.eq_ignore_ascii_case(&marker))
}

/// The `PREVAREQ:<Flag>,0` gate a race's `_abilities_globalvar.lst`
/// declares for each of its standard traits, keyed by trait key.
///
/// # Why a second source exists at all
///
/// `decisions.md §26` describes the swap protocol as a negated fact-check
/// on the standard trait's own row (`!PREFACT:1,ABILITIES,<Flag>=True`).
/// That is true, and it is not the whole protocol. PCGen states every
/// gate a **second** time, per race, as a `.MOD` in
/// `core_essentials/races/<race>/<race>_abilities_globalvar.lst`:
///
/// ```text
/// CATEGORY=Special Ability|Aasimar ~ Default.MOD
///     ABILITY:Aasimar Racial Trait|AUTOMATIC|Aasimar ~ Skilled|PREVAREQ:Aasimar_ReplaceSkilled,0
/// ```
///
/// Read: *grant `Aasimar ~ Skilled` while `Aasimar_ReplaceSkilled` is 0* —
/// the same statement `!PREFACT` makes, inverted. Some races' `.MOD` rows
/// address `CATEGORY=Internal|Racial Traits ~ <Race>.MOD` instead of
/// `CATEGORY=Special Ability|<Race> ~ Default.MOD`; both are read, because
/// the operative token is the `ABILITY:` grant either way.
///
/// # Why reading it is transcription, not invention
///
/// It is checkable against the first source and is checked, per row, by
/// the caller: where a trait row carries its own `!PREFACT`, the globalvar
/// gate must name every flag that row names, and a contradiction fails the
/// run. Across the 18 in-scope races the two agree on all 166 rows that
/// carry a `!PREFACT`, and the globalvar speaks for 9 more — Aasimar's,
/// whose `_abilities_race.lst` carries no `!PREFACT` token at all. Those 9
/// are why ARG's 9 Aasimar alternate racial traits could be offered to a
/// player and never work.
///
/// Only `PREVAREQ:<Flag>,0` clauses are read. A `PREVAREQ:<Flag>,1` is the
/// opposite statement — a *positive* requirement, used by Duergar's two
/// mutually-exclusive spell-like-ability rows — and treating it as a
/// suppressor would invert the rule.
fn globalvar_gates(text: &str, race_key: &str) -> BTreeMap<String, Vec<String>> {
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for row in parse_rows(text) {
        for (key, value) in row.tokens() {
            if key != "ABILITY" {
                continue;
            }
            let parts: Vec<&str> = value.split('|').collect();
            // `<Race> Racial Trait|AUTOMATIC|<TraitKey>[|PREVAREQ:...]`
            if parts.len() < 3 || parts[0] != format!("{race_key} Racial Trait") || parts[1] != "AUTOMATIC" {
                continue;
            }
            let trait_key = parts[2].trim();
            let mut flags: Vec<String> = Vec::new();
            for clause in &parts[3..] {
                let Some(rest) = clause.trim().strip_prefix("PREVAREQ:") else { continue };
                let Some((flag, target)) = rest.rsplit_once(',') else { continue };
                if target.trim() != "0" {
                    continue;
                }
                let flag = flag.trim().to_string();
                if !flags.contains(&flag) {
                    flags.push(flag);
                }
            }
            if flags.is_empty() {
                continue;
            }
            let entry = out.entry(trait_key.to_string()).or_default();
            for flag in flags {
                if !entry.contains(&flag) {
                    entry.push(flag);
                }
            }
        }
    }
    out
}

/// Extracts the replace-flag names from a `!PREFACT:1,ABILITIES,<Flag>=True`
/// token (`decisions.md §26`'s swap protocol: the trait applies *unless*
/// the named flag is set).
///
/// Returns every flag whose value is `true` in any ASCII case — the
/// upstream corpus writes both `=True` (107 rows) and `=true` (57 rows).
/// A row can name more than one flag: Duergar's two `Spell-Like Ability`
/// traits each carry `!PREFACT:1,ABILITIES,Duergar_ReplaceSpellLikeAbilities=True,
/// Duergar_ReplaceSLA<X>=True`, meaning either flag suppresses the trait.
fn replace_flags(row: &LstRow) -> Vec<String> {
    let Some((_, value)) = row.tokens().find(|(k, _)| *k == "!PREFACT") else {
        return Vec::new();
    };
    let parts: Vec<&str> = value.split(',').collect();
    // Shape is `<count>,ABILITIES,<Flag>=<bool>[,<Flag>=<bool>...]`.
    if parts.len() < 3 || !parts[1].eq_ignore_ascii_case("ABILITIES") {
        return Vec::new();
    }
    parts[2..]
        .iter()
        .filter_map(|p| {
            let (flag, val) = p.split_once('=')?;
            val.trim().eq_ignore_ascii_case("true").then(|| flag.trim().to_string())
        })
        .collect()
}

/// `SOURCEPAGE:` -> `source_page`, with the `p.xx` placeholder mapped to
/// `None` rather than transcribed (`decisions.md §26`). A populated
/// `source_page` therefore always means a real page.
fn source_page(row: &LstRow) -> Option<String> {
    row.first("SOURCEPAGE").filter(|v| !v.trim().eq_ignore_ascii_case(PLACEHOLDER_SOURCE_PAGE)).map(str::to_string)
}

/// The owning race key, read off the trait's own `KEY:` (`Dwarf ~ Greed`
/// -> `Dwarf`), so traits resolve per race without re-parsing key strings
/// downstream. Verified against the chassis key by the caller.
fn race_key_from_trait_key(key: &str) -> Option<&str> {
    key.split_once(" ~ ").map(|(race, _)| race)
}

/// Builds one trait payload. `sets_replace_flags` is deliberately left
/// empty: a *standard* trait declares the flag that suppresses it, it
/// never sets one. ARG's alternate racial traits are what set flags, and
/// they are a separate ingest.
///
/// `gates` is [`globalvar_gates`]' reading of the race's second gate
/// source. It is used **only where the row itself declares no
/// `!PREFACT`** — a globalvar read can never overwrite a gate the trait
/// row states, so the 166 rows that already carried one are byte-identical
/// after this change. Where both speak, the caller cross-checks them and
/// fails the run on a contradiction; the check lives there rather than
/// here because it is a per-race reconciliation, not a per-row parse.
///
/// When the gate comes from the globalvar file, the grant token that
/// declared it is appended verbatim to `raw_tokens` under the key
/// `GLOBALVAR:ABILITY`. That key is not an LST token name, so nothing can
/// mistake it for something the trait row said, and the record carries its
/// own evidence rather than an unfalsifiable conclusion.
fn parse_trait(
    row: &LstRow,
    race_key: &str,
    gates: &BTreeMap<String, Vec<String>>,
) -> Result<RaceTraitCacheData, String> {
    let key = row.first("KEY").ok_or_else(|| format!("line {}: standard trait row has no KEY: token", row.line_no))?;
    let types = type_tokens(row);
    let mut flags = replace_flags(row);
    let mut raw_tokens = raw_tokens_excluding_bonus(row);
    if flags.is_empty()
        && let Some(from_globalvar) = gates.get(key)
    {
        flags = from_globalvar.clone();
        raw_tokens.push(RawToken {
            key: "GLOBALVAR:ABILITY".to_string(),
            value: format!(
                "{race_key} Racial Trait|AUTOMATIC|{key}|{}",
                from_globalvar.iter().map(|flag| format!("PREVAREQ:{flag},0")).collect::<Vec<_>>().join("|")
            ),
        });
    }
    let rendered = render_description(row).map_err(|e| format!("line {}: {e}", row.line_no))?;
    Ok(RaceTraitCacheData {
        key: key.to_string(),
        name: row.name().to_string(),
        race_key: race_key.to_string(),
        category: row.first("CATEGORY").map(str::to_string),
        is_racial_default: is_racial_default(&types, race_key),
        type_tokens: types,
        // Schema carries a single flag; where upstream names more than
        // one, the first is stored and the full `!PREFACT` token is
        // preserved verbatim in `raw_tokens` so nothing is lost on disk.
        // The tool reports every such row.
        suppressed_by_flag: flags.first().cloned(),
        sets_replace_flags: Vec::new(),
        // Player-facing prose, not the raw token: `%N` is substituted
        // from the row's own literals and any argument tail is stripped.
        description: rendered.text,
        source_page: source_page(row),
        raw_tokens,
        raw_bonus_chains: raw_bonus_chains(row),
    })
}

// ---------------------------------------------------------------------
// Output
// ---------------------------------------------------------------------

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

/// Lowercase ASCII slug, identical in behaviour to
/// `src/bin/sd27_gen_book_cache.rs::slugify` so race records file the
/// same way every other corpus record already does.
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

/// Returns the blacklisted terms present in a record's free text, if any.
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
    if let Ok(v) = std::env::var("SD27_INGESTED_AT") {
        return v;
    }
    let output = std::process::Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .expect("date -u must be available to stamp ingested_at");
    String::from_utf8(output.stdout).expect("date output is valid UTF-8").trim().to_string()
}

fn pcgen_data_root() -> PathBuf {
    if let Ok(v) = std::env::var("PCGEN_DATA_ROOT") {
        return PathBuf::from(v);
    }
    let home = std::env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
    PathBuf::from(home).join("workspace/repos/pcgen/data")
}

/// Finds the single file in `dir` whose name ends with `suffix`. LST
/// basenames do not reliably match their directory (`half_elf/` holds
/// `halfelf_races.lst`), so discovery is by suffix; more than one match
/// is an error rather than an arbitrary pick.
fn find_single(dir: &Path, suffix: &str) -> Result<PathBuf, String> {
    let mut matches: Vec<PathBuf> = fs::read_dir(dir)
        .map_err(|e| format!("cannot read {dir:?}: {e}"))?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.file_name().and_then(|n| n.to_str()).map(|n| n.ends_with(suffix)).unwrap_or(false))
        .collect();
    matches.sort();
    match matches.len() {
        1 => Ok(matches.remove(0)),
        0 => Err(format!("no *{suffix} in {dir:?}")),
        n => Err(format!("{n} candidate *{suffix} files in {dir:?}: {matches:?}")),
    }
}

struct BookTally {
    races: usize,
    traits: usize,
}

fn main() {
    let data_root = pcgen_data_root();
    let races_root = data_root.join(RACES_RELATIVE);
    let out_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/corpus");
    let stamp = ingested_at();

    // Clear only the two content-kind directories this tool owns, so a
    // race removed from scope cannot linger as a stale record.
    for book in ["core_rulebook", "beastiary"] {
        for kind in ["race", "race_trait"] {
            let dir = out_root.join(book).join(kind);
            if dir.exists() {
                fs::remove_dir_all(&dir).unwrap_or_else(|e| panic!("failed to clear {dir:?}: {e}"));
            }
        }
    }

    let mut tallies: BTreeMap<&str, BookTally> = BTreeMap::new();
    let mut errors: Vec<String> = Vec::new();
    let mut multi_flag_rows: Vec<String> = Vec::new();
    let mut placeholder_page_traits = 0usize;
    let mut real_page_traits = 0usize;
    let mut non_default_traits: Vec<String> = Vec::new();
    let mut unresolved_desc_args: Vec<String> = Vec::new();
    let mut rewritten_descriptions: Vec<String> = Vec::new();
    // The two gate sources, tallied so the reconciliation is a printed
    // measurement rather than a claim in a doc comment.
    let mut gates_agreeing = 0usize;
    let mut gates_from_globalvar: Vec<String> = Vec::new();
    let mut gate_supersets: Vec<String> = Vec::new();
    let mut ungated_traits: Vec<String> = Vec::new();

    for spec in IN_SCOPE_RACES {
        let dir = races_root.join(spec.dir);
        let chassis_path = find_single(&dir, "_races.lst").unwrap_or_else(|e| panic!("{e}"));
        let abilities_path = find_single(&dir, "_abilities_race.lst").unwrap_or_else(|e| panic!("{e}"));
        // The second gate source. `find_single` matches by suffix, so the
        // per-subrace file (`*_abilities_globalvar_subrace.lst`) does not
        // collide with it. Subrace content is out of scope for the 18.
        let globalvar_path = find_single(&dir, "_abilities_globalvar.lst").unwrap_or_else(|e| panic!("{e}"));
        let globalvar_bytes =
            fs::read(&globalvar_path).unwrap_or_else(|e| panic!("cannot read {globalvar_path:?}: {e}"));
        let globalvar_rel = format!(
            "{RACES_RELATIVE}/{}/{}",
            spec.dir,
            globalvar_path.file_name().unwrap().to_string_lossy()
        );

        let chassis_bytes = fs::read(&chassis_path).unwrap_or_else(|e| panic!("cannot read {chassis_path:?}: {e}"));
        let abilities_bytes = fs::read(&abilities_path).unwrap_or_else(|e| panic!("cannot read {abilities_path:?}: {e}"));
        let chassis_sha = sha256_hex(&chassis_bytes);
        let abilities_sha = sha256_hex(&abilities_bytes);
        let chassis_rel = format!(
            "{RACES_RELATIVE}/{}/{}",
            spec.dir,
            chassis_path.file_name().unwrap().to_string_lossy()
        );
        let abilities_rel = format!(
            "{RACES_RELATIVE}/{}/{}",
            spec.dir,
            abilities_path.file_name().unwrap().to_string_lossy()
        );

        let chassis_rows = parse_rows(&String::from_utf8_lossy(&chassis_bytes));
        if chassis_rows.len() != 1 {
            errors.push(format!("{chassis_rel}: expected exactly 1 chassis row, found {}", chassis_rows.len()));
            continue;
        }
        let chassis_row = &chassis_rows[0];
        let chassis = parse_chassis(chassis_row);
        let race_key = chassis.key.clone();

        let hits = pi_hits(&[&chassis.key, &chassis.name]);
        if !hits.is_empty() {
            errors.push(format!("PI-blacklist hit on race {race_key}: {hits:?}"));
        }

        let record = CorpusRecordV1 {
            population: Population::InScope,
            // The chassis row is fully captured (every token is in
            // `raw_tokens`), but a *race* is only complete once its
            // traits are resolved on top, so the record is honestly
            // labelled for what it is: the chassis.
            completeness: Completeness::ChassisOnly,
            ingested_at: stamp.clone(),
            data: chassis,
            source: CorpusSource::LstToken {
                path: chassis_rel.clone(),
                sha256: chassis_sha,
                line: chassis_row.line_no,
                record_key: race_key.clone(),
            },
            license: Some(License::Ogl),
            pi_field: None,
            pi_marker: None,
        };
        let race_slug = slugify(&race_key);
        write_record(&out_root.join(spec.book).join("race").join(format!("{race_slug}.json")), &record);
        tallies.entry(spec.book).or_insert(BookTally { races: 0, traits: 0 }).races += 1;

        // --- standard racial traits ---
        let gates = globalvar_gates(&String::from_utf8_lossy(&globalvar_bytes), &race_key);
        if gates.is_empty() {
            errors.push(format!("{globalvar_rel}: declares no PREVAREQ gate for any {race_key} trait"));
        }
        let trait_dir = out_root.join(spec.book).join("race_trait").join(&race_slug);
        let mut seen_slugs: BTreeMap<String, String> = BTreeMap::new();
        let mut trait_count = 0usize;

        for row in parse_rows(&String::from_utf8_lossy(&abilities_bytes)) {
            if !is_standard_racial_trait(&row) {
                continue;
            }
            if row.name().contains(".MOD") {
                errors.push(format!("{abilities_rel}:{}: .MOD row matched the standard-trait selector", row.line_no));
                continue;
            }
            let data = match parse_trait(&row, &race_key, &gates) {
                Ok(d) => d,
                Err(e) => {
                    errors.push(format!("{abilities_rel}: {e}"));
                    continue;
                }
            };

            // The two gate sources, reconciled per row. A `!PREFACT` flag
            // the globalvar file does not also name is a contradiction
            // between two statements of the same protocol, and there is no
            // honest way to pick a winner — so it fails the run rather
            // than being resolved by preference. Today there are none:
            // 166 rows agree exactly, 2 of them (Duergar's `Spell-Like
            // Abilities`, Tengu's `Languages`) have a globalvar gate that
            // names strictly more flags, and 9 (Aasimar's) have no
            // `!PREFACT` at all.
            let row_flags = replace_flags(&row);
            match gates.get(&data.key) {
                Some(gate_flags) => {
                    let missing: Vec<&String> =
                        row_flags.iter().filter(|flag| !gate_flags.contains(flag)).collect();
                    if !missing.is_empty() {
                        errors.push(format!(
                            "{abilities_rel}:{}: {}'s own !PREFACT names {missing:?}, which {globalvar_rel} \
                             does not gate on ({gate_flags:?}); the two statements of the swap protocol \
                             contradict each other",
                            row.line_no, data.key
                        ));
                    }
                    if row_flags.is_empty() {
                        gates_from_globalvar.push(format!("{} -> {gate_flags:?} ({globalvar_rel})", data.key));
                    } else {
                        gates_agreeing += 1;
                        let extra: Vec<&String> =
                            gate_flags.iter().filter(|flag| !row_flags.contains(flag)).collect();
                        if !extra.is_empty() {
                            gate_supersets.push(format!("{} -> globalvar also gates on {extra:?}", data.key));
                        }
                    }
                }
                None if !row_flags.is_empty() => {
                    errors.push(format!(
                        "{globalvar_rel}: no PREVAREQ gate for {}, whose own row declares {row_flags:?}",
                        data.key
                    ));
                }
                None => ungated_traits.push(format!("{} ({abilities_rel}:{})", data.key, row.line_no)),
            }

            // The trait's own KEY must agree with the chassis on which
            // race owns it; a mismatch would silently file a trait under
            // the wrong race.
            match race_key_from_trait_key(&data.key) {
                Some(k) if k == race_key => {}
                other => {
                    errors.push(format!(
                        "{abilities_rel}:{}: trait KEY {:?} names race {:?}, chassis says {race_key:?}",
                        row.line_no, data.key, other
                    ));
                    continue;
                }
            }
            if !data.type_tokens.iter().any(|t| t.eq_ignore_ascii_case(&format!("{race_key} Racial Trait"))) {
                errors.push(format!(
                    "{abilities_rel}:{}: {:?} leads with RacialTraits but has no {race_key:?} Racial Trait token",
                    row.line_no, data.key
                ));
                continue;
            }
            if !data.is_racial_default {
                non_default_traits.push(format!("{} ({abilities_rel}:{})", data.key, row.line_no));
            }
            if replace_flags(&row).len() > 1 {
                multi_flag_rows.push(format!("{} -> {:?}", data.key, replace_flags(&row)));
            }
            if data.source_page.is_some() {
                real_page_traits += 1;
            } else {
                placeholder_page_traits += 1;
            }

            // `DESC:` rendering: report every argument that would not
            // resolve, and refuse to ship a description that still
            // carries PCGen substitution syntax.
            if let Ok(rendered) = render_description(&row) {
                for arg in rendered.unresolved_args {
                    unresolved_desc_args
                        .push(format!("{} -> DESC arg {arg:?} is not a same-row literal (dropped, not guessed)", data.key));
                }
            }
            let raw_desc = row.first("DESC").unwrap_or_default();
            if let Some(desc) = data.description.as_deref() {
                if desc != raw_desc {
                    rewritten_descriptions.push(format!("{}\n      raw: {raw_desc}\n      out: {desc}", data.key));
                }
                if let Some(leak) = leaked_pcgen_syntax(desc) {
                    errors.push(format!(
                        "{abilities_rel}:{}: {} would ship a {leak} to the player: {desc}",
                        row.line_no, data.key
                    ));
                }
            }

            let desc = data.description.clone().unwrap_or_default();
            let hits = pi_hits(&[&data.key, &data.name, &desc]);
            if !hits.is_empty() {
                errors.push(format!("PI-blacklist hit on trait {}: {hits:?}", data.key));
            }

            let slug = slugify(&data.key);
            if let Some(prev) = seen_slugs.insert(slug.clone(), data.key.clone()) {
                errors.push(format!("slug collision {slug:?}: {prev:?} and {:?}", data.key));
                continue;
            }

            let record = CorpusRecordV1 {
                population: Population::InScope,
                // Every token on the trait row is captured, either as a
                // named field, a raw token, or a bonus chain.
                completeness: Completeness::Full,
                ingested_at: stamp.clone(),
                source: CorpusSource::LstToken {
                    path: abilities_rel.clone(),
                    sha256: abilities_sha.clone(),
                    line: row.line_no,
                    record_key: data.key.clone(),
                },
                data,
                license: Some(License::Ogl),
                pi_field: None,
                pi_marker: None,
            };
            write_record(&trait_dir.join(format!("{slug}.json")), &record);
            trait_count += 1;
        }

        if trait_count == 0 {
            errors.push(format!("{abilities_rel}: no standard racial traits found for {race_key}"));
        }
        tallies.entry(spec.book).or_insert(BookTally { races: 0, traits: 0 }).traits += trait_count;
        println!("{:<14} {race_key:<12} chassis=1 traits={trait_count}", spec.book);
    }

    println!("\n--- totals ---");
    let mut total_races = 0;
    let mut total_traits = 0;
    for (book, t) in &tallies {
        println!("{book:<14} race={} race_trait={}", t.races, t.traits);
        total_races += t.races;
        total_traits += t.traits;
    }
    println!("ALL            race={total_races} race_trait={total_traits}");
    println!("ingested_at={stamp}");
    println!("trait source_page: {real_page_traits} real, {placeholder_page_traits} placeholder (p.xx -> null)");
    println!("traits NOT flagged '<Race> Racial Default': {}", non_default_traits.len());
    for t in &non_default_traits {
        println!("  {t}");
    }
    println!("trait rows naming >1 replace flag (first stored, full token kept in raw_tokens): {}", multi_flag_rows.len());
    for t in &multi_flag_rows {
        println!("  {t}");
    }
    println!("descriptions rewritten from the raw DESC: token: {}", rewritten_descriptions.len());
    for t in &rewritten_descriptions {
        println!("  {t}");
    }
    println!("DESC args that would not resolve to a same-row literal: {}", unresolved_desc_args.len());
    for t in &unresolved_desc_args {
        println!("  {t}");
    }
    println!("\n--- replace-flag gates: the two sources reconciled ---");
    println!("rows where the trait's own !PREFACT and the globalvar PREVAREQ agree: {gates_agreeing}");
    println!("rows gated ONLY by the globalvar file (no !PREFACT on the row): {}", gates_from_globalvar.len());
    for t in &gates_from_globalvar {
        println!("  {t}");
    }
    println!("rows where the globalvar file gates on strictly more flags: {}", gate_supersets.len());
    for t in &gate_supersets {
        println!("  {t}");
    }
    println!("rows with no gate in either source: {}", ungated_traits.len());
    for t in &ungated_traits {
        println!("  {t}");
    }

    if !errors.is_empty() {
        eprintln!("\n{} ERROR(S):", errors.len());
        for e in &errors {
            eprintln!("  {e}");
        }
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds an LST line from fields, inserting deliberate runs of empty
    /// columns exactly the way PCGen's PrettyLST grid does — the parser
    /// must discard them rather than read positionally.
    fn padded_line(fields: &[&str]) -> String {
        fields.join("\t\t\t")
    }

    /// The real Dwarf chassis row, field-for-field from
    /// `core_essentials/races/dwarf/dwarf_races.lst` line 6.
    fn dwarf_chassis_line() -> String {
        padded_line(&[
            "Dwarf",
            "SORTKEY:a_base_pc",
            "STARTFEATS:1",
            "FACT:BaseSize|M",
            "MOVE:Walk,20",
            "ABILITY:Internal|AUTOMATIC|Racial Traits ~ Dwarf",
            "LEGS:2",
            "HANDS:2",
            "RACETYPE:Humanoid",
            "TYPE:Humanoid.Base.PC",
            "TEMPLATE:Dwarf",
            "SOURCEPAGE:p.xx",
            "FACT:IsPC|true",
        ])
    }

    /// The real Dwarf Greed trait row (`decisions.md §26`'s exemplar).
    fn dwarf_greed_line() -> String {
        padded_line(&[
            "Greed",
            "KEY:Dwarf ~ Greed",
            "CATEGORY:Special Ability",
            "TYPE:RacialTraits.Dwarf Racial Trait.Dwarf Racial Default.SpecialQuality",
            "!PREFACT:1,ABILITIES,Dwarf_ReplaceGreed=True",
            "DESC:Dwarves receive a +2 racial bonus on Appraise skill checks made to determine the price of nonmagical goods that contain precious metals or gemstones.",
            "BONUS:SITUATION|Appraise=to assess nonmagical metals or gemstones|2|TYPE=Racial",
            "SOURCEPAGE:p.21",
        ])
    }

    fn one_row(line: &str) -> LstRow {
        let rows = parse_rows(line);
        assert_eq!(rows.len(), 1, "expected exactly one real row");
        rows.into_iter().next().unwrap()
    }

    /// An empty second gate source, for the rows whose own `!PREFACT` is
    /// what is under test.
    fn no_gates() -> BTreeMap<String, Vec<String>> {
        BTreeMap::new()
    }

    #[test]
    fn parse_rows_discards_consecutive_empty_columns_and_comment_lines() {
        let text = format!(
            "# Wed May 12 00:08:30 2021 -- reformated by PCGen PrettyLST v6.08.00\n\
             \n\
             ###Block: Playable Races\n\
             # Race Name\t\tSource Page\n\
             {}\n",
            dwarf_chassis_line()
        );
        let rows = parse_rows(&text);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].line_no, 5, "line number must be the real 1-indexed file line");
        assert_eq!(rows[0].fields.len(), 13, "13 real fields, every padding tab discarded");
        assert_eq!(rows[0].name(), "Dwarf");
    }

    #[test]
    fn split_token_splits_on_the_first_colon_only() {
        assert_eq!(split_token("FACT:BaseSize|M"), ("FACT", "BaseSize|M"));
        assert_eq!(
            split_token("BONUS:SITUATION|Perception=to notice unusual stonework|X|PREVAREQ:Foo,0"),
            ("BONUS", "SITUATION|Perception=to notice unusual stonework|X|PREVAREQ:Foo,0")
        );
        assert_eq!(split_token("Dwarf"), ("Dwarf", ""));
    }

    #[test]
    fn dwarf_chassis_parses_every_named_field_and_preserves_all_tokens() {
        let data = parse_chassis(&one_row(&dwarf_chassis_line()));
        assert_eq!(data.key, "Dwarf");
        assert_eq!(data.name, "Dwarf");
        assert_eq!(data.base_size.as_deref(), Some("M"));
        assert_eq!(data.base_move_walk, Some(20));
        assert_eq!(data.race_type.as_deref(), Some("Humanoid"));
        assert_eq!(data.type_tokens, vec!["Humanoid", "Base", "PC"]);
        assert_eq!(data.legs, Some(2));
        assert_eq!(data.hands, Some(2));
        // Every token after the name column survives, including both
        // `FACT:` tokens and the placeholder SOURCEPAGE.
        assert_eq!(data.raw_tokens.len(), 12);
        assert!(data.raw_tokens.iter().any(|t| t.key == "SOURCEPAGE" && t.value == "p.xx"));
        assert_eq!(data.raw_tokens.iter().filter(|t| t.key == "FACT").count(), 2);
    }

    /// `RaceCacheData` has no page field at all, so the placeholder
    /// cannot be emitted as a citation — it survives only as raw source
    /// text (`decisions.md §26`).
    #[test]
    fn chassis_placeholder_page_is_never_promoted_to_a_citation() {
        let json = serde_json::to_value(parse_chassis(&one_row(&dwarf_chassis_line()))).unwrap();
        assert!(json.get("source_page").is_none());
        assert_eq!(json["raw_tokens"].as_array().unwrap().iter().filter(|t| t["key"] == "SOURCEPAGE").count(), 1);
    }

    /// Halfling's chassis row carries a `BONUS:` clause; `RaceCacheData`
    /// has no bonus-chain field, so "preserve every token" must include
    /// it rather than silently dropping it.
    #[test]
    fn chassis_raw_tokens_include_bonus_clauses() {
        let line = padded_line(&[
            "Halfling",
            "FACT:BaseSize|S",
            "MOVE:Walk,20",
            "BONUS:SAVE|ALL|Halfling_HalflingLuck_SaveBonus|TYPE=Racial",
            "LEGS:2",
        ]);
        let data = parse_chassis(&one_row(&line));
        assert_eq!(data.base_size.as_deref(), Some("S"));
        assert!(data.raw_tokens.iter().any(|t| t.key == "BONUS" && t.value.starts_with("SAVE|ALL|")));
    }

    /// Goblin/Hobgoblin genuinely carry `MOVE:Walk,0` upstream. Recording
    /// the real 0 (not a "corrected" 30) is the point.
    #[test]
    fn chassis_move_walk_zero_is_recorded_faithfully() {
        let line = padded_line(&["Goblin", "FACT:BaseSize|S", "MOVE:Walk,0"]);
        assert_eq!(parse_chassis(&one_row(&line)).base_move_walk, Some(0));
    }

    #[test]
    fn base_move_walk_reads_pairwise_not_positionally() {
        let line = padded_line(&["Merfolk", "MOVE:Swim,50,Walk,5"]);
        assert_eq!(parse_chassis(&one_row(&line)).base_move_walk, Some(5));
    }

    #[test]
    fn dwarf_greed_trait_parses_to_the_documented_exemplar() {
        let row = one_row(&dwarf_greed_line());
        assert!(is_standard_racial_trait(&row));
        let data = parse_trait(&row, "Dwarf", &no_gates()).expect("Greed must parse");
        assert_eq!(data.key, "Dwarf ~ Greed");
        assert_eq!(data.name, "Greed");
        assert_eq!(data.race_key, "Dwarf");
        assert_eq!(data.category.as_deref(), Some("Special Ability"));
        assert!(data.is_racial_default);
        assert_eq!(data.suppressed_by_flag.as_deref(), Some("Dwarf_ReplaceGreed"));
        assert!(data.sets_replace_flags.is_empty(), "a standard trait sets no flags");
        assert_eq!(data.source_page.as_deref(), Some("p.21"));
        assert!(data.description.unwrap().starts_with("Dwarves receive a +2 racial bonus on Appraise"));
        assert_eq!(
            data.raw_bonus_chains,
            vec![RawBonusChain {
                qualifiers: vec![
                    "SITUATION".into(),
                    "Appraise=to assess nonmagical metals or gemstones".into(),
                    "2".into(),
                    "TYPE=Racial".into(),
                ],
            }]
        );
        // BONUS lives in `raw_bonus_chains`, never duplicated into
        // `raw_tokens` (matches every pre-existing on-disk record).
        assert!(data.raw_tokens.iter().all(|t| t.key != "BONUS"));
        assert!(data.raw_tokens.iter().any(|t| t.key == "!PREFACT"));
    }

    #[test]
    fn replace_flag_extraction_is_case_insensitive_on_the_boolean() {
        let upper = one_row(&padded_line(&["X", "TYPE:RacialTraits.Elf Racial Trait", "!PREFACT:1,ABILITIES,Elf_ReplaceVision=True"]));
        let lower = one_row(&padded_line(&["X", "TYPE:RacialTraits.Elf Racial Trait", "!PREFACT:1,ABILITIES,Elf_ReplaceVision=true"]));
        assert_eq!(replace_flags(&upper), vec!["Elf_ReplaceVision"]);
        assert_eq!(replace_flags(&lower), vec!["Elf_ReplaceVision"]);
    }

    /// Duergar's two `Spell-Like Ability` rows name two flags each. The
    /// schema stores one; the raw token keeps both.
    #[test]
    fn multi_flag_prefact_stores_the_first_flag_and_keeps_the_whole_token() {
        let row = one_row(&padded_line(&[
            "Spell-Like Ability",
            "KEY:Duergar ~ Spell-Like Ability ~ Enlarge Person",
            "TYPE:RacialTraits.Duergar Racial Trait.SpecialQuality",
            "!PREFACT:1,ABILITIES,Duergar_ReplaceSpellLikeAbilities=True,Duergar_ReplaceSLAEnlargePerson=True",
            "SOURCEPAGE:p.xx",
        ]));
        assert_eq!(replace_flags(&row), vec!["Duergar_ReplaceSpellLikeAbilities", "Duergar_ReplaceSLAEnlargePerson"]);
        let data = parse_trait(&row, "Duergar", &no_gates()).unwrap();
        assert_eq!(data.suppressed_by_flag.as_deref(), Some("Duergar_ReplaceSpellLikeAbilities"));
        assert!(
            data.raw_tokens
                .iter()
                .any(|t| t.key == "!PREFACT" && t.value.contains("Duergar_ReplaceSLAEnlargePerson=True"))
        );
        // ...and this row is genuinely not part of the default roster.
        assert!(!data.is_racial_default);
    }

    /// No gate in either source is still `None`, never a fabricated flag.
    #[test]
    fn trait_gated_by_neither_source_has_no_suppression_flag() {
        let row = one_row(&padded_line(&[
            "Skilled",
            "KEY:Aasimar ~ Skilled",
            "TYPE:RacialTraits.Aasimar Racial Trait.Aasimar Racial Default.SpecialQuality",
            "SOURCEPAGE:p.7",
        ]));
        let data = parse_trait(&row, "Aasimar", &no_gates()).unwrap();
        assert_eq!(data.suppressed_by_flag, None);
        assert!(data.is_racial_default);
        assert_eq!(data.source_page.as_deref(), Some("p.7"));
        assert!(
            !data.raw_tokens.iter().any(|t| t.key == "GLOBALVAR:ABILITY"),
            "no evidence token where there was no second source to read"
        );
    }

    /// Aasimar's 9 standard traits carry no `!PREFACT` at all. Their gate
    /// is stated only in `aasimar_abilities_globalvar.lst`, and reading it
    /// is what stops ARG's 9 Aasimar alternates from being an affordance
    /// that can never succeed.
    #[test]
    fn a_trait_without_prefact_takes_its_gate_from_the_globalvar_file() {
        let row = one_row(&padded_line(&[
            "Skilled",
            "KEY:Aasimar ~ Skilled",
            "TYPE:RacialTraits.Aasimar Racial Trait.Aasimar Racial Default.SpecialQuality",
            "SOURCEPAGE:p.7",
        ]));
        let gates = globalvar_gates(
            "CATEGORY=Special Ability|Aasimar ~ Default.MOD\tABILITY:Aasimar Racial Trait|AUTOMATIC|\
             Aasimar ~ Skilled|PREVAREQ:Aasimar_ReplaceSkilled,0",
            "Aasimar",
        );
        let data = parse_trait(&row, "Aasimar", &gates).unwrap();
        assert_eq!(data.suppressed_by_flag.as_deref(), Some("Aasimar_ReplaceSkilled"));
        // The record carries the evidence, under a key no LST row can use.
        assert!(data.raw_tokens.iter().any(|t| {
            t.key == "GLOBALVAR:ABILITY"
                && t.value == "Aasimar Racial Trait|AUTOMATIC|Aasimar ~ Skilled|PREVAREQ:Aasimar_ReplaceSkilled,0"
        }));
        assert!(!data.raw_tokens.iter().any(|t| t.key == "!PREFACT"), "the row itself said nothing");
    }

    /// A row's own `!PREFACT` always wins: the globalvar read is a
    /// fallback, never an override, so the 166 rows that already carried a
    /// gate are untouched by this source.
    #[test]
    fn the_globalvar_gate_never_overrides_a_gate_the_row_itself_declares() {
        let row = one_row(&padded_line(&[
            "Greed",
            "KEY:Dwarf ~ Greed",
            "TYPE:RacialTraits.Dwarf Racial Trait.Dwarf Racial Default",
            "!PREFACT:1,ABILITIES,Dwarf_ReplaceGreed=True",
        ]));
        let mut gates = BTreeMap::new();
        gates.insert("Dwarf ~ Greed".to_string(), vec!["Dwarf_SomethingElse".to_string()]);
        let data = parse_trait(&row, "Dwarf", &gates).unwrap();
        assert_eq!(data.suppressed_by_flag.as_deref(), Some("Dwarf_ReplaceGreed"));
        assert!(!data.raw_tokens.iter().any(|t| t.key == "GLOBALVAR:ABILITY"));
    }

    /// The globalvar reader takes `PREVAREQ:<Flag>,0` and only that.
    ///
    /// `,1` is the opposite statement — a *positive* requirement, which
    /// Duergar's two mutually-exclusive spell-like-ability rows use to
    /// keep each other out. Reading it as a suppressor would invert the
    /// rule, so this drives the real parser with Duergar's real row.
    #[test]
    fn the_globalvar_reader_takes_negative_gates_and_never_positive_requirements() {
        let gates = globalvar_gates(
            "CATEGORY=Internal|Racial Traits ~ Duergar.MOD\tDEFINE:Duergar_ReplaceSLAEnlargePerson|0\t\
             ABILITY:Duergar Racial Trait|AUTOMATIC|Duergar ~ Spell-Like Ability ~ Enlarge Person|\
             PREVAREQ:Duergar_ReplaceSpellLikeAbilities,0|PREVAREQ:Duergar_ReplaceSLAEnlargePerson,0|\
             PREVAREQ:Duergar_ReplaceSLAInvisibility,1",
            "Duergar",
        );
        assert_eq!(
            gates.get("Duergar ~ Spell-Like Ability ~ Enlarge Person"),
            Some(&vec![
                "Duergar_ReplaceSpellLikeAbilities".to_string(),
                "Duergar_ReplaceSLAEnlargePerson".to_string(),
            ]),
            "the `,1` clause is a requirement, not a suppressor, and must not be read as one"
        );

        // A grant for another race's trait, and a non-AUTOMATIC grant, are
        // both ignored rather than mis-filed.
        let other = globalvar_gates(
            "CATEGORY=Internal|X.MOD\tABILITY:Dwarf Racial Trait|AUTOMATIC|Dwarf ~ Greed|\
             PREVAREQ:Dwarf_ReplaceGreed,0\tABILITY:Duergar Racial Trait|VIRTUAL|Duergar ~ Size|\
             PREVAREQ:Duergar_ReplaceSize,0",
            "Duergar",
        );
        assert!(other.is_empty(), "{other:?}");
    }

    /// Drow's `Poison Use` row spells the marker `Drow Racial default`.
    /// It is the same marker in the same position; a case-sensitive read
    /// would drop a genuine default trait.
    #[test]
    fn racial_default_marker_match_is_case_insensitive() {
        let row = one_row(&padded_line(&[
            "Poison Use",
            "KEY:Drow ~ Poison Use",
            "TYPE:RacialTraits.Drow Racial Trait.Drow Racial default",
            "SOURCEPAGE:p.xx",
        ]));
        assert!(parse_trait(&row, "Drow", &no_gates()).unwrap().is_racial_default);
    }

    #[test]
    fn trait_placeholder_source_page_becomes_none() {
        let row = one_row(&padded_line(&[
            "Greed",
            "KEY:Dwarf ~ Greed",
            "TYPE:RacialTraits.Dwarf Racial Trait.Dwarf Racial Default",
            "SOURCEPAGE:p.xx",
        ]));
        let data = parse_trait(&row, "Dwarf", &no_gates()).unwrap();
        assert_eq!(data.source_page, None, "p.xx is a placeholder, not a citation");
        assert!(
            data.raw_tokens.iter().any(|t| t.key == "SOURCEPAGE" && t.value == "p.xx"),
            "the raw token is still preserved verbatim"
        );
    }

    /// The standard-trait selector must reject every other row shape the
    /// same files contain.
    #[test]
    fn standard_trait_selector_rejects_non_trait_rows() {
        let rejected = [
            padded_line(&["Racial Traits ~ Dwarf", "CATEGORY:Internal", "DEFINE:Dwarf_RacialCastingStat|0"]),
            padded_line(&["CATEGORY=Special Ability|Remove Excess Points from Pool.MOD", "TYPE:Dwarf Racial Trait"]),
            padded_line(&[
                "Humanoid (Dwarf)",
                "KEY:Favored Enemy ~ Humanoid (Dwarf)",
                "TYPE:RangerClassFeatures.FavoredEnemy.SpecialAttack.Extraordinary.AttackOption",
            ]),
            padded_line(&["Dwarf", "KEY:Adopted Race ~ Dwarf", "TYPE:AdoptiveRace"]),
            padded_line(&["Scion of Humanity", "KEY:Aasimar ~ Scion of Humanity", "TYPE:Aasimar Subrace"]),
        ];
        for line in &rejected {
            assert!(!is_standard_racial_trait(&one_row(line)), "must reject: {line}");
        }
        assert!(is_standard_racial_trait(&one_row(&dwarf_greed_line())));
    }

    #[test]
    fn race_key_is_read_off_the_trait_key_including_hyphenated_races() {
        assert_eq!(race_key_from_trait_key("Half-Elf ~ Ability Scores"), Some("Half-Elf"));
        assert_eq!(race_key_from_trait_key("Duergar ~ Spell-Like Ability ~ Invisibility"), Some("Duergar"));
        assert_eq!(race_key_from_trait_key("Dwarf"), None);
    }

    /// Every record this tool actually wrote must load back through the
    /// *real* committed schema types, not just through the writer that
    /// produced them. This reads the committed corpus off disk, so it
    /// fails if a record is hand-edited into a shape the engine cannot
    /// consume — the property that matters for the desktop app's live
    /// corpus loader.
    #[test]
    fn every_committed_race_record_on_disk_deserializes_through_the_shape_b_v1_schema() {
        use codex::rules_core::shape_b_v1::validate_license;

        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/corpus");
        let mut races = 0usize;
        let mut traits = 0usize;

        for book in ["core_rulebook", "beastiary"] {
            let race_dir = root.join(book).join("race");
            let mut race_files: Vec<PathBuf> =
                fs::read_dir(&race_dir).expect("race dir must exist").filter_map(Result::ok).map(|e| e.path()).collect();
            race_files.sort();
            for path in race_files {
                let text = fs::read_to_string(&path).unwrap();
                let record: CorpusRecordV1<RaceCacheData> =
                    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{path:?} is not a valid race record: {e}"));
                validate_license(&record).unwrap_or_else(|e| panic!("{path:?}: {e}"));
                assert!(!record.data.key.is_empty());
                assert!(
                    !serde_json::to_string(&record.data).unwrap().contains("\"source_page\""),
                    "{path:?}: the chassis payload must carry no page field at all"
                );
                races += 1;
            }

            let trait_root = root.join(book).join("race_trait");
            let mut race_dirs: Vec<PathBuf> = fs::read_dir(&trait_root)
                .expect("race_trait dir must exist")
                .filter_map(Result::ok)
                .map(|e| e.path())
                .collect();
            race_dirs.sort();
            for race_dir in race_dirs {
                let mut files: Vec<PathBuf> =
                    fs::read_dir(&race_dir).unwrap().filter_map(Result::ok).map(|e| e.path()).collect();
                files.sort();
                for path in files {
                    let text = fs::read_to_string(&path).unwrap();
                    let record: CorpusRecordV1<RaceTraitCacheData> = serde_json::from_str(&text)
                        .unwrap_or_else(|e| panic!("{path:?} is not a valid race-trait record: {e}"));
                    validate_license(&record).unwrap_or_else(|e| panic!("{path:?}: {e}"));
                    assert!(
                        record.data.sets_replace_flags.is_empty(),
                        "{path:?}: a standard trait sets no replace flags"
                    );
                    assert_ne!(
                        record.data.source_page.as_deref(),
                        Some(PLACEHOLDER_SOURCE_PAGE),
                        "{path:?}: the p.xx placeholder must never be stored as a citation"
                    );
                    assert!(record.data.key.starts_with(&record.data.race_key));
                    traits += 1;
                }
            }
        }

        // Pinned counts, derived from the real corpus (`decisions.md
        // §25.3`: Core Rulebook's 7 races + Bestiary 1's 11).
        assert_eq!(races, 18, "18 in-scope race chassis records");
        assert_eq!(traits, 175, "175 standard racial trait records");
    }

    // -----------------------------------------------------------------
    // DESC rendering
    // -----------------------------------------------------------------

    /// The real Dwarf Defensive Training row, field-for-field from
    /// `core_essentials/races/dwarf/dwarf_abilities_race.lst` line 22
    /// (`ASPECT:` tokens included — they carry the same `%1`, and are the
    /// alternative phrasing the brief asked to be weighed).
    fn dwarf_defensive_training_line() -> String {
        padded_line(&[
            "Defensive Training",
            "KEY:Dwarf ~ Defensive Training",
            "CATEGORY:Special Ability",
            "TYPE:RacialTraits.Dwarf Racial Trait.Dwarf Racial Default.SpecialQuality.Defensive",
            "!PREFACT:1,ABILITIES,Dwarf_ReplaceDefensiveTraining=True",
            "DEFINE:RacialDefensiveTrainingBonus|0",
            "DESC:Dwarves get a +%1 dodge bonus to AC against monsters of the giant subtype.|RacialDefensiveTrainingBonus",
            "BONUS:VAR|RacialDefensiveTrainingBonus|4",
            "SOURCEPAGE:p.21",
            "ASPECT:CombatBonus|+%1 dodge bonus to AC against monsters of the giant subtype.|RacialDefensiveTrainingBonus",
        ])
    }

    /// The real Half-Orc Orc Ferocity row from
    /// `core_essentials/races/half_orc/halforc_abilities_race.lst` line 24:
    /// four `DESC:` tokens, three of them gated by a `PREVAR*` clause on a
    /// variable this same row sets to 1.
    fn orc_ferocity_line() -> String {
        padded_line(&[
            "Orc Ferocity",
            "KEY:Half-Orc ~ Orc Ferocity",
            "CATEGORY:Special Ability",
            "TYPE:RacialTraits.Half-Orc Racial Trait.Half-Orc Racial Default.SpecialQuality.Defensive",
            "!PREFACT:1,ABILITIES,HalfOrc_ReplaceOrcFerocity=true",
            "DEFINE:Orc_OrcFerocity_Times|0",
            "DESC:Once|PREVARLTEQ:Orc_OrcFerocity_Times,1",
            "DESC:Twice|PREVAREQ:Orc_OrcFerocity_Times,2",
            "DESC:%1 times|Orc_OrcFerocity_Times|Orc_OrcFerocity_Times|PREVARGTEQ:Orc_OrcFerocity_Times,3",
            "DESC:per day, when a half-orc is brought below 0 hit points but not killed, he can fight on for one more round as if disabled. At the end of his next turn, unless brought to above 0 hit points, he immediately falls unconscious and begins dying.",
            "BONUS:VAR|Orc_OrcFerocity_Times|1",
            "SOURCEPAGE:p.25",
        ])
    }

    #[test]
    fn same_row_vars_resolve_define_base_plus_bonus_var_literal() {
        let vars = same_row_vars(&one_row(&dwarf_defensive_training_line()));
        assert_eq!(vars.get("RacialDefensiveTrainingBonus"), Some(&Some(4)));
    }

    /// A `BONUS:VAR` whose value is another variable is a formula, and
    /// `decisions.md §24` forbids interpreting one. The honest result is
    /// "unresolvable", never a guessed number.
    #[test]
    fn same_row_vars_refuse_a_non_literal_bonus_var_formula() {
        let row = one_row(&padded_line(&[
            "Humanoid (Dwarf)",
            "DEFINE:FavoredHumanoidDwarf|0",
            "BONUS:VAR|FavoredHumanoidDwarf|FavoredBaseBonus",
        ]));
        assert_eq!(same_row_vars(&row).get("FavoredHumanoidDwarf"), Some(&None));
    }

    /// A variable the row only *reads* (no `DEFINE:` of its own) is
    /// defined somewhere else entirely, so this row cannot resolve it.
    #[test]
    fn same_row_vars_refuse_a_variable_this_row_never_defines() {
        let row = one_row(&padded_line(&["X", "BONUS:VAR|SomeGlobal|2"]));
        assert_eq!(same_row_vars(&row).get("SomeGlobal"), Some(&None));
    }

    /// A conditional `BONUS:VAR` does not apply unconditionally, so its
    /// contribution cannot be folded into a flat literal.
    #[test]
    fn same_row_vars_refuse_a_conditional_bonus_var() {
        let row =
            one_row(&padded_line(&["X", "DEFINE:Foo|0", "BONUS:VAR|Foo|2|PRELEVEL:MIN=5"]));
        assert_eq!(same_row_vars(&row).get("Foo"), Some(&None));
    }

    #[test]
    fn prevar_gate_evaluates_the_closed_comparison_family_against_same_row_literals() {
        let vars: BTreeMap<String, Option<i64>> = [("T".to_string(), Some(1i64))].into_iter().collect();
        for (token, expected) in [
            ("PREVAREQ:T,1", true),
            ("PREVAREQ:T,2", false),
            ("PREVARNEQ:T,2", true),
            ("PREVARLT:T,2", true),
            ("PREVARLT:T,1", false),
            ("PREVARLTEQ:T,1", true),
            ("PREVARGT:T,0", true),
            ("PREVARGTEQ:T,3", false),
            ("!PREVAREQ:T,1", false),
        ] {
            assert_eq!(eval_prevar_gate(token, &vars), Ok(expected), "{token}");
        }
        // Both pairs must hold.
        assert_eq!(eval_prevar_gate("PREVARGTEQ:T,1,T,2", &vars), Ok(false));
        // Undecidable rather than guessed.
        assert!(eval_prevar_gate("PREVAREQ:Unknown,1", &vars).is_err());
        assert!(eval_prevar_gate("PREVARSOMETHING:T,1", &vars).is_err());
    }

    /// The defect this change exists to fix, pinned on the real row:
    /// the player must see the resolved `+4`, never `%1` and never the
    /// `|RacialDefensiveTrainingBonus` argument tail.
    #[test]
    fn defensive_training_renders_the_resolved_bonus_not_the_raw_desc() {
        let row = one_row(&dwarf_defensive_training_line());
        let rendered = render_description(&row).expect("Defensive Training must render");
        assert!(rendered.unresolved_args.is_empty());
        assert_eq!(
            rendered.text.as_deref(),
            Some("Dwarves get a +4 dodge bonus to AC against monsters of the giant subtype.")
        );
        let data = parse_trait(&row, "Dwarf", &no_gates()).expect("row must parse");
        assert_eq!(
            data.description.as_deref(),
            Some("Dwarves get a +4 dodge bonus to AC against monsters of the giant subtype.")
        );
    }

    /// Four `DESC:` tokens, three `PREVAR*`-gated on a variable this row
    /// sets to 1: only `Once` survives, and it joins the ungated tail into
    /// one sentence. `%1 times` is dropped whole because its gate
    /// (`Times >= 3`) is false — not because the argument was stripped.
    #[test]
    fn orc_ferocity_renders_the_gated_segment_that_actually_applies() {
        let row = one_row(&orc_ferocity_line());
        let rendered = render_description(&row).expect("Orc Ferocity must render");
        assert!(rendered.unresolved_args.is_empty());
        assert_eq!(
            rendered.text.as_deref(),
            Some(
                "Once per day, when a half-orc is brought below 0 hit points but not killed, \
                 he can fight on for one more round as if disabled. At the end of his next turn, \
                 unless brought to above 0 hit points, he immediately falls unconscious and begins dying."
            )
        );
    }

    /// When the argument is a formula rather than a literal, no value is
    /// invented: the placeholder and its argument tail are both removed,
    /// the sign that introduced it goes with them, and the row is reported.
    #[test]
    fn unresolvable_desc_argument_is_dropped_never_guessed() {
        let row = one_row(&padded_line(&[
            "Made Up",
            "DEFINE:Foo|0",
            "BONUS:VAR|Foo|SomeOtherVariable",
            "DESC:You gain a +%1 bonus on Bluff checks.|Foo",
        ]));
        let rendered = render_description(&row).expect("row must render");
        assert_eq!(rendered.unresolved_args, vec!["Foo".to_string()]);
        let text = rendered.text.expect("prose survives");
        assert_eq!(text, "You gain a bonus on Bluff checks.");
        assert!(!text.contains('%'));
        assert!(!text.contains('|'));
    }

    /// A `%N` with no matching argument at all is still never shown.
    #[test]
    fn placeholder_with_no_argument_is_dropped() {
        let row = one_row(&padded_line(&["X", "DESC:A +%1 bonus applies."]));
        let rendered = render_description(&row).unwrap();
        assert_eq!(rendered.text.as_deref(), Some("A bonus applies."));
    }

    /// A description with nothing to substitute must come through
    /// byte-identical — the cleanup may not reflow untouched prose.
    #[test]
    fn plain_description_passes_through_unchanged() {
        let row = one_row(&dwarf_greed_line());
        assert_eq!(
            render_description(&row).unwrap().text.as_deref(),
            Some(
                "Dwarves receive a +2 racial bonus on Appraise skill checks made to determine \
                 the price of nonmagical goods that contain precious metals or gemstones."
            )
        );
    }

    /// An undecidable gate is a hard error, not a coin flip: including or
    /// dropping a gated segment changes what the rules text *says*
    /// ("Once" vs "Twice per day"), so the run fails loudly instead.
    #[test]
    fn undecidable_desc_gate_is_an_error_not_a_guess() {
        let row = one_row(&padded_line(&["X", "DESC:Once|PREVARLTEQ:SomeGlobalVar,1"]));
        assert!(render_description(&row).is_err());
    }

    /// The property the player actually experiences: nothing PCGen-shaped
    /// survives into a served description. Scoped to the two books this
    /// binary writes.
    #[test]
    fn no_committed_trait_description_leaks_pcgen_substitution_syntax() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/corpus");
        let mut checked = 0usize;
        let mut with_description = 0usize;

        for book in ["core_rulebook", "beastiary"] {
            let trait_root = root.join(book).join("race_trait");
            let mut race_dirs: Vec<PathBuf> = fs::read_dir(&trait_root)
                .expect("race_trait dir must exist")
                .filter_map(Result::ok)
                .map(|e| e.path())
                .collect();
            race_dirs.sort();
            for race_dir in race_dirs {
                let mut files: Vec<PathBuf> =
                    fs::read_dir(&race_dir).unwrap().filter_map(Result::ok).map(|e| e.path()).collect();
                files.sort();
                for path in files {
                    let record: CorpusRecordV1<RaceTraitCacheData> =
                        serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
                    checked += 1;
                    let Some(desc) = record.data.description.as_deref() else { continue };
                    with_description += 1;
                    let chars: Vec<char> = desc.chars().collect();
                    for (i, c) in chars.iter().enumerate() {
                        assert!(
                            !(*c == '%' && chars.get(i + 1).is_some_and(char::is_ascii_digit)),
                            "{path:?}: served description carries an unsubstituted '%N' argument reference: {desc}"
                        );
                    }
                    assert!(
                        !desc.contains('|'),
                        "{path:?}: served description carries a raw PCGen argument tail: {desc}"
                    );
                    assert_eq!(desc.trim(), desc, "{path:?}: served description has stray edge whitespace");
                }
            }
        }

        assert_eq!(checked, 175, "175 standard racial trait records");
        assert_eq!(with_description, 175, "every one of them carries a DESC:");
    }

    /// The two rows named in the defect report, pinned verbatim so a
    /// regression names itself.
    #[test]
    fn the_two_reported_rows_render_exactly_as_specified() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/corpus/core_rulebook/race_trait");
        let read = |rel: &str| -> String {
            let record: CorpusRecordV1<RaceTraitCacheData> =
                serde_json::from_str(&fs::read_to_string(root.join(rel)).unwrap()).unwrap();
            record.data.description.expect("record must carry a description")
        };
        assert_eq!(
            read("dwarf/dwarf_defensive_training.json"),
            "Dwarves get a +4 dodge bonus to AC against monsters of the giant subtype."
        );
        assert_eq!(
            read("half_orc/half_orc_orc_ferocity.json"),
            "Once per day, when a half-orc is brought below 0 hit points but not killed, \
             he can fight on for one more round as if disabled. At the end of his next turn, \
             unless brought to above 0 hit points, he immediately falls unconscious and begins dying."
        );
    }

    #[test]
    fn slugify_disambiguates_the_two_duergar_spell_like_ability_traits() {
        let a = slugify("Duergar ~ Spell-Like Ability ~ Enlarge Person");
        let b = slugify("Duergar ~ Spell-Like Ability ~ Invisibility");
        assert_eq!(a, "duergar_spell_like_ability_enlarge_person");
        assert_eq!(b, "duergar_spell_like_ability_invisibility");
        assert_ne!(a, b, "same display name, distinct keys -> distinct files");
        assert_eq!(slugify("Half-Elf"), "half_elf");
    }
}
