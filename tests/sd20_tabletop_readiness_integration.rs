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
//! **Finding 1 (CLOSED as of this cycle, `integration:epic_wiring_closure`
//! / Cycle 7 of the wiring project, `adaptive-squishing-mccarthy.md`) —
//! Epics 2/3/4/5/6/7's engines were not wired into `PilotReceipt` /
//! `printed_sheet_cell_map` at all.** Originally (Epic 8's first closure
//! pass): `contract.rs`'s `PilotReceipt` (Epic 1) composed only
//! `PilotBaseChassisComputation` (BAB, saves, the deterministic baseline
//! AC/melee-attack-bonus, three hardcoded skill modifiers, ability
//! modifiers) and `CorpusDerivedSection` (spell-school coverage identity,
//! equipped-item identity — both from SD-19's original bounded seam,
//! `pilot_compute_corpus.rs`). None of Epic 2's
//! `spellbook::compute_spellbook_coverage`, Epic 3's
//! `feat_prereqs::{evaluate_feat_prerequisites, compute_feat_effects}`,
//! Epic 4's `skill_allocation::allocate_skill_ranks`, Epic 5's
//! `equipment_effects::compute_equipment_effects`, Epic 6's
//! `damage_total::resolve_*` functions, or Epic 7's
//! `level_up::compute_level_up_grants` was called from `contract.rs`,
//! `pilot_compute_corpus.rs`, or anywhere else in the receipt-computation
//! path. `printed_sheet_cell_map` had no cells for skill totals, equipment
//! effects, damage rolls, feat effects, spell coverage, or Level Up
//! grants — there was nothing for an integration test to assert against
//! on that surface for those six epics' real output.
//!
//! **This has since been fully wired**, across the 8-cycle
//! `adaptive-squishing-mccarthy.md` wiring project (all landed on
//! `tranche/4` before this cycle):
//!
//! | Cycle | What it wired | Commit SHA |
//! |---|---|---|
//! | 0 `contract:receipt_signature_threading` | Widened `to_pilot_receipt` to take `input`/`corpus` | `52ed2ea` |
//! | 1 `contract:skill_wiring` | `PilotReceipt.skills: SkillTotals` (Epic 4) | `4859b77` |
//! | 2 `contract:spellbook_wiring` | `PilotReceipt.spellbook: SpellbookCoverage` (Epic 2) | `2dbe0c8` |
//! | 3 `contract:feat_wiring` | `PilotReceipt.feats: Vec<ResolvedFeat>` (Epic 3) | `0066599` |
//! | 4 `contract:equipment_wiring` | `PilotReceipt.equipment_effects: EquipmentEffects` (Epic 5) | `2942875` |
//! | 5a `damage:aggregate_weapons` | `resolve_weapon_damage_breakdown` aggregator (Epic 6, lands in `damage_total.rs`) | `89fba8c` |
//! | 5b `contract:damage_wiring` | `PilotReceipt.weapon_damage: Vec<WeaponDamageBreakdown>` (Epic 6) | `8510151` |
//! | 6 `contract:level_up_preview` | standalone `compute_level_up_preview` pass-through (Epic 7) | `62f7783` |
//!
//! This cycle (7, `integration:epic_wiring_closure`) is the closing proof:
//! `tabletop_readiness_fighter_level_1_chassis_composes_via_printed_sheet_cell_map`
//! below now asserts, for each of the six wired pieces, that
//! `receipt.<field>` (produced by the real `to_pilot_receipt` call) is
//! byte-identical (`PartialEq`, not just independently-both-exist) to
//! that epic's own function called directly against the same fixture
//! `CharacterInput`/corpus — proving the wired path and the direct path
//! genuinely **agree**, not merely that both happen to exist. See that
//! test's body for the six parity assertions and their cross-references
//! back to each `epic_probe_*` test below (which continues to serve as
//! the "engine is real and correct" proof; the wired-path parity
//! assertions are the *new* "engine is reachable and agrees" proof this
//! cycle adds).
//!
//! This file's `epic_probe_*` tests below still call each of those six
//! engines **directly** against data mirroring this exact fixture
//! character, to prove the individual engines are real and correct for a
//! level-1 Fighter (satisfying the letter of the original task's "every
//! value must be REAL — computed by actually running the engine" for each
//! domain). They are no longer proving an unreachable-output gap (that gap
//! is closed) — they now double as the ground-truth oracle the primary
//! test's parity assertions cross-check against.
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
//! one of the 15 chassis-derived `printed_sheet_cell_map` cells is a
//! real, non-Blocked, non-fabricated number for this fixture — proving
//! the underlying chassis computation is genuinely correct for a
//! fully-supported Fighter (Finding 2 is closed; Finding 1 above is also
//! now closed, as of this cycle).
//!
//! # Feat-id fixture quirk (resolved, kept intentionally)
//!
//! This fixture's `input.selected_feats` mixes 3 namespaced ids
//! (`feat:dodge`, `feat:weapon_focus`, `feat:power_attack` — none of
//! which match any entry in `rules_tables::crb::feats::feat_tables()`)
//! with 3 plain catalog names (`Dodge`, `Weapon Focus`, `Power Attack` —
//! all real matches). Cycle 3 of the wiring project
//! (`contract:feat_wiring`) found this quirk and decided deliberately, by
//! reading `feats.rs`'s own doc comment first: "Almost no `cr_feats.lst`
//! record in this catalog's 4 categories carries an explicit `KEY:`
//! token ... so `key` equals `name` for every entry here today" — i.e.
//! the CRB feat catalog has no namespaced-id convention at all, ever; it
//! is plain display names only (`Dodge`, not `feat:dodge`). So the
//! namespaced ids are not an alternate-but-valid catalog format — they
//! cannot ever match, by construction. This cycle (7) re-confirms that
//! same reading and ratifies Cycle 3's decision: **keep the mix**, do not
//! clean the fixture up to plain names only. Two reasons this file
//! commits to (b), not (a): (1) `tests/sd20_contract_feat_wiring.rs`
//! already landed a dedicated regression test
//! (`unrecognized_feat_id_is_honestly_skipped_not_fabricated`, see that
//! file's module doc comment) that explicitly cites this exact fixture's
//! `selected_feats` shape as its own precedent for the honest-skip
//! behavior — silently "cleaning up" the fixture now would orphan that
//! cross-reference and contradict an already-landed, already-passing
//! test's own stated rationale; (2) the mix is free,
//! real coverage: it proves `to_pilot_receipt`'s feat-resolution loop
//! (`entry.key == feat_id || entry.name == feat_id`) honestly skips 3
//! unmatched ids in the *exact same* character that also has 3 correctly
//! resolved feats, in one fixture, with no risk of dishonesty (the
//! skipped ids are not fabricated into anything — `receipt.feats.len()`
//! is asserted to be exactly 3 below, not 6). The alternative (removing
//! the 3 namespaced ids) would make the fixture marginally "cleaner" but
//! would delete this coverage and break the cross-reference above for no
//! net gain.
//!
//! # The "8 wire-fixture parity fixtures" gate (now met)
//!
//! `SD-20-rules-engine-completeness-loop-instruction.md`'s closure
//! definition and
//! `programs/codex/requirements/SD-20-rules-engine-completeness/acceptance-and-verification.md`
//! gate 3 both require "at least 8 wire-fixture parity JSON fixtures ...
//! one for boundary contract, one each for spellbook / feat prereqs /
//! skill ranks / equipment effects / damage total / Level Up grants /
//! integration closure." At Epic 8's original closure only one such
//! fixture existed on disk. As of this cycle, `tests/fixtures/wire/sd20/`
//! holds all 8: `boundary_contract_parity.json`,
//! `human_fighter_level_1_tabletop.json` (this file's own fixture),
//! `spellbook_parity.json`, `feat_prereqs_parity.json`,
//! `skill_allocation_parity.json`, `equipment_effects_parity.json`,
//! `damage_total_parity.json`, `level_up_parity.json` — landed
//! incrementally by the epic cycles between Epic 8's original closure and
//! this wiring project's own cycles. Gate 3 is met.

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
    classify_character_input, compute_level_up_preview, printed_sheet_cell_map, to_pilot_receipt,
    PrintedSheetCellValue, ResolvedFeat,
};
use codex::rules_core::damage_total::{
    resolve_base_damage_dice, resolve_critical_multiplier, resolve_critical_threat_range,
    resolve_feat_damage_effect, resolve_str_damage_modifier, resolve_weapon_damage_breakdown,
    resolve_weapon_enhancement_modifier, WeaponHandSlot,
};
use codex::rules_core::equipment_effects::compute_equipment_effects;
use codex::rules_core::feat_prereqs::{compute_feat_effects, evaluate_feat_prerequisites, FeatKey};
use codex::rules_core::level_up::compute_level_up_grants;
use codex::rules_core::pilot_compute_corpus::{compute_pilot_with_corpus, DerivedEquipmentStats};
use codex::rules_core::rules_tables::crb::feats::{feat_tables, FeatCategory};
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
                applied_modifiers: Vec::new(),
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
            selected_traits: Vec::new(),
            race_id: input.get("race_id").as_str().to_owned(),
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

