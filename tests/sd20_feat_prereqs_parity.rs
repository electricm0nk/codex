//! SD-20 Epic 3 (feat prerequisite engine) wire-fixture parity test.
//!
//! Reads `tests/fixtures/wire/sd20/feat_prereqs_parity.json` and asserts
//! the real `feat_prereqs::{evaluate_feat_prerequisites, compute_feat_effects}`
//! seam's output matches the fixture's `expected_output` exactly. See
//! `tests/sd20_spellbook_parity.rs`'s module doc comment (and the
//! fixture's own `shape_note` field) for the shared fixture-shape decision
//! this file and its five siblings all follow.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use codex::rules_core::feat_prereqs::{compute_feat_effects, evaluate_feat_prerequisites, FeatKey};
use codex::rules_core::rules_tables::crb::feats::FeatCategory;

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
    path.push("tests/fixtures/wire/sd20/feat_prereqs_parity.json");
    let raw = fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read fixture at {path:?}: {err}"));
    parse_json(&raw)
}

fn category_from_str(name: &str) -> FeatCategory {
    match name {
        "General" => FeatCategory::General,
        "Combat" => FeatCategory::Combat,
        "ItemCreation" => FeatCategory::ItemCreation,
        "Metamagic" => FeatCategory::Metamagic,
        other => panic!("unrecognized FeatCategory: {other:?}"),
    }
}

#[test]
fn feat_prereqs_general_parity_fixture_round_trips_through_the_real_engine() {
    let fixture = load_fixture();
    let expected_feats = fixture.get("expected_output").get("feats").as_array();

    for expected_feat in expected_feats {
        let feat_id = expected_feat.get("feat_id").as_str();
        let category_str = fixture
            .get("input")
            .get("feats")
            .as_array()
            .iter()
            .find(|entry| entry.get("feat_id").as_str() == feat_id)
            .expect("every expected_output feat must have a matching input.feats entry")
            .get("category")
            .as_str();
        let key = FeatKey {
            feat_id: feat_id.to_string(),
            category: category_from_str(category_str),
        };

        let evaluation = evaluate_feat_prerequisites(&key);
        let expected_evaluation = expected_feat.get("evaluation");
        assert_eq!(
            evaluation.is_eligible,
            expected_evaluation.get("is_eligible").as_bool(),
            "{feat_id}: is_eligible must match the fixture"
        );
        let expected_failing = expected_evaluation.get("failing_prerequisites").as_array();
        assert_eq!(
            evaluation.failing_prerequisites.len(),
            expected_failing.len(),
            "{feat_id}: failing_prerequisites length must match the fixture"
        );
        for (actual, expected) in evaluation.failing_prerequisites.iter().zip(expected_failing.iter()) {
            assert_eq!(actual.reason, expected.as_str(), "{feat_id}: failing_prerequisites reason must match the fixture");
        }

        let effects = compute_feat_effects(&key);
        let expected_effects = expected_feat.get("effects");
        assert_eq!(effects.feat_id, expected_effects.get("feat_id").as_str());
        let expected_description = expected_effects.get("description");
        if expected_description.is_null() {
            assert!(effects.description.is_none(), "{feat_id}: description must be None per the fixture");
        } else {
            assert_eq!(effects.description.as_deref(), Some(expected_description.as_str()));
        }
        let expected_table_cell = expected_effects.get("table_cell");
        if expected_table_cell.is_null() {
            assert!(effects.table_cell.is_none(), "{feat_id}: table_cell must be None per the fixture");
        } else {
            let table_cell = effects.table_cell.as_ref().expect("expected a real TableCellRef");
            assert_eq!(table_cell.table, expected_table_cell.get("table").as_str());
            assert_eq!(table_cell.row_key, expected_table_cell.get("row_key").as_str());
        }
    }
}
