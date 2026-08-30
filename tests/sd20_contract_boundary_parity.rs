//! SD-20 Epic 1 (boundary contract): first wire-fixture parity JSON for
//! the boundary contract itself — cycle 4, closing Epic 1.
//!
//! Per `SD-20-rules-engine-completeness-loop-instruction.md` Step 5 and
//! `technical-design.md` §1.2, wire-fixture parity tests are golden JSON
//! files at `tests/fixtures/wire/sd20/<criterion>.json` shaped as
//! `{ "name", "input", "expected_output", "expected_diagnostics" }`. This
//! is Epic 1's fourth and final work-unit (after `CharacterInput` types,
//! `PilotReceipt` types, and the printed-sheet cell map): the first parity
//! fixture that exercises the *whole* boundary-contract round trip —
//! `classify_character_input` -> the existing corpus-aware compute seam
//! (`compute_pilot_with_corpus`) -> `to_pilot_receipt` ->
//! `printed_sheet_cell_map` — reading a real on-disk JSON file and
//! asserting the engine's live output matches the fixture's golden
//! `expected_output` exactly.
//!
//! This crate has no `serde`/`serde_json` dependency (`Cargo.toml` has an
//! empty `[dependencies]` table), and adding one is out of this cycle's
//! file-touch partition (Epic 1 cycles touch `src/rules_core/contract.rs`,
//! `docs/release/SD-20/boundary-contract.md`, and `tests/fixtures/wire/sd20/*.json`
//! plus `tests/sd20_<criterion>.rs` only — not `Cargo.toml`). So this test
//! file carries a small self-contained JSON reader (object/array/string/
//! number/bool/null) built on `std` alone, used only to parse this
//! fixture and walk its `Json` value tree; it is not a general-purpose
//! library and is scoped to this test file.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState,
};
use codex::rules_core::contract::{
    classify_character_input, printed_sheet_cell_map, to_pilot_receipt, PrintedSheetCellValue,
};
use codex::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

// --- minimal std-only JSON reader (test-scoped; see module doc comment) ---

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
        assert_eq!(
            got,
            Some(expected),
            "expected byte {:?} at position {}, got {:?}",
            expected as char,
            self.pos,
            got.map(|b| b as char)
        );
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
    assert_eq!(
        parser.pos,
        parser.bytes.len(),
        "unexpected trailing content after top-level JSON value"
    );
    value
}

fn load_fixture() -> Json {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/fixtures/wire/sd20/boundary_contract_parity.json");
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read fixture at {path:?}: {err}"));
    parse_json(&raw)
}

fn empty_corpus() -> SourcePackageContent<'static> {
    let source_ref = SourceRef {
        lst_file: "sd20_contract_boundary_parity".to_string(),
        line: 1,
    };
    SourcePackageContent::empty("sd20_contract_boundary_parity", source_ref)
}

