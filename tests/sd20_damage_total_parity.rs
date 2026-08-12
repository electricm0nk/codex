//! SD-20 Epic 6 (damage-total engine) wire-fixture parity test.
//!
//! Reads `tests/fixtures/wire/sd20/damage_total_parity.json` and asserts
//! the real `damage_total::{resolve_base_damage_dice,
//! resolve_str_damage_modifier, resolve_critical_threat_range,
//! resolve_critical_multiplier}` seams' output matches the fixture's
//! `expected_output` exactly, for the same real weapon (a Longsword),
//! covering base-dice, STR-modifier, critical-threat-range, and
//! critical-multiplier in one fixture. See
//! `tests/sd20_spellbook_parity.rs`'s module doc comment (and the
//! fixture's own `shape_note` field) for the shared fixture-shape decision
//! this file and its five siblings all follow.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
use codex::rules_core::damage_total::{
    resolve_base_damage_dice, resolve_critical_multiplier, resolve_critical_threat_range,
    resolve_str_damage_modifier, WeaponHandSlot,
};
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
    path.push("tests/fixtures/wire/sd20/damage_total_parity.json");
    let raw = fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read fixture at {path:?}: {err}"));
    parse_json(&raw)
}

fn corpus_from(raw_row: &str) -> SourcePackageContent<'static> {
    let text = format!("{raw_row}\n");
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

#[test]
fn damage_total_longsword_parity_fixture_round_trips_through_the_real_engine() {
    let fixture = load_fixture();
    let input = fixture.get("input");
    let corpus = corpus_from(input.get("corpus_equipment").get("raw_lst_row").as_str());
    let weapon_item_id = input.get("weapon_item_id").as_str();
    let str_modifier = input.get("str_modifier").as_i64() as i16;
    let hand = match input.get("hand").as_str() {
        "Primary" => WeaponHandSlot::Primary,
        "OffHand" => WeaponHandSlot::OffHand,
        other => panic!("unrecognized hand: {other:?}"),
    };

    let expected = fixture.get("expected_output");

    let base_dice = resolve_base_damage_dice(weapon_item_id, &corpus).expect("weapon must resolve real base damage dice");
    let expected_base_dice = expected.get("base_dice");
    assert_eq!(base_dice.weapon_record_key, expected_base_dice.get("weapon_record_key").as_str());
    let expected_dice = expected_base_dice.get("base_dice");
    assert_eq!(base_dice.base_dice.count, expected_dice.get("count").as_i64() as u8);
    assert_eq!(base_dice.base_dice.die_size, expected_dice.get("die_size").as_i64() as u8);
    assert_eq!(base_dice.table_cell.is_some(), expected_base_dice.get("has_table_cell").as_bool());

    let str_result = resolve_str_damage_modifier(weapon_item_id, &corpus, str_modifier, hand)
        .expect("weapon must resolve a real STR damage modifier");
    let expected_str = expected.get("str_modifier");
    assert_eq!(str_result.weapon_record_key, expected_str.get("weapon_record_key").as_str());
    assert_eq!(format!("{:?}", str_result.wield_category), expected_str.get("wield_category").as_str());
    assert_eq!(format!("{:?}", str_result.hand), expected_str.get("hand").as_str());
    assert_eq!(str_result.str_damage_modifier, expected_str.get("str_damage_modifier").as_i64() as i16);
    assert_eq!(str_result.table_cell.is_some(), expected_str.get("has_table_cell").as_bool());

    let crit_range = resolve_critical_threat_range(weapon_item_id, &corpus).expect("weapon must resolve a real critical threat range");
    let expected_crit_range = expected.get("critical_threat_range");
    assert_eq!(crit_range.weapon_record_key, expected_crit_range.get("weapon_record_key").as_str());
    let expected_range_pair = expected_crit_range.get("critical_threat_range").as_array();
    assert_eq!(
        crit_range.critical_threat_range,
        (expected_range_pair[0].as_i64() as u8, expected_range_pair[1].as_i64() as u8)
    );
    assert_eq!(crit_range.table_cell.is_some(), expected_crit_range.get("has_table_cell").as_bool());

    let crit_mult = resolve_critical_multiplier(weapon_item_id, &corpus).expect("weapon must resolve a real critical multiplier");
    let expected_crit_mult = expected.get("critical_multiplier");
    assert_eq!(crit_mult.weapon_record_key, expected_crit_mult.get("weapon_record_key").as_str());
    assert_eq!(crit_mult.critical_multiplier, expected_crit_mult.get("critical_multiplier").as_i64() as u8);
    assert_eq!(crit_mult.table_cell.is_some(), expected_crit_mult.get("has_table_cell").as_bool());
}
