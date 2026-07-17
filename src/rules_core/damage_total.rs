//! Epic 6 — damage-total engine (SD-20 §1.6).
//!
//! Sequential after Epic 5 (equipment-effect engine, closed at `98613ae`)
//! because the full damage-modifier picture reads from equipment stat
//! breadth (STR mod + weapon enhancement + relevant feat effects) —
//! per `scope-draft.md` §1.6. This is Epic 6's only module (no
//! per-category subdirectory, unlike Epics 2/3/5/7 — the file-touch
//! partition lists `src/rules_core/damage_total.rs` as a single
//! one-cycle-at-a-time file, not a directory of per-category files).
//!
//! Work-unit order per Step 2 (one damage-class criterion per cycle):
//! base-dice round-trip, then STR-modifier handling, then
//! weapon-enhancement modifier, then feat-effect modifier, then
//! critical-threat-range, then critical-multiplier. The first work-unit
//! (base-dice round-trip) landed at `208f326`; this cycle lands the
//! second: STR-modifier handling (`resolve_str_damage_modifier`, per
//! PF1's Strength Bonus rule, CRB p.187 — full STR mod for a
//! one-handed/light primary-hand weapon, 1.5x for a two-handed weapon,
//! 0.5x for an off-hand weapon, verified against the corpus's real
//! `WIELD:` token the same way `resolve_base_damage_dice` reads
//! `DAMAGE:`).
//!
//! Adapts `technical-design.md` §2.5's illustrative `compute_damage`
//! seam to this repo's real types per §2.0 (`RulesTables` retired — no
//! `rules_tables: &RulesTables` parameter anywhere; a table-store read,
//! when this epic needs one, imports the specific
//! `rules_tables::crb::<table>` item directly). The full
//! `compute_damage(attacker, weapon, target, attack_roll) -> DamageRoll`
//! signature is not landed yet — it depends on STR-modifier, weapon-
//! enhancement, feat-effect, and critical-rules work-units this cycle
//! does not touch, and landing it now would mean fabricating those
//! fields. This cycle lands only the base-dice slice of that eventual
//! `DamageRoll`: `resolve_base_damage_dice`, which resolves a weapon
//! `item_id` against the corpus (the exact `equipment_id_resolve` /
//! `equipment_key_token` path `equipment_effects.rs` already uses — see
//! that module's own doc comment) and reads its real `DAMAGE:` token
//! into a structured `DiceExpression`. Verified directly against the
//! live corpus (`core_rulebook/cr_equip_arms_armor.lst`: `KEY:Longsword
//! (Base)` carries `DAMAGE:1d8`, `KEY:Dagger (Base)` carries
//! `DAMAGE:1d4`) — the same `DAMAGE:1d8` token
//! `equipment_effects/arms_armor.rs`'s own unit test already copied
//! verbatim for its weapon-control-record case.
//!
//! This cycle's addition, `resolve_str_damage_modifier`, lands the
//! STR-modifier slice the same way: it resolves a weapon `item_id`
//! against the corpus via the identical `equipment_id_resolve` path,
//! reads its real `WIELD:` token, and computes the STR contribution per
//! PF1's Strength Bonus rule. Verified directly against the live corpus:
//! `KEY:Longsword (Base)` carries `WIELD:OneHanded`, `KEY:Dagger (Base)`
//! carries `WIELD:Light`, `KEY:Longspear (Base)` carries
//! `WIELD:TwoHanded` (`core_rulebook/cr_equip_arms_armor.lst` lines 165,
//! 142, 151 respectively).

use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;
use crate::rules_core::equipment_resolver::{equipment_id_resolve, equipment_key_token};
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::source_content::SourcePackageContent;

/// A PF1 dice expression, e.g. `"1d8"` -> `{ count: 1, die_size: 8 }`,
/// `"2d6"` -> `{ count: 2, die_size: 6 }`. `count` dice, each with
/// `die_size` faces, summed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiceExpression {
    pub count: u8,
    pub die_size: u8,
}

impl DiceExpression {
    /// Parses a raw corpus `DAMAGE:` token value into a structured
    /// `DiceExpression`. Returns `None` for anything that does not match
    /// PF1's canonical `<count>d<size>` shape, including the degenerate
    /// `0d<n>` / `<n>d0` cases — honest absence rather than a fabricated
    /// default roll.
    pub fn parse(raw: &str) -> Option<DiceExpression> {
        let (count_str, size_str) = raw.split_once('d')?;
        let count: u8 = count_str.parse().ok()?;
        let die_size: u8 = size_str.parse().ok()?;
        if count == 0 || die_size == 0 {
            return None;
        }
        Some(DiceExpression { count, die_size })
    }
}

