//! SD-20 Epic 7 (Level Up grant model) wire-fixture parity test.
//!
//! Reads `tests/fixtures/wire/sd20/level_up_parity.json` and asserts the
//! real `level_up::compute_level_up_grants` seam's output matches the
//! fixture's `expected_output` exactly, on the fields that matter (grant
//! name fragment, source-table provenance, and effect value -- see the
//! fixture's own `shape_note` field for why free-text grant descriptions
//! are deliberately not captured here). See
//! `tests/sd20_spellbook_parity.rs`'s module doc comment for the shared
//! fixture-shape decision this file and its five siblings all follow.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use codex::rules_core::character_input::{
    AbilityScores, ActiveState, CharacterClassLevel, CharacterInput, ChosenCharacterState,
    EquipmentSelection, SelectedChoice, SkillAllocation,
};
use codex::rules_core::level_up::compute_level_up_grants;

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

    fn as_i64(&self) -> i64 {
        match self {
            Json::Number(n) => *n as i64,
            _ => panic!("expected a JSON number, got {self:?}"),
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
    path.push("tests/fixtures/wire/sd20/level_up_parity.json");
    let raw = fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read fixture at {path:?}: {err}"));
    parse_json(&raw)
}

fn character_input_from_fixture(character_json: &Json) -> CharacterInput {
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
    let selected_feats = character_json
        .get("selected_feats")
        .as_array()
        .iter()
        .map(|entry| entry.as_str().to_owned())
        .collect();
    let skill_allocations = character_json
        .get("skill_allocations")
        .as_array()
        .iter()
        .map(|entry| SkillAllocation {
            skill_id: entry.get("skill_id").as_str().to_owned(),
            ranks: entry.get("ranks").as_i64() as u8,
        })
        .collect();
    let equipment_selections = character_json
        .get("equipment_selections")
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
                applied_modifiers: Vec::new(),
            }
        })
        .collect();
    let selected_choices = character_json
        .get("selected_choices")
        .as_array()
        .iter()
        .map(|entry| SelectedChoice {
            choice_set_id: entry.get("choice_set_id").as_str().to_owned(),
            selection_id: entry.get("selection_id").as_str().to_owned(),
        })
        .collect();

    CharacterInput {
        case_id: Some("sd20_level_up_parity".to_string()),
        source_package_id: "sd20_level_up_parity".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: character_json.get("race_id").as_str().to_owned(),
            class_levels,
            ability_scores,
            selected_feats,
            skill_allocations,
            equipment_selections,
            selected_choices,
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

#[test]
fn level_up_fighter_1_to_2_parity_fixture_round_trips_through_the_real_engine() {
    let fixture = load_fixture();
    let input_json = fixture.get("input");
    let input = character_input_from_fixture(input_json.get("character"));
    let from_level = input_json.get("from_level").as_i64() as u8;
    let to_level = input_json.get("to_level").as_i64() as u8;

    let plan = compute_level_up_grants(&input, from_level, to_level);

    let expected = fixture.get("expected_output");
    let expected_features = expected.get("automatic_features").as_array();
    assert_eq!(
        plan.automatic_features.len(),
        expected_features.len(),
        "automatic_features length must match the fixture: {:?}",
        plan.automatic_features
    );

    for (actual_grant, expected_grant) in plan.automatic_features.iter().zip(expected_features.iter()) {
        let name_contains = expected_grant.get("name_contains").as_str();
        assert!(
            actual_grant.name.contains(name_contains),
            "expected grant name {:?} to contain {name_contains:?}",
            actual_grant.name
        );
        let expected_source_table = expected_grant.get("source_table");
        assert_eq!(actual_grant.source_table.table, expected_source_table.get("table").as_str());
        assert_eq!(actual_grant.source_table.row_key, expected_source_table.get("row_key").as_str());
        assert_eq!(actual_grant.source_table.column_key, expected_source_table.get("column_key").as_str());
        assert_eq!(actual_grant.effects.len(), 1, "every grant in this fixture carries exactly one effect");
        assert_eq!(actual_grant.effects[0].value, expected_grant.get("effects_value").as_i64() as i16);
    }

    assert_eq!(
        plan.resource_pool_change.pools.len(),
        expected.get("resource_pool_change_pools").as_array().len()
    );
    assert_eq!(plan.capstone_threshold, expected.get("capstone_threshold").as_bool());
}
