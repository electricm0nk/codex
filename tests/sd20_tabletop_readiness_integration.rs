//! SD-20 Epic 8 (tabletop-readiness integration closure): the single
//! integration-closure fixture + test, per the loop instruction's Step 2
//! note ("not a cycle — the integration-closure epic is the single test
//! fixture + single integration test file. It lands in one slice") and
//! `SD-20-rules-engine-completeness-loop-instruction.md`'s "What
//! tabletop-readiness closure actually means for SD-20" §3 ("Epic 8
//! closed: the canonical tabletop scenario fixture
//! (`tests/fixtures/wire/sd20/human_fighter_level_1_tabletop.json`)
//! lands; the integration test (`tests/sd20_tabletop_readiness_integration.rs`)
//! passes; the engine produces a `PilotReceipt` whose every displayed
//! sheet cell matches the table cells referenced by `TableCellRef`s").
//!
//! # What this file actually proves, and what it cannot
//!
//! `tabletop_readiness_fighter_level_1_chassis_composes_via_printed_sheet_cell_map`
//! below is the load-bearing test: it reads the fixture's `CharacterInput`
//! from disk, runs the real Epic 1 boundary-contract pipeline
//! (`classify_character_input` -> `compute_pilot_with_corpus` ->
//! `to_pilot_receipt` -> `printed_sheet_cell_map`), and asserts every one
//! of the 15 cells `printed_sheet_cell_map` currently defines is a real,
//! non-`Blocked` `Number` that matches the fixture's golden
//! `expected_output` exactly — genuinely computed by running the engine,
//! never hand-typed.
//!
//! Building this fixture surfaced a real, load-bearing integration gap
//! this epic exists to catch (see the task brief's point 3: "this is
//! exactly the kind of integration bug Epic 8 exists to catch"), not a
//! failure of this cycle:
//!
//! **Finding 1 — Epics 2/3/4/5/6/7's engines are not wired into
//! `PilotReceipt` / `printed_sheet_cell_map` at all.** `contract.rs`'s
//! `PilotReceipt` (Epic 1) composes only `PilotBaseChassisComputation`
//! (BAB, saves, the deterministic baseline AC/melee-attack-bonus, three
//! hardcoded skill modifiers, ability modifiers) and `CorpusDerivedSection`
//! (spell-school coverage identity, equipped-item identity — both from
//! SD-19's original bounded seam, `pilot_compute_corpus.rs`). None of
//! Epic 2's `spellbook::compute_spellbook_coverage`, Epic 3's
//! `feat_prereqs::{evaluate_feat_prerequisites, compute_feat_effects}`,
//! Epic 4's `skill_allocation::allocate_skill_ranks`, Epic 5's
//! `equipment_effects::compute_equipment_effects`, Epic 6's
//! `damage_total::resolve_*` functions, or Epic 7's
//! `level_up::compute_level_up_grants` is called from `contract.rs`,
//! `pilot_compute_corpus.rs`, or anywhere else in the receipt-computation
//! path (confirmed by grep across `src/` — every call site for each of
//! these functions is either that function's own module or its own
//! dedicated `tests/sd20_<epic>_*.rs` file). `technical-design.md` §1.3
//! required each subsystem engine to "first extend the boundary contract
//! and add the parity test fixture" before contributing `PilotReceipt`
//! fields; that extension step was never taken by any of Epics 2-7. So
//! `printed_sheet_cell_map` today literally has no cells for skill
//! totals, equipment effects, damage rolls, feat effects, spell
//! coverage, or Level Up grants — there is nothing for an integration
//! test to assert against on that surface for those six epics'
//! real output.
//!
//! This file's `epic_probe_*` tests below call each of those six engines
//! **directly** against data mirroring this exact fixture character, to
//! prove the individual engines are real and correct for a level-1
//! Fighter (satisfying the letter of the task's "every value must be
//! REAL — computed by actually running the engine" for each domain) —
//! while proving, by construction, that none of that output is reachable
//! through `PilotReceipt` today. That gap is documented here precisely,
//! not fabricated around.
//!
//! **Finding 2 (fixed) — `printed_sheet_cell_map`'s `Blocked` gate
//! originally checked only one of at least three claim-blocking
//! diagnostic ids that zero out chassis fields.** `contract.rs`'s
//! `printed_sheet_cell_map` originally rendered
//! `PrintedSheetCellValue::Blocked` only when the receipt carried a
//! claim-blocking `class_chassis.unsupported` diagnostic. But
//! `pilot_compute.rs::compute_combat_baseline` zeroes
//! `baseline_armor_class` / `baseline_melee_attack_bonus` and pushes a
//! *different* claim-blocking diagnostic, `combat.baseline_unsupported`,
//! whenever the character deviates even slightly from one exact
//! hardcoded "Longsword + Chain Shirt + Dodge + Weapon Focus + no
//! shield" posture; `compute_selected_skill_modifiers` does the same for
//! `selected_skill_modifiers` with `skill.selected_modifier.unsupported`
//! (which additionally requires *exactly* Climb/Intimidate/Swim at rank
//! 1 and no other skill allocation at all); `compute_total_saves` does
//! the same for `total_saves` with `defense.total_save.unsupported`.
//! Originally, none of those three diagnostic ids were checked by
//! `printed_sheet_cell_map`'s `chassis_unsupported` gate, so for any
//! character that didn't hit that one exact posture, the
//! `sheet.armor_class` / `sheet.melee_attack_bonus` / `sheet.skill.*`
//! (and, in principle, `sheet.save.*`) cells silently rendered
//! `Number(0)` — a fabricated zero presented as a real computed value —
//! instead of `Blocked`. This was not a new bug introduced by this cycle:
//! it was already visible in the existing, already-landed
//! `tests/fixtures/wire/sd20/boundary_contract_parity.json` fixture,
//! whose `expected_diagnostics` names both `combat.baseline_unsupported`
//! and `skill.selected_modifier.unsupported` as claim-blocking while its
//! `expected_output.cells` originally showed `sheet.armor_class` /
//! `sheet.melee_attack_bonus` / `sheet.skill.climb` /
//! `sheet.skill.intimidate` / `sheet.skill.swim` all as `Number(0)`.
//!
//! This has since been fixed: `printed_sheet_cell_map` now also checks
//! `combat.baseline_unsupported`, `skill.selected_modifier.unsupported`,
//! and `defense.total_save.unsupported`, each OR'd additively with the
//! universal `class_chassis.unsupported` fallback (see `contract.rs`'s
//! `COMBAT_BASELINE_UNSUPPORTED_DIAGNOSTIC_ID` /
//! `SKILL_SELECTED_MODIFIER_UNSUPPORTED_DIAGNOSTIC_ID` /
//! `TOTAL_SAVE_UNSUPPORTED_DIAGNOSTIC_ID` constants).
//! `tabletop_readiness_combat_baseline_deviation_is_blocked_not_zeroed` and
//! `tabletop_readiness_selected_skill_posture_deviation_is_blocked_not_zeroed`
//! below (originally
//! `..._is_silently_zeroed_not_blocked`, which pinned the bug) now assert
//! the fixed `Blocked` rendering with a fresh, independently-built
//! deviating character. `boundary_contract_parity.json`'s
//! `expected_output.cells` was updated the same way (verified by actually
//! running the engine against that fixture's own `CharacterInput`, not
//! hand-guessed) — see `tests/sd20_contract_boundary_parity.rs`.
//!
//! This epic's own fixture avoids the gap in Finding 2 by construction:
//! the fixture's `CharacterInput` deliberately matches the exact
//! deterministic posture both `compute_combat_baseline` and
//! `compute_selected_skill_modifiers` require (Longsword + Chain Shirt
//! equipped, no shield, Power Attack selected-but-inactive, Dodge +
//! Weapon Focus (Longsword) feats with their canonical choice-slot
//! selections, and *exactly* Climb/Intimidate/Swim at rank 1), so every
//! one of the 15 `printed_sheet_cell_map` cells is a real, non-Blocked,
//! non-fabricated number for this fixture — proving the underlying
//! chassis computation is genuinely correct for a fully-supported
//! Fighter, while Finding 1 above documents what remains unresolved for
//! SD-20 as a whole (Finding 2 is now closed).
//!
//! # The "8 wire-fixture parity fixtures" gate
//!
//! `SD-20-rules-engine-completeness-loop-instruction.md`'s closure
//! definition and
//! `programs/codex/requirements/SD-20-rules-engine-completeness/acceptance-and-verification.md`
//! gate 3 both require "at least 8 wire-fixture parity JSON fixtures ...
//! one for boundary contract, one each for spellbook / feat prereqs /
//! skill ranks / equipment effects / damage total / Level Up grants /
//! integration closure." Before this cycle, exactly one such fixture
//! existed on disk (`tests/fixtures/wire/sd20/boundary_contract_parity.json`)
//! — confirmed by listing `tests/fixtures/wire/sd20/`. Every Epic 2-7
//! cycle's own `tests/sd20_<epic>_*.rs` test instead builds its
//! `CharacterInput` / corpus data as inline Rust literals (confirmed by
//! grep: no `tests/sd20_spellbook_*.rs`, `tests/sd20_feat_*.rs`,
//! `tests/sd20_skill_allocation_*.rs`, `tests/sd20_equipment_*.rs`,
//! `tests/sd20_damage_*.rs`, or `tests/sd20_levelup_*.rs` file reads a
//! JSON fixture at all). This cycle lands the second on-disk fixture
//! (`human_fighter_level_1_tabletop.json`), bringing the total to 2 of
//! the required 8. This is a real, unresolved gap — see this cycle's
//! final report and the progress-doc update for the explicit statement
//! that gate 3 is NOT met and SD-20 is therefore NOT fully closed by
//! this cycle alone. Epic 8's own file-touch partition (this file plus
//! the one fixture) does not authorize landing the other six epics'
//! fixtures; that is out of this slice's scope.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
use codex::rules_core::character_input::{
    AbilityScores, ActiveState, CharacterClassLevel, CharacterInput, ChosenCharacterState,
    EquipmentSelection, SelectedChoice, SkillAllocation,
};
use codex::rules_core::contract::{
    classify_character_input, printed_sheet_cell_map, to_pilot_receipt, PrintedSheetCellValue,
};
use codex::rules_core::damage_total::{
    resolve_base_damage_dice, resolve_critical_multiplier, resolve_critical_threat_range,
    resolve_feat_damage_effect, resolve_str_damage_modifier, resolve_weapon_enhancement_modifier,
    WeaponHandSlot,
};
use codex::rules_core::equipment_effects::compute_equipment_effects;
use codex::rules_core::feat_prereqs::{compute_feat_effects, evaluate_feat_prerequisites, FeatKey};
use codex::rules_core::level_up::compute_level_up_grants;
use codex::rules_core::pilot_compute_corpus::{compute_pilot_with_corpus, DerivedEquipmentStats};
use codex::rules_core::rules_tables::crb::feats::FeatCategory;
use codex::rules_core::skill_allocation::allocate_skill_ranks;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};
use codex::rules_core::spellbook::compute_spellbook_coverage;