/// Build the engine's `CharacterInput` from the fixture's `input` section —
/// read from the on-disk JSON, not hand-duplicated Rust literals, so this
/// test genuinely proves the fixture's wire shape round-trips into the
/// engine's real input type.
fn character_input_from_fixture(input: &Json) -> CharacterInput {
    let class_levels = input
        .get("class_levels")
        .as_array()
        .iter()
        .map(|entry| CharacterClassLevel {
            class_id: entry.get("class_id").as_str().to_owned(),
            level: entry.get("level").as_i64() as u8,
        })
        .collect();

    let ability_scores_json = input.get("ability_scores");
    let ability_scores = AbilityScores {
        strength: ability_scores_json.get("strength").as_i64() as i16,
        dexterity: ability_scores_json.get("dexterity").as_i64() as i16,
        constitution: ability_scores_json.get("constitution").as_i64() as i16,
        intelligence: ability_scores_json.get("intelligence").as_i64() as i16,
        wisdom: ability_scores_json.get("wisdom").as_i64() as i16,
        charisma: ability_scores_json.get("charisma").as_i64() as i16,
    };

    CharacterInput {
        case_id: Some(input.get("case_id").as_str().to_owned()),
        source_package_id: input.get("source_package_id").as_str().to_owned(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: input.get("race_id").as_str().to_owned(),
            class_levels,
            ability_scores,
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

#[test]
fn boundary_contract_parity_fixture_round_trips_end_to_end() {
    let fixture = load_fixture();
    let input_json = fixture.get("input");

    // The fixture's `input` section names an entirely brand-new,
    // no-selections fighter: empty arrays for feats/skills/equipment/
    // choices/spells. If a future edit of the fixture adds any of those
    // selections, this assertion catches the drift before the (currently
    // hard-coded empty) `character_input_from_fixture` silently drops
    // them on the floor.
    for empty_selection_key in [
        "selected_feats",
        "skill_allocations",
        "equipment_selections",
        "selected_choices",
        "spells_selected",
    ] {
        assert!(
            input_json.get(empty_selection_key).as_array().is_empty(),
            "fixture's input.{empty_selection_key} must stay empty for this test's input builder to stay accurate; extend character_input_from_fixture before adding entries"
        );
    }

    let input = character_input_from_fixture(input_json);

    // classify_character_input -> ...
    let permutation = classify_character_input(&input);
    let expected_permutation = fixture.get("expected_permutation").as_str();
    assert_eq!(
        format!("{permutation:?}"),
        expected_permutation,
        "classify_character_input permutation must match the fixture's expected_permutation"
    );

    // ... -> the existing corpus-aware compute seam -> to_pilot_receipt ...
    let corpus = empty_corpus();
    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);

    let expected_chassis = fixture.get("expected_output").get("chassis");
    let expected_ability_modifiers = expected_chassis.get("ability_modifiers");
    assert_eq!(
        receipt.chassis.ability_modifiers.strength,
        expected_ability_modifiers.get("strength").as_i64() as i16
    );
    assert_eq!(
        receipt.chassis.ability_modifiers.dexterity,
        expected_ability_modifiers.get("dexterity").as_i64() as i16
    );
    assert_eq!(
        receipt.chassis.ability_modifiers.constitution,
        expected_ability_modifiers.get("constitution").as_i64() as i16
    );
    assert_eq!(
        receipt.chassis.ability_modifiers.intelligence,
        expected_ability_modifiers.get("intelligence").as_i64() as i16
    );
    assert_eq!(
        receipt.chassis.ability_modifiers.wisdom,
        expected_ability_modifiers.get("wisdom").as_i64() as i16
    );
    assert_eq!(
        receipt.chassis.ability_modifiers.charisma,
        expected_ability_modifiers.get("charisma").as_i64() as i16
    );
    assert_eq!(
        receipt.chassis.base_attack_bonus,
        expected_chassis.get("base_attack_bonus").as_i64() as i16
    );
    let expected_base_saves = expected_chassis.get("base_saves");
    assert_eq!(
        receipt.chassis.base_saves.fortitude,
        expected_base_saves.get("fortitude").as_i64() as i16
    );
    assert_eq!(
        receipt.chassis.base_saves.reflex,
        expected_base_saves.get("reflex").as_i64() as i16
    );
    assert_eq!(
        receipt.chassis.base_saves.will,
        expected_base_saves.get("will").as_i64() as i16
    );
    assert_eq!(
        receipt.chassis.baseline_melee_attack_bonus,
        expected_chassis.get("baseline_melee_attack_bonus").as_i64() as i16
    );
    assert_eq!(
        receipt.chassis.baseline_armor_class,
        expected_chassis.get("baseline_armor_class").as_i64() as i16
    );
    let expected_total_saves = expected_chassis.get("total_saves");
    assert_eq!(
        receipt.chassis.total_saves.fortitude,
        expected_total_saves.get("fortitude").as_i64() as i16
    );
    assert_eq!(
        receipt.chassis.total_saves.reflex,
        expected_total_saves.get("reflex").as_i64() as i16
    );
    assert_eq!(
        receipt.chassis.total_saves.will,
        expected_total_saves.get("will").as_i64() as i16
    );
    let expected_selected_skill_modifiers = expected_chassis.get("selected_skill_modifiers");
    assert_eq!(
        receipt.chassis.selected_skill_modifiers.climb,
        expected_selected_skill_modifiers.get("climb").as_i64() as i16
    );
    assert_eq!(
        receipt.chassis.selected_skill_modifiers.intimidate,
        expected_selected_skill_modifiers.get("intimidate").as_i64() as i16
    );
    assert_eq!(
        receipt.chassis.selected_skill_modifiers.swim,
        expected_selected_skill_modifiers.get("swim").as_i64() as i16
    );

    let expected_corpus_derived = fixture.get("expected_output").get("corpus_derived");
    assert!(
        receipt.corpus_derived.school_coverage.is_empty(),
        "expected empty school_coverage for this fixture's no-spells input"
    );
    assert!(
        match expected_corpus_derived.get("school_coverage") {
            Json::Object(map) => map.is_empty(),
            other => panic!("expected school_coverage to be a JSON object, got {other:?}"),
        },
        "fixture's expected_output.corpus_derived.school_coverage must itself be empty"
    );
    assert!(
        receipt.corpus_derived.equipped_items.is_empty(),
        "expected empty equipped_items for this fixture's no-equipment input"
    );
    assert!(
        expected_corpus_derived.get("equipped_items").as_array().is_empty(),
        "fixture's expected_output.corpus_derived.equipped_items must itself be empty"
    );

    // ... -> printed_sheet_cell_map, the full boundary-contract round trip.
    let cells = printed_sheet_cell_map(&receipt);
    let expected_cells = fixture.get("expected_output").get("cells").as_array();
    assert_eq!(
        cells.len(),
        expected_cells.len(),
        "printed_sheet_cell_map must produce exactly the cells the fixture names, in the same order"
    );
    for (cell, expected_cell) in cells.iter().zip(expected_cells.iter()) {
        assert_eq!(cell.cell_id, expected_cell.get("cell_id").as_str());
        assert_eq!(cell.source_field, expected_cell.get("source_field").as_str());
        let expected_value = expected_cell.get("value");
        match expected_value.get("kind").as_str() {
            "Number" => {
                let expected_number = expected_value.get("value").as_i64() as i16;
                assert_eq!(
                    cell.value,
                    PrintedSheetCellValue::Number(expected_number),
                    "cell {} expected Number({expected_number})",
                    cell.cell_id
                );
            }
            "Blocked" => {
                assert_eq!(
                    cell.value,
                    PrintedSheetCellValue::Blocked,
                    "cell {} expected Blocked",
                    cell.cell_id
                );
            }
            other => panic!("unrecognized expected cell value kind: {other:?}"),
        }
    }

    // Diagnostics: the fixture names every claim_blocking: true diagnostic
    // the engine must produce for this input (technical-design.md §1.2:
    // "expected_diagnostics: list of claim_blocking: true diagnostics if
    // any"). Assert an exact set match on ids, not just a subset check, so
    // a future engine change that silently drops or adds a blocking
    // diagnostic for this exact input fails this parity test.
    let expected_blocking_ids: BTreeSet<String> = fixture
        .get("expected_diagnostics")
        .as_array()
        .iter()
        .map(|entry| {
            assert!(
                entry.get("claim_blocking").as_bool(),
                "expected_diagnostics entries must all be claim_blocking: true per technical-design.md §1.2"
            );
            entry.get("id").as_str().to_owned()
        })
        .collect();
    let actual_blocking_ids: BTreeSet<String> = receipt
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.claim_blocking)
        .map(|diagnostic| diagnostic.id.clone())
        .collect();
    assert_eq!(
        actual_blocking_ids, expected_blocking_ids,
        "the receipt's claim_blocking diagnostics must exactly match the fixture's expected_diagnostics"
    );
}
