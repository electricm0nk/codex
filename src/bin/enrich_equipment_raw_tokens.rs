//! Book-agnostic Shape B v1 enrichment: populates `raw_tokens`/
//! `raw_bonus_chains` on every existing on-disk equipment record.
//!
//! Discovered 2026-07-30 while wiring the desktop app's real corpus loading
//! (`docs/release/v0.6/book-agnostic-backend-gaps-scoping.md` finding 4):
//! every book's equipment codegen pipeline has independently evolved (CRB
//! reads from a hand-curated 2,977-entry static table, APG/ACG/Bestiary
//! from their own pre-compiled tables using a `weight` field name instead
//! of CRB's `weight_lbs`, ARG/PU parse raw LST directly) -- retrofitting
//! each one individually to also capture raw mechanical tokens would mean
//! touching and re-verifying up to 6 different pipelines with their own
//! histories and, as this file's own first version proved the hard way,
//! their own field-name divergences.
//!
//! Every Shape B v1 record already carries an exact citation
//! (`source.path` + `source.line`, an `LstToken`-kind source) back to its
//! real PCGen LST source line -- regardless of which pipeline produced it.
//! This tool uses that citation directly: re-parse the cited raw LST file,
//! find the record whose `header_line_number` matches `source.line`, and
//! add `raw_tokens`/`raw_bonus_chains` keys onto the on-disk JSON's `data`
//! object.
//!
//! **Deliberately operates on raw `serde_json::Value`, never a typed Rust
//! struct.** The first version of this tool deserialized into
//! `CorpusRecordV1<EquipmentCacheData>` and re-serialized the whole record
//! -- which silently *dropped* every field that struct doesn't know about
//! (real, caught-in-review data loss: APG/ACG/Bestiary's `weight` field --
//! a different name than CRB's `weight_lbs` -- and PU's `equip_type`/
//! `plus` fields all vanished on the first run, reverted before commit).
//! Operating on `Value` and only ever inserting the 2 new keys means every
//! book-specific field this tool doesn't know about survives untouched,
//! by construction, regardless of what else diverges between books' schemas.
//!
//! Records whose `source.kind` is not `"lst_token"` (a `web_second_source`
//! or `same_book_fallback` record -- no raw LST line to enrich from) are
//! left untouched and counted separately, not treated as an error.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, BonusToken, EquipmentRecord, EquipmentToken};
use codex::rules_core::pi_screening::{classify_field, declared_product_identity, DeclaredProductIdentity};
use codex::rules_core::shape_b_v1::{License, REDACTED_PI_MARKER};
use serde_json::{json, Value};

/// The identity a `.COPY=<name>` row's first column names as its base --
/// the string before `.COPY=`. `None` for a plain row. Mirrors `gen_
/// equipment_gap_tables.rs`'s own `.COPY=` split exactly (same literal
/// PCGen syntax, one predicate, so the two tools agree on what a `.COPY=`
/// row's base identity is).
fn copy_base_identity(line: &str) -> Option<&str> {
    let first = line.split('\t').next().unwrap_or("");
    first.split_once(".COPY=").map(|(base, _)| base)
}

/// Resolves a `.COPY=` row's base identity to the PLAIN (non-`.COPY=`)
/// `EquipmentRecord` that declares it -- the record whose own header line's
/// `KEY:` token (or, absent one, whose bare first-column text) equals the
/// identity. Never matches a `.COPY=`-declared record (at most one hop,
/// mirrors `gen_equipment_gap_tables.rs`'s `collect_base_fields` and
/// `corpus_literal_sweep`'s `Sweep::copy_base_row` exactly, so all three
/// tools agree on "the base").
fn find_copy_base<'a>(entries: &'a [EquipmentRecord], identity: &str) -> Option<&'a EquipmentRecord> {
    entries.iter().find(|r| {
        let first = r.header_raw_line.split('\t').next().unwrap_or("").trim();
        if first.contains(".COPY=") {
            return false;
        }
        match r.tokens_on_line(r.header_line_number).into_iter().find(|t| t.key == "KEY") {
            Some(t) => t.value == identity,
            None => first == identity,
        }
    })
}