// --- minimal std-only JSON reader, copied from the Epic 1 precedent
// (`tests/sd20_contract_boundary_parity.rs`) for the same reason that
// file documents: this crate carries no serde dependency and Cargo.toml
// is out of this epic's file-touch partition. ---

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
    path.push("tests/fixtures/wire/sd20/human_fighter_level_1_tabletop.json");
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read fixture at {path:?}: {err}"));
    parse_json(&raw)
}

/// Build the engine's `CharacterInput` from the fixture's `input` section
/// — read from the on-disk JSON, not hand-duplicated Rust literals.
/// Unlike the Epic 1 precedent (`sd20_contract_boundary_parity.rs`),
/// which only needed to round-trip an empty-selections `CharacterInput`,
/// this fixture's `input` section carries real feats/skills/equipment/
/// choices, so every `ChosenCharacterState` field is parsed here.
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

    let selected_feats = input
        .get("selected_feats")
        .as_array()
        .iter()
        .map(|entry| entry.as_str().to_owned())
        .collect();

    let skill_allocations = input
        .get("skill_allocations")
        .as_array()
        .iter()
        .map(|entry| SkillAllocation {
            skill_id: entry.get("skill_id").as_str().to_owned(),
            ranks: entry.get("ranks").as_i64() as u8,
        })
        .collect();

    let equipment_selections = input
        .get("equipment_selections")
        .as_array()
        .iter()
        .map(|entry| {
            let active_state = match entry.get("active_state").as_str() {
                "equipped_active" => ActiveState::EquippedActive,
                "absent" => ActiveState::Absent,
                "selected_inactive" => ActiveState::SelectedInactive,
                other => panic!("unrecognized active_state token: {other:?}"),
            };
            EquipmentSelection {
                item_id: entry.get("item_id").as_str().to_owned(),
                equipped_or_active: matches!(active_state, ActiveState::EquippedActive),
                active_state,
            }
        })
        .collect();

    let selected_choices = input
        .get("selected_choices")
        .as_array()
        .iter()
        .map(|entry| SelectedChoice {
            choice_set_id: entry.get("choice_set_id").as_str().to_owned(),
            selection_id: entry.get("selection_id").as_str().to_owned(),
        })
        .collect();

    assert!(
        input.get("spells_selected").as_array().is_empty(),
        "this fixture's Fighter carries no spells; extend this builder before adding any"
    );

    CharacterInput {
        case_id: Some(input.get("case_id").as_str().to_owned()),
        source_package_id: input.get("source_package_id").as_str().to_owned(),
        chosen: ChosenCharacterState {
            race_id: input.get("race_id").as_str().to_owned(),
            class_levels,
            ability_scores,
            selected_feats,
            skill_allocations,
            equipment_selections,
            selected_choices,
            spells_selected: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

/// Real verbatim rows copied from the live corpus
/// (`/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/`):
/// `cr_equip_arms_armor.lst` (`KEY:Longsword (Base)` line 165,
/// `KEY:Chain Shirt (Base)` line 40) and `cr_equipmods.lst`
/// (`KEY:Special Quality ~ Masterwork ~ Weapon` line 18, the "Masterwork
/// (Weapon)" quality — a `TOHIT`-only `TYPE=Enhancement` bonus, unlike
/// the full `DAMAGE,TOHIT` a true magical "+N" weapon carries, per
/// `damage_total.rs`'s own doc comment on the affected-roll distinction;
/// chosen instead of a magical "+1" weapon so this level-1 fixture's
/// loadout stays tabletop-plausible on a level-1 starting-gold budget).
/// Same parser (`parse_equipment_entries`) and IR converter
/// (`convert_equipment_record`) every sibling `tests/sd20_equipment_*.rs`
/// / `tests/sd20_damage_*.rs` cycle already uses, so this corpus is
/// genuinely resolvable, not a hand-rolled stand-in.
const FIGHTER_GEAR_FIXTURE_TEXT: &str = "\
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Resizable.Melee.Martial.OneHanded.Slashing.Sword.BladeHeavy.Weapon Group Blades Heavy\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\tWIELD:OneHanded\tSIZE:M
Chain Shirt\tKEY:Chain Shirt (Base)\tTYPE:Armor.Light\tCOST:100\tWT:25\tACCHECK:-2\tMAXDEX:4\tSPELLFAILURE:20\tBONUS:COMBAT|AC|4|TYPE=Armor|PREVAREQ:DisableArmorBonus,0
Masterwork (Weapon)\tKEY:Special Quality ~ Masterwork ~ Weapon\tTYPE:MasterworkQuality.Weapon\tCOST:0\tBONUS:WEAPON|TOHIT|1|TYPE=Enhancement
";

fn corpus_with_fighter_gear() -> SourcePackageContent<'static> {
    let result = parse_equipment_entries("cr_equip_arms_armor.lst", FIGHTER_GEAR_FIXTURE_TEXT);
    assert!(
        result.diagnostics.is_empty(),
        "fixture text must parse cleanly: {:?}",
        result.diagnostics
    );
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

// ---------------------------------------------------------------------
// Primary integration test: the fixture's CharacterInput round-trips
// through the real boundary-contract pipeline into a PilotReceipt whose
// every printed_sheet_cell_map cell is a real, non-Blocked number.
// ---------------------------------------------------------------------

#[test]
fn tabletop_readiness_fighter_level_1_chassis_composes_via_printed_sheet_cell_map() {
    let fixture = load_fixture();
    let input_json = fixture.get("input");
    let input = character_input_from_fixture(input_json);

    let permutation = classify_character_input(&input);
    assert_eq!(
        format!("{permutation:?}"),
        fixture.get("expected_permutation").as_str(),
        "classify_character_input permutation must match the fixture's expected_permutation"
    );

    let corpus = corpus_with_fighter_gear();
    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt);

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
        expected_chassis.get("baseline_melee_attack_bonus").as_i64() as i16,
        "baseline_melee_attack_bonus must be the real deterministic-posture computation, not a \
         fabricated zero -- this fixture's equipment_selections/selected_feats/selected_choices \
         deliberately match the exact posture compute_combat_baseline requires (see this file's \
         module doc comment, Finding 2)"
    );
    assert_eq!(
        receipt.chassis.baseline_armor_class,
        expected_chassis.get("baseline_armor_class").as_i64() as i16,
        "baseline_armor_class must be the real deterministic-posture computation, not a \
         fabricated zero"
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
        expected_selected_skill_modifiers.get("climb").as_i64() as i16,
        "selected_skill_modifiers.climb must be the real deterministic-posture computation, not \
         a fabricated zero -- this fixture allocates exactly Climb/Intimidate/Swim at rank 1 and \
         nothing else (see this file's module doc comment, Finding 2)"
    );
    assert_eq!(
        receipt.chassis.selected_skill_modifiers.intimidate,
        expected_selected_skill_modifiers.get("intimidate").as_i64() as i16
    );
    assert_eq!(
        receipt.chassis.selected_skill_modifiers.swim,
        expected_selected_skill_modifiers.get("swim").as_i64() as i16
    );

    // corpus_derived: both equipped legacy-namespace item ids
    // ("item:longsword", "item:chain_shirt") resolve against this
    // fixture's real corpus via equipment_resolver's documented
    // normalized-name fallback (its own doc comment: "the legacy
    // 'item:longsword'-style fixture namespace ... a last-resort
    // fallback"), landing on the exact same KEY-identified records the
    // real corpus-key selection ("Special Quality ~ Masterwork ~
    // Weapon") resolves via exact KEY match. "item:shield" and
    // "power_attack" carry no matching corpus record in this fixture's
    // synthetic corpus and are honestly absent from equipped_items.
    assert!(
        receipt.corpus_derived.school_coverage.is_empty(),
        "expected empty school_coverage: this Fighter has no spells_selected"
    );
    let equipped = &receipt.corpus_derived.equipped_items;
    let expected_equipped = fixture
        .get("expected_output")
        .get("corpus_derived")
        .get("equipped_items")
        .as_array();
    assert_eq!(
        equipped.len(),
        expected_equipped.len(),
        "corpus_derived.equipped_items count must match the fixture"
    );
    for (item, expected_item) in equipped.iter().zip(expected_equipped.iter()) {
        assert_eq!(item.item_id, expected_item.get("item_id").as_str());
        assert_eq!(
            item.equipment_record_name,
            expected_item.get("equipment_record_name").as_str()
        );
        assert_eq!(
            item.equipment_record_key,
            expected_item.get("equipment_record_key").as_str()
        );
        // Every field here is `None`/default -- per this file's module
        // doc comment (Finding 1), the SD-19 seam that populates
        // `corpus_derived.equipped_items` has never been wired to Epic
        // 5's real `compute_equipment_effects` output. This is the
        // *current, real* behavior, asserted here as a pinned fact, not
        // papered over: `epic_probe_equipment_effects_are_real_but_not_reflected_in_corpus_derived`
        // below shows the same "Chain Shirt (Base)" item's *real*
        // Epic-5-computed armor stats side by side with this default.
        assert_eq!(
            item.derived_stats,
            DerivedEquipmentStats::default(),
            "corpus_derived.equipped_items[].derived_stats is still SD-19's unpopulated \
             bounded-baseline default for every item -- Epic 5's real per-item stats are not \
             wired into the receipt (Finding 1)"
        );
        let expected_table_cell = expected_item.get("table_cell");
        let table_cell = item
            .table_cell
            .as_ref()
            .unwrap_or_else(|| panic!("expected a table_cell for {}", item.item_id));
        assert_eq!(table_cell.table, expected_table_cell.get("table").as_str());
        assert_eq!(
            table_cell.row_key,
            expected_table_cell.get("row_key").as_str()
        );
        assert_eq!(
            table_cell.column_key,
            expected_table_cell.get("column_key").as_str()
        );
    }

    // printed_sheet_cell_map: the full boundary-contract round trip.
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
            "Blocked" => panic!(
                "the tabletop-readiness fixture's cells must never be Blocked -- a level-1 \
                 Fighter is a fully-supported class per Epic 1's own contract; a Blocked entry \
                 here in expected_output would itself be a fixture-authoring bug"
            ),
            other => panic!("unrecognized expected cell value kind: {other:?}"),
        }
    }
    // Doubly explicit, independent of the fixture's own JSON: no cell in
    // the live output is ever Blocked for this fully-supported Fighter,
    // per this epic's brief point 2 ("no PrintedSheetCellValue::Blocked
    // cells for anything a level-1 fighter should have fully supported").
    assert!(
        cells
            .iter()
            .all(|cell| !matches!(cell.value, PrintedSheetCellValue::Blocked)),
        "no cell may render Blocked for this fully-supported level-1 Fighter posture: {cells:?}"
    );

    // Diagnostics: exact claim_blocking id-set match, mirroring the Epic
    // 1 precedent. This fixture's posture is constructed to trip none.
    let expected_blocking_ids: BTreeSet<String> = fixture
        .get("expected_diagnostics")
        .as_array()
        .iter()
        .map(|entry| {
            assert!(
                entry.get("claim_blocking").as_bool(),
                "expected_diagnostics entries must all be claim_blocking: true"
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
        "the receipt's claim_blocking diagnostics must exactly match the fixture's \
         expected_diagnostics (empty: this fixture's posture is fully supported)"
    );
}

// ---------------------------------------------------------------------
// Fixed-behavior regression tests (Finding 2, now closed): a character
// that deviates from the one exact deterministic posture gets a
// claim-blocking diagnostic, and `printed_sheet_cell_map` correctly
// renders `Blocked` for the cell(s) that diagnostic invalidates -- never a
// silently fabricated `Number(0)`.
//
// These two tests originally pinned Finding 2 as a *known, unresolved
// gap*: `contract.rs::printed_sheet_cell_map`'s `Blocked` gate checked
// only `class_chassis.unsupported`, so a character that tripped
// `combat.baseline_unsupported` or `skill.selected_modifier.unsupported`
// without also tripping `class_chassis.unsupported` still rendered a
// fabricated `Number(0)`. `printed_sheet_cell_map` now also checks
// `combat.baseline_unsupported`, `skill.selected_modifier.unsupported`,
// and `defense.total_save.unsupported` (the third diagnostic Finding 2
// named, not separately exercised below since it always co-fires with
// `class_chassis.unsupported` for the Fighter-only compute surface this
// suite exercises), each OR'd additively with the universal
// `class_chassis.unsupported` fallback -- see `contract.rs`'s
// `COMBAT_BASELINE_UNSUPPORTED_DIAGNOSTIC_ID` /
// `SKILL_SELECTED_MODIFIER_UNSUPPORTED_DIAGNOSTIC_ID` /
// `TOTAL_SAVE_UNSUPPORTED_DIAGNOSTIC_ID` constants. These tests now assert
// the fixed (`Blocked`) behavior instead of pinning the bug.
// ---------------------------------------------------------------------

/// A variant of the fixture's base character with `feat:dodge` (and its
/// canonical `choice:human_bonus_feat` selection) dropped, which trips
/// `compute_combat_baseline`'s `unmet_combat_posture_conditions` (it
/// requires `selected_feats` to contain `feat:dodge`) without touching
/// anything the selected-skill posture depends on.
fn character_missing_dodge_feat() -> CharacterInput {
    let fixture = load_fixture();
    let mut input = character_input_from_fixture(fixture.get("input"));
    input.chosen.selected_feats.retain(|feat| feat != "feat:dodge");
    input
        .chosen
        .selected_choices
        .retain(|choice| choice.choice_set_id != "choice:human_bonus_feat");
    input
}

#[test]
fn tabletop_readiness_combat_baseline_deviation_is_blocked_not_zeroed() {
    let input = character_missing_dodge_feat();
    let corpus = corpus_with_fighter_gear();
    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt);

    assert!(
        receipt
            .diagnostics
            .iter()
            .any(|d| d.id == "combat.baseline_unsupported" && d.claim_blocking),
        "dropping feat:dodge must trip combat.baseline_unsupported (claim_blocking: true): {:?}",
        receipt.diagnostics
    );
    // The underlying chassis field is still a fabricated-looking zero at
    // the `PilotBaseChassisComputation` level (that shape has no
    // "blocked" variant) -- it is `printed_sheet_cell_map`'s job to not
    // present that zero as real data.
    assert_eq!(receipt.chassis.baseline_armor_class, 0);
    assert_eq!(receipt.chassis.baseline_melee_attack_bonus, 0);

    let cells = printed_sheet_cell_map(&receipt);
    let armor_class_cell = cells
        .iter()
        .find(|cell| cell.cell_id == "sheet.armor_class")
        .expect("sheet.armor_class cell must exist");
    let melee_attack_bonus_cell = cells
        .iter()
        .find(|cell| cell.cell_id == "sheet.melee_attack_bonus")
        .expect("sheet.melee_attack_bonus cell must exist");
    // Fixed behavior: printed_sheet_cell_map now also gates on
    // combat.baseline_unsupported, so these cells render Blocked instead
    // of a fabricated Number(0).
    assert_eq!(
        armor_class_cell.value,
        PrintedSheetCellValue::Blocked,
        "combat.baseline_unsupported (claim_blocking: true) must block sheet.armor_class"
    );
    assert_eq!(
        melee_attack_bonus_cell.value,
        PrintedSheetCellValue::Blocked,
        "combat.baseline_unsupported (claim_blocking: true) must block sheet.melee_attack_bonus"
    );
}