/// One resolved weapon's base damage dice, with its corpus provenance.
/// This is the base-dice slice of the eventual `DamageRoll`
/// (`technical-design.md` §2.5) — `damage_modifier`,
/// `weapon_specialization_bonus`, `critical_threat_range`,
/// `critical_multiplier`, and `expected_damage` are later work-units'
/// fields, not fabricated here.
#[derive(Debug, Clone, PartialEq)]
pub struct DamageRollBaseDice {
    pub weapon_item_id: String,
    pub weapon_record_key: String,
    pub base_dice: DiceExpression,
    pub table_cell: Option<TableCellRef>,
}

/// The damage-total engine's first work-unit (SD-20 §1.6): resolves a
/// weapon selection's `item_id` against the corpus (same resolver path
/// `equipment_effects.rs` uses) and reads its real `DAMAGE:` token into a
/// structured `DiceExpression`.
///
/// Returns `None` when the item does not resolve against the corpus at
/// all, or resolves but carries no `DAMAGE:` token (e.g. armor, or any
/// other non-weapon item) — both are honest absence, not a fabricated
/// dice expression.
pub fn resolve_base_damage_dice(
    weapon_item_id: &str,
    corpus: &SourcePackageContent,
) -> Option<DamageRollBaseDice> {
    let (record, table_cell) = equipment_id_resolve(weapon_item_id, RuleSetId::Crb, corpus)?;
    let base_dice = damage_dice_token(record)?;
    let weapon_record_key = equipment_key_token(record)
        .unwrap_or(&record.name)
        .to_string();

    Some(DamageRollBaseDice {
        weapon_item_id: weapon_item_id.to_string(),
        weapon_record_key,
        base_dice,
        table_cell,
    })
}

fn damage_dice_token(record: &EquipmentRecord) -> Option<DiceExpression> {
    record
        .tokens
        .iter()
        .find(|token| token.key == "DAMAGE")
        .and_then(|token| DiceExpression::parse(&token.value))
}

/// Which of PF1's three `WIELD:` corpus categories a weapon record
/// carries — governs how much of the wielder's STR modifier applies to
/// its damage roll (CRB p.187, "Strength Bonus"): `Light` and
/// `OneHanded` weapons wielded in the primary hand get the wielder's
/// full STR modifier; a `TwoHanded` weapon gets 1.5x; any weapon in the
/// off hand gets 0.5x. Fractions always round down, even when the
/// fraction is exactly one-half or the modifier itself is negative
/// (CRB: "such fractions are always rounded down, even if the total is
/// 0 or less").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WieldCategory {
    Light,
    OneHanded,
    TwoHanded,
}

/// Which hand slot the weapon occupies for this attack — caller-supplied
/// context (a `Light`/`OneHanded` weapon is wielded one-handed by
/// default, but the same physical weapon can be the off-hand weapon in
/// a two-weapon-fighting attack, which halves its STR contribution
/// regardless of its own `WieldCategory`). A `TwoHanded`-category weapon
/// ignores this field (see `str_damage_modifier_for`) — PF1 does not let
/// a two-handed weapon be wielded in an off hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponHandSlot {
    Primary,
    OffHand,
}

/// One resolved weapon's STR-modifier contribution to damage, with its
/// corpus provenance. This is the STR-modifier slice of the eventual
/// `DamageRoll` (`technical-design.md` §2.5) — `weapon_specialization_bonus`,
/// `critical_threat_range`, and `critical_multiplier` are later
/// work-units' fields, not fabricated here. The eventual `DamageRoll`'s
/// `damage_modifier` sums this STR contribution with weapon enhancement
/// and feat effects (later work-units); this slice reports only the STR
/// term.
#[derive(Debug, Clone, PartialEq)]
pub struct DamageRollStrModifier {
    pub weapon_item_id: String,
    pub weapon_record_key: String,
    pub wield_category: WieldCategory,
    pub hand: WeaponHandSlot,
    pub str_damage_modifier: i16,
    pub table_cell: Option<TableCellRef>,
}