/// `§53.5`'s declared-PI reader, applied directly to a raw `.lst` row's own
/// tab-separated fields -- **not** `EquipmentRecord::tokens_on_line`, whose
/// `KNOWN_TAGS` allowlist (`pcgen_import::lst_parser::equipment`) does not
/// include `NAMEISPI:`/`DESCISPI:` at all, so those declarations never
/// reach the parsed `EquipmentToken` list this tool otherwise reads from.
/// Splits every tab-delimited field on its first `:` (the same rule
/// `gen_equipment_gap_tables.rs`'s own `declared_pi_at` uses) and hands the
/// whole set to the shared [`declared_product_identity`] primitive, so the
/// two tools agree on what a row declares.
fn declared_pi_on_line(line_text: &str) -> DeclaredProductIdentity {
    let tokens: Vec<(&str, &str)> = line_text.split('\t').filter_map(|field| field.split_once(':')).collect();
    declared_product_identity(tokens)
}

/// PI-screen one `raw_tokens` field value before it ships: a declared
/// `DESCISPI:YES` covering `DESC`-keyed fields, unioned with a blacklist
/// term scan ([`classify_field`]) over EVERY field regardless of key --
/// SD-30 `§52.3`/`§53.5`, byte-identical union contract to
/// `enrich_monster_ability_raw_tokens.rs`'s function of the same name.
///
/// **Fixes a confirmed wave-12 PI exposure**: this tool previously wrote
/// `raw_tokens` completely unscreened while `gen_equipment_gap_tables.rs`
/// correctly redacted the same record's `description` field -- 28
/// `inner_sea_gods` records shipped a blacklisted deity/place name verbatim
/// in `raw_tokens` (e.g. `cloak_of_the_night_sky.json`'s `DESC` token
/// naming "Desna" three times and a `SPELLS:...|PREDEITY:1,Desna` field)
/// under an affirmative `license: "OGL"`, `pi_field: null`. Both contracts
/// now run on every token this tool writes, not just name/description.
fn screen_field_value(key: &str, value: &str, declared_description: bool) -> String {
    if key.eq_ignore_ascii_case("DESC") && declared_description {
        return REDACTED_PI_MARKER.to_string();
    }
    let (license, ..) = classify_field(key, value);
    if license == License::PiRedacted {
        return REDACTED_PI_MARKER.to_string();
    }
    value.to_string()
}

fn pcgen_data_root() -> PathBuf {
    if let Ok(v) = env::var("PCGEN_DATA_ROOT") {
        return PathBuf::from(v);
    }
    let home = env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
    PathBuf::from(home).join("workspace/repos/pcgen/data")
}

fn find_equipment_json_files(book_dir: &Path) -> Vec<PathBuf> {
    let equipment_dir = book_dir.join("equipment");
    let mut out = Vec::new();
    if !equipment_dir.is_dir() {
        return out;
    }
    let mut stack = vec![equipment_dir];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
                out.push(path);
            }
        }
    }
    out
}

enum Outcome {
    Enriched,
    NoLstCitation,
    AlreadyEnriched,
    CitationMiss(String),
    MergedEntryMismatch(String),
    DroppedPi(String),
}

