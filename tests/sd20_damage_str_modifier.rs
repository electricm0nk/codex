//! SD-20 Epic 6 (damage-total engine): second damage-class criterion per
//! `scope-draft.md` §1.6's work-unit order — STR-modifier handling
//! (sequential after the first work-unit, base-dice round-trip, landed
//! at `208f326`).
//!
//! PF1's Strength Bonus rule (CRB p.187, "Strength Bonus"): a melee
//! weapon wielded one-handed (or a light weapon) in the primary hand
//! adds the wielder's full STR modifier to damage; a weapon wielded
//! two-handed adds 1.5x the STR modifier; an off-hand weapon adds only
//! 0.5x. Fractions are always rounded down, even when the fraction is
//! exactly one-half or the modifier itself is negative.
//!
//! Builds a corpus from real, verbatim tokens copied from
//! `core_rulebook/cr_equip_arms_armor.lst` — confirmed directly against
//! the live corpus at
//! `$HOME/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst`:
//! `KEY:Longsword (Base)` (line 165) carries `WIELD:OneHanded`,
//! `KEY:Dagger (Base)` (line 142) carries `WIELD:Light`, `KEY:Longspear
//! (Base)` (line 151) carries `WIELD:TwoHanded`. `KEY:Leather Armor
//! (Base)` carries no `WIELD:` token at all (a control record, same
//! fixture the sibling `sd20_damage_base_dice.rs` cycle already used for
//! its own "no token" honest-absence case). Same parser
//! (`parse_equipment_entries`) and IR converter
//! (`convert_equipment_record`) the sibling tests already use — so this
//! test genuinely proves the engine's real `resolve_str_damage_modifier`
//! seam resolves a weapon `item_id`'s `WIELD:` token into a real,
//! structured STR-damage contribution, not a hand-rolled fixture.

use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
use codex::rules_core::damage_total::{
    resolve_str_damage_modifier, WeaponHandSlot, WieldCategory,
};
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

/// Real verbatim rows copied from `core_rulebook/cr_equip_arms_armor.lst`
/// (three weapon-block records spanning all three `WIELD:` categories,
/// and an armor-block control record with no `WIELD:` token at all).
const ARMS_ARMOR_FIXTURE_TEXT: &str = "\
Leather Armor\tKEY:Leather Armor (Base)\tTYPE:Armor.Light\tCOST:10\tWT:15\tACCHECK:0\tMAXDEX:6\tSPELLFAILURE:10\tBONUS:COMBAT|AC|2|TYPE=Armor|PREVAREQ:DisableArmorBonus,0
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Resizable.Melee.Martial.OneHanded.Slashing.Sword.BladeHeavy.Weapon Group Blades Heavy\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\tWIELD:OneHanded\tSIZE:M
Dagger\tKEY:Dagger (Base)\tTYPE:Weapon.Resizable.Melee.Simple.Light.Finesseable.Ranged.Thrown.Piercing.Slashing.Dagger.BladeLight.Weapon Group Blades Light.Weapon Group Thrown\tCOST:2\tWT:1\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d4\tWIELD:Light\tRANGE:10\tSIZE:M
Longspear\tKEY:Longspear (Base)\tTYPE:Weapon.Resizable.Melee.Simple.TwoHanded.Piercing.Spear.Weapon Group Spears\tCOST:5\tWT:9\tCRITMULT:x3\tCRITRANGE:1\tDAMAGE:1d8\tWIELD:TwoHanded\tREACHMULT:2\tSIZE:M
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
fn one_handed_weapon_in_primary_hand_adds_full_str_modifier() {
    let corpus = corpus_from(ARMS_ARMOR_FIXTURE_TEXT);

    let resolved =
        resolve_str_damage_modifier("Longsword (Base)", &corpus, 3, WeaponHandSlot::Primary)
            .expect("Longsword (Base) must resolve a STR-modifier round-trip");

    assert_eq!(resolved.weapon_record_key, "Longsword (Base)");
    assert_eq!(resolved.wield_category, WieldCategory::OneHanded);
    assert_eq!(resolved.str_damage_modifier, 3, "one-handed primary hand gets full STR mod");
}

#[test]
fn two_handed_weapon_adds_one_and_a_half_times_str_modifier_rounded_down() {
    let corpus = corpus_from(ARMS_ARMOR_FIXTURE_TEXT);

    let resolved =
        resolve_str_damage_modifier("Longspear (Base)", &corpus, 3, WeaponHandSlot::Primary)
            .expect("Longspear (Base) must resolve a STR-modifier round-trip");

    assert_eq!(resolved.wield_category, WieldCategory::TwoHanded);
    assert_eq!(
        resolved.str_damage_modifier, 4,
        "1.5 * 3 = 4.5, rounded down to 4"
    );
}

#[test]
fn off_hand_weapon_adds_half_str_modifier_rounded_down() {
    let corpus = corpus_from(ARMS_ARMOR_FIXTURE_TEXT);

    let resolved = resolve_str_damage_modifier("Dagger (Base)", &corpus, 3, WeaponHandSlot::OffHand)
        .expect("Dagger (Base) must resolve a STR-modifier round-trip");

    assert_eq!(resolved.wield_category, WieldCategory::Light);
    assert_eq!(
        resolved.str_damage_modifier, 1,
        "0.5 * 3 = 1.5, rounded down to 1"
    );
}

#[test]
fn negative_str_modifier_still_rounds_down_even_below_zero() {
    let corpus = corpus_from(ARMS_ARMOR_FIXTURE_TEXT);

    let resolved =
        resolve_str_damage_modifier("Dagger (Base)", &corpus, -1, WeaponHandSlot::OffHand)
            .expect("Dagger (Base) must resolve a STR-modifier round-trip");

    assert_eq!(
        resolved.str_damage_modifier, -1,
        "0.5 * -1 = -0.5, rounded down to -1 per CRB's 'even if the total is 0 or less' rule"
    );
}

#[test]
fn light_weapon_wielded_in_primary_hand_adds_full_str_modifier() {
    let corpus = corpus_from(ARMS_ARMOR_FIXTURE_TEXT);

    let resolved = resolve_str_damage_modifier("Dagger (Base)", &corpus, 2, WeaponHandSlot::Primary)
        .expect("Dagger (Base) must resolve a STR-modifier round-trip");

    assert_eq!(resolved.str_damage_modifier, 2, "light weapon in primary hand gets full STR mod");
}

#[test]
fn non_weapon_item_has_no_str_modifier_honest_absence() {
    let corpus = corpus_from(ARMS_ARMOR_FIXTURE_TEXT);

    let resolved =
        resolve_str_damage_modifier("Leather Armor (Base)", &corpus, 3, WeaponHandSlot::Primary);

    assert!(
        resolved.is_none(),
        "armor carries no WIELD: token; absence must not be papered over with a fabricated modifier"
    );
}

#[test]
fn unresolvable_item_id_yields_none_not_fabricated() {
    let corpus = corpus_from(ARMS_ARMOR_FIXTURE_TEXT);

    let resolved = resolve_str_damage_modifier(
        "item:does-not-exist-in-this-corpus",
        &corpus,
        3,
        WeaponHandSlot::Primary,
    );

    assert!(resolved.is_none());
}
