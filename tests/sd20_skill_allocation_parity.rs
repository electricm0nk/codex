//! SD-20 Epic 4 (skill-rank allocation engine) wire-fixture parity test.
//!
//! Reads `tests/fixtures/wire/sd20/skill_allocation_parity.json` and
//! asserts the real `skill_allocation::allocate_skill_ranks` seam's output
//! matches the fixture's `expected_output` exactly. See
//! `tests/sd20_spellbook_parity.rs`'s module doc comment (and the
//! fixture's own `shape_note` field) for the shared fixture-shape decision
//! this file and its five siblings all follow.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState, SkillAllocation,
};
use codex::rules_core::skill_allocation::allocate_skill_ranks;

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

    fn as_object(&self) -> &BTreeMap<String, Json> {
        match self {
            Json::Object(map) => map,
            _ => panic!("expected a JSON object, got {self:?}"),
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
    path.push("tests/fixtures/wire/sd20/skill_allocation_parity.json");
    let raw = fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read fixture at {path:?}: {err}"));
    parse_json(&raw)
}

#[test]
fn skill_allocation_parity_fixture_round_trips_through_the_real_engine() {
    let fixture = load_fixture();
    let character_json = fixture.get("input").get("character");

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
    let skill_allocations = character_json
        .get("skill_allocations")
        .as_array()
        .iter()
        .map(|entry| SkillAllocation {
            skill_id: entry.get("skill_id").as_str().to_owned(),
            ranks: entry.get("ranks").as_i64() as u8,
        })
        .collect();

    let input = CharacterInput {
        case_id: Some("sd20_skill_allocation_parity".to_string()),
        source_package_id: "sd20_skill_allocation_parity".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: character_json.get("race_id").as_str().to_owned(),
            class_levels,
            ability_scores,
            selected_feats: Vec::new(),
            skill_allocations,
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    };

    let totals = allocate_skill_ranks(&input);

    let expected = fixture.get("expected_output");
    let expected_class_skills: Vec<String> = expected
        .get("class_skills")
        .as_array()
        .iter()
        .map(|entry| entry.as_str().to_owned())
        .collect();
    assert_eq!(totals.class_skills, expected_class_skills);
    assert_eq!(totals.cross_class_penalty_applied, expected.get("cross_class_penalty_applied").as_bool());

    for (skill_id, expected_total) in expected.get("totals").as_object() {
        let actual = totals
            .totals
            .get(skill_id)
            .unwrap_or_else(|| panic!("expected totals to contain {skill_id}"));
        assert_eq!(actual.ranks, expected_total.get("ranks").as_i64() as u8, "{skill_id}.ranks");
        assert_eq!(actual.ability_modifier, expected_total.get("ability_modifier").as_i64() as i8, "{skill_id}.ability_modifier");
        assert_eq!(actual.class_skill_bonus, expected_total.get("class_skill_bonus").as_i64() as i8, "{skill_id}.class_skill_bonus");
        assert_eq!(actual.misc_modifier, expected_total.get("misc_modifier").as_i64() as i8, "{skill_id}.misc_modifier");
        assert_eq!(actual.total_modifier, expected_total.get("total_modifier").as_i64() as i8, "{skill_id}.total_modifier");
    }
    assert_eq!(totals.totals.len(), expected.get("totals").as_object().len(), "totals must have exactly the fixture's entries");

    for (skill_id, expected_value) in expected.get("untrained_use").as_object() {
        assert_eq!(
            totals.untrained_use.get(skill_id).copied(),
            Some(expected_value.as_i64() as i8),
            "untrained_use[{skill_id}]"
        );
    }
    assert_eq!(totals.untrained_use.len(), expected.get("untrained_use").as_object().len());

    let expected_diagnostics = expected.get("diagnostics").as_array();
    assert_eq!(totals.diagnostics.len(), expected_diagnostics.len(), "diagnostics length must match the fixture");
    for (actual, expected_diag) in totals.diagnostics.iter().zip(expected_diagnostics.iter()) {
        assert_eq!(actual.id, expected_diag.get("id").as_str());
        assert_eq!(actual.claim_blocking, expected_diag.get("claim_blocking").as_bool());
        assert_eq!(actual.message, expected_diag.get("message").as_str());
    }
}