fn enrich_one(path: &Path, data_root: &Path) -> Outcome {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    let mut root: Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("parse {path:?} as JSON: {e}"));

    {
        let data = root.get("data").unwrap_or_else(|| panic!("{path:?}: no top-level \"data\" object"));
        if data.get("raw_tokens").is_some() || data.get("raw_bonus_chains").is_some() {
            return Outcome::AlreadyEnriched;
        }
    }

    let source = root["source"].clone();
    if source.get("kind").and_then(Value::as_str) != Some("lst_token") {
        return Outcome::NoLstCitation;
    }
    let lst_rel_path = source["path"].as_str().expect("lst_token source must carry a path").to_string();
    let line = source["line"].as_u64().expect("lst_token source must carry a line") as usize;
    let record_key = source.get("record_key").and_then(Value::as_str).unwrap_or("<unknown>").to_string();

    let lst_full_path = data_root.join(&lst_rel_path);
    let Ok(lst_text) = fs::read_to_string(&lst_full_path) else {
        return Outcome::CitationMiss(format!("cited LST file not found: {lst_full_path:?}"));
    };
    let parsed = parse_equipment_entries(&lst_rel_path, &lst_text);
    let Some(raw_record) = parsed.entries.iter().find(|e| e.header_line_number == line) else {
        return Outcome::CitationMiss(format!("no record at {lst_rel_path}:{line} (record_key={record_key:?})"));
    };

    // `OPEN-ISSUES.md` row 61, ROOT-CAUSED (`SD31-E6-F5-003`): `open_record`'s
    // same-name merge (this parser's own documented handling of PCGen
    // restating one logical item across multiple lines) is deliberate and
    // correct for its designed case, but nothing about "these rows merged
    // into one logical record" says which of the merged rows is the
    // SPECIFIC one this corpus citation (`source.line`) points at.  Three
    // real shipped records proved a caller that takes the whole merged
    // `raw_record.tokens`/`bonus_chains` ships tokens the CITED line never
    // states: `bastard_s_sting` (line 447, a bare `.COPY=` row, shipped an
    // unrelated `EQMOD:Material ~ Steel` and a duplicated `VISIBLE:YES`
    // pulled in from a DIFFERENT `.COPY=` variant sharing the same base
    // template), `mountain_pattern_armor` (every token doubled from a
    // second, near-identical restated row at line 46) and `hunter_s_stand`
    // (three genuinely distinct `.COPY=` items' tokens merged into one).
    //
    // Fix: `EquipmentRecord::tokens_on_line`/`bonus_chains_on_line`
    // (`lst_parser::equipment`) filter the merged record's own token list
    // down to exactly the tokens whose OWN `line_number` is this citation's
    // `line` -- every `EquipmentToken` already carries that field, so this
    // is a pure, zero-risk filter with no change to `open_record`'s merge
    // behavior (which OTHER callers still rely on for genuine multi-line
    // restatements). The byte-present-on-cited-line guard below is kept as
    // a second, independent proof of the same invariant (defense in depth,
    // now expected to always pass by construction) rather than removed.
    let cited_line_text = lst_text.lines().nth(line.saturating_sub(1)).unwrap_or("");
    let line_tokens = raw_record.tokens_on_line(line);
    let line_bonus_chains = raw_record.bonus_chains_on_line(line);
    for token in &line_tokens {
        let rendered = format!("{}:{}", token.key, token.value);
        if !cited_line_text.contains(&rendered) {
            return Outcome::MergedEntryMismatch(format!(
                "{lst_rel_path}:{line} (record_key={record_key:?}): token {rendered:?} not byte-present on the cited line -- likely a same-name merge with a different row"
            ));
        }
    }

    // `SD31-E6-F6-001`: when the cited line is ITSELF a `.COPY=` declaration,
    // fold in the base record's own tokens too -- otherwise `raw_tokens`
    // (this exact field) is the ONLY provenance surface
    // `sd27_equipment_modifier_price_matches_corpus_cost_token.rs` and other
    // callers check, and a genuinely inherited, corpus-real `cost_gp`/
    // `weight_lbs`/`description` (`gen_equipment_gap_tables.rs`'s own `.COPY=`
    // inheritance) would read as fabricated -- a real value with no token to
    // justify it in the ONE field meant to justify it. Resolved by the
    // IDENTICAL `KEY:`-or-bare-name rule the generator and `corpus_literal_
    // sweep` already use, so all three tools agree on "the base". At most
    // one hop: `find_copy_base` never matches another `.COPY=` row.
    let mut all_tokens: Vec<&EquipmentToken> = line_tokens;
    let mut all_bonus_chains: Vec<&BonusToken> = line_bonus_chains;
    if let Some(base_identity) = copy_base_identity(cited_line_text)
        && let Some(base_record) = find_copy_base(&parsed.entries, base_identity)
    {
        let base_line = base_record.header_line_number;
        let base_line_text = lst_text.lines().nth(base_line.saturating_sub(1)).unwrap_or("");
        let base_tokens = base_record.tokens_on_line(base_line);
        for token in &base_tokens {
            let rendered = format!("{}:{}", token.key, token.value);
            if !base_line_text.contains(&rendered) {
                return Outcome::MergedEntryMismatch(format!(
                    "{lst_rel_path}:{line} (record_key={record_key:?}): inherited base token \
                     {rendered:?} not byte-present on the base's own line {base_line} -- \
                     refusing to ship an unprovable inheritance"
                ));
            }
        }
        all_tokens.extend(base_tokens);
        all_bonus_chains.extend(base_record.bonus_chains_on_line(base_line));
    }

    // PI screen, over the WHOLE closure (cited line + inherited `.COPY=`
    // base line, when one was folded in above) -- SD-30 `§52.3`/`§53.5`,
    // mirrors `enrich_monster_ability_raw_tokens.rs`'s identical closure-wide
    // read. A name cannot be redacted (decisions.md §50.3): drop the whole
    // enrichment rather than ship a `raw_tokens` array whose byte-identity
    // to the corpus would betray a name the generator already excluded from
    // `description`/`name`.
    let mut declared = declared_pi_on_line(cited_line_text);
    if let Some(base_identity) = copy_base_identity(cited_line_text)
        && let Some(base_record) = find_copy_base(&parsed.entries, base_identity)
    {
        let base_line_text = lst_text.lines().nth(base_record.header_line_number.saturating_sub(1)).unwrap_or("");
        let base_declared = declared_pi_on_line(base_line_text);
        declared.name = declared.name || base_declared.name;
        declared.description = declared.description || base_declared.description;
    }
    if declared.name {
        return Outcome::DroppedPi(format!(
            "{lst_rel_path}:{line} (record_key={record_key:?}) declares NAMEISPI:YES in its own \
             closure -- a name cannot be redacted, refusing to write raw_tokens for it"
        ));
    }

    // `SD31-E6-F10-004`: use the byte-exact value split from `raw_pair`,
    // never the TRIMMED `t.value` -- real corpus reproduction,
    // `inner_sea_gods/isg_equip.lst:220`'s `Safecamp Wagon`, whose `DESC:`
    // field carries a literal trailing space before the line's own end.
    // `t.value` strips it (correct for every OTHER caller that wants a
    // clean value); this is the ONE call site whose whole job is a
    // byte-exact citation, and `corpus_literal_sweep`'s own `tab_tokens`
    // does not trim the corpus side (`whitespace_is_not_normalised_away`),
    // so shipping the trimmed value here made the two sides of that byte
    // comparison disagree over incidental formatting whitespace neither
    // side treats as content. `raw_pair`'s own doc comment already
    // promises the byte-exact field text; this is the fix reaching its
    // one real consumer.
    let raw_tokens: Vec<Value> = all_tokens
        .iter()
        .map(|t| {
            let value = t.raw_pair.split_once(':').map(|(_, v)| v).unwrap_or(t.value.as_str());
            let stored = screen_field_value(&t.key, value, declared.description);
            json!({ "key": t.key, "value": stored })
        })
        .collect();
    // Same union screen applied to each bonus chain's qualifiers, joined as
    // one value for the blacklist scan (a chain is mechanical by shape, but
    // a `TYPE=`/`PREVAREQ:` qualifier is still free text nothing stops from
    // naming a deity or place, and the same "never assume a shape is safe"
    // principle applies as for `raw_tokens`).
    let raw_bonus_chains: Vec<Value> = all_bonus_chains
        .iter()
        .map(|b| {
            let joined = b.qualifiers.join("|");
            let (license, ..) = classify_field("BONUS", &joined);
            let qualifiers: Vec<String> = if license == License::PiRedacted {
                vec![REDACTED_PI_MARKER.to_string()]
            } else {
                b.qualifiers.clone()
            };
            json!({ "qualifiers": qualifiers })
        })
        .collect();

    let data_obj = root
        .get_mut("data")
        .and_then(Value::as_object_mut)
        .expect("\"data\" must be a JSON object");
    data_obj.insert("raw_tokens".to_string(), Value::Array(raw_tokens));
    data_obj.insert("raw_bonus_chains".to_string(), Value::Array(raw_bonus_chains));

    let new_json = serde_json::to_string_pretty(&root).expect("serialize enriched record");
    fs::write(path, new_json + "\n").unwrap_or_else(|e| panic!("write {path:?}: {e}"));
    Outcome::Enriched
}

