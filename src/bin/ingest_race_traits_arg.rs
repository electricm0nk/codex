//! SD-27 -- ingests the Advanced Race Guide's **alternate racial traits**
//! from `advanced_race_guide/arg_abilities_race.lst` into Shape B v1
//! `RaceTraitCacheData` records under
//! `data/corpus/advanced_race_guide/race_trait/<race>/<trait>.json`.
//!
//! **Why this is ARG's real contribution.** `decisions.md §25.2` records that
//! ARG declares *zero* races of its own -- all 37 races in `arg_races.lst` are
//! `.MOD` reprints whose chassis lives in PCGen's shared `core_essentials/`
//! storage. ARG's genuine own content is the alternate-racial-trait corpus in
//! this one file (`decisions.md §25.4`), and that is what this binary ingests.
//!
//! **Scope filter (`decisions.md §25.3`).** Only the **18** races whose true
//! source book is already ingested are emitted: Core Rulebook's 7 (Dwarf, Elf,
//! Gnome, Half-Elf, Half-Orc, Halfling, Human) and Bestiary 1's 11 (Aasimar,
//! Drow, Duergar, Goblin, Hobgoblin, Kobold, Merfolk, Orc, Svirfneblin, Tengu,
//! Tiefling). Traits belonging to B2/B3/B4/ISWG races are counted and reported,
//! never written -- emitting them would manufacture content for a book nobody
//! has audited. `core_essentials` is never used as a book attribution; these
//! records are ARG's own, so they file under `advanced_race_guide`.
//!
//! **The replace-flag protocol (`decisions.md §26`), read off the corpus, not
//! guessed.** A standard racial trait in `core_essentials/races/<race>/
//! <race>_abilities_race.lst` is gated on a negated fact check naming its own
//! flag, e.g. `!PREFACT:1,ABILITIES,Dwarf_ReplaceGreed=True`. An ARG alternate
//! *sets* that flag with a trailing token of the exact form
//!
//! ```text
//! FACT:Dwarf_ReplaceGreed|True
//! ```
//!
//! That `FACT:<Race>_Replace<Trait>|True` token -- **not** the
//! `!PREFACT:...=true` occurrences, which only *read* flags -- is what
//! [`RaceTraitCacheData::sets_replace_flags`] captures.
//!
//! Alternates additionally carry a `PREMULT:1,[PREABILITY:...this ability...],
//! [!PREFACT:1,ABILITIES,<flag>=true]` clause. That is a **self-exclusion
//! guard** ("you may not take a second trait replacing something you already
//! replaced"), not a suppression by some other trait: verified over the corpus,
//! every flag named inside such a `PREMULT` is a flag the same row itself sets.
//! It is therefore preserved verbatim in `raw_tokens` rather than being
//! laundered into `suppressed_by_flag`, which is reserved for a *standalone*
//! `!PREFACT` gate (the shape standard traits use).
//!
//! Run with `cargo run --bin ingest_race_traits_arg`. `PCGEN_CORPUS_ROOT` may
//! point at a local PCGen `data/` checkout; it defaults to
//! `$HOME/workspace/repos/pcgen/data`.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use codex::rules_core::shape_b_v1::{
    Completeness, CorpusRecordV1, CorpusSource, License, Population, RaceTraitCacheData, RawBonusChain, RawToken,
};

/// The one source file this binary ingests, relative to the PCGen `data/` root.
/// The same string is written into every record's `source.path`.
const LST_RELATIVE: &str = "pathfinder/paizo/roleplaying_game/advanced_race_guide/arg_abilities_race.lst";

/// The 18 in-scope races (`decisions.md §25.3`), spelled exactly as the corpus
/// spells them in its `TYPE:<Race> Racial Trait` component.
const IN_SCOPE_RACES: [&str; 18] = [
    // Core Rulebook (7)
    "Dwarf",
    "Elf",
    "Gnome",
    "Half-Elf",
    "Half-Orc",
    "Halfling",
    "Human",
    // Bestiary 1 (11)
    "Aasimar",
    "Drow",
    "Duergar",
    "Goblin",
    "Hobgoblin",
    "Kobold",
    "Merfolk",
    "Orc",
    "Svirfneblin",
    "Tengu",
    "Tiefling",
];

