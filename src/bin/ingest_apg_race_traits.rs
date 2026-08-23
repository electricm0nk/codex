//! SD28-E16 -- ingests the Advanced Player's Guide's **alternate racial
//! traits** from `advanced_players_guide/apg_abilities_race.lst` into Shape B
//! v1 `RaceTraitCacheData` records under
//! `data/corpus/advanced_players_guide/race_trait/<race>/<trait>.json`.
//!
//! **Corrected scope, 2026-08-08: 1 record, not 50.** `decisions.md §37`'s
//! six-book trace found 50 real `<Race> Racial Trait`-shaped rows in this
//! file and treated all 50 as closable gap. A first ingest pass proved that
//! wrong: **49 of the 50 share their exact `KEY:` with an already-ingested
//! ARG record** -- ARG (`SOURCEDATE:2012-06`) republished the bulk of APG's
//! (`SOURCEDATE:2010-08`) own 7-CRB-race alternate traits, with minor wording
//! revisions. ARG being the later book means its wording is the current one;
//! shipping APG's copy over it would be a two-year regression, not new
//! content, and `race_resolver::load_race_corpus` has no book-scoping in its
//! key space, so loading both directories would silently let whichever book
//! loads last shadow the other -- exactly the corruption a first full-50
//! ingest produced (`decisions.md §39`). Only **`Half-Orc ~ Plagueborn`**
//! (`apg_abilities_race.lst:83`) is a genuinely new key, absent from ARG's
//! 156. This binary now emits that one record only, filtering every row
//! whose key collides with `data/corpus/advanced_race_guide/race_trait/`'s
//! own on-disk key set -- read from disk, not hand-listed, so a future APG
//! revision or a future book reusing this binary's shape does not need its
//! exclusion list maintained by hand.
//!
//! **Scope filter.** Only the 7 CRB races (all already-ingested, so their
//! alternates have a real chassis to attach to), minus any row whose `KEY:`
//! already exists in ARG's ingested corpus. No Bestiary-race alternates
//! exist in this file (verified: `TYPE:<Race> Racial Trait` markers name only
//! the 7 CRB races, re-derived directly against the corpus, not transcribed).
//!
//! **The replace-flag protocol, FCB row exclusion, and `DESC:` rendering are
//! all identical to `src/bin/ingest_race_traits.rs`**, which this binary
//! is deliberately modelled on rather than sharing code with (matching this
//! program's existing convention of small, independently-readable generator
//! binaries -- the same convention `gen_cache_beastiary.rs`'s
//! `count_on_disk_records` duplication follows). The FCB and `CATEGORY:Choice`
//! trap rules `v06_work_inventory.rs` gained for ARG (`decisions.md §35`)
//! already apply to this book unmodified -- confirmed via the real generator
//! run before writing this binary: `race_favored_class_bonus_row: 54`,
//! `race_choice_suboption_row: 2` both appear in APG's own `trap_hits`.
//!
//! Run with `cargo run --bin ingest_apg_race_traits`. `PCGEN_CORPUS_ROOT` may
//! point at a local PCGen `data/` checkout; it defaults to
//! `$HOME/workspace/repos/pcgen/data`.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use codex::rules_core::cache_gen::WiringClassIndex;
use codex::rules_core::pi_screening;
use codex::rules_core::shape_b_v1::{
    Completeness, CorpusRecordV1, CorpusSource, Population, RaceTraitCacheData, RawBonusChain, RawToken,
};

/// The one source file this binary ingests, relative to the PCGen `data/` root.
const LST_RELATIVE: &str = "pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_abilities_race.lst";

/// The 7 in-scope races, spelled exactly as the corpus spells them in its
/// `TYPE:<Race> Racial Trait` component. Re-derived directly against
/// `apg_abilities_race.lst` (`decisions.md §37`), not transcribed from ARG's
/// roster -- APG carries no Bestiary-race alternates at all.
const IN_SCOPE_RACES: [&str; 7] = ["Dwarf", "Elf", "Gnome", "Half-Elf", "Half-Orc", "Halfling", "Human"];

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

/// Same slug rule `gen_book_cache.rs`/`ingest_race_traits.rs` use.
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