fn main() {
    let data_root = pcgen_data_root();
    let corpus_root = PathBuf::from("data/corpus");
    let books = [
        "core_rulebook",
        "advanced_players_guide",
        "advanced_class_guide",
        "beastiary",
        "advanced_race_guide",
        "pathfinder_unchained",
        // SD-31 SD31-E6-F5-001: Ultimate Equipment's cache
        // (`data/corpus/ultimate_equipment/equipment/*.json`,
        // `gen_cache_ultimate_equipment`) is real `lst_token` records now
        // and needs the same `raw_tokens`/`raw_bonus_chains` enrichment
        // every other equipment book gets -- omitting it here would
        // silently leave every new UE record at its thin
        // KEY:-token-only fallback (`corpus_loader.rs`'s
        // `equipment_record_from_json`), never wiring a single
        // `BONUS:STAT` effect despite the cache existing on disk.
        "ultimate_equipment",
        // SD-31 SD31-E6-F5-002: `gen_cache_equipment_gap` (`cache_gen::
        // equipment_gap`) landed real `lst_token` equipment/equipment_modifier
        // records for these 4 books' `not-ingested` gap residue this cycle --
        // same reasoning as the `ultimate_equipment` entry above, these need
        // the same enrichment pass or their new records stay at the thin
        // KEY:-token-only fallback.
        "ultimate_combat",
        "ultimate_intrigue",
        "ultimate_psionics",
        "ultimate_wilderness",
        // SD-31 SD31-E6-F5-003: `gen_cache_hand_authored_equipment`
        // (`cache_gen::hand_authored_equipment`) landed 620 real
        // `lst_token` equipment records this cycle across Ultimate
        // Psionics/Combat/Intrigue/Magic's already-compiled but never-
        // dumped `equipment_tables()`. The first three books were already
        // in this list (their `equipmods` residue landed via
        // `SD31-E6-F5-002`); Ultimate Magic was not -- add it now or its
        // 18 new records stay at the thin KEY:-token-only fallback, same
        // reasoning as every entry above.
        "ultimate_magic",
        // SD31-E6-F10-003: `gen_cache_equipment_gap` extended to 8 further
        // already-compiled books this cycle (none has a hand-authored
        // `equipment_tables` module; every row comes from the gap lane,
        // same shape as the `ultimate_*` entries above) -- same reasoning:
        // omitting them here leaves their new records at the thin
        // KEY:-token-only fallback and, critically, never eligible for
        // `corpus_literal_sweep`'s `literal-verified` done rung, which is
        // the reason this list exists at all.
        "occult_adventures",
        "horror_adventures",
        "inner_sea_races",
        "inner_sea_world_guide",
        "monster_codex",
        "bestiary_2",
        "bestiary_3",
        "bestiary_4",
        // SD31-E6-F10-004: `gen_equipment_gap_tables` extended to 5 further
        // already-compiled books this cycle (`OPEN-ISSUES.md` row 186's own
        // named follow-on -- a per-record blacklist pre-filter now reaches
        // them without weakening the whole-file hard stop). Same reasoning
        // as every entry above: omitting them here leaves their new records
        // at the thin KEY:-token-only fallback and ineligible for
        // `corpus_literal_sweep`'s `literal-verified` done rung.
        "inner_sea_gods",
        "mythic_adventures",
        "inner_sea_combat",
        "inner_sea_intrigue",
        "book_of_the_damned_volume_2",
    ];

    let mut total_enriched = 0u32;
    let mut total_no_citation = 0u32;
    let mut total_already = 0u32;
    let mut total_dropped_pi = 0u32;
    let mut misses: Vec<String> = Vec::new();
    let mut merged_entry_mismatches: Vec<String> = Vec::new();
    let mut dropped_pi: Vec<String> = Vec::new();

    for book in books {
        let book_dir = corpus_root.join(book);
        if !book_dir.is_dir() {
            continue;
        }
        let files = find_equipment_json_files(&book_dir);
        let mut book_enriched = 0u32;
        for file in &files {
            match enrich_one(file, &data_root) {
                Outcome::Enriched => {
                    total_enriched += 1;
                    book_enriched += 1;
                }
                Outcome::NoLstCitation => total_no_citation += 1,
                Outcome::AlreadyEnriched => total_already += 1,
                Outcome::CitationMiss(msg) => misses.push(format!("{}: {}", file.display(), msg)),
                Outcome::MergedEntryMismatch(msg) => {
                    merged_entry_mismatches.push(format!("{}: {}", file.display(), msg))
                }
                Outcome::DroppedPi(msg) => {
                    total_dropped_pi += 1;
                    dropped_pi.push(format!("{}: {}", file.display(), msg));
                }
            }
        }
        eprintln!("{book}: {} equipment files scanned, {book_enriched} enriched", files.len());
    }

    eprintln!(
        "\nenrich_equipment_raw_tokens: {total_enriched} enriched, {total_no_citation} no-LST-citation (untouched), {total_already} already-enriched, {total_dropped_pi} skipped (declared NAMEISPI:YES), {} citation misses, {} merged-entry mismatches (left un-enriched)",
        misses.len(),
        merged_entry_mismatches.len()
    );
    if !misses.is_empty() {
        eprintln!("\nCitation misses (not enriched, real gaps to investigate):");
        for miss in &misses {
            eprintln!("  {miss}");
        }
    }
    if !merged_entry_mismatches.is_empty() {
        eprintln!(
            "\nMerged-entry mismatches (parse_equipment_entries's same-name merge pulled in a \
             token from a DIFFERENT row than the one cited -- left un-enriched, OPEN-ISSUES.md row 48/49):"
        );
        for mismatch in &merged_entry_mismatches {
            eprintln!("  {mismatch}");
        }
    }
    if !dropped_pi.is_empty() {
        eprintln!("\nSkipped for declared Product Identity (raw_tokens NOT written for these):");
        for d in &dropped_pi {
            eprintln!("  {d}");
        }
    }
}