#[test]
fn tabletop_readiness_selected_skill_posture_deviation_is_blocked_not_zeroed() {
    let fixture = load_fixture();
    let mut input = character_input_from_fixture(fixture.get("input"));
    // Widen beyond the exact Climb/Intimidate/Swim rank-1 posture
    // `unmet_selected_skill_posture_conditions` requires -- a single
    // extra, otherwise perfectly legal skill allocation is enough to
    // trip it ("Refuse any widening beyond exactly the three selected
    // skills").
    input.chosen.skill_allocations.push(SkillAllocation {
        skill_id: "skill:diplomacy".to_string(),
        ranks: 1,
    });

    let corpus = corpus_with_fighter_gear();
    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt);

    assert!(
        receipt
            .diagnostics
            .iter()
            .any(|d| d.id == "skill.selected_modifier.unsupported" && d.claim_blocking),
        "adding a diplomacy allocation must trip skill.selected_modifier.unsupported \
         (claim_blocking: true): {:?}",
        receipt.diagnostics
    );
    assert_eq!(receipt.chassis.selected_skill_modifiers.climb, 0);

    let cells = printed_sheet_cell_map(&receipt);
    let climb_cell = cells
        .iter()
        .find(|cell| cell.cell_id == "sheet.skill.climb")
        .expect("sheet.skill.climb cell must exist");
    let intimidate_cell = cells
        .iter()
        .find(|cell| cell.cell_id == "sheet.skill.intimidate")
        .expect("sheet.skill.intimidate cell must exist");
    let swim_cell = cells
        .iter()
        .find(|cell| cell.cell_id == "sheet.skill.swim")
        .expect("sheet.skill.swim cell must exist");
    // Fixed behavior: printed_sheet_cell_map now also gates on
    // skill.selected_modifier.unsupported, so these cells render Blocked
    // instead of a fabricated Number(0).
    assert_eq!(
        climb_cell.value,
        PrintedSheetCellValue::Blocked,
        "skill.selected_modifier.unsupported (claim_blocking: true) must block sheet.skill.climb"
    );
    assert_eq!(
        intimidate_cell.value,
        PrintedSheetCellValue::Blocked,
        "skill.selected_modifier.unsupported (claim_blocking: true) must block \
         sheet.skill.intimidate"
    );
    assert_eq!(
        swim_cell.value,
        PrintedSheetCellValue::Blocked,
        "skill.selected_modifier.unsupported (claim_blocking: true) must block sheet.skill.swim"
    );
}

