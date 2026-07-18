//! SD-20 Epic 5 (equipment-effect engine) wire-fixture parity test.
//!
//! Reads `tests/fixtures/wire/sd20/equipment_effects_parity.json` and
//! asserts the real `equipment_effects::compute_equipment_effects` seam's
//! output matches the fixture's `expected_output` exactly. See
//! `tests/sd20_spellbook_parity.rs`'s module doc comment (and the
//! fixture's own `shape_note` field) for the shared fixture-shape decision
//! this file and its five siblings all follow.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
use codex::rules_core::character_input::{ActiveState, EquipmentSelection};
use codex::rules_core::equipment_effects::compute_equipment_effects;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

// --- minimal std-only JSON reader (test-scoped; see sd20_spellbook_parity.rs) ---

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

    fn as_bool(&self) -> bool {
        match self {
            Json::Bool(b) => *b,
            _ => panic!("expected a JSON bool, got {self:?}"),
        }
    }

    fn is_null(&self) -> bool {
        matches!(self, Json::Null)
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
    path.push("tests/fixtures/wire/sd20/equipment_effects_parity.json");
    let raw = fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read fixture at {path:?}: {err}"));
    parse_json(&raw)
}

fn corpus_from(raw_rows: &[Json]) -> SourcePackageContent<'static> {
    let text: String = raw_rows.iter().map(|row| format!("{}\n", row.as_str())).collect();
    let result = parse_equipment_entries("cr_equip_arms_armor.lst", &text);
    assert!(result.diagnostics.is_empty(), "fixture text must parse cleanly: {:?}", result.diagnostics);
    let source_ref = SourceRef {
        lst_file: "cr_equip_arms_armor.lst".to_string(),
        line: 1,
    };
    let mut corpus = SourcePackageContent::empty("core_rulebook", source_ref);
    for record in result.entries {
        let record: &'static EquipmentRecord = Box::leak(Box::new(record));
        corpus.push(convert_equipment_record(record));
    }
    corpus
}

fn expected_f32(value: &Json) -> Option<f32> {
    if value.is_null() {
        None
    } else {
        Some(value.as_f64() as f32)
    }
}

fn expected_i16(value: &Json) -> Option<i16> {
    if value.is_null() {
        None
    } else {
        Some(value.as_i64() as i16)
    }
}

#[test]
fn equipment_effects_arms_armor_parity_fixture_round_trips_through_the_real_engine() {
    let fixture = load_fixture();
    let input = fixture.get("input");
    let corpus = corpus_from(input.get("corpus_equipment").get("raw_lst_rows").as_array());

    let equipped: Vec<EquipmentSelection> = input
        .get("equipped")
        .as_array()
        .iter()
        .map(|entry| {
            let active_state = match entry.get("active_state").as_str() {
                "equipped_active" => ActiveState::EquippedActive,
                "absent" => ActiveState::Absent,
                "selected_inactive" => ActiveState::SelectedInactive,
                other => panic!("unrecognized active_state: {other:?}"),
            };
            EquipmentSelection {
                item_id: entry.get("item_id").as_str().to_owned(),
                equipped_or_active: matches!(active_state, ActiveState::EquippedActive),
                active_state,
            }
        })
        .collect();

    let effects = compute_equipment_effects(&equipped, &corpus);

    let expected = fixture.get("expected_output");
    let expected_per_item = expected.get("per_item").as_array();
    assert_eq!(effects.per_item.len(), expected_per_item.len(), "per_item length must match the fixture");

    for expected_item in expected_per_item {
        let item_id = expected_item.get("item_id").as_str();
        let actual = effects
            .per_item
            .iter()
            .find(|item| item.item_id == item_id)
            .unwrap_or_else(|| panic!("expected {item_id} to resolve"));
        assert_eq!(actual.equipment_record_key, expected_item.get("equipment_record_key").as_str());
        assert_eq!(format!("{:?}", actual.category), expected_item.get("category").as_str());
        assert_eq!(actual.armor_class_bonus, expected_i16(expected_item.get("armor_class_bonus")));
        assert_eq!(actual.max_dex, expected_i16(expected_item.get("max_dex")));
        assert_eq!(actual.spell_failure, expected_f32(expected_item.get("spell_failure")));
        assert_eq!(
            actual.table_cell.is_some(),
            expected_item.get("has_table_cell").as_bool(),
            "{item_id}: table_cell presence must match the fixture"
        );
    }

    assert_eq!(effects.armor_class_delta, expected.get("armor_class_delta").as_i64() as i16);
    assert_eq!(effects.max_dex_cap, expected_i16(expected.get("max_dex_cap")));
    assert_eq!(effects.spell_failure_chance, expected_f32(expected.get("spell_failure_chance")));
}