/// Real verbatim rows copied from the live corpus
/// (`$HOME/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/`):
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
    // As of the `contract:skill_wiring` cycle (`adaptive-squishing-mccarthy.md`),
    // two cells (`sheet.skill.diplomacy`, `sheet.skill.disable_device`) are
    // *honestly* Blocked for this fixture, and that is not a regression of
    // the "fully-supported Fighter never Blocked" invariant below: this
    // fixture's `input.skill_allocations` deliberately allocates exactly
    // climb/intimidate/swim (a level-1 human Fighter's real skill-point
    // budget: 2 base + 1 Int-0 + 1 human bonus = 3 points, already fully
    // spent -- allocating Diplomacy/Disable Device too would fabricate
    // skill points this character doesn't have). `allocate_skill_ranks`
    // has no entry in `SkillTotals.totals` for a skill the character never
    // allocated ranks to at all (see `contract.rs::printed_sheet_cell_map`'s
    // doc comment), so `Blocked` here means "this character never trained
    // this skill", a different and legitimate case from the
    // unsupported-chassis `Blocked` the rest of this assertion still
    // guards against.
    const HONESTLY_UNTRAINED_SKILL_CELLS: &[&str] =
        &["sheet.skill.diplomacy", "sheet.skill.disable_device"];
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
            "Blocked" => assert!(
                HONESTLY_UNTRAINED_SKILL_CELLS.contains(&cell.cell_id.as_str()),
                "the tabletop-readiness fixture's cells must never be Blocked by an unsupported \
                 chassis posture -- a level-1 Fighter is a fully-supported class per Epic 1's own \
                 contract; a Blocked entry here in expected_output for any cell other than {:?} \
                 would itself be a fixture-authoring bug: got Blocked for {}",
                HONESTLY_UNTRAINED_SKILL_CELLS,
                cell.cell_id
            ),
            other => panic!("unrecognized expected cell value kind: {other:?}"),
        }
    }
    // Doubly explicit, independent of the fixture's own JSON: no cell in
    // the live output is ever Blocked for this fully-supported Fighter
    // *because of chassis support* -- per this epic's brief point 2 ("no
    // PrintedSheetCellValue::Blocked cells for anything a level-1 fighter
    // should have fully supported"). The two honestly-untrained skill
    // cells above are the sole, documented exception (see the comment
    // above): they are Blocked because this character never allocated
    // ranks to those skills, not because the chassis is unsupported.
    assert!(
        cells.iter().all(|cell| !matches!(cell.value, PrintedSheetCellValue::Blocked)
            || HONESTLY_UNTRAINED_SKILL_CELLS.contains(&cell.cell_id.as_str())),
        "no cell may render Blocked for this fully-supported level-1 Fighter posture, except the \
         two honestly-untrained skill cells {HONESTLY_UNTRAINED_SKILL_CELLS:?}: {cells:?}"
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

    // -------------------------------------------------------------
    // Closing proof (Cycle 7, `integration:epic_wiring_closure`): for
    // each of the 6 newly-wired PilotReceipt fields, the wired path
    // (`receipt.<field>`, produced above by the real `to_pilot_receipt`
    // call) must be byte-identical to that epic's own function called
    // directly on this exact fixture's CharacterInput/corpus -- not just
    // that both independently exist, but that they agree. This is what
    // closes this file's own Finding 1 (see this file's module doc
    // comment for the full cycle-SHA history).
    // -------------------------------------------------------------

    // (1) Skills (Epic 4) -- cross-references
    // epic_probe_skill_allocation_covers_class_cross_class_and_untrained_breadth
    // below, which proves allocate_skill_ranks itself is real and
    // correct; this assertion proves the wired path reaches the exact
    // same engine, not a reinterpretation of it.
    let direct_skills = allocate_skill_ranks(&input);
    assert_eq!(
        receipt.skills, direct_skills,
        "receipt.skills must be byte-identical to a direct allocate_skill_ranks(&input) call \
         on this fixture's CharacterInput"
    );
    // Real, non-vacuous values (mirrors the sheet.skill.* cell assertions
    // above, checked here directly against the SkillTotals field so the
    // cell-map round trip and the raw field are both pinned).
    assert_eq!(
        direct_skills
            .totals
            .get("skill:climb")
            .expect("climb must resolve")
            .total_modifier,
        7
    );
    assert_eq!(
        direct_skills
            .totals
            .get("skill:intimidate")
            .expect("intimidate must resolve")
            .total_modifier,
        3
    );
    assert_eq!(
        direct_skills
            .totals
            .get("skill:swim")
            .expect("swim must resolve")
            .total_modifier,
        7
    );

    // (2) Spellbook (Epic 2) -- cross-references
    // epic_probe_spellbook_is_honestly_empty_for_a_non_caster_fighter
    // below. A Fighter fixture legitimately has no spells; the "closure"
    // story here is proving the wired path is honest about that
    // emptiness, not manufacturing a caster scenario just to get
    // non-empty spell data (see this file's brief: forcing a caster
    // scenario into a Fighter fixture would be dishonest).
    let direct_spellbook = compute_spellbook_coverage(&input, &corpus);
    assert_eq!(
        receipt.spellbook, direct_spellbook,
        "receipt.spellbook must be byte-identical to a direct compute_spellbook_coverage(&input, \
         &corpus) call on this fixture's CharacterInput/corpus"
    );
    assert_eq!(
        receipt.spellbook,
        codex::rules_core::spellbook::SpellbookCoverage::default(),
        "this Fighter fixture has no spells_selected, so the wired spellbook field must be \
         genuinely empty, not fabricated"
    );
    assert!(
        !cells
            .iter()
            .any(|cell| cell.cell_id.starts_with("sheet.spellbook.")),
        "a non-caster fixture must produce zero sheet.spellbook.* cells of any kind: {:?}",
        cells
            .iter()
            .filter(|cell| cell.cell_id.starts_with("sheet.spellbook."))
            .collect::<Vec<_>>()
    );

    // (3) Feats (Epic 3) -- cross-references
    // epic_probe_feat_prereqs_resolve_real_effects_for_this_fighters_feats
    // below. Replicates contract.rs::to_pilot_receipt's exact resolution
    // algorithm (scan selected_feats, match entry.key == feat_id ||
    // entry.name == feat_id against feat_tables(), build one ResolvedFeat
    // per match) independently in this test, then asserts the wired
    // receipt.feats equals it -- proving the wiring composes the engine's
    // real output, not a reinterpretation. This fixture's selected_feats
    // deliberately mixes 3 unmatched namespaced ids with 3 matching plain
    // names (see this file's module doc comment, "Feat-id fixture quirk"
    // section, for why that mix is kept intentionally) -- so this
    // assertion also proves the honest-skip behavior end to end through
    // the wired path, not just in isolation.
    let expected_feats: Vec<ResolvedFeat> = input
        .chosen
        .selected_feats
        .iter()
        .filter_map(|feat_id| {
            feat_tables()
                .iter()
                .find(|entry| entry.key == feat_id || entry.name == feat_id)
        })
        .map(|matched_entry| {
            let key = FeatKey {
                feat_id: matched_entry.key.to_string(),
                category: matched_entry.category,
            };
            ResolvedFeat {
                feat_id: matched_entry.key.to_string(),
                prerequisites: evaluate_feat_prerequisites(&key),
                effects: compute_feat_effects(&key),
            }
        })
        .collect();
    assert_eq!(
        receipt.feats, expected_feats,
        "receipt.feats must be byte-identical to independently replaying to_pilot_receipt's own \
         documented feat-resolution algorithm against this fixture's selected_feats"
    );
    assert_eq!(
        receipt.feats.len(),
        3,
        "exactly 3 of this fixture's 6 selected_feats entries match the catalog (Dodge, Weapon \
         Focus, Power Attack); the 3 namespaced ids (feat:dodge, feat:weapon_focus, \
         feat:power_attack) must be honestly skipped, not fabricated into 6 entries"
    );
    let resolved_feat_ids: Vec<&str> = receipt
        .feats
        .iter()
        .map(|resolved| resolved.feat_id.as_str())
        .collect();
    assert_eq!(resolved_feat_ids, vec!["Dodge", "Weapon Focus", "Power Attack"]);

    // (4) Equipment effects (Epic 5) -- cross-references
    // epic_probe_equipment_effects_are_real_but_not_reflected_in_corpus_derived
    // below (whose own name records the *old*, now-superseded gap: that
    // probe's title is a fossil of Finding 1 before this cycle closed it
    // -- Epic 5's output IS now reflected in receipt.equipment_effects,
    // just still not in corpus_derived.equipped_items[].derived_stats,
    // which remains the deliberately out-of-scope SD-19 stub per the
    // plan file's fact 2).
    let equipped_active: Vec<EquipmentSelection> = input
        .chosen
        .equipment_selections
        .iter()
        .filter(|selection| selection.active_state == ActiveState::EquippedActive)
        .cloned()
        .collect();
    let direct_equipment_effects = compute_equipment_effects(&equipped_active, &corpus);
    assert_eq!(
        receipt.equipment_effects, direct_equipment_effects,
        "receipt.equipment_effects must be byte-identical to a direct \
         compute_equipment_effects(&equipped_active, &corpus) call over the same \
         EquippedActive-filtered slice"
    );
    assert_eq!(receipt.equipment_effects.armor_class_delta, 4);
    assert_eq!(receipt.equipment_effects.max_dex_cap, Some(4));

    // (5) Weapon damage (Epic 6) -- cross-references
    // epic_probe_damage_total_covers_base_dice_str_enhancement_crit_and_honest_feat_absence
    // below. Reuses the exact same equipment_effects local just computed
    // above (matching to_pilot_receipt's own internal reuse discipline --
    // see contract.rs's PilotReceipt.weapon_damage doc comment) and the
    // chassis's real STR modifier.
    let direct_weapon_damage = resolve_weapon_damage_breakdown(
        &input,
        &corpus,
        &direct_equipment_effects,
        receipt.chassis.ability_modifiers.strength,
    );
    assert_eq!(
        receipt.weapon_damage, direct_weapon_damage,
        "receipt.weapon_damage must be byte-identical to a direct \
         resolve_weapon_damage_breakdown(...) call over the same equipment_effects and STR \
         modifier"
    );
    assert_eq!(
        receipt.weapon_damage.len(),
        1,
        "exactly one weapon (the Longsword) should appear; the Chain Shirt and the Masterwork \
         weapon-quality record carry no DAMAGE: token and must be excluded"
    );
    // weapon_item_id is the raw `item_id` string from
    // `equipment_selections`, not the resolved corpus key -- this
    // fixture's equipment_selections use the legacy "item:longsword"
    // namespace (see this file's module doc comment on
    // `corpus_derived.equipped_items`'s normalized-name fallback
    // resolution), so that is what appears here verbatim.
    assert_eq!(receipt.weapon_damage[0].weapon_item_id, "item:longsword");

    // (6) Level Up preview (Epic 7, standalone, not part of PilotReceipt)
    // -- cross-references epic_probe_level_up_grants_fighter_level_1_to_2
    // below, which Cycle 6's own `tests/sd20_contract_level_up_preview.rs`
    // already proved agrees with compute_level_up_preview for this exact
    // transition. This is a final confirming assertion in the closing
    // integration test, not new discovery work.
    let preview = compute_level_up_preview(&input, 1, 2);
    let direct_level_up = compute_level_up_grants(&input, 1, 2);
    assert_eq!(
        preview, direct_level_up,
        "compute_level_up_preview must be byte-identical to a direct \
         compute_level_up_grants(&input, 1, 2) call for this fixture's character"
    );
    let bab_grant = preview
        .automatic_features
        .iter()
        .find(|grant| grant.name.contains("base_attack_bonus"))
        .expect("expected a base_attack_bonus grant for the level 1->2 transition");
    assert_eq!(bab_grant.effects[0].value, 2, "fighter level 2 full BAB must be +2");
    let bravery_grant = preview
        .automatic_features
        .iter()
        .find(|grant| grant.name.contains("bravery"))
        .expect("expected a bravery grant at level 2");
    assert_eq!(bravery_grant.effects[0].value, 1);
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
// `combat.baseline_unsupported` and `defense.total_save.unsupported`
// (the third diagnostic Finding 2 named, not separately exercised below
// since it always co-fires with `class_chassis.unsupported` for the
// Fighter-only compute surface this suite exercises), each OR'd
// additively with the universal `class_chassis.unsupported` fallback --
// see `contract.rs`'s `COMBAT_BASELINE_UNSUPPORTED_DIAGNOSTIC_ID` /
// `TOTAL_SAVE_UNSUPPORTED_DIAGNOSTIC_ID` constants.
//
// UPDATE (`contract:skill_wiring` cycle, `adaptive-squishing-mccarthy.md`):
// the skill half of Finding 2's fix (gating `sheet.skill.*` on
// `skill.selected_modifier.unsupported` /
// `SKILL_SELECTED_MODIFIER_UNSUPPORTED_DIAGNOSTIC_ID`) has since been
// *superseded*, not merely extended: `sheet.skill.*` cells no longer
// source from the chassis field that diagnostic gates at all -- they
// source from Epic 4's real `allocate_skill_ranks` via
// `PilotReceipt.skills`, whose diagnostics are all `claim_blocking:
// false`. `SKILL_SELECTED_MODIFIER_UNSUPPORTED_DIAGNOSTIC_ID` has been
// deleted from `contract.rs` (dead code); the diagnostic id string itself
// still fires from `pilot_compute.rs` (untouched trunk) and still appears
// in `receipt.diagnostics`, it just no longer gates any cell.
// `tabletop_readiness_selected_skill_posture_deviation_no_longer_blocks_skill_cells`
// below (originally `..._is_blocked_not_zeroed`) now asserts this
// superseding behavior instead of the old fixed-but-now-stale one; the
// combat-baseline test below is unaffected (combat-baseline gating is
// unchanged by this cycle).
// ---------------------------------------------------------------------

/// A variant of the fixture's base character with Weapon Focus (and its
/// canonical `choice:fighter_bonus_feat` selection) dropped, which trips
/// `compute_combat_baseline`'s `unmet_combat_posture_conditions` without
/// touching anything the selected-skill posture depends on.
///
/// **Was Dodge; is now Weapon Focus, and that swap is the point.** Dodge
/// stopped being a *precondition* of this baseline (it is a conditional
/// contribution now -- `compute_combat_baseline`'s `dodge_armor_class_bonus`),
/// because requiring it forced `compose_character_input` to claim
/// `feat:dodge` for every freshly created character whether or not any
/// choice slot had granted it. So dropping Dodge is no longer a deviation
/// at all, and a fixture built on it would assert nothing. Weapon Focus is
/// still genuinely required -- this baseline's whole attack formula is
/// Longsword-specific -- so it is the honest replacement vehicle. Every
/// assertion below is unchanged: what this test protects
/// (`printed_sheet_cell_map` renders `Blocked`, never a fabricated
/// `Number(0)`, when a claim-blocking diagnostic fires) is exactly as
/// strict as it was.
///
/// **Every shape of Weapon Focus is dropped, not just the
/// `feat:weapon_focus` spelling.** This fixture's own `selected_feats`
/// carries it twice -- once as the engine token `"feat:weapon_focus"` and
/// once as the catalog key `"Weapon Focus"` the feat picker sends -- which
/// is exactly the mixed-shape list real saved characters hold. The posture
/// gate matches through `rules_core::feat_identity`'s shared fold rather
/// than by string equality (so a player who picks it from the catalog is
/// not left with no combat baseline at all), and under that fold removing
/// one spelling leaves the character still holding the feat. Removing only
/// `"feat:weapon_focus"` would make this test assert "a character who HAS
/// Weapon Focus is blocked", which is the opposite of its intent.
fn character_missing_weapon_focus_feat() -> CharacterInput {
    let fixture = load_fixture();
    let mut input = character_input_from_fixture(fixture.get("input"));
    input
        .chosen
        .selected_feats
        .retain(|feat| !codex::rules_core::feat_identity::same(feat, "feat:weapon_focus"));
    input
        .chosen
        .selected_choices
        .retain(|choice| choice.choice_set_id != "choice:fighter_bonus_feat");
    input
}

#[test]
fn tabletop_readiness_combat_baseline_deviation_is_blocked_not_zeroed() {
    let input = character_missing_weapon_focus_feat();
    let corpus = corpus_with_fighter_gear();
    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);

    assert!(
        receipt
            .diagnostics
            .iter()
            .any(|d| d.id == "combat.baseline_unsupported" && d.claim_blocking),
        "dropping feat:weapon_focus must trip combat.baseline_unsupported (claim_blocking: true): {:?}",
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

/// Superseded by the `contract:skill_wiring` cycle
/// (`adaptive-squishing-mccarthy.md`), which is a deliberate,
/// documented behavior change (the plan's "Skill wiring replaces, not
/// adds" design decision), not a silent regression: originally named
/// `tabletop_readiness_selected_skill_posture_deviation_is_blocked_not_zeroed`
/// and asserted that widening skill_allocations beyond the chassis's old
/// exact Climb/Intimidate/Swim rank-1 posture check
/// (`unmet_selected_skill_posture_conditions`) blocked the
/// `sheet.skill.*` cells via claim-blocking `skill.selected_modifier.unsupported`.
/// That diagnostic still fires -- `pilot_compute.rs` is untouched trunk --
/// but `sheet.skill.*` cells no longer source from the chassis field it
/// gates at all: they source from `PilotReceipt.skills`
/// (`skill_allocation::allocate_skill_ranks`), whose diagnostics are all
/// `claim_blocking: false` and which has no notion of "exactly these
/// three skills and no more". So the same input that used to render
/// `Blocked` for climb/intimidate/swim now correctly renders their real,
/// independently-computed totals -- a real, out-of-the-box improvement
/// this project's wiring is meant to deliver (see
/// `tests/sd20_contract_skill_wiring.rs` for the dedicated regression
/// test), not a bug.
#[test]
fn tabletop_readiness_selected_skill_posture_deviation_no_longer_blocks_skill_cells() {
    let fixture = load_fixture();
    let mut input = character_input_from_fixture(fixture.get("input"));
    // Same deviation as before this cycle: widen beyond the exact
    // Climb/Intimidate/Swim rank-1 posture `unmet_selected_skill_posture_conditions`
    // requires -- a single extra, otherwise perfectly legal skill
    // allocation is enough to trip it.
    input.chosen.skill_allocations.push(SkillAllocation {
        skill_id: "skill:diplomacy".to_string(),
        ranks: 1,
    });

    let corpus = corpus_with_fighter_gear();
    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);

    // The old chassis-level diagnostic still fires (unrelated trunk
    // behavior, unchanged by this cycle) -- confirming this test still
    // exercises the exact same deviation as before.
    assert!(
        receipt
            .diagnostics
            .iter()
            .any(|d| d.id == "skill.selected_modifier.unsupported" && d.claim_blocking),
        "adding a diplomacy allocation must still trip skill.selected_modifier.unsupported \
         (claim_blocking: true) at the chassis level: {:?}",
        receipt.diagnostics
    );
    assert_eq!(receipt.chassis.selected_skill_modifiers.climb, 0);

    // Epic 4's allocate_skill_ranks has no notion of that chassis posture
    // and computes climb/intimidate/swim exactly as it would without the
    // extra diplomacy allocation (each skill is computed independently).
    let direct = allocate_skill_ranks(&input);
    let expected_climb = direct.totals.get("skill:climb").expect("climb resolves").total_modifier;
    let expected_intimidate = direct
        .totals
        .get("skill:intimidate")
        .expect("intimidate resolves")
        .total_modifier;
    let expected_swim = direct.totals.get("skill:swim").expect("swim resolves").total_modifier;

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
    // The load-bearing assertion: despite the old chassis-level diagnostic
    // firing claim_blocking: true above, these cells now render the real
    // Numbers Epic 4 computed -- never Blocked.
    assert_eq!(
        climb_cell.value,
        PrintedSheetCellValue::Number(expected_climb as i16),
        "sheet.skill.climb must render the real allocate_skill_ranks total, not Blocked, even \
         though the old chassis single-posture diagnostic still fires"
    );
    assert_eq!(
        intimidate_cell.value,
        PrintedSheetCellValue::Number(expected_intimidate as i16)
    );
    assert_eq!(swim_cell.value, PrintedSheetCellValue::Number(expected_swim as i16));
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

    // Re-derived 2026-09-01 (`AT-34-E3-003` / `c5c4a1b788`): Fighter's real, full
    // 62-skill class-skill list, same widening as tests/sd20_skill_allocation_class_skill.rs.
    assert_eq!(
        totals.class_skills,
        vec![
            "skill:climb".to_string(),
            "skill:craft_alchemy".to_string(),
            "skill:craft_armor".to_string(),
            "skill:craft_baskets".to_string(),
            "skill:craft_blacksmithing".to_string(),
            "skill:craft_books".to_string(),
            "skill:craft_bows".to_string(),
            "skill:craft_calligraphy".to_string(),
            "skill:craft_carpentry".to_string(),
            "skill:craft_cloth".to_string(),
            "skill:craft_clothing".to_string(),
            "skill:craft_gemcutting".to_string(),
            "skill:craft_glass".to_string(),
            "skill:craft_jewelry".to_string(),
            "skill:craft_leather".to_string(),
            "skill:craft_locks".to_string(),
            "skill:craft_paintings".to_string(),
            "skill:craft_pottery".to_string(),
            "skill:craft_sculptures".to_string(),
            "skill:craft_ships".to_string(),
            "skill:craft_shoes".to_string(),
            "skill:craft_stonemasonry".to_string(),
            "skill:craft_traps".to_string(),
            "skill:craft_weapons".to_string(),
            "skill:handle_animal".to_string(),
            "skill:intimidate".to_string(),
            "skill:knowledge_dungeoneering".to_string(),
            "skill:knowledge_engineering".to_string(),
            "skill:profession_architect".to_string(),
            "skill:profession_baker".to_string(),
            "skill:profession_barrister".to_string(),
            "skill:profession_brewer".to_string(),
            "skill:profession_butcher".to_string(),
            "skill:profession_clerk".to_string(),
            "skill:profession_cook".to_string(),
            "skill:profession_courtesan".to_string(),
            "skill:profession_driver".to_string(),
            "skill:profession_engineer".to_string(),
            "skill:profession_farmer".to_string(),
            "skill:profession_fisherman".to_string(),
            "skill:profession_gambler".to_string(),
            "skill:profession_gardener".to_string(),
            "skill:profession_herbalist".to_string(),
            "skill:profession_innkeeper".to_string(),
            "skill:profession_librarian".to_string(),
            "skill:profession_merchant".to_string(),
            "skill:profession_midwife".to_string(),
            "skill:profession_miller".to_string(),
            "skill:profession_miner".to_string(),
            "skill:profession_porter".to_string(),
            "skill:profession_sailor".to_string(),
            "skill:profession_scribe".to_string(),
            "skill:profession_shepherd".to_string(),
            "skill:profession_soldier".to_string(),
            "skill:profession_soothsayer".to_string(),
            "skill:profession_stable_master".to_string(),
            "skill:profession_tanner".to_string(),
            "skill:profession_trapper".to_string(),
            "skill:profession_woodcutter".to_string(),
            "skill:ride".to_string(),
            "skill:survival".to_string(),
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
            applied_modifiers: Vec::new(),
        },
        EquipmentSelection {
            item_id: "Chain Shirt (Base)".to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
        },
        EquipmentSelection {
            item_id: "Special Quality ~ Masterwork ~ Weapon".to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
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
    assert_eq!(enhancement.tohit_bonus, Some(1));
    assert_eq!(enhancement.damage_bonus, None);
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
            applied_modifiers: Vec::new(),
        },
        EquipmentSelection {
            item_id: "Special Quality ~ Masterwork ~ Weapon".to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
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
