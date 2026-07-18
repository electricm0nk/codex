//! SD-20 Epic 6 (damage-total engine): sixth and FINAL damage-class
//! criterion per `scope-draft.md` §1.6's work-unit order —
//! critical-multiplier (sequential after base-dice round-trip `208f326`,
//! STR-modifier handling `f1188fe`, weapon-enhancement modifier `1eb2eec`,
//! feat-effect modifier `e63745b`, and critical-threat-range `0b5dd5e`).
//!
//! **This closes Epic 6** when it lands — all six damage-class criteria
//! done.
//!
//! PF1's critical-multiplier rule: on a confirmed critical hit, a
//! weapon's damage is multiplied by its own critical multiplier (e.g. a
//! longsword's x2, a scythe's x4) rather than a uniform x2 across every
//! weapon. The corpus's `CRITMULT:` token carries this multiplier as an
//! `x<N>` string (e.g. `x2`, `x3`, `x4`). This work-unit resolves a
//! weapon's `item_id` against the corpus (the identical
//! `equipment_id_resolve` path every prior work-unit uses, and the same
//! "read tokens straight off the resolved record" pattern
//! `equipment_effects/arms_armor.rs` already established) and parses the
//! raw `CRITMULT:x<N>` token into the numeric multiplier
//! `technical-design.md` §2.5's illustrative `DamageRoll.
//! critical_multiplier` field describes.
//!
//! Builds a corpus from real, verbatim tokens copied from
//! `core_rulebook/cr_equip_arms_armor.lst` (confirmed directly against
//! the live corpus at
//! `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst`):
//! `KEY:Longsword (Base)` carries `CRITMULT:x2` (line 165), `KEY:Longspear
//! (Base)` carries `CRITMULT:x3` (line 151), `KEY:Scythe (Base)` carries
//! `CRITMULT:x4` (line 181). Same parser (`parse_equipment_entries`) and
//! IR converter (`convert_equipment_record`) the sibling
//! `sd20_damage_critical_threat_range.rs` test already uses.

use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
use codex::rules_core::damage_total::resolve_critical_multiplier;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

/// Real verbatim rows from `core_rulebook/cr_equip_arms_armor.lst`:
/// `Longsword (Base)` (`CRITMULT:x2`), `Longspear (Base)` (`CRITMULT:x3`),
/// `Scythe (Base)` (`CRITMULT:x4`), and `Leather Armor (Base)` (no
/// `CRITMULT:` token at all — armor is not a weapon).
const FIXTURE_TEXT: &str = "\
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Resizable.Melee.Martial.OneHanded.Slashing.Sword.BladeHeavy.Weapon Group Blades Heavy\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\tWIELD:OneHanded\tSIZE:M
Longspear\tKEY:Longspear (Base)\tTYPE:Weapon.Resizable.Melee.Simple.TwoHanded.Piercing.Spear.Weapon Group Spears\tCOST:5\tWT:9\tCRITMULT:x3\tCRITRANGE:1\tDAMAGE:1d8\tWIELD:TwoHanded\tSIZE:M
Scythe\tKEY:Scythe (Base)\tTYPE:Weapon.Resizable.Melee.Martial.TwoHanded.Piercing.Slashing.Scythe.BladeHeavy.Weapon Group Blades Heavy\tCOST:18\tWT:10\tCRITMULT:x4\tCRITRANGE:1\tDAMAGE:2d4\tWIELD:TwoHanded\tSIZE:M
Leather Armor\tKEY:Leather Armor (Base)\tTYPE:Armor.Light\tCOST:10\tWT:15\tACCHECK:0\tMAXDEX:6\tSPELLFAILURE:10\tBONUS:COMBAT|AC|2|TYPE=Armor|PREVAREQ:DisableArmorBonus,0
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
fn longsword_critmult_x2_yields_multiplier_2() {
    let corpus = corpus_from(FIXTURE_TEXT);

    let resolved = resolve_critical_multiplier("Longsword (Base)", &corpus)
        .expect("Longsword (Base) must resolve a critical-multiplier round-trip");

    assert_eq!(resolved.weapon_record_key, "Longsword (Base)");
    assert_eq!(
        resolved.critical_multiplier, 2,
        "CRITMULT:x2 means the weapon's critical hits deal double damage"
    );
}

#[test]
fn longspear_critmult_x3_yields_multiplier_3() {
    let corpus = corpus_from(FIXTURE_TEXT);

    let resolved = resolve_critical_multiplier("Longspear (Base)", &corpus)
        .expect("Longspear (Base) must resolve a critical-multiplier round-trip");

    assert_eq!(resolved.critical_multiplier, 3);
}

#[test]
fn scythe_critmult_x4_yields_multiplier_4() {
    let corpus = corpus_from(FIXTURE_TEXT);

    let resolved = resolve_critical_multiplier("Scythe (Base)", &corpus)
        .expect("Scythe (Base) must resolve a critical-multiplier round-trip");

    assert_eq!(resolved.critical_multiplier, 4);
}

#[test]
fn armor_record_has_no_critical_multiplier() {
    let corpus = corpus_from(FIXTURE_TEXT);

    assert!(
        resolve_critical_multiplier("Leather Armor (Base)", &corpus).is_none(),
        "armor carries no CRITMULT: token and must not fabricate a multiplier"
    );
}

#[test]
fn unresolvable_item_id_yields_none_not_fabricated() {
    let corpus = corpus_from(FIXTURE_TEXT);

    assert!(resolve_critical_multiplier("item:does-not-exist-in-this-corpus", &corpus).is_none());
}