/// The damage-total engine's second work-unit (SD-20 §1.6): resolves a
/// weapon selection's `item_id` against the corpus (same resolver path
/// `resolve_base_damage_dice` and `equipment_effects.rs` use), reads its
/// real `WIELD:` token, and computes the wielder's STR-modifier
/// contribution to damage per PF1's Strength Bonus rule (CRB p.187).
///
/// Returns `None` when the item does not resolve against the corpus at
/// all, or resolves but carries no `WIELD:` token (e.g. armor, or any
/// other non-weapon item) — both are honest absence, not a fabricated
/// modifier.
pub fn resolve_str_damage_modifier(
    weapon_item_id: &str,
    corpus: &SourcePackageContent,
    str_modifier: i16,
    hand: WeaponHandSlot,
) -> Option<DamageRollStrModifier> {
    let (record, table_cell) = equipment_id_resolve(weapon_item_id, RuleSetId::Crb, corpus)?;
    let wield_category = wield_category_token(record)?;
    let weapon_record_key = equipment_key_token(record)
        .unwrap_or(&record.name)
        .to_string();
    let str_damage_modifier = str_damage_modifier_for(str_modifier, wield_category, hand);

    Some(DamageRollStrModifier {
        weapon_item_id: weapon_item_id.to_string(),
        weapon_record_key,
        wield_category,
        hand,
        str_damage_modifier,
        table_cell,
    })
}

fn wield_category_token(record: &EquipmentRecord) -> Option<WieldCategory> {
    record
        .tokens
        .iter()
        .find(|token| token.key == "WIELD")
        .and_then(|token| match token.value.as_str() {
            "Light" => Some(WieldCategory::Light),
            "OneHanded" => Some(WieldCategory::OneHanded),
            "TwoHanded" => Some(WieldCategory::TwoHanded),
            _ => None,
        })
}