#[cfg(test)]
mod copy_base_tests {
    use super::*;

    /// The real corpus shape (`Exclusionary_AMF`): the cited line is a bare
    /// `.COPY=` declaration with no `COST:` of its own; the base row two
    /// lines away states the real value. `find_copy_base` must resolve it
    /// by the base's `KEY:` token, not by any bare-name coincidence.
    #[test]
    fn find_copy_base_resolves_by_key_token() {
        let text = "Exclusionary\t\tKEY:Special Ability ~ Exclusionary ~ Amulet of Mighty Fists\t\tCOST:3750\n\
                     Special Ability ~ Exclusionary ~ Amulet of Mighty Fists.COPY=Exclusionary_AMF\t\tVISIBLE:NO\n";
        let parsed = parse_equipment_entries("test.lst", text);
        let base = find_copy_base(
            &parsed.entries,
            "Special Ability ~ Exclusionary ~ Amulet of Mighty Fists",
        )
        .expect("base record must resolve");
        let cost = base
            .tokens_on_line(base.header_line_number)
            .into_iter()
            .find(|t| t.key == "COST")
            .expect("base must carry its own COST: token");
        assert_eq!(cost.value, "3750");
    }

    /// A `.COPY=` row is never itself matched as a base -- proves
    /// inheritance is at most one hop, mirroring `gen_equipment_gap_
    /// tables.rs`'s and `corpus_literal_sweep`'s identical restriction.
    #[test]
    fn find_copy_base_never_matches_another_copy_row() {
        let text = "Base\t\tKEY:X\t\tCOST:1\n\
                     X.COPY=Mid\t\tVISIBLE:NO\n\
                     Mid.COPY=Leaf\t\tVISIBLE:NO\n";
        let parsed = parse_equipment_entries("test.lst", text);
        assert!(
            find_copy_base(&parsed.entries, "Mid").is_none(),
            "Mid is itself a .COPY= row (X.COPY=Mid) and must never serve as a base"
        );
    }

