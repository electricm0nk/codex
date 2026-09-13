//! Build-time producer for the desktop's bundled corpus-fixture package.
//!
//! SD-35 `AT-35-E6-003-RULED` cycle 9, `decisions.md` §11 ("not one line of
//! PCGen in our live code") and §19 (operator ruling B16: naming
//! `pcgen_import` in shipping code under a live root is a hit).
//!
//! ## What this closes
//!
//! The desktop crate bundles four real corpus records as a Tauri resource so
//! `compute_pilot_with_corpus` can be proved end-to-end in the live UI. Those
//! records shipped as **raw `.lst` row text**, which meant the shipping binary
//! parsed PCGen grammar and ran the converter over it at start-up:
//! `rules_core::corpus_loader::load_lst_fixture_corpus` called
//! `lst_parser::spell::parse_lst_spell_row`,
//! `lst_parser::equipment::parse_equipment_entries`,
//! `ir_converter::convert_spell_record` and
//! `ir_converter::convert_equipment_record`. Every Epic 6 census since cycle 1
//! named the clearing condition for those hits in the same words: *"clears when
//! that package is produced at build time and read as data."* This binary is
//! that producer.
//!
//! The `.txt` fixtures stay exactly where they are — they are this tool's
//! **input**, and `decisions.md` §11 keeps the converter, the parser and their
//! inputs. What changes is that the conversion happens here, once, at authoring
//! time, and the desktop ships the **converted** records.
//!
//! ## What it writes
//!
//! One `BookCorpusRoot`-shaped tree beside the fixtures themselves, so the live
//! side reads it with the same two loaders it already uses for the real corpus
//! and needs no fixture-specific loader at all:
//!
//! ```text
//! apps/desktop/src-tauri/resources/corpus_fixtures/
//!   spell_abjuration.txt          <- converter input, kept
//!   equip_longsword.txt           <- converter input, kept
//!   spell/spell_abjuration.json   <- converted, shipped, read as data
//!   equipment/equip_longsword.json
//! ```
//!
//! The equipment records are written in the same Shape B v1 `data.raw_tokens` /
//! `data.raw_bonus_chains` form the real `data/corpus/<book>/equipment/*.json`
//! records carry, because that is what `corpus_loader::load_equipment_corpus`
//! reads. The spell records are written as **converted** fields only — `key`,
//! `school`, `description` and the rest of the settled field set — with no
//! token array at all, because `corpus_loader::load_spell_corpus` has read
//! converted spell fields since cycle 8.
//!
//! ## Determinism
//!
//! `serde_json`'s object type is a `BTreeMap` in this build (no
//! `preserve_order` feature), so key order is sorted and byte-stable. `--check`
//! regenerates every file in memory and compares bytes, failing on any drift —
//! the same contract `sheet_rule_convert --check` carries.
//!
//! ## Usage
//!
//! ```text
//! cargo run --locked --bin gen_desktop_fixture_corpus            # write
//! cargo run --locked --bin gen_desktop_fixture_corpus -- --check # verify
//! ```

use std::fs;
use std::path::{Path, PathBuf};

use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
use codex::pcgen_import::lst_parser::spell::{parse_lst_spell_row, LstSpellRecord};
use serde_json::{json, Value};

const FIXTURE_ROOT: &str = "apps/desktop/src-tauri/resources/corpus_fixtures";
const SPELL_FIXTURES: &[&str] = &["spell_abjuration.txt", "spell_illusion.txt"];
const EQUIPMENT_FIXTURES: &[&str] = &["equip_longsword.txt", "equip_chain_shirt.txt"];

