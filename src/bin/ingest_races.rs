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
fn parse_trait(row: &LstRow, race_key: &str) -> Result<RaceTraitCacheData, String> {
    let key = row.first("KEY").ok_or_else(|| format!("line {}: standard trait row has no KEY: token", row.line_no))?;
    let types = type_tokens(row);
    let flags = replace_flags(row);
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
        description: row.first("DESC").map(str::to_string),
        source_page: source_page(row),
        raw_tokens: raw_tokens_excluding_bonus(row),
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

    for spec in IN_SCOPE_RACES {
        let dir = races_root.join(spec.dir);
        let chassis_path = find_single(&dir, "_races.lst").unwrap_or_else(|e| panic!("{e}"));
        let abilities_path = find_single(&dir, "_abilities_race.lst").unwrap_or_else(|e| panic!("{e}"));

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
            let data = match parse_trait(&row, &race_key) {
                Ok(d) => d,
                Err(e) => {
                    errors.push(format!("{abilities_rel}: {e}"));
                    continue;
                }
            };

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
        let data = parse_trait(&row, "Dwarf").expect("Greed must parse");
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
        let data = parse_trait(&row, "Duergar").unwrap();
        assert_eq!(data.suppressed_by_flag.as_deref(), Some("Duergar_ReplaceSpellLikeAbilities"));
        assert!(
            data.raw_tokens
                .iter()
                .any(|t| t.key == "!PREFACT" && t.value.contains("Duergar_ReplaceSLAEnlargePerson=True"))
        );
        // ...and this row is genuinely not part of the default roster.
        assert!(!data.is_racial_default);
    }

    /// Aasimar's 9 standard traits carry no `!PREFACT` at all — the
    /// honest result is `None`, not a fabricated flag.
    #[test]
    fn trait_without_prefact_has_no_suppression_flag() {
        let row = one_row(&padded_line(&[
            "Skilled",
            "KEY:Aasimar ~ Skilled",
            "TYPE:RacialTraits.Aasimar Racial Trait.Aasimar Racial Default.SpecialQuality",
            "SOURCEPAGE:p.7",
        ]));
        let data = parse_trait(&row, "Aasimar").unwrap();
        assert_eq!(data.suppressed_by_flag, None);
        assert!(data.is_racial_default);
        assert_eq!(data.source_page.as_deref(), Some("p.7"));
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
        assert!(parse_trait(&row, "Drow").unwrap().is_racial_default);
    }

    #[test]
    fn trait_placeholder_source_page_becomes_none() {
        let row = one_row(&padded_line(&[
            "Greed",
            "KEY:Dwarf ~ Greed",
            "TYPE:RacialTraits.Dwarf Racial Trait.Dwarf Racial Default",
            "SOURCEPAGE:p.xx",
        ]));
        let data = parse_trait(&row, "Dwarf").unwrap();
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