const RACIAL_TRAIT_TYPE_SUFFIX: &str = " Racial Trait";
const RACIAL_DEFAULT_TYPE_SUFFIX: &str = " Racial Default";

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

fn ingested_at_now() -> String {
    let output = std::process::Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .expect("`date -u` must be available to stamp ingested_at");
    String::from_utf8(output.stdout).expect("date output is valid UTF-8").trim().to_string()
}

/// Same slug rule `sd27_gen_book_cache.rs` already uses for every other
/// content kind, so `race_trait/` paths read like their `feat/`/`spell/`
/// siblings.
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

fn pcgen_data_root() -> PathBuf {
    if let Ok(v) = std::env::var("PCGEN_CORPUS_ROOT") {
        return PathBuf::from(v);
    }
    let home = std::env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
    PathBuf::from(home).join("workspace/repos/pcgen/data")
}

/// One tab-delimited `KEY:VALUE` field off a corpus row. The row's single
/// bare (colon-less) leading field is the ability's display name and is not
/// represented here -- it becomes `RaceTraitCacheData::name`.
struct Field {
    key: String,
    value: String,
}

/// Splits a raw LST row into its non-empty tab-delimited fields. PrettyLST
/// pads columns with runs of tabs, so empty fields are structural padding and
/// carry no content.
fn split_fields(line: &str) -> Vec<&str> {
    line.split('\t').map(str::trim).filter(|f| !f.is_empty()).collect()
}

/// The parsed shape of one alternate-racial-trait row.
struct TraitRow {
    line_number: u32,
    name: String,
    key: String,
    race_key: String,
    category: Option<String>,
    type_tokens: Vec<String>,
    is_racial_default: bool,
    suppressed_by_flag: Option<String>,
    sets_replace_flags: Vec<String>,
    description: Option<String>,
    source_page: Option<String>,
    raw_tokens: Vec<RawToken>,
    raw_bonus_chains: Vec<RawBonusChain>,
}

/// Extracts the flag name a `!PREFACT:1,ABILITIES,<flag>=true` clause reads.
/// Used only for *standalone* `!PREFACT` fields; `PREMULT`-wrapped ones are a
/// different construct (see this module's doc comment).
fn prefact_flag(clause_value: &str) -> Option<String> {
    // Value shape: `1,ABILITIES,<flag>=true`
    let mut parts = clause_value.split(',');
    let _count = parts.next()?;
    let scope = parts.next()?;
    if scope != "ABILITIES" {
        return None;
    }
    let assignment = parts.next()?;
    let (flag, truth) = assignment.split_once('=')?;
    if !truth.eq_ignore_ascii_case("true") {
        return None;
    }
    Some(flag.trim().to_string())
}

/// `DESC:` values carry optional trailing `|`-delimited arguments: PCGen
/// prerequisite clauses (`|!PREABILITY:...`) and `%n` substitution arguments
/// (`|Halfling_AdaptableLuck_Times`). Only the leading segment is prose. The
/// full untouched value is still preserved in `raw_tokens`, so nothing is lost.
fn desc_prose(value: &str) -> &str {
    value.split('|').next().unwrap_or(value).trim()
}