// ---------------------------------------------------------------------
// Epic probe tests (Finding 1): each calls one epic's real engine
// function directly against data mirroring this fixture's character,
// proving the engine is real and correct for this exact scenario even
// though its output is not reachable through PilotReceipt today.
// ---------------------------------------------------------------------

/// Epic 2 (spellbook): a Fighter has no spells. Per acceptance criterion
/// 10 ("none for non-spellcasters"), the engine must handle that
/// honestly -- an empty `SpellbookCoverage`, not an error or a
/// fabricated entry.
#[test]
fn epic_probe_spellbook_is_honestly_empty_for_a_non_caster_fighter() {
    let fixture = load_fixture();
    let input = character_input_from_fixture(fixture.get("input"));
    let corpus = corpus_with_fighter_gear();

    let coverage = compute_spellbook_coverage(&input, &corpus);

    assert_eq!(
        coverage,
        codex::rules_core::spellbook::SpellbookCoverage::default(),
        "a Fighter with no spells_selected must produce a genuinely empty SpellbookCoverage, \
         not a fabricated one"
    );
}

/// Epic 3 (feat prereqs): the fixture's three real feat picks (Power
/// Attack, Dodge, Weapon Focus, all Combat-category) each resolve as
/// eligible with a real description + TableCellRef -- plus one
/// catalog-only probe (Toughness, General-category) not on this
/// character, exercising category breadth beyond Combat.
#[test]
fn epic_probe_feat_prereqs_resolve_real_effects_for_this_fighters_feats() {
    let combat_feats = [
        ("Power Attack", "You can make exceptionally deadly melee attacks by sacrificing accuracy for strength."),
        ("Dodge", "Your training and reflexes allow you to react swiftly to avoid an opponent's attack."),
        ("Weapon Focus", "You are especially good at using your chosen weapon."),
    ];
    for (feat_id, expected_description) in combat_feats {
        let key = FeatKey {
            feat_id: feat_id.to_string(),
            category: FeatCategory::Combat,
        };
        let eligibility = evaluate_feat_prerequisites(&key);
        assert!(
            eligibility.is_eligible,
            "{feat_id} must be eligible under a Combat-category catalog lookup"
        );
        assert!(eligibility.failing_prerequisites.is_empty());

        let effects = compute_feat_effects(&key);
        assert_eq!(effects.feat_id, feat_id);
        assert_eq!(effects.description.as_deref(), Some(expected_description));
        assert!(
            effects.table_cell.is_some(),
            "{feat_id} must cite a real TableCellRef"
        );
    }

    // General-category breadth probe (not part of this character's own
    // feat list -- a level-1 Human Fighter legally has exactly 3 feats,
    // all already accounted for above).
    let toughness = FeatKey {
        feat_id: "Toughness".to_string(),
        category: FeatCategory::General,
    };
    let eligibility = evaluate_feat_prerequisites(&toughness);
    assert!(eligibility.is_eligible);
    let effects = compute_feat_effects(&toughness);
    assert_eq!(
        effects.description.as_deref(),
        Some("You have enhanced physical stamina.")
    );
}

