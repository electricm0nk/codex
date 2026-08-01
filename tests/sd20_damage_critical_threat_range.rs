//! SD-20 Epic 6 (damage-total engine): fifth damage-class criterion per
//! `scope-draft.md` §1.6's work-unit order — critical-threat-range
//! (sequential after base-dice round-trip `208f326`, STR-modifier
//! handling `f1188fe`, and weapon-enhancement modifier `1eb2eec`; the
//! fourth work-unit, feat-effect modifier, is a separate concurrent
//! cycle's territory and is not touched here).
//!
//! PF1's critical-threat-range rule: a weapon threatens a critical hit on
//! a natural attack roll within its threat range (e.g. a longsword
//! threatens on a natural 19 or 20; a rapier threatens on 18-20). The
//! corpus's `CRITRANGE:` token carries the *width* of that range (the
//! count of consecutive top rolls that threaten), not the raw bounds —
//! `CRITRANGE:1` means "threatens only on a natural 20" (width 1),
//! `CRITRANGE:2` means "threatens on 19-20" (width 2), `CRITRANGE:3`
//! means "threatens on 18-20" (width 3). This work-unit resolves a
//! weapon's `item_id` against the corpus (the identical
//! `equipment_id_resolve` path `resolve_base_damage_dice` and
//! `resolve_str_damage_modifier` already use, and the same token-reading
//! pattern `equipment_effects/arms_armor.rs` already established for
//! reading a stat straight off a resolved `EquipmentRecord`'s tokens) and
//! converts the raw `CRITRANGE:` width into the inclusive `(low, high)`
//! bounds `technical-design.md` §2.5's illustrative `DamageRoll.
//! critical_threat_range: (u8, u8)` field describes.
//!
//! Builds a corpus from real, verbatim tokens copied from
//! `core_rulebook/cr_equip_arms_armor.lst` (confirmed directly against
//! the live corpus at
//! `$HOME/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst`):
//! `KEY:Longsword (Base)` carries `CRITRANGE:2` (line 165), `KEY:Rapier
//! (Base)` carries `CRITRANGE:3` (line 167), `KEY:Dagger (Base)` carries
//! `CRITRANGE:2` (line 142). Same parser (`parse_equipment_entries`) and
//! IR converter (`convert_equipment_record`) the sibling
//! `sd20_damage_str_modifier.rs` / `sd20_damage_weapon_enhancement.rs`
//! tests already use.

use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
use codex::rules_core::damage_total::resolve_critical_threat_range;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

/// Real verbatim rows from `core_rulebook/cr_equip_arms_armor.lst`:
/// `Longsword (Base)` (`CRITRANGE:2`), `Rapier (Base)` (`CRITRANGE:3`),
/// `Dagger (Base)` (`CRITRANGE:2`), and `Leather Armor (Base)` (no
/// `CRITRANGE:` token at all — armor is not a weapon).
const FIXTURE_TEXT: &str = "\
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Resizable.Melee.Martial.OneHanded.Slashing.Sword.BladeHeavy.Weapon Group Blades Heavy\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\tWIELD:OneHanded\tSIZE:M
Rapier\tKEY:Rapier (Base)\tTYPE:Weapon.Resizable.Melee.Martial.OneHanded.Finesseable.Piercing.MeleePiercing.Sword.BladeLight.Weapon Group Blades Light.Weapon Group Melee OneHanded Piercing\tCOST:20\tWT:2\tCRITMULT:x2\tCRITRANGE:3\tDAMAGE:1d6\tWIELD:OneHanded\tSIZE:M
Dagger\tKEY:Dagger (Base)\tTYPE:Weapon.Resizable.Melee.Simple.Light.Finesseable.Ranged.Thrown.Piercing.Slashing.Dagger.BladeLight.Weapon Group Blades Light.Weapon Group Thrown\tCOST:2\tWT:1\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d4\tWIELD:Light\tSIZE:M
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
fn longsword_threatens_on_19_and_20() {
    let corpus = corpus_from(FIXTURE_TEXT);

    let resolved = resolve_critical_threat_range("Longsword (Base)", &corpus)
        .expect("Longsword (Base) must resolve a critical-threat-range round-trip");

    assert_eq!(resolved.weapon_record_key, "Longsword (Base)");
    assert_eq!(
        resolved.critical_threat_range,
        (19, 20),
        "CRITRANGE:2 means the weapon threatens on natural 19-20"
    );
}

#[test]
fn rapier_threatens_on_18_through_20() {
    let corpus = corpus_from(FIXTURE_TEXT);

    let resolved = resolve_critical_threat_range("Rapier (Base)", &corpus)
        .expect("Rapier (Base) must resolve a critical-threat-range round-trip");

    assert_eq!(
        resolved.critical_threat_range,
        (18, 20),
        "CRITRANGE:3 means the weapon threatens on natural 18-20"
    );
}

#[test]
fn dagger_threatens_on_19_and_20() {
    let corpus = corpus_from(FIXTURE_TEXT);

    let resolved = resolve_critical_threat_range("Dagger (Base)", &corpus)
        .expect("Dagger (Base) must resolve a critical-threat-range round-trip");

    assert_eq!(resolved.critical_threat_range, (19, 20));
}

#[test]
fn armor_record_has_no_critical_threat_range() {
    let corpus = corpus_from(FIXTURE_TEXT);

    assert!(
        resolve_critical_threat_range("Leather Armor (Base)", &corpus).is_none(),
        "armor carries no CRITRANGE: token and must not fabricate a threat range"
    );
}

#[test]
fn unresolvable_item_id_yields_none_not_fabricated() {
    let corpus = corpus_from(FIXTURE_TEXT);

    assert!(resolve_critical_threat_range("item:does-not-exist-in-this-corpus", &corpus).is_none());
}