fn parse_row(line_number: u32, line: &str) -> Option<TraitRow> {
    let fields = split_fields(line);
    if fields.is_empty() {
        return None;
    }

    let mut name: Option<String> = None;
    let mut parsed: Vec<Field> = Vec::new();
    for field in &fields {
        match field.split_once(':') {
            Some((key, value)) => parsed.push(Field { key: key.to_string(), value: value.to_string() }),
            None => {
                if name.is_none() {
                    name = Some((*field).to_string());
                } else {
                    panic!("line {line_number}: more than one bare (colon-less) field: {field:?}");
                }
            }
        }
    }

    // TYPE components decide whether this row is a racial trait at all, and
    // which race owns it. `TYPE:RacialTraits.Dwarf Racial Trait.SpecialQuality`
    // -> race "Dwarf". Rows may carry more than one TYPE field.
    let mut type_tokens: Vec<String> = Vec::new();
    for f in parsed.iter().filter(|f| f.key == "TYPE") {
        type_tokens.extend(f.value.split('.').map(|t| t.trim().to_string()).filter(|t| !t.is_empty()));
    }
    let race_key = type_tokens
        .iter()
        .find_map(|t| t.strip_suffix(RACIAL_TRAIT_TYPE_SUFFIX))
        .map(|r| r.trim().to_string())?;

    let key = parsed
        .iter()
        .find(|f| f.key == "KEY")
        .map(|f| f.value.clone())
        .unwrap_or_else(|| panic!("line {line_number}: racial-trait row has no KEY: field"));
    let name = name.unwrap_or_else(|| panic!("line {line_number}: racial-trait row has no display-name field"));

    // Read the default marker off the corpus rather than assuming alternates
    // are never defaults -- `decisions.md §26` notes the standard set is
    // self-identifying via `TYPE:...<Race> Racial Default...`.
    let default_marker = format!("{race_key}{RACIAL_DEFAULT_TYPE_SUFFIX}");
    let is_racial_default = type_tokens.iter().any(|t| t == &default_marker);

    // `FACT:<flag>|True` is the *setting* form. `!PREFACT:...` occurrences read
    // flags and are deliberately not counted here.
    let mut sets_replace_flags: Vec<String> = Vec::new();
    for f in parsed.iter().filter(|f| f.key == "FACT") {
        let Some((flag, truth)) = f.value.split_once('|') else { continue };
        if !flag.contains("_Replace") || !truth.trim().eq_ignore_ascii_case("true") {
            continue;
        }
        let flag = flag.trim().to_string();
        if !sets_replace_flags.contains(&flag) {
            sets_replace_flags.push(flag);
        }
    }

    // Standalone `!PREFACT` only. A `PREMULT`-wrapped one is the self-exclusion
    // guard, not a suppression gate.
    let suppressed_by_flag = parsed.iter().filter(|f| f.key == "!PREFACT").find_map(|f| prefact_flag(&f.value));

    let description = {
        let parts: Vec<&str> =
            parsed.iter().filter(|f| f.key == "DESC").map(|f| desc_prose(&f.value)).filter(|s| !s.is_empty()).collect();
        if parts.is_empty() { None } else { Some(parts.join(" ")) }
    };

    let source_page = parsed.iter().find(|f| f.key == "SOURCEPAGE").map(|f| f.value.clone());

    let raw_bonus_chains: Vec<RawBonusChain> = parsed
        .iter()
        .filter(|f| f.key == "BONUS")
        .map(|f| RawBonusChain {
            qualifiers: f.value.split('|').map(|q| q.trim().to_string()).filter(|q| !q.is_empty()).collect(),
        })
        .collect();

    // Everything except the BONUS chains (which have their own field), kept in
    // source order and verbatim -- this is what preserves PREMULT, ASPECT,
    // ABILITY, DEFINE, VISION and the rest for downstream resolvers.
    let raw_tokens: Vec<RawToken> = parsed
        .iter()
        .filter(|f| f.key != "BONUS")
        .map(|f| RawToken { key: f.key.clone(), value: f.value.clone() })
        .collect();

    Some(TraitRow {
        line_number,
        name,
        key,
        race_key,
        category: parsed.iter().find(|f| f.key == "CATEGORY").map(|f| f.value.clone()),
        type_tokens,
        is_racial_default,
        suppressed_by_flag,
        sets_replace_flags,
        description,
        source_page,
        raw_tokens,
        raw_bonus_chains,
    })
}