/// Epic 4 (skill-rank allocation): a broader skill allocation than the
/// fixture's own posture-preserving Climb/Intimidate/Swim-only slice,
/// exercising class-skill, cross-class, untrained, and max-rank-cap
/// breadth in one call -- real output from `allocate_skill_ranks`,
/// never reachable via the fixture's own `printed_sheet_cell_map` cells
/// (Finding 1 and Finding 2 both apply to why the fixture itself can't
/// carry this broader allocation and still stay fully un-Blocked).
#[test]
fn epic_probe_skill_allocation_covers_class_cross_class_and_untrained_breadth() {
    let fixture = load_fixture();
    let mut input = character_input_from_fixture(fixture.get("input"));
    input.chosen.skill_allocations = vec![
        SkillAllocation {
            skill_id: "skill:climb".to_string(),
            ranks: 5, // over the level-1 class-skill cap of 4 -> capped + diagnostic
        },
        SkillAllocation {
            skill_id: "skill:intimidate".to_string(),
            ranks: 1,
        },
        SkillAllocation {
            skill_id: "skill:swim".to_string(),
            ranks: 0, // untrained use
        },
        SkillAllocation {
            skill_id: "skill:diplomacy".to_string(),
            ranks: 1, // cross-class, at its cap of 1
        },
        SkillAllocation {
            skill_id: "skill:disable_device".to_string(),
            ranks: 1, // cross-class AND trained-only
        },
    ];

    let totals = allocate_skill_ranks(&input);

    assert_eq!(
        totals.class_skills,
        vec![
            "skill:climb".to_string(),
            "skill:intimidate".to_string(),
            "skill:swim".to_string()
        ]
    );
    assert!(totals.cross_class_penalty_applied);

    let climb = totals.totals.get("skill:climb").expect("climb must resolve");
    assert_eq!(climb.ranks, 4, "climb's raw 5 ranks must be capped to the level-1 cap of 4");
    assert_eq!(climb.class_skill_bonus, 3);
    assert_eq!(climb.total_modifier, 10);
    assert!(
        totals
            .diagnostics
            .iter()
            .any(|d| d.id == "skill_allocation.class_skill_max_rank_exceeded" && !d.claim_blocking),
        "the over-allocated climb ranks must produce a non-claim-blocking cap diagnostic, and \
         the total above must already be the real capped value, never fabricated uncapped: {:?}",
        totals.diagnostics
    );

    let intimidate = totals
        .totals
        .get("skill:intimidate")
        .expect("intimidate must resolve");
    assert_eq!(intimidate.total_modifier, 3);

    let swim = totals.totals.get("skill:swim").expect("swim must resolve");
    assert_eq!(swim.ranks, 0);
    assert_eq!(
        totals.untrained_use.get("skill:swim").copied(),
        Some(3),
        "swim at 0 ranks must be recorded as usable untrained at the raw STR modifier"
    );

    let diplomacy = totals
        .totals
        .get("skill:diplomacy")
        .expect("diplomacy must resolve");
    assert_eq!(diplomacy.ranks, 1, "diplomacy's 1 rank is exactly at the cross-class cap");
    assert_eq!(diplomacy.class_skill_bonus, 0, "cross-class skills never carry the trained bonus");

    let disable_device = totals
        .totals
        .get("skill:disable_device")
        .expect("disable_device must resolve (nonzero ranks -- trained-only, but invested)");
    assert_eq!(disable_device.ranks, 1);
}