/// The fixture's single record line: the first line that is neither a `#`
/// comment nor blank. Same convention `tests/fixtures/rules_core/` uses and the
/// desktop crate used to apply itself.
fn record_line(fixture_text: &str) -> Option<&str> {
    fixture_text
        .lines()
        .find(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
}

/// The converted spell document for one parsed row.
///
/// Settled values only. The row's `SCHOOL:` / `CASTTIME:` column tags are
/// stripped by the parser and never reach this document — `school` is
/// `"Abjuration"`, `casting_time` is `"1 standard action"`. A field the row did
/// not declare is omitted rather than written as `null`, so the document states
/// what the source says and nothing else.
fn spell_document(label: &str, record: &LstSpellRecord) -> Value {
    let mut data = serde_json::Map::new();
    data.insert("key".to_string(), json!(record.name));
    let mut put = |name: &str, value: &Option<String>| {
        if let Some(value) = value {
            data.insert(name.to_string(), json!(value));
        }
    };
    put("school", &record.school);
    put("sub_school", &record.sub_school);
    put("descriptor", &record.descriptor);
    put("spell_type", &record.spell_type);
    put("classes", &record.classes);
    put("components", &record.components);
    put("casting_time", &record.casting_time);
    put("range", &record.range);
    put("item", &record.item);
    put("target_area", &record.target_area);
    put("duration", &record.duration);
    put("save_info", &record.save_info);
    put("spell_resistance", &record.spell_resistance);
    put("source_page", &record.source_page);
    put("output_name", &record.output_name);
    put("description", &record.description);

    json!({
        "population": "in_scope",
        "completeness": "full",
        "data": Value::Object(data),
        "source": {
            "kind": "desktop_ui_fixture",
            "path": format!("{FIXTURE_ROOT}/{label}"),
            "line": record.line_number,
            "record_key": record.name,
        },
        "license": "OGL",
    })
}

/// The equipment document for one parsed record.
///
/// Shape B v1, the same form `enrich_equipment_raw_tokens.rs` writes onto the
/// real corpus: `raw_tokens` carries the record's `KEY:VAL` pairs with the
/// **byte-exact** value split off `raw_pair` (never the trimmed `value` — the
/// real enrichment made that correction in `SD31-E6-F10-004` and this producer
/// must not reintroduce it), and `raw_bonus_chains` carries each `BONUS:`
/// clause as its pipe-delimited qualifier list. The parser routes every
/// `BONUS:` token into `bonus_chains` and never into `tokens`, so the two
/// arrays are disjoint by construction.
fn equipment_document(label: &str, record: &EquipmentRecord) -> Value {
    let raw_tokens: Vec<Value> = record
        .tokens
        .iter()
        .map(|token| {
            let value =
                token.raw_pair.split_once(':').map(|(_, v)| v).unwrap_or(token.value.as_str());
            json!({ "key": token.key, "value": value })
        })
        .collect();
    let raw_bonus_chains: Vec<Value> = record
        .bonus_chains
        .iter()
        .map(|bonus| json!({ "qualifiers": bonus.qualifiers }))
        .collect();

    // A record's corpus identity is its `KEY:` token when the row carried one
    // and its own name when it did not — the rule `equipment_id_resolve` and
    // `equipment_catalog_rows` both already apply.
    let key = record
        .tokens
        .iter()
        .find(|token| token.key == "KEY")
        .map(|token| token.value.clone())
        .unwrap_or_else(|| record.name.clone());
    let description = record
        .tokens
        .iter()
        .find(|token| token.key == "DESC")
        .map(|token| token.value.clone());

    let mut data = serde_json::Map::new();
    data.insert("key".to_string(), json!(key));
    data.insert("name".to_string(), json!(record.name));
    if let Some(description) = description {
        data.insert("description".to_string(), json!(description));
    }
    data.insert("raw_tokens".to_string(), Value::Array(raw_tokens));
    data.insert("raw_bonus_chains".to_string(), Value::Array(raw_bonus_chains));

    json!({
        "population": "in_scope",
        "completeness": "full",
        "data": Value::Object(data),
        "source": {
            "kind": "desktop_ui_fixture",
            "path": format!("{FIXTURE_ROOT}/{label}"),
            "line": record.header_line_number,
            "record_key": key,
        },
        "license": "OGL",
    })
}

struct Generated {
    path: PathBuf,
    text: String,
}

fn generate(root: &Path) -> Result<Vec<Generated>, String> {
    let mut out = Vec::new();

    for label in SPELL_FIXTURES {
        let text = fs::read_to_string(root.join(label))
            .map_err(|err| format!("could not read fixture '{label}': {err}"))?;
        let line = record_line(&text)
            .ok_or_else(|| format!("spell fixture '{label}' has no record line"))?;
        let parsed = parse_lst_spell_row(format!("{FIXTURE_ROOT}/{label}"), 1, line);
        let record = parsed
            .record
            .ok_or_else(|| format!("spell fixture '{label}' failed to parse: {line}"))?;
        out.push(Generated {
            path: root.join("spell").join(format!("{}.json", label.trim_end_matches(".txt"))),
            text: render(&spell_document(label, &record)),
        });
    }

    for label in EQUIPMENT_FIXTURES {
        let text = fs::read_to_string(root.join(label))
            .map_err(|err| format!("could not read fixture '{label}': {err}"))?;
        let line = record_line(&text)
            .ok_or_else(|| format!("equipment fixture '{label}' has no record line"))?;
        let result = parse_equipment_entries(&format!("{FIXTURE_ROOT}/{label}"), line);
        let entry = result
            .entries
            .first()
            .ok_or_else(|| format!("equipment fixture '{label}' produced no record: {line}"))?;
        if result.entries.len() != 1 {
            return Err(format!(
                "equipment fixture '{label}' produced {} records; one fixture file is one record",
                result.entries.len()
            ));
        }
        out.push(Generated {
            path: root.join("equipment").join(format!("{}.json", label.trim_end_matches(".txt"))),
            text: render(&equipment_document(label, entry)),
        });
    }

    Ok(out)
}

fn render(value: &Value) -> String {
    let mut text = serde_json::to_string_pretty(value).expect("serde_json cannot fail on a Value");
    text.push('\n');
    text
}

fn main() {
    let check = std::env::args().any(|arg| arg == "--check");
    let root = Path::new(FIXTURE_ROOT);
    if !root.is_dir() {
        eprintln!("gen_desktop_fixture_corpus: {FIXTURE_ROOT} is not a directory (run from the repo root)");
        std::process::exit(1);
    }

    let generated = match generate(root) {
        Ok(generated) => generated,
        Err(err) => {
            eprintln!("gen_desktop_fixture_corpus: {err}");
            std::process::exit(1);
        }
    };

    let mut drift = 0usize;
    let mut written = 0usize;
    for item in &generated {
        let on_disk = fs::read_to_string(&item.path).ok();
        if on_disk.as_deref() == Some(item.text.as_str()) {
            continue;
        }
        if check {
            drift += 1;
            eprintln!("DRIFT {}", item.path.display());
            continue;
        }
        if let Some(parent) = item.path.parent()
            && let Err(err) = fs::create_dir_all(parent)
        {
            eprintln!("gen_desktop_fixture_corpus: could not create {}: {err}", parent.display());
            std::process::exit(1);
        }
        if let Err(err) = fs::write(&item.path, &item.text) {
            eprintln!("gen_desktop_fixture_corpus: could not write {}: {err}", item.path.display());
            std::process::exit(1);
        }
        written += 1;
    }

    if check {
        println!(
            "gen_desktop_fixture_corpus --check: files={} drift={} verdict={}",
            generated.len(),
            drift,
            if drift == 0 { "PASS" } else { "FAIL" }
        );
        if drift > 0 {
            std::process::exit(1);
        }
    } else {
        println!(
            "gen_desktop_fixture_corpus: files={} written={} unchanged={}",
            generated.len(),
            written,
            generated.len() - written
        );
    }
}