/// PF1's Strength Bonus rule (CRB p.187): full STR mod for a one-handed
/// (or light) primary-hand weapon, 1.5x for a two-handed weapon, 0.5x
/// for an off-hand weapon — fractions always round down. `div_euclid`
/// floors toward negative infinity for a positive divisor, which matches
/// the CRB's "even if the total is 0 or less" rounding rule for negative
/// STR modifiers too (e.g. `-1 * 0.5 = -0.5` rounds down to `-1`, not
/// truncates toward zero to `0`).
fn str_damage_modifier_for(str_modifier: i16, wield: WieldCategory, hand: WeaponHandSlot) -> i16 {
    match (wield, hand) {
        (WieldCategory::TwoHanded, _) => (str_modifier * 3).div_euclid(2),
        (_, WeaponHandSlot::OffHand) => str_modifier.div_euclid(2),
        (WieldCategory::Light, WeaponHandSlot::Primary)
        | (WieldCategory::OneHanded, WeaponHandSlot::Primary) => str_modifier,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcgen_import::ir_converter::convert_equipment_record;
    use crate::pcgen_import::lst_parser::equipment::parse_equipment_entries;
    use crate::rules_core::source_content::SourceRef;

    fn corpus_from(text: &str) -> SourcePackageContent<'static> {
        let result = parse_equipment_entries("cr_equip_arms_armor.lst", text);
        assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
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

    /// Real verbatim tokens copied from `KEY:Longsword (Base)` in
    /// `core_rulebook/cr_equip_arms_armor.lst`.
    #[test]
    fn longsword_base_yields_its_real_damage_dice() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n";
        let corpus = corpus_from(text);

        let resolved = resolve_base_damage_dice("Longsword (Base)", &corpus)
            .expect("Longsword (Base) must resolve");
        assert_eq!(
            resolved.base_dice,
            DiceExpression {
                count: 1,
                die_size: 8
            }
        );
        assert_eq!(resolved.weapon_record_key, "Longsword (Base)");
    }

    /// Real verbatim tokens copied from `KEY:Leather Armor (Base)` — no
    /// `DAMAGE:` token on armor.
    #[test]
    fn armor_record_has_no_base_dice() {
        let text = "Leather Armor\tKEY:Leather Armor (Base)\tTYPE:Armor.Light\tCOST:10\tWT:15\tACCHECK:0\tMAXDEX:6\tSPELLFAILURE:10\tBONUS:COMBAT|AC|2|TYPE=Armor|PREVAREQ:DisableArmorBonus,0\n";
        let corpus = corpus_from(text);

        assert!(resolve_base_damage_dice("Leather Armor (Base)", &corpus).is_none());
    }

    #[test]
    fn dice_expression_parse_examples() {
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
        assert_eq!(DiceExpression::parse("0d8"), None);
        assert_eq!(DiceExpression::parse("1d0"), None);
        assert_eq!(DiceExpression::parse("garbage"), None);
    }

    /// Real verbatim tokens copied from `KEY:Longsword (Base)` in
    /// `core_rulebook/cr_equip_arms_armor.lst` — `WIELD:OneHanded`.
    #[test]
    fn longsword_one_handed_primary_hand_adds_full_str_modifier() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\tWIELD:OneHanded\n";
        let corpus = corpus_from(text);

        let resolved =
            resolve_str_damage_modifier("Longsword (Base)", &corpus, 3, WeaponHandSlot::Primary)
                .expect("Longsword (Base) must resolve");
        assert_eq!(resolved.wield_category, WieldCategory::OneHanded);
        assert_eq!(resolved.str_damage_modifier, 3);
    }

    /// Real verbatim tokens copied from `KEY:Longspear (Base)` in
    /// `core_rulebook/cr_equip_arms_armor.lst` — `WIELD:TwoHanded`.
    #[test]
    fn longspear_two_handed_adds_one_and_a_half_times_str_modifier() {
        let text = "Longspear\tKEY:Longspear (Base)\tTYPE:Weapon.Melee.Simple\tCOST:5\tWT:9\tCRITMULT:x3\tCRITRANGE:1\tDAMAGE:1d8\tWIELD:TwoHanded\n";
        let corpus = corpus_from(text);

        let resolved =
            resolve_str_damage_modifier("Longspear (Base)", &corpus, 3, WeaponHandSlot::Primary)
                .expect("Longspear (Base) must resolve");
        assert_eq!(resolved.wield_category, WieldCategory::TwoHanded);
        assert_eq!(resolved.str_damage_modifier, 4, "floor(1.5 * 3) = 4");
    }

    /// Real verbatim tokens copied from `KEY:Dagger (Base)` in
    /// `core_rulebook/cr_equip_arms_armor.lst` — `WIELD:Light`.
    #[test]
    fn dagger_off_hand_adds_half_str_modifier_rounded_down() {
        let text = "Dagger\tKEY:Dagger (Base)\tTYPE:Weapon.Melee.Simple\tCOST:2\tWT:1\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d4\tWIELD:Light\n";
        let corpus = corpus_from(text);

        let resolved =
            resolve_str_damage_modifier("Dagger (Base)", &corpus, 3, WeaponHandSlot::OffHand)
                .expect("Dagger (Base) must resolve");
        assert_eq!(resolved.wield_category, WieldCategory::Light);
        assert_eq!(resolved.str_damage_modifier, 1, "floor(0.5 * 3) = 1");
    }

    #[test]
    fn armor_record_has_no_str_damage_modifier() {
        let text = "Leather Armor\tKEY:Leather Armor (Base)\tTYPE:Armor.Light\tCOST:10\tWT:15\tACCHECK:0\tMAXDEX:6\tSPELLFAILURE:10\tBONUS:COMBAT|AC|2|TYPE=Armor|PREVAREQ:DisableArmorBonus,0\n";
        let corpus = corpus_from(text);

        assert!(resolve_str_damage_modifier(
            "Leather Armor (Base)",
            &corpus,
            3,
            WeaponHandSlot::Primary
        )
        .is_none());
    }

    #[test]
    fn str_damage_modifier_for_examples() {
        assert_eq!(
            str_damage_modifier_for(3, WieldCategory::OneHanded, WeaponHandSlot::Primary),
            3
        );
        assert_eq!(
            str_damage_modifier_for(3, WieldCategory::TwoHanded, WeaponHandSlot::Primary),
            4
        );
        assert_eq!(
            str_damage_modifier_for(3, WieldCategory::Light, WeaponHandSlot::OffHand),
            1
        );
        assert_eq!(
            str_damage_modifier_for(-1, WieldCategory::Light, WeaponHandSlot::OffHand),
            -1,
            "floor(-0.5) = -1, per CRB's round-down-even-below-zero rule"
        );
        assert_eq!(
            str_damage_modifier_for(-3, WieldCategory::TwoHanded, WeaponHandSlot::Primary),
            -5,
            "floor(1.5 * -3) = floor(-4.5) = -5"
        );
    }
}