/// Epic 5 (equipment effects): the same three real corpus items the
/// fixture equips, resolved through Epic 5's real per-item engine. The
/// Chain Shirt's real armor_class_bonus/max_dex/spell_failure here
/// contrast directly with the `DerivedEquipmentStats::default()` the
/// primary test above shows for the exact same physical item inside
/// `receipt.corpus_derived.equipped_items` -- concrete, side-by-side
/// evidence of Finding 1.
#[test]
fn epic_probe_equipment_effects_are_real_but_not_reflected_in_corpus_derived() {
    let corpus = corpus_with_fighter_gear();
    let equipped = vec![
        EquipmentSelection {
            item_id: "Longsword (Base)".to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
        },
        EquipmentSelection {
            item_id: "Chain Shirt (Base)".to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
        },
        EquipmentSelection {
            item_id: "Special Quality ~ Masterwork ~ Weapon".to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
        },
    ];

    let effects = compute_equipment_effects(&equipped, &corpus);

    assert_eq!(effects.per_item.len(), 3);
    assert_eq!(effects.armor_class_delta, 4, "Chain Shirt's real +4 armor bonus");
    assert_eq!(effects.max_dex_cap, Some(4));
    assert_eq!(effects.spell_failure_chance, Some(20.0));

    let chain_shirt = effects
        .per_item
        .iter()
        .find(|item| item.equipment_record_key == "Chain Shirt (Base)")
        .expect("Chain Shirt (Base) must resolve");
    assert_eq!(chain_shirt.armor_class_bonus, Some(4));
    assert_eq!(chain_shirt.max_dex, Some(4));
    assert_eq!(chain_shirt.spell_failure, Some(20.0));

    let masterwork = effects
        .per_item
        .iter()
        .find(|item| item.equipment_record_key == "Special Quality ~ Masterwork ~ Weapon")
        .expect("the masterwork weapon quality must resolve");
    let enhancement = masterwork
        .weapon_enhancement_bonus
        .as_ref()
        .expect("masterwork weapon quality must carry a real weapon-enhancement bonus");
    assert_eq!(enhancement.affects, "TOHIT");
    assert_eq!(enhancement.bonus, 1);
}

