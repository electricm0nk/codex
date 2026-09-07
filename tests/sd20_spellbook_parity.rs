//! SD-20 Epic 2 (spellbook engine) wire-fixture parity test.
//!
//! Reads `tests/fixtures/wire/sd20/spellbook_parity.json` and asserts the
//! real `spellbook::compute_spellbook_coverage` seam's output matches the
//! fixture's `expected_output` exactly, on the fields that matter. This is
//! one of the "at least 8 wire-fixture parity JSON fixtures" the loop
//! instruction's and `acceptance-and-verification.md` gate 3's closure
//! definition require (one per epic + the boundary contract itself) — see
//! the fixture's own `shape_note` field for the documented shape decision
//! this file and its five siblings (`tests/sd20_feat_prereqs_parity.rs`,
//! `tests/sd20_skill_allocation_parity.rs`,
//! `tests/sd20_equipment_effects_parity.rs`,
//! `tests/sd20_damage_total_parity.rs`, `tests/sd20_level_up_parity.rs`)
//! all follow: Epics 2-7's real compute seams are not wired into
//! `PilotReceipt` (per `tests/sd20_tabletop_readiness_integration.rs`'s
//! Finding 1), so this fixture captures the epic's own real
//! input -> output round trip directly, not a `CharacterInput` ->
//! `PilotReceipt` pair.
//!
//! Uses the same minimal std-only JSON reader
//! `tests/sd20_contract_boundary_parity.rs` established (this crate has no
//! `serde`/`serde_json` dependency and `Cargo.toml` is outside this
//! fixture's file-touch scope).

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use codex::pcgen_import::ir_converter::convert_spell_record;
use codex::pcgen_import::lst_parser::spell::parse_lst_spell_row;
use codex::rules_core::character_input::{
    AbilityScores, AcquisitionMode, CharacterClassLevel, CharacterInput, ChosenCharacterState,
    SpellSelection,
};
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};
use codex::rules_core::spellbook::compute_spellbook_coverage;

// --- minimal std-only JSON reader (test-scoped; see module doc comment) ---

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(BTreeMap<String, Json>),
}

impl Json {
    fn get(&self, key: &str) -> &Json {
        match self {
            Json::Object(map) => map
                .get(key)
                .unwrap_or_else(|| panic!("expected JSON object to have key {key:?}, got {self:?}")),
            _ => panic!("expected a JSON object to read key {key:?}, got {self:?}"),
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Json::String(s) => s,
            _ => panic!("expected a JSON string, got {self:?}"),
        }
    }

    fn as_f64(&self) -> f64 {
        match self {
            Json::Number(n) => *n,
            _ => panic!("expected a JSON number, got {self:?}"),
        }
    }

    fn as_i64(&self) -> i64 {
        self.as_f64() as i64
    }

    fn as_array(&self) -> &[Json] {
        match self {
            Json::Array(items) => items,
            _ => panic!("expected a JSON array, got {self:?}"),
        }
    }
}

