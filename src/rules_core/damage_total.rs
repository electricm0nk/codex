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
//! critical-threat-range, then critical-multiplier. This cycle lands the
//! first work-unit: base-dice round-trip.
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
}