/// Epic 6 (damage total): the fixture's Longsword, exercising
/// base-dice, STR-modifier, weapon-enhancement (composed with Epic 5's
/// real output above), critical-threat-range, and critical-multiplier —
/// plus feat-effect, proven to honestly return `None` for all three of
/// this fighter's real feats (none carries a bare constant `DAMAGE`
/// bonus token; Power Attack's is formula-based, Dodge's and Weapon
/// Focus's target AC/TOHIT, not DAMAGE), rather than fabricating a
/// value.
#[test]
fn epic_probe_damage_total_covers_base_dice_str_enhancement_crit_and_honest_feat_absence() {
    let corpus = corpus_with_fighter_gear();
    let equipped = vec![
        EquipmentSelection {
            item_id: "Longsword (Base)".to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
        },
        EquipmentSelection {
            item_id: "Special Quality ~ Masterwork ~ Weapon".to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
        },
    ];
    let equipment_effects = compute_equipment_effects(&equipped, &corpus);

    let base_dice = resolve_base_damage_dice("Longsword (Base)", &corpus)
        .expect("Longsword (Base) must resolve real base damage dice");
    assert_eq!(base_dice.base_dice.count, 1);
    assert_eq!(base_dice.base_dice.die_size, 8);

    let str_modifier = resolve_str_damage_modifier(
        "Longsword (Base)",
        &corpus,
        3, // this fixture's STR modifier (STR 16)
        WeaponHandSlot::Primary,
    )
    .expect("Longsword (Base) must resolve a real STR damage modifier");
    assert_eq!(
        str_modifier.str_damage_modifier, 3,
        "a one-handed weapon in the primary hand gets the full STR modifier"
    );

    let enhancement =
        resolve_weapon_enhancement_modifier("Longsword (Base)", &corpus, &equipment_effects)
            .expect("Longsword (Base) must resolve a real weapon-enhancement contribution");
    assert_eq!(
        enhancement.attack_bonus, 1,
        "the masterwork quality's TOHIT-only enhancement contributes to the attack roll"
    );
    assert_eq!(
        enhancement.damage_bonus, 0,
        "a masterwork (TOHIT-only) enhancement contributes nothing to the damage roll"
    );

    let crit_range = resolve_critical_threat_range("Longsword (Base)", &corpus)
        .expect("Longsword (Base) must resolve a real critical threat range");
    assert_eq!(crit_range.critical_threat_range, (19, 20));

    let crit_mult = resolve_critical_multiplier("Longsword (Base)", &corpus)
        .expect("Longsword (Base) must resolve a real critical multiplier");
    assert_eq!(crit_mult.critical_multiplier, 2);

    // Honest absence, not fabrication, for all three of this fighter's
    // real feats -- none carries a bare constant DAMAGE bonus token.
    for feat_key in ["Power Attack", "Dodge", "Weapon Focus"] {
        assert!(
            resolve_feat_damage_effect(feat_key).is_none(),
            "{feat_key} must not resolve a fabricated damage bonus: Power Attack's is \
             formula-based (BAB-scaled, out of this engine's constant-only bound), Dodge's and \
             Weapon Focus's target AC/TOHIT, not DAMAGE"
        );
    }
}