struct JsonParser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> JsonParser<'a> {
    fn new(input: &'a str) -> Self {
        JsonParser {
            bytes: input.as_bytes(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn bump(&mut self) -> Option<u8> {
        let byte = self.peek();
        if byte.is_some() {
            self.pos += 1;
        }
        byte
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(b) if b.is_ascii_whitespace()) {
            self.pos += 1;
        }
    }

    fn expect(&mut self, expected: u8) {
        let got = self.bump();
        assert_eq!(got, Some(expected), "expected byte {:?} at position {}, got {:?}", expected as char, self.pos, got.map(|b| b as char));
    }

    fn parse_value(&mut self) -> Json {
        self.skip_ws();
        match self.peek() {
            Some(b'{') => self.parse_object(),
            Some(b'[') => self.parse_array(),
            Some(b'"') => Json::String(self.parse_string()),
            Some(b't') => {
                self.expect_literal("true");
                Json::Bool(true)
            }
            Some(b'f') => {
                self.expect_literal("false");
                Json::Bool(false)
            }
            Some(b'n') => {
                self.expect_literal("null");
                Json::Null
            }
            Some(b) if b == b'-' || b.is_ascii_digit() => self.parse_number(),
            other => panic!("unexpected byte at position {}: {:?}", self.pos, other),
        }
    }

    fn expect_literal(&mut self, literal: &str) {
        for expected in literal.bytes() {
            self.expect(expected);
        }
    }

    fn parse_object(&mut self) -> Json {
        self.expect(b'{');
        let mut map = BTreeMap::new();
        self.skip_ws();
        if self.peek() == Some(b'}') {
            self.bump();
            return Json::Object(map);
        }
        loop {
            self.skip_ws();
            let key = self.parse_string();
            self.skip_ws();
            self.expect(b':');
            let value = self.parse_value();
            map.insert(key, value);
            self.skip_ws();
            match self.bump() {
                Some(b',') => continue,
                Some(b'}') => break,
                other => panic!("expected ',' or '}}' in object, got {other:?}"),
            }
        }
        Json::Object(map)
    }

    fn parse_array(&mut self) -> Json {
        self.expect(b'[');
        let mut items = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b']') {
            self.bump();
            return Json::Array(items);
        }
        loop {
            let value = self.parse_value();
            items.push(value);
            self.skip_ws();
            match self.bump() {
                Some(b',') => continue,
                Some(b']') => break,
                other => panic!("expected ',' or ']' in array, got {other:?}"),
            }
        }
        Json::Array(items)
    }

    fn parse_string(&mut self) -> String {
        self.skip_ws();
        self.expect(b'"');
        let mut out = String::new();
        loop {
            match self.bump() {
                Some(b'"') => break,
                Some(b'\\') => match self.bump() {
                    Some(b'"') => out.push('"'),
                    Some(b'\\') => out.push('\\'),
                    Some(b'/') => out.push('/'),
                    Some(b'n') => out.push('\n'),
                    Some(b't') => out.push('\t'),
                    Some(b'r') => out.push('\r'),
                    other => panic!("unsupported escape sequence: {other:?}"),
                },
                Some(b) => out.push(b as char),
                None => panic!("unterminated JSON string"),
            }
        }
        out
    }

    fn parse_number(&mut self) -> Json {
        let start = self.pos;
        if self.peek() == Some(b'-') {
            self.bump();
        }
        while matches!(self.peek(), Some(b) if b.is_ascii_digit() || b == b'.' || b == b'e' || b == b'E' || b == b'+' || b == b'-')
        {
            self.bump();
        }
        let text = std::str::from_utf8(&self.bytes[start..self.pos]).expect("valid utf8 number");
        Json::Number(text.parse().expect("valid JSON number"))
    }
}

fn parse_json(input: &str) -> Json {
    let mut parser = JsonParser::new(input);
    let value = parser.parse_value();
    parser.skip_ws();
    assert_eq!(parser.pos, parser.bytes.len(), "unexpected trailing content after top-level JSON value");
    value
}

fn load_fixture() -> Json {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/fixtures/wire/sd20/spellbook_parity.json");
    let raw = fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read fixture at {path:?}: {err}"));
    parse_json(&raw)
}