/// One tab-delimited `KEY:VALUE` field off a corpus row.
struct Field {
    key: String,
    value: String,
}

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
    unresolved_desc_args: Vec<String>,
    source_page: Option<String>,
    raw_tokens: Vec<RawToken>,
    raw_bonus_chains: Vec<RawBonusChain>,
}

/// Extracts the flag name a `!PREFACT:1,ABILITIES,<flag>=true` clause reads.
fn prefact_flag(clause_value: &str) -> Option<String> {
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

// ---------------------------------------------------------------------
// `DESC:` rendering -- identical to `ingest_race_traits.rs`; see that
// binary's own doc comments for the full rationale.
// ---------------------------------------------------------------------

fn same_row_vars(parsed: &[Field]) -> BTreeMap<String, Option<i64>> {
    let mut vars: BTreeMap<String, Option<i64>> = BTreeMap::new();

    for f in parsed.iter().filter(|f| f.key == "DEFINE") {
        let Some((name, base)) = f.value.split_once('|') else { continue };
        vars.insert(name.trim().to_string(), base.trim().parse::<i64>().ok());
    }

    for f in parsed.iter().filter(|f| f.key == "BONUS") {
        let quals: Vec<&str> = f.value.split('|').collect();
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

fn is_prerequisite_arg(arg: &str) -> bool {
    arg.contains(':') && (arg.starts_with("PRE") || arg.starts_with("!PRE"))
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
struct RenderedDescription {
    text: Option<String>,
    unresolved_args: Vec<String>,
}

fn render_description(parsed: &[Field]) -> Result<RenderedDescription, String> {
    let vars = same_row_vars(parsed);
    let mut segments: Vec<String> = Vec::new();
    let mut unresolved_args: Vec<String> = Vec::new();
    let mut saw_desc = false;

    for f in parsed.iter().filter(|f| f.key == "DESC") {
        saw_desc = true;
        let mut parts = f.value.split('|');
        let prose = parts.next().unwrap_or_default();
        let (gates, args): (Vec<&str>, Vec<&str>) = parts.partition(|p| is_prerequisite_arg(p));

        let mut applies = true;
        for gate in &gates {
            if !gate.trim_start_matches('!').starts_with("PREVAR") {
                continue;
            }
            applies &= eval_prevar_gate(gate, &vars)?;
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
    Ok(RenderedDescription { text, unresolved_args })
}

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

    let default_marker = format!("{race_key}{RACIAL_DEFAULT_TYPE_SUFFIX}");
    let is_racial_default = type_tokens.iter().any(|t| t == &default_marker);

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

    let suppressed_by_flag = parsed.iter().filter(|f| f.key == "!PREFACT").find_map(|f| prefact_flag(&f.value));

    let rendered = render_description(&parsed).unwrap_or_else(|e| panic!("line {line_number}: {e}"));

    let source_page = parsed.iter().find(|f| f.key == "SOURCEPAGE").map(|f| f.value.clone());

    let raw_bonus_chains: Vec<RawBonusChain> = parsed
        .iter()
        .filter(|f| f.key == "BONUS")
        .map(|f| RawBonusChain {
            qualifiers: f.value.split('|').map(|q| q.trim().to_string()).filter(|q| !q.is_empty()).collect(),
        })
        .collect();

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
        description: rendered.text,
        unresolved_desc_args: rendered.unresolved_args,
        source_page,
        raw_tokens,
        raw_bonus_chains,
    })
}

fn main() {
    let data_root = pcgen_data_root();
    let lst_path = data_root.join(LST_RELATIVE);
    let bytes =
        fs::read(&lst_path).unwrap_or_else(|e| panic!("failed to read the APG racial-ability corpus {lst_path:?}: {e}"));
    let sha256 = sha256_hex(&bytes);
    let text = String::from_utf8_lossy(&bytes).to_string();

    let out_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/corpus/advanced_players_guide/race_trait");
    let ingested_at = ingested_at_now();
    let apg_book_dir = data_root.join("pathfinder/paizo/roleplaying_game/advanced_players_guide");
    let wiring_index = WiringClassIndex::build("advanced_players_guide", &apg_book_dir);
    let mut wiring_lines = wiring_index.lines();
    let lst_basename = LST_RELATIVE.rsplit('/').next().unwrap_or(LST_RELATIVE);

    let in_scope: BTreeSet<&str> = IN_SCOPE_RACES.into_iter().collect();
    // Read from disk, not hand-listed (decisions.md §39): ARG is the later
    // book (SOURCEDATE 2012-06 vs APG's 2010-08), so any APG row sharing an
    // ARG key is APG's older wording and must not overwrite the current one.
    let arg_keys = already_ingested_keys(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/corpus/advanced_race_guide/race_trait"),
    );

    let mut rows: Vec<TraitRow> = Vec::new();
    let mut skipped: BTreeMap<String, usize> = BTreeMap::new();
    let mut real_lines = 0usize;
    let mut key_collisions: Vec<String> = Vec::new();

    for (idx, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        real_lines += 1;
        let Some(row) = parse_row((idx + 1) as u32, line) else { continue };
        if !in_scope.contains(row.race_key.as_str()) {
            *skipped.entry(row.race_key.clone()).or_default() += 1;
            continue;
        }
        if arg_keys.contains(&row.key) {
            key_collisions.push(row.key.clone());
            continue;
        }
        rows.push(row);
    }

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
    let mut unresolved_desc_args: Vec<String> = Vec::new();
    let mut leaks: Vec<String> = Vec::new();

    for row in &rows {
        if let Some(desc) = row.description.as_deref()
            && let Some(leak) = leaked_pcgen_syntax(desc)
        {
            leaks.push(format!("{LST_RELATIVE}:{}: {} would ship a {leak}: {desc}", row.line_number, row.key));
        }
        for arg in &row.unresolved_desc_args {
            unresolved_desc_args
                .push(format!("{} -> DESC arg {arg:?} is not a same-row literal (dropped, not guessed)", row.key));
        }

        if row.is_racial_default {
            defaults_seen.push(row.key.clone());
        }
        if let Some(flag) = &row.suppressed_by_flag {
            gated_alternates.push((row.key.clone(), flag.clone()));
        }
        flags_total += row.sets_replace_flags.len();
        *per_race.entry(row.race_key.clone()).or_default() += 1;
        *per_race_flags.entry(row.race_key.clone()).or_default() += row.sets_replace_flags.len();

        let (wiring_class, wiring_class_signals) = wiring_index.wiring_class_for(
            &mut wiring_lines,
            lst_basename,
            row.line_number,
            &row.key,
            &row.key,
        );
        let (license, pi_field, pi_marker, stored_desc) =
            pi_screening::classify_optional_field("description", row.description.as_deref());
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
                description: stored_desc,
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
            license: Some(license),
            pi_field,
            pi_marker,
            wiring_class,
            wiring_class_signals,
            description_source: None,
        };

        let path = out_root.join(slugify(&row.race_key)).join(format!("{}.json", slugify(&row.key)));
        if !written_paths.insert(path.clone()) {
            panic!("slug collision: two APG racial traits both resolve to {path:?}");
        }
        fs::create_dir_all(path.parent().expect("record path has a parent")).expect("failed to create output dir");
        let json = serde_json::to_string_pretty(&record).expect("record must serialize");
        fs::write(&path, json + "\n").unwrap_or_else(|e| panic!("failed to write {path:?}: {e}"));
        written += 1;
    }

    let skipped_total: usize = skipped.values().sum();
    println!("APG alternate racial traits -- source {LST_RELATIVE}");
    println!("  sha256                        : {sha256}");
    println!("  real (non-comment) lines      : {real_lines}");
    println!("  records emitted               : {written}");
    println!("  distinct races covered        : {}", per_race.len());
    println!("  replace-flags captured        : {flags_total}");
    println!("  skipped, out-of-scope races   : {skipped_total} across {} races", skipped.len());
    println!(
        "  skipped, key collides with ARG: {} (decisions.md §39 -- ARG's later wording wins)",
        key_collisions.len()
    );
    for k in &key_collisions {
        println!("    {k}");
    }
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

    println!("\n  DESC args that are not same-row literals (dropped, never guessed) : {}", unresolved_desc_args.len());
    for line in &unresolved_desc_args {
        println!("    {line}");
    }

    assert_eq!(written, rows.len(), "every in-scope row must produce exactly one record");
    let on_disk = count_json(&out_root);
    assert_eq!(on_disk, written, "records written to disk must match records emitted");

    if !leaks.is_empty() {
        for line in &leaks {
            eprintln!("LEAK  {line}");
        }
        panic!("{} description(s) carry PCGen syntax; refusing to ship them", leaks.len());
    }
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

/// Every `data.key` value of every JSON record under `dir`, read directly
/// from disk rather than hand-maintained (decisions.md §39's proposed fix,
/// applied narrowly here for APG's own real collision rather than built as
/// the general cross-book trap). Empty if `dir` does not exist -- a book
/// with no `race_trait/` directory contributes no keys to collide with.
fn already_ingested_keys(dir: &Path) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    fn walk(dir: &Path, keys: &mut BTreeSet<String>) {
        let Ok(entries) = fs::read_dir(dir) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, keys);
            } else if path.extension().is_some_and(|e| e == "json") {
                let Ok(text) = fs::read_to_string(&path) else { continue };
                let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
                if let Some(key) = value["data"]["key"].as_str() {
                    keys.insert(key.to_string());
                }
            }
        }
    }
    walk(dir, &mut keys);
    keys
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `apg_abilities_race.lst:16` verbatim except for the elided tab
    /// padding (real corpus pads with tab runs, which `split_fields`
    /// discards). Chosen because it exercises the core shape: a `PREMULT`
    /// self-exclusion guard, two `DESC:` fields (one carrying a
    /// `|!PREABILITY` condition), a `FACT:...|true` replace-flag setting,
    /// and `ASPECT:` fields (preserved verbatim in `raw_tokens`, not
    /// rendered).
    const ANCIENT_ENMITY: &str = concat!(
        "Ancient Enmity\t",
        "KEY:Dwarf ~ Ancient Enmity\t",
        "CATEGORY:Special Ability\t",
        "TYPE:RacialTraits.Dwarf Racial Trait.SpecialAttack.Special Attack\t",
        "PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Ancient Enmity],",
        "[!PREFACT:1,ABILITIES,Dwarf_ReplaceHatred=true]\t",
        "DESC:Dwarves have long been in conflict with elves, especially the hated drow. ",
        "Dwarves with this racial trait receive a +1 bonus on attack rolls against ",
        "humanoid creatures of the elf subtype.\t",
        "DESC:This racial trait replaces the hatred racial trait.",
        "|!PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Ancient Enmity\t",
        "COST:0\t",
        "SOURCEPAGE:p.11\t",
        "ASPECT:CombatBonus|+1 bonus on attack rolls against humanoid creatures of the elf subtype.\t",
        "ASPECT:StatBlockName|+1 on attack rolls against elf humanoids\t",
        "FACT:Dwarf_ReplaceHatred|true",
    );

    /// `apg_abilities_race.lst:32` (`Elf ~ Dreamspeaker`) verbatim except
    /// elided padding. Exercises an `ABILITY:Spell-Like Ability|AUTOMATIC|...`
    /// grant token (preserved in `raw_tokens`) and a `BONUS:DC` chain.
    const DREAMSPEAKER: &str = concat!(
        "Dreamspeaker\t",
        "KEY:Elf ~ Dreamspeaker\t",
        "CATEGORY:Special Ability\t",
        "TYPE:RacialTraits.Elf Racial Trait.SpecialQuality.Special Quality.Applied Bonus\t",
        "PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Elf ~ Dreamspeaker],",
        "[!PREFACT:1,ABILITIES,Elf_ReplaceElvenImmunities=true]\t",
        "DESC:A few elves have the ability to tap into the power of sleep, dreams, and ",
        "prescient reverie. Elves with this racial trait add +1 to saving throw DCs for ",
        "spells of the divination school and sleep effects they cast. In addition, elves ",
        "with a Charisma of 15 or higher may use dream once per day as a spell-like ",
        "ability (caster level is equal to the elf's character level).\t",
        "DESC:This racial trait replaces the elven immunities racial trait.",
        "|!PREABILITY:1,CATEGORY=Special Ability,Elf ~ Dreamspeaker\t",
        "ABILITY:Spell-Like Ability|AUTOMATIC|Racial SLA ~ Dream|PRESTAT:1,CHA=15\t",
        "BONUS:DC|SCHOOL.Divination|1\t",
        "COST:0\t",
        "SOURCEPAGE:p.13\t",
        "FACT:Elf_ReplaceElvenImmunities|true",
    );

    /// `apg_abilities_race.lst:1323`-shaped (truncated) -- a favored-class-bonus
    /// row, the same shape ARG's `ingest_race_traits.rs` proved must never
    /// be treated as a racial trait.
    const FCB_ROW: &str = concat!(
        "Bonus Skill Points\t",
        "KEY:Favored Class Bonus ~ Bonus Skill Points\t",
        "CATEGORY:Special Ability\t",
        "TYPE:SpecialQuality.FavoredClassBonus.FavoredClassAlchemist",
    );

    #[test]
    fn alternate_row_sets_its_replace_flag_and_is_not_suppressed_by_its_own_guard() {
        let row = parse_row(16, ANCIENT_ENMITY).expect("row is a racial trait");
        assert_eq!(row.key, "Dwarf ~ Ancient Enmity");
        assert_eq!(row.name, "Ancient Enmity");
        assert_eq!(row.race_key, "Dwarf");
        assert_eq!(row.category.as_deref(), Some("Special Ability"));
        assert_eq!(row.source_page.as_deref(), Some("p.11"));
        assert_eq!(row.sets_replace_flags, vec!["Dwarf_ReplaceHatred"]);
        assert_eq!(row.suppressed_by_flag, None);
        assert!(!row.is_racial_default);
        let desc = row.description.expect("description");
        assert!(desc.ends_with("This racial trait replaces the hatred racial trait."), "got {desc:?}");
        assert!(!desc.contains("PREABILITY"));
        assert_eq!(leaked_pcgen_syntax(&desc), None);
        // ASPECT fields preserved verbatim, not rendered into the description.
        assert!(row.raw_tokens.iter().any(|t| t.key == "ASPECT" && t.value.starts_with("CombatBonus")));
    }

    #[test]
    fn spell_like_ability_grant_and_dc_bonus_are_preserved_verbatim() {
        let row = parse_row(32, DREAMSPEAKER).expect("row is a racial trait");
        assert_eq!(row.key, "Elf ~ Dreamspeaker");
        assert_eq!(row.race_key, "Elf");
        assert_eq!(row.sets_replace_flags, vec!["Elf_ReplaceElvenImmunities"]);
        assert!(row.raw_tokens.iter().any(|t| t.key == "ABILITY" && t.value.contains("Racial SLA ~ Dream")));
        assert_eq!(row.raw_bonus_chains.len(), 1);
        assert_eq!(row.raw_bonus_chains[0].qualifiers, vec!["DC", "SCHOOL.Divination", "1"]);
        let desc = row.description.expect("description");
        assert_eq!(leaked_pcgen_syntax(&desc), None);
    }

    #[test]
    fn favored_class_bonus_rows_are_not_racial_traits() {
        assert!(parse_row(1323, FCB_ROW).is_none());
    }

    /// The regression guard for decisions.md §39: `already_ingested_keys`
    /// must read real `data.key` values off disk, not a hand-listed set, so
    /// it stays correct if ARG's own roster ever changes without this
    /// binary's code changing too.
    #[test]
    fn already_ingested_keys_reads_real_keys_off_disk_not_a_hand_list() {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/corpus/advanced_race_guide/race_trait");
        let keys = already_ingested_keys(&dir);
        assert_eq!(
            keys.len(),
            421,
            "ARG's 421 ingested race-trait keys (156 -> 201 by SD-31 Epic 1-F2, 2026-08-15; \
             201 -> 259 by SD-31-E6-F4-002's own 6-race chassis batch; 259 -> 283 by \
             SD-31-E6-F4-003's own 24-record alternate-trait batch for those same 6 races, \
             both 2026-08-16; 283 -> 321 by SD31-E6-F4-004's own 4-race chassis batch \
             (Gillman/Nagaji/Vanara/Vishkanya), 2026-08-17; 321 -> 332 by SD31-E6-F4-006's \
             own 11-record alternate-trait batch for those same 4 races, 2026-08-17; \
             332 -> 350 by SD31-E6-F4-007's own 2-race chassis batch (Changeling/Samsaran), \
             2026-08-17, closing arg_races.lst's full 37-row playable-race roster -- every \
             one of those 149 new keys is `<NewRace> ~ ...` and shares no key with any of \
             APG's 7 CRB-race rows, so this discriminator's own behavior is unaffected; \
             350 -> 414 by the Core Essentials removal, 2026-08-18; 414 -> 421 by SD-32 \
             card-11 T2b lane, 2026-08-23, decisions.md §16 item 2: the 7 `Human ~ Adoptive \
             Parentage` CHOOSE-pool members (bare keys `Drow`/`Dwarf`/`Elf`/`Gnome`/`Grippli`/ \
             `Halfling`/`Orc`) -- verified against the pinned oracle to share none of those \
             keys with any `apg_abilities_race.lst` row, so this discriminator is unaffected \
             here too)"
        );
        assert!(keys.contains("Dwarf ~ Ancient Enmity"), "the exact collision this fix exists for");
        assert!(!keys.contains("Half-Orc ~ Plagueborn"), "APG's one genuinely unique key must not be in ARG's set");
    }

    /// A nonexistent directory (a book with no ingested race_trait/ at all)
    /// contributes no keys to collide with -- not an error.
    #[test]
    fn already_ingested_keys_is_empty_for_a_nonexistent_directory() {
        let keys = already_ingested_keys(Path::new("/nonexistent/path/that/does/not/exist"));
        assert!(keys.is_empty());
    }

    #[test]
    fn in_scope_roster_is_exactly_the_7_crb_races() {
        assert_eq!(IN_SCOPE_RACES.len(), 7);
        let unique: BTreeSet<&str> = IN_SCOPE_RACES.into_iter().collect();
        assert_eq!(unique.len(), 7, "roster must not repeat a race");
    }

    #[test]
    fn race_directory_slugs_match_the_corpus_directory_convention() {
        assert_eq!(slugify("Half-Elf"), "half_elf");
        assert_eq!(slugify("Half-Orc"), "half_orc");
        assert_eq!(slugify("Dwarf ~ Ancient Enmity"), "dwarf_ancient_enmity");
    }

    /// The property the player actually experiences: nothing PCGen-shaped
    /// survives into a served description. Scoped to the one book this
    /// binary writes.
    #[test]
    fn no_committed_apg_trait_description_leaks_pcgen_syntax() {
        use codex::rules_core::shape_b_v1::CorpusRecordV1;

        // **The follow-up landed, and this test is load-bearing again.**
        // `decisions.md §39` withheld the one real record this binary emits
        // (`Half-Orc ~ Plagueborn`) because `race_resolver.rs`'s
        // `ALTERNATE_TRAIT_REPLACE_FLAGS` table did not know it (`§36`
        // instance 15) and shipping the corpus record without that table row
        // would have been a stub. SD-29's race-trait extend lane landed both
        // halves in one change (SD-29 `decisions.md §44.3`), so the count
        // below moves from a deliberate 0 to the real 1. The directory read
        // stays tolerant of absence so the failure is the count assertion's
        // clear message rather than an unwrap panic.
        let trait_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/corpus/advanced_players_guide/race_trait");
        let mut race_dirs: Vec<PathBuf> = match fs::read_dir(&trait_root) {
            Ok(entries) => entries.filter_map(Result::ok).map(|e| e.path()).collect(),
            Err(_) => Vec::new(),
        };
        race_dirs.sort();

        let mut checked = 0usize;
        for race_dir in race_dirs {
            let mut files: Vec<PathBuf> =
                fs::read_dir(&race_dir).unwrap().filter_map(Result::ok).map(|e| e.path()).collect();
            files.sort();
            for path in files {
                let record: CorpusRecordV1<RaceTraitCacheData> =
                    serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
                checked += 1;
                let Some(desc) = record.data.description.as_deref() else { continue };
                assert_eq!(
                    leaked_pcgen_syntax(desc),
                    None,
                    "{path:?}: served description carries PCGen syntax: {desc}"
                );
                assert_eq!(desc.trim(), desc, "{path:?}: served description has stray edge whitespace");
            }
        }
        assert_eq!(
            checked, 1,
            "1 committed APG race_trait record: `Half-Orc ~ Plagueborn`. It was withheld under \
             decisions.md §39, blocked on race_resolver.rs's ALTERNATE_TRAIT_REPLACE_FLAGS table \
             (§36 instance 15) -- shipping the record without that table row would have offered \
             it in the picker and refused it at character-save time. SD-29's race-trait extend \
             lane landed both halves together (SD-29 decisions.md §44.3), so the assertion moves \
             from 'deliberately 0' to 'exactly the 1 real record'"
        );
    }
}