    /// `copy_base_identity` mirrors the identical split every sibling tool
    /// (`gen_equipment_gap_tables.rs`, `corpus_literal_sweep`) uses.
    #[test]
    fn copy_base_identity_splits_on_the_literal_marker() {
        assert_eq!(
            copy_base_identity("Special Ability ~ Answering ~ Weapon.COPY=Answering\t\tVISIBLE:NO"),
            Some("Special Ability ~ Answering ~ Weapon")
        );
        assert_eq!(copy_base_identity("Plain Record\t\tCOST:5"), None);
    }
}

#[cfg(test)]
mod pi_screen_tests {
    use super::*;

    /// `declared_pi_on_line` must read `NAMEISPI:`/`DESCISPI:` directly off
    /// the raw tab-separated line -- `EquipmentToken`'s `KNOWN_TAGS`
    /// allowlist does not include either tag, so a caller that read from
    /// the parsed `EquipmentToken` list instead would never see a
    /// declaration at all.
    #[test]
    fn declared_pi_on_line_reads_nameispi_and_descispi_off_the_raw_line() {
        let d = declared_pi_on_line("Belkzen Battle Standard\t\tCOST:34000\tNAMEISPI:YES\n");
        assert!(d.name);
        assert!(!d.description);

        let d = declared_pi_on_line("Ordinary Banner\t\tCOST:100\tDESC:A plain banner.\tDESCISPI:YES\n");
        assert!(!d.name);
        assert!(d.description);

        let d = declared_pi_on_line("Masterwork Backpack\t\tCOST:5\n");
        assert!(!d.name);
        assert!(!d.description);
    }