#[test]
fn spellbook_abjuration_parity_fixture_round_trips_through_the_real_engine() {
    let fixture = load_fixture();
    let input_json = fixture.get("input");

    // Build the corpus from the fixture's raw LST-shaped row, the same way
    // tests/sd20_spellbook_abjuration.rs does.
    let corpus_json = input_json.get("corpus_spell");
    let raw_line = corpus_json.get("raw_lst_row").as_str();
    let parsed = parse_lst_spell_row("sd20_spellbook_parity", 1, raw_line);
    let record = parsed.record.expect("fixture's raw_lst_row must parse into a real LstSpellRecord");
    let source_ref = SourceRef {
        lst_file: "sd20_spellbook_parity".to_string(),
        line: 1,
    };
    let mut corpus = SourcePackageContent::empty("sd20_spellbook_parity", source_ref);
    let record: &'static _ = Box::leak(Box::new(record));
    corpus.push(convert_spell_record(record));

    // Build the CharacterInput from the fixture's own JSON, not a
    // hand-duplicated Rust literal.
    let character_json = input_json.get("character");
    let ability_scores_json = character_json.get("ability_scores");
    let ability_scores = AbilityScores {
        strength: ability_scores_json.get("strength").as_i64() as i16,
        dexterity: ability_scores_json.get("dexterity").as_i64() as i16,
        constitution: ability_scores_json.get("constitution").as_i64() as i16,
        intelligence: ability_scores_json.get("intelligence").as_i64() as i16,
        wisdom: ability_scores_json.get("wisdom").as_i64() as i16,
        charisma: ability_scores_json.get("charisma").as_i64() as i16,
    };
    let class_levels = character_json
        .get("class_levels")
        .as_array()
        .iter()
        .map(|entry| CharacterClassLevel {
            class_id: entry.get("class_id").as_str().to_owned(),
            level: entry.get("level").as_i64() as u8,
        })
        .collect();
    let spells_selected = character_json
        .get("spells_selected")
        .as_array()
        .iter()
        .map(|entry| SpellSelection {
            spell_id: entry.get("spell_id").as_str().to_owned(),
            source_class_id: entry.get("source_class_id").as_str().to_owned(),
            acquisition_mode: match entry.get("acquisition_mode").as_str() {
                "Prepared" => AcquisitionMode::Prepared,
                "Known" => AcquisitionMode::Known,
                "Granted" => AcquisitionMode::Granted,
                other => panic!("unrecognized acquisition_mode: {other:?}"),
            },
        })
        .collect();

    let input = CharacterInput {
        case_id: Some("sd20_spellbook_parity".to_string()),
        source_package_id: "sd20_spellbook_parity".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: character_json.get("race_id").as_str().to_owned(),
            class_levels,
            ability_scores,
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            spells_selected,
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    };

    let coverage = compute_spellbook_coverage(&input, &corpus);

    let expected = fixture.get("expected_output");
    let expected_prepared = expected.get("spells_prepared").as_array();
    assert_eq!(
        coverage.spells_prepared.len(),
        expected_prepared.len(),
        "spells_prepared length must match the fixture"
    );
    let prepared = &coverage.spells_prepared[0];
    let expected_first = &expected_prepared[0];
    assert_eq!(prepared.source_class_id, expected_first.get("source_class_id").as_str());
    let expected_effect = expected_first.get("effect");
    assert_eq!(prepared.effect.spell_id, expected_effect.get("spell_id").as_str());
    assert_eq!(format!("{:?}", prepared.effect.school), expected_effect.get("school").as_str());
    assert_eq!(prepared.effect.level, expected_effect.get("level").as_i64() as u8);
    assert_eq!(prepared.effect.effect_text, expected_effect.get("effect_text").as_str());
    let expected_table_cell = expected_effect.get("table_cell");
    assert_eq!(prepared.effect.table_cell.table, expected_table_cell.get("table").as_str());
    assert_eq!(prepared.effect.table_cell.row_key, expected_table_cell.get("row_key").as_str());

    assert!(
        coverage.spells_known.is_empty(),
        "fixture's expected_output.spells_known is empty"
    );

    let expected_dc = expected.get("spell_save_dc");
    if let Json::Object(map) = expected_dc {
        for (class_id, expected_value) in map {
            assert_eq!(
                coverage.spell_save_dc.get(class_id).copied(),
                Some(expected_value.as_i64() as u8),
                "spell_save_dc[{class_id}] must match the fixture"
            );
        }
    } else {
        panic!("expected spell_save_dc to be a JSON object");
    }

    let expected_bonus_slots = expected.get("bonus_slots_from_ability");
    if let Json::Object(map) = expected_bonus_slots {
        assert_eq!(
            coverage.bonus_slots_from_ability.len(),
            map.len(),
            "bonus_slots_from_ability must have exactly the fixture's entries"
        );
        for (level_str, expected_value) in map {
            let level: u8 = level_str.parse().expect("bonus_slots_from_ability key must be a level number");
            assert_eq!(
                coverage.bonus_slots_from_ability.get(&level).copied(),
                Some(expected_value.as_i64() as u8),
                "bonus_slots_from_ability[{level}] must match the fixture"
            );
        }
    } else {
        panic!("expected bonus_slots_from_ability to be a JSON object");
    }
}