/// Epic 7 (Level Up grant model): this fixture's character advanced from
/// level 1 to level 2, cross-checked against the independently-landed
/// `tests/sd20_levelup_fighter.rs` cycle's own grounded numbers for the
/// identical transition (full BAB, good Fortitude, Bravery, the level-2
/// Bonus Feat slot).
#[test]
fn epic_probe_level_up_grants_fighter_level_1_to_2() {
    let fixture = load_fixture();
    let input = character_input_from_fixture(fixture.get("input"));

    let plan = compute_level_up_grants(&input, 1, 2);

    let grant = |name_fragment: &str| {
        plan.automatic_features
            .iter()
            .find(|grant| grant.name.contains(name_fragment))
    };

    let bab_grant = grant("base_attack_bonus")
        .unwrap_or_else(|| panic!("expected a base_attack_bonus grant: {:?}", plan.automatic_features));
    assert_eq!(bab_grant.effects[0].value, 2, "fighter level 2 full BAB must be +2");
    assert_eq!(bab_grant.source_table.table, "class_tables");
    assert_eq!(bab_grant.source_table.row_key, "fighter:2");

    let fort_grant = grant("fort_save")
        .unwrap_or_else(|| panic!("expected a fort_save grant: {:?}", plan.automatic_features));
    assert_eq!(fort_grant.effects[0].value, 3, "fighter level 2 good Fortitude save must be +3");

    let bravery_grant = grant("bravery")
        .unwrap_or_else(|| panic!("expected a bravery grant at level 2: {:?}", plan.automatic_features));
    assert_eq!(bravery_grant.effects[0].value, 1);

    let bonus_feat_grant = grant("level 2 bonus feat").unwrap_or_else(|| {
        panic!("expected a level-2 bonus feat slot grant: {:?}", plan.automatic_features)
    });
    assert_eq!(bonus_feat_grant.effects[0].value, 0);

    assert!(plan.resource_pool_change.pools.is_empty(), "Fighter has no per-level resource pool");
    assert!(!plan.capstone_threshold, "level 2 is not the level-20 capstone");
}