    /// **Wave-12 fix, mutation-proof**: `screen_field_value` must redact
    /// ANY token key whose value contains a blacklist term -- the exact
    /// shape the confirmed PI exposure found (`SPELLS` and `DESC` tokens on
    /// `inner_sea_gods` records shipping "Desna" verbatim in `raw_tokens`
    /// while `description` was already correctly redacted). Reproduced
    /// verbatim from the real corpus row.
    #[test]
    fn screen_field_value_redacts_a_blacklist_term_hit_on_any_key_not_just_desc() {
        assert_eq!(
            screen_field_value(
                "SPELLS",
                "Cloak of the Night Sky|TIMES=1|CASTERLEVEL=5|Longstrider|Flare,10|PREDEITY:1,Desna",
                false
            ),
            REDACTED_PI_MARKER
        );
        assert_eq!(
            screen_field_value(
                "DESC",
                "If Desna is the wearer's patron, the cloak grants a bonus.",
                false
            ),
            REDACTED_PI_MARKER
        );
        assert_eq!(screen_field_value("COST", "2500", false), "2500", "a clean value must pass through unchanged");
    }

    /// A `DESC` field with no blacklist term still redacts when the row's
    /// own `DESCISPI:YES` declares it -- the declared-PI half of the union,
    /// distinct from the blacklist-scan half tested above.
    #[test]
    fn screen_field_value_redacts_desc_when_declared_even_without_a_blacklist_hit() {
        assert_eq!(
            screen_field_value("DESC", "A perfectly ordinary sentence naming no deity.", true),
            REDACTED_PI_MARKER
        );
    }

    /// **MUTATION PROOF**: this test drives `enrich_one`, the real
    /// production function, end to end against a throwaway corpus -- not a
    /// hand-rolled restatement (the exact shape wave-12 review found and
    /// fixed in `gen_equipment_gap_tables.rs`'s sibling test). Reproduces
    /// the confirmed defect: a `DESC` token naming a blacklisted deity, on
    /// a book with no `NAMEISPI:`/`DESCISPI:` declaration at all (the
    /// undeclared shape the declared-PI reader alone cannot catch).
    struct Scratch {
        data_root: PathBuf,
        corpus_root: PathBuf,
    }

    impl Scratch {
        fn new(name: &str) -> Self {
            let base = std::env::temp_dir()
                .join(format!("codex_enrich_equipment_raw_tokens_pi_{name}_{}", std::process::id()));
            let _ = fs::remove_dir_all(&base);
            let data_root = base.join("pcgen_data");
            let corpus_root = base.join("data_corpus");
            fs::create_dir_all(data_root.join("pathfinder/paizo/campaign_setting/x_book")).unwrap();
            fs::create_dir_all(corpus_root.join("x_book/equipment")).unwrap();
            Scratch { data_root, corpus_root }
        }