fn main() {
    let data_root = pcgen_data_root();
    let lst_path = data_root.join(LST_RELATIVE);
    let bytes =
        fs::read(&lst_path).unwrap_or_else(|e| panic!("failed to read the ARG racial-ability corpus {lst_path:?}: {e}"));
    let sha256 = sha256_hex(&bytes);
    let text = String::from_utf8_lossy(&bytes).to_string();

    let out_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/corpus/advanced_race_guide/race_trait");
    let ingested_at = ingested_at_now();

    let in_scope: BTreeSet<&str> = IN_SCOPE_RACES.into_iter().collect();

    let mut rows: Vec<TraitRow> = Vec::new();
    let mut skipped: BTreeMap<String, usize> = BTreeMap::new();
    let mut real_lines = 0usize;

    for (idx, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        real_lines += 1;
        let Some(row) = parse_row((idx + 1) as u32, line) else { continue };
        if in_scope.contains(row.race_key.as_str()) {
            rows.push(row);
        } else {
            *skipped.entry(row.race_key.clone()).or_default() += 1;
        }
    }

    // A stale record from a previous run with different scope would be
    // indistinguishable from a fresh one, so the output tree is rebuilt.
    if out_root.exists() {
        fs::remove_dir_all(&out_root).unwrap_or_else(|e| panic!("failed to clear {out_root:?}: {e}"));
    }

    let mut written = 0usize;
    let mut flags_total = 0usize;
    let mut per_race: BTreeMap<String, usize> = BTreeMap::new();
    let mut per_race_flags: BTreeMap<String, usize> = BTreeMap::new();
    let mut defaults_seen: Vec<String> = Vec::new();
    let mut gated_alternates: Vec<(String, String)> = Vec::new();
    let mut written_paths: BTreeSet<PathBuf> = BTreeSet::new();

    for row in &rows {
        if row.is_racial_default {
            defaults_seen.push(row.key.clone());
        }
        if let Some(flag) = &row.suppressed_by_flag {
            gated_alternates.push((row.key.clone(), flag.clone()));
        }
        flags_total += row.sets_replace_flags.len();
        *per_race.entry(row.race_key.clone()).or_default() += 1;
        *per_race_flags.entry(row.race_key.clone()).or_default() += row.sets_replace_flags.len();

        let record = CorpusRecordV1 {
            population: Population::InScope,
            completeness: Completeness::Full,
            ingested_at: ingested_at.clone(),
            data: RaceTraitCacheData {
                key: row.key.clone(),
                name: row.name.clone(),
                race_key: row.race_key.clone(),
                category: row.category.clone(),
                type_tokens: row.type_tokens.clone(),
                is_racial_default: row.is_racial_default,
                suppressed_by_flag: row.suppressed_by_flag.clone(),
                sets_replace_flags: row.sets_replace_flags.clone(),
                description: row.description.clone(),
                source_page: row.source_page.clone(),
                raw_tokens: row.raw_tokens.clone(),
                raw_bonus_chains: row.raw_bonus_chains.clone(),
            },
            source: CorpusSource::LstToken {
                path: LST_RELATIVE.to_string(),
                sha256: sha256.clone(),
                line: row.line_number,
                record_key: row.key.clone(),
            },
            license: Some(License::Ogl),
            pi_field: None,
            pi_marker: None,
        };

        let path = out_root.join(slugify(&row.race_key)).join(format!("{}.json", slugify(&row.key)));
        if !written_paths.insert(path.clone()) {
            panic!("slug collision: two ARG racial traits both resolve to {path:?}");
        }
        fs::create_dir_all(path.parent().expect("record path has a parent")).expect("failed to create output dir");
        let json = serde_json::to_string_pretty(&record).expect("record must serialize");
        fs::write(&path, json + "\n").unwrap_or_else(|e| panic!("failed to write {path:?}: {e}"));
        written += 1;
    }

    let skipped_total: usize = skipped.values().sum();
    println!("ARG alternate racial traits -- source {LST_RELATIVE}");
    println!("  sha256                        : {sha256}");
    println!("  real (non-comment) lines      : {real_lines}");
    println!("  records emitted               : {written}");
    println!("  distinct races covered        : {}", per_race.len());
    println!("  replace-flags captured        : {flags_total}");
    println!("  skipped, out-of-scope races   : {skipped_total} across {} races", skipped.len());
    println!("  ingested_at                   : {ingested_at}");
    println!("\n  per in-scope race (records / replace-flags):");
    for (race, n) in &per_race {
        println!("    {race:<12} {n:>4} / {:>4}", per_race_flags.get(race).copied().unwrap_or(0));
    }
    println!("\n  skipped per out-of-scope race:");
    for (race, n) in &skipped {
        println!("    {race:<12} {n:>4}");
    }
    println!("\n  rows carrying a \"<Race> Racial Default\" TYPE marker : {}", defaults_seen.len());
    for k in &defaults_seen {
        println!("    {k}");
    }
    println!("  in-scope alternates gated by a standalone !PREFACT   : {}", gated_alternates.len());
    for (k, flag) in &gated_alternates {
        println!("    {k} <- {flag}");
    }

    assert_eq!(written, rows.len(), "every in-scope row must produce exactly one record");
    let on_disk = count_json(&out_root);
    assert_eq!(on_disk, written, "records written to disk must match records emitted");
}

