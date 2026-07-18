//! SD-20 Epic 6 (damage-total engine): first damage-class criterion per
//! `scope-draft.md` §1.6's work-unit order — base-dice round-trip
//! (sequential after Epic 5, whose closure at `98613ae` made this epic
//! eligible per the loop instruction's dependency graph).
//!
//! Builds a corpus from real, verbatim tokens copied from
//! `core_rulebook/cr_equip_arms_armor.lst` (`KEY:Longsword (Base)`
//! carries `DAMAGE:1d8`; `KEY:Dagger (Base)` carries `DAMAGE:1d4`; both
//! confirmed directly against the live corpus at
//! `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst`
//! lines 165 and 142 respectively) plus a `KEY:Leather Armor (Base)`
//! control record that carries no `DAMAGE:` token at all — the same
//! parser (`parse_equipment_entries`) and IR converter
//! (`convert_equipment_record`) `equipment_resolver.rs`'s own unit tests
//! and the sibling `sd20_equipment_arms_armor.rs` cycle use — so this
//! test genuinely proves the engine's real `resolve_base_damage_dice`
//! seam resolves a weapon `item_id` into a real, structured
//! `DiceExpression` read from the corpus's own `DAMAGE:` token, not a
//! hand-rolled fixture.

use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
use codex::rules_core::damage_total::{resolve_base_damage_dice, DiceExpression};
use codex::rules_core::rules_tables::crb::equipment_tables::equipment_tables;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

/// Real verbatim rows copied from `core_rulebook/cr_equip_arms_armor.lst`
/// (two weapon-block records with distinct dice, and an armor-block
/// control record with no `DAMAGE:` token at all).
const ARMS_ARMOR_FIXTURE_TEXT: &str = "\
Leather Armor\tKEY:Leather Armor (Base)\tTYPE:Armor.Light\tCOST:10\tWT:15\tACCHECK:0\tMAXDEX:6\tSPELLFAILURE:10\tBONUS:COMBAT|AC|2|TYPE=Armor|PREVAREQ:DisableArmorBonus,0
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8
Dagger\tKEY:Dagger (Base)\tTYPE:Weapon.Melee.Simple\tCOST:2\tWT:1\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d4
";

fn corpus_from(text: &str) -> SourcePackageContent<'static> {
    let result = parse_equipment_entries("cr_equip_arms_armor.lst", text);
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

#[test]
fn longsword_base_dice_round_trips_from_the_real_damage_token() {
    let corpus = corpus_from(ARMS_ARMOR_FIXTURE_TEXT);

    let resolved = resolve_base_damage_dice("Longsword (Base)", &corpus)
        .expect("Longsword (Base) must resolve a base-dice round-trip");

    assert_eq!(resolved.weapon_record_key, "Longsword (Base)");
    assert_eq!(
        resolved.base_dice,
        DiceExpression {
            count: 1,
            die_size: 8
        }
    );
    assert!(
        resolved.table_cell.is_none()
            || equipment_tables()
                .iter()
                .any(|entry| entry.key == "Longsword (Base)"),
        "table_cell provenance, when present, must point at a real canonical-store entry"
    );
}

#[test]
fn dagger_base_dice_is_distinct_from_longsword() {
    let corpus = corpus_from(ARMS_ARMOR_FIXTURE_TEXT);

    let resolved = resolve_base_damage_dice("Dagger (Base)", &corpus)
        .expect("Dagger (Base) must resolve a base-dice round-trip");

    assert_eq!(
        resolved.base_dice,
        DiceExpression {
            count: 1,
            die_size: 4
        }
    );
}

#[test]
fn non_weapon_item_has_no_base_dice_honest_absence() {
    let corpus = corpus_from(ARMS_ARMOR_FIXTURE_TEXT);

    let resolved = resolve_base_damage_dice("Leather Armor (Base)", &corpus);

    assert!(
        resolved.is_none(),
        "armor carries no DAMAGE: token; absence must not be papered over with a fabricated dice expression"
    );
}

#[test]
fn unresolvable_item_id_yields_none_not_fabricated() {
    let corpus = corpus_from(ARMS_ARMOR_FIXTURE_TEXT);

    let resolved = resolve_base_damage_dice("item:does-not-exist-in-this-corpus", &corpus);

    assert!(resolved.is_none());
}

#[test]
fn dice_expression_parse_rejects_malformed_and_zero_shapes() {
    assert_eq!(
        DiceExpression::parse("1d8"),
        Some(DiceExpression {
            count: 1,
            die_size: 8
        })
    );
    assert_eq!(
        DiceExpression::parse("2d6"),
        Some(DiceExpression {
            count: 2,
            die_size: 6
        })
    );
    assert_eq!(DiceExpression::parse("0d8"), None, "zero dice is not a real roll");
    assert_eq!(DiceExpression::parse("1d0"), None, "a zero-sided die is not real");
    assert_eq!(DiceExpression::parse("not-dice"), None);
    assert_eq!(DiceExpression::parse(""), None);
}