        fn write_lst(&self, contents: &str) {
            fs::write(self.data_root.join("pathfinder/paizo/campaign_setting/x_book/x_equip.lst"), contents).unwrap();
        }

        fn write_json(&self, name: &str, contents: &str) -> PathBuf {
            let path = self.corpus_root.join("x_book/equipment").join(name);
            fs::write(&path, contents).unwrap();
            path
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(self.data_root.parent().unwrap());
        }
    }

    #[test]
    fn enrich_one_redacts_an_undeclared_blacklist_hit_in_raw_tokens() {
        let scratch = Scratch::new("undeclared");
        scratch.write_lst(
            "Cloak Of The Night Sky\t\tKEY:Cloak Of The Night Sky\t\tCOST:2500\tWT:1\t\
             DESC:If Desna is the wearer's patron, this cloak grants extra power.\n",
        );
        let json = r#"{
  "completeness": "full",
  "data": { "key": "Cloak Of The Night Sky", "name": "Cloak Of The Night Sky", "category": "General", "cost_gp": 2500.0, "weight_lbs": 1.0, "description": "[redacted PI]" },
  "source": { "kind": "lst_token", "path": "pathfinder/paizo/campaign_setting/x_book/x_equip.lst", "line": 1, "record_key": "Cloak Of The Night Sky", "sha256": "x" },
  "license": "PI-REDACTED", "pi_field": "description", "pi_marker": "redacted",
  "population": "in_scope", "wiring_class": "static", "wiring_class_signals": [], "ingested_at": "2026-01-01T00:00:00Z"
}"#;
        let path = scratch.write_json("cloak_of_the_night_sky.json", json);

        let outcome = enrich_one(&path, &scratch.data_root);
        assert!(matches!(outcome, Outcome::Enriched));

        let written: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        let raw_tokens = written["data"]["raw_tokens"].as_array().unwrap();
        let desc_tok = raw_tokens.iter().find(|t| t["key"] == "DESC").expect("DESC token must be present");
        assert_eq!(
            desc_tok["value"], REDACTED_PI_MARKER,
            "an undeclared blacklisted deity name in raw_tokens must be redacted, not shipped verbatim \
             (this is the confirmed wave-12 exposure: description was already redacted while raw_tokens shipped it unscreened)"
        );
        // The un-redacted deity name must not survive anywhere in the written file.
        let raw = serde_json::to_string(&written).unwrap();
        assert!(!raw.contains("Desna"), "the deity name must not appear anywhere in the enriched output");
    }

    /// A record whose own corpus row declares `NAMEISPI:YES` must not be
    /// enriched at all (raw_tokens withheld) -- a name cannot be redacted.
    #[test]
    fn enrich_one_skips_enrichment_for_a_declared_nameispi_record() {
        let scratch = Scratch::new("declared_name");
        scratch.write_lst("Belkzen Battle Standard\t\tKEY:Belkzen Battle Standard\t\tCOST:34000\tNAMEISPI:YES\n");
        let json = r#"{
  "completeness": "full",
  "data": { "key": "Belkzen Battle Standard", "name": "Belkzen Battle Standard", "category": "General", "cost_gp": 34000.0, "weight_lbs": null, "description": null },
  "source": { "kind": "lst_token", "path": "pathfinder/paizo/campaign_setting/x_book/x_equip.lst", "line": 1, "record_key": "Belkzen Battle Standard", "sha256": "x" },
  "license": "OGL", "pi_field": null, "pi_marker": null,
  "population": "in_scope", "wiring_class": "static", "wiring_class_signals": [], "ingested_at": "2026-01-01T00:00:00Z"
}"#;
        let path = scratch.write_json("belkzen_battle_standard.json", json);

        let outcome = enrich_one(&path, &scratch.data_root);
        assert!(matches!(outcome, Outcome::DroppedPi(_)));

        let written: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert!(written["data"]["raw_tokens"].is_null(), "raw_tokens must not be written for a declared NAMEISPI:YES record");
    }
}