fn count_json(dir: &Path) -> usize {
    let mut n = 0;
    for entry in fs::read_dir(dir).unwrap_or_else(|e| panic!("failed to read {dir:?}: {e}")) {
        let entry = entry.expect("readable dir entry");
        let path = entry.path();
        if path.is_dir() {
            n += count_json(&path);
        } else if path.extension().is_some_and(|e| e == "json") {
            n += 1;
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `arg_abilities_race.lst:38` verbatim except for a shortened first
    /// `DESC:` (tokens joined with single tabs; the corpus pads with tab
    /// runs, which `split_fields` discards). Chosen
    /// because it exercises every branch at once: two `TYPE:` fields, a
    /// `PREMULT`-wrapped `!PREFACT` self-exclusion guard, two `DESC:` fields
    /// (one carrying a `|!PREABILITY` condition), and a `FACT:...|True`
    /// replace-flag setting.
    const MAGIC_RESISTANT: &str = concat!(
        "Magic Resistant\t",
        "KEY:Dwarf ~ Magic Resistant\t",
        "CATEGORY:Special Ability\t",
        "TYPE:RacialTraits.Dwarf Racial Trait.SpecialQuality.Special Quality\t",
        "TYPE:Replaces Dwarf Hardy\t",
        "PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Magic Resistant],",
        "[!PREFACT:1,ABILITIES,Dwarf_ReplaceHardy=true]\t",
        "DESC:Some of the older dwarven clans are particularly resistant to magic.\t",
        "DESC:This racial trait replaces hardy.|!PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Magic Resistant\t",
        "SR:5+(TL-HD)\t",
        "COST:0\t",
        "SOURCEPAGE:p.12\t",
        "FACT:Dwarf_ReplaceHardy|True",
    );

    /// `arg_abilities_race.lst:43` (`Dwarf ~ Saltbeard`), verbatim except for
    /// a shortened first `DESC:` and its 3 `ASPECT:` fields elided. It sets
    /// **4** replace flags while its `PREMULT` guard names only 3 of them —
    /// the precise case that proves `sets_replace_flags` is read off the
    /// `FACT:` fields and not off the `PREMULT` clause. It also carries 3
    /// `BONUS:` chains.
    const SALTBEARD: &str = concat!(
        "Saltbeard\t",
        "KEY:Dwarf ~ Saltbeard\t",
        "CATEGORY:Special Ability\t",
        "TYPE:RacialTraits.Dwarf Racial Trait.SpecialAttack.Special Attack.Defensive\t",
        "PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Saltbeard],",
        "[!PREFACT:1,ABILITIES,Dwarf_ReplaceDefensiveTraining=true,",
        "Dwarf_ReplaceHatred=true,Dwarf_ReplaceStonecunning=true]\t",
        "DEFINE:RacialDefensiveTrainingBonus|0\t",
        "DESC:Dwarves occasionally found iron cities along rugged seacoasts.\t",
        "DESC:This racial trait replaces defensive training, hatred, and stonecunning.",
        "|!PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Saltbeard\t",
        "ABILITY:Dwarf Racial Trait|AUTOMATIC|Saltbeard ~ Dwarf ~ Greed\t",
        "BONUS:SITUATION|Survival=while at sea|2|TYPE=Racial\t",
        "BONUS:SKILL|Profession (Sailor)|2|TYPE=Racial\t",
        "BONUS:VAR|RacialDefensiveTrainingBonus|2\t",
        "COST:0\t",
        "SOURCEPAGE:p.12\t",
        "FACT:Dwarf_ReplaceDefensiveTraining|True\t",
        "FACT:Dwarf_ReplaceHatred|True\t",
        "FACT:Dwarf_ReplaceStonecunning|True\t",
        "FACT:Dwarf_ReplaceGreed|True",
    );

    /// `core_essentials/races/dwarf/dwarf_abilities_race.lst:23` verbatim
    /// (shortened `DESC`) — a *standard* trait, i.e. the other end of the
    /// protocol. It is the shape that legitimately populates
    /// `suppressed_by_flag`, and it self-identifies as a racial default.
    const STANDARD_GREED: &str = concat!(
        "Greed\t",
        "KEY:Dwarf ~ Greed\t",
        "CATEGORY:Special Ability\t",
        "TYPE:RacialTraits.Dwarf Racial Trait.Dwarf Racial Default.SpecialQuality\t",
        "!PREFACT:1,ABILITIES,Dwarf_ReplaceGreed=True\t",
        "DESC:Dwarves receive a +2 racial bonus on Appraise skill checks.",
    );

    #[test]
    fn alternate_row_sets_its_replace_flag_and_is_not_suppressed_by_its_own_guard() {
        let row = parse_row(38, MAGIC_RESISTANT).expect("row is a racial trait");
        assert_eq!(row.key, "Dwarf ~ Magic Resistant");
        assert_eq!(row.name, "Magic Resistant");
        assert_eq!(row.race_key, "Dwarf");
        assert_eq!(row.category.as_deref(), Some("Special Ability"));
        assert_eq!(row.source_page.as_deref(), Some("p.12"));
        assert_eq!(row.sets_replace_flags, vec!["Dwarf_ReplaceHardy"]);
        // The `PREMULT`-wrapped `!PREFACT` is a self-exclusion guard, not a
        // suppression gate, so it must NOT be laundered into this field.
        assert_eq!(row.suppressed_by_flag, None);
        assert!(!row.is_racial_default);
        // Both TYPE fields contribute, split on `.`.
        assert!(row.type_tokens.contains(&"Dwarf Racial Trait".to_string()));
        assert!(row.type_tokens.contains(&"Replaces Dwarf Hardy".to_string()));
        // ...but the guard is still preserved verbatim, so nothing is lost.
        let premult = row.raw_tokens.iter().find(|t| t.key == "PREMULT").expect("PREMULT preserved");
        assert!(premult.value.contains("!PREFACT:1,ABILITIES,Dwarf_ReplaceHardy=true"));
        // DESC prose is joined; the `|!PREABILITY` condition is stripped from
        // the prose but kept whole in raw_tokens.
        let desc = row.description.expect("description");
        assert!(desc.ends_with("This racial trait replaces hardy."), "got {desc:?}");
        assert!(!desc.contains("PREABILITY"));
        assert!(row.raw_tokens.iter().any(|t| t.key == "DESC" && t.value.contains("!PREABILITY")));
    }

    #[test]
    fn alternate_row_captures_every_flag_it_sets_not_just_the_guarded_ones() {
        let row = parse_row(43, SALTBEARD).expect("row is a racial trait");
        assert_eq!(row.race_key, "Dwarf");
        // 4 set; the PREMULT guard names only the first 3.
        assert_eq!(
            row.sets_replace_flags,
            vec![
                "Dwarf_ReplaceDefensiveTraining",
                "Dwarf_ReplaceHatred",
                "Dwarf_ReplaceStonecunning",
                "Dwarf_ReplaceGreed",
            ]
        );
        assert_eq!(row.suppressed_by_flag, None);
        assert_eq!(row.raw_bonus_chains.len(), 3);
        assert_eq!(row.raw_bonus_chains[0].qualifiers, vec!["SITUATION", "Survival=while at sea", "2", "TYPE=Racial"]);
        assert_eq!(row.raw_bonus_chains[2].qualifiers, vec!["VAR", "RacialDefensiveTrainingBonus", "2"]);
        // BONUS lives in its own field and is not duplicated into raw_tokens,
        // but every other field is preserved.
        assert!(row.raw_tokens.iter().all(|t| t.key != "BONUS"));
        assert!(row.raw_tokens.iter().any(|t| t.key == "DEFINE"));
        assert!(row.raw_tokens.iter().any(|t| t.key == "ABILITY"));
    }

    #[test]
    fn standard_row_populates_suppressed_by_flag_and_the_racial_default_marker() {
        let row = parse_row(23, STANDARD_GREED).expect("row is a racial trait");
        assert_eq!(row.race_key, "Dwarf");
        assert_eq!(row.suppressed_by_flag.as_deref(), Some("Dwarf_ReplaceGreed"));
        assert!(row.sets_replace_flags.is_empty());
        // Read off the corpus, never forced: this row really is a default.
        assert!(row.is_racial_default);
    }

    #[test]
    fn rows_without_a_racial_trait_type_are_not_racial_traits() {
        // `arg_abilities_race.lst:23` verbatim — a `.MOD` row from one of the
        // file's 37 `Racial Traits` blocks. It only re-stamps a SOURCEPAGE on
        // a trait `core_essentials` declares; it carries no TYPE at all.
        assert!(parse_row(23, "CATEGORY=Special Ability|Dwarf ~ Greed.MOD\t\t\tSOURCEPAGE:p.10").is_none());
        // `arg_abilities_race.lst:1323` (truncated) — a favored-class-bonus
        // row. TYPE is present, but names no `<Race> Racial Trait`.
        assert!(
            parse_row(
                1323,
                concat!(
                    "Bonus Acid and Earth Spell Damage\t\t\t",
                    "KEY:Favored Class Bonus ~ Acid and Earth Spell Damage\t\t\t",
                    "CATEGORY:Special Ability\t",
                    "TYPE:SpecialQuality.FavoredClassBonus.FavoredClassSorcerer",
                )
            )
            .is_none()
        );
    }

    #[test]
    fn only_true_valued_replace_facts_count_as_settings() {
        // `FACT:` fields that are not replace flags, and replace flags set to
        // something other than True, are both excluded.
        let line = concat!(
            "X\tKEY:Dwarf ~ X\tCATEGORY:Special Ability\tTYPE:Dwarf Racial Trait\t",
            "FACT:Dwarf_ReplaceGreed|True\tFACT:BaseSize|M\tFACT:Dwarf_ReplaceHardy|False"
        );
        let row = parse_row(1, line).expect("row is a racial trait");
        assert_eq!(row.sets_replace_flags, vec!["Dwarf_ReplaceGreed"]);
    }

    #[test]
    fn prefact_flag_reads_only_abilities_scoped_true_assertions() {
        assert_eq!(prefact_flag("1,ABILITIES,Dwarf_ReplaceGreed=True").as_deref(), Some("Dwarf_ReplaceGreed"));
        assert_eq!(prefact_flag("1,ABILITIES,Dwarf_ReplaceGreed=false"), None);
        assert_eq!(prefact_flag("1,VAR,Something=True"), None);
        assert_eq!(prefact_flag("garbage"), None);
    }

    #[test]
    fn desc_prose_drops_pcgen_argument_and_prerequisite_segments() {
        assert_eq!(desc_prose("Plain text."), "Plain text.");
        assert_eq!(desc_prose("Replaces greed.|!PREABILITY:1,CATEGORY=Special Ability,X"), "Replaces greed.");
        assert_eq!(desc_prose("%1 times per day|Halfling_AdaptableLuck_Times"), "%1 times per day");
    }

    #[test]
    fn race_directory_slugs_match_the_corpus_directory_convention() {
        assert_eq!(slugify("Half-Elf"), "half_elf");
        assert_eq!(slugify("Half-Orc"), "half_orc");
        assert_eq!(slugify("Svirfneblin"), "svirfneblin");
        assert_eq!(slugify("Dwarf ~ Ancient Enmity"), "dwarf_ancient_enmity");
    }

    #[test]
    fn in_scope_roster_is_exactly_the_18_races_decisions_25_3_names() {
        assert_eq!(IN_SCOPE_RACES.len(), 18);
        let unique: BTreeSet<&str> = IN_SCOPE_RACES.into_iter().collect();
        assert_eq!(unique.len(), 18, "roster must not repeat a race");
        // The out-of-scope races decisions.md §25.3 defers to SD-28 must not
        // have crept in.
        for deferred in [
            "Dhampir",
            "Fetchling",
            "Grippli",
            "Ifrit",
            "Oread",
            "Sylph",
            "Undine",
            "Catfolk",
            "Ratfolk",
            "Suli",
            "Vanara",
            "Vishkanya",
            "Changeling",
            "Kitsune",
            "Nagaji",
            "Samsaran",
            "Wayang",
            "Gillman",
            "Strix",
        ] {
            assert!(!unique.contains(deferred), "{deferred} is deferred to SD-28 and must not be in scope");
        }
    }
}
