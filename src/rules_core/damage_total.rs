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
//! This cycle lands the fourth work-unit, `resolve_feat_damage_effect`:
//! a feat's damage-modifier contribution, **bounded to feats whose
//! `BONUS:` token is a directly-usable constant** (e.g. Weapon
//! Specialization / Greater Weapon Specialization's real
//! `BONUS:WEAPONPROF=%LIST|DAMAGE|2`), not the PCGen-formula-over-BAB
//! shape feats like Power Attack carry. See
//! `resolve_feat_damage_effect`'s own doc comment for the full in-scope
//! vs. out-of-scope boundary — this cycle resolves the prior blocked
//! attempt (`cycle-2026-07-17T1738`, recorded in the progress doc's
//! `damage:feat_effect` Open Blockers entry), a real gap
//! (`rules_tables::crb::feats::FeatTableEntry` had no numeric effect
//! field at all) since resolved by `3d962c2`.
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
//! The second work-unit, `resolve_str_damage_modifier`, lands the
//! STR-modifier slice the same way: it resolves a weapon `item_id`
//! against the corpus via the identical `equipment_id_resolve` path,
//! reads its real `WIELD:` token, and computes the STR contribution per
//! PF1's Strength Bonus rule. Verified directly against the live corpus:
//! `KEY:Longsword (Base)` carries `WIELD:OneHanded`, `KEY:Dagger (Base)`
//! carries `WIELD:Light`, `KEY:Longspear (Base)` carries
//! `WIELD:TwoHanded` (`core_rulebook/cr_equip_arms_armor.lst` lines 165,
//! 142, 151 respectively).
//!
//! This cycle lands the third work-unit, `resolve_weapon_enhancement_modifier`:
//! a weapon's magical enhancement bonus (e.g. a "+1" weapon), which PF1
//! adds to both the attack roll and the damage roll. Unlike the first two
//! work-units (which read a corpus token directly off the weapon
//! record), this one composes with Epic 5's already-landed,
//! already-closed equipment-effect engine
//! (`equipment_effects::compute_equipment_effects` /
//! `equipment_effects::equipmods::compute_equipmods_effect`) rather than
//! re-deriving the `BONUS:WEAPON|...|TYPE=Enhancement` corpus lookup
//! independently — per this cycle's brief, that lookup is Epic 5's closed
//! authority for this token family. See `resolve_weapon_enhancement_modifier`'s
//! own doc comment for the bounded no-attachment-model scope this
//! composition works within.
//!
//! This cycle lands the fifth work-unit, `resolve_critical_threat_range`
//! (the fourth work-unit, feat-effect modifier, is a separate concurrent
//! cycle's territory and is not touched here): a weapon's
//! critical-threat-range, read directly off its own `CRITRANGE:` corpus
//! token via the identical `equipment_id_resolve` path the first two
//! work-units use — the same "read tokens straight off the resolved
//! record" pattern `equipment_effects/arms_armor.rs` already established.
//! Verified directly against the live corpus
//! (`core_rulebook/cr_equip_arms_armor.lst`: `KEY:Longsword (Base)`
//! carries `CRITRANGE:2` -> threatens 19-20, `KEY:Rapier (Base)` carries
//! `CRITRANGE:3` -> threatens 18-20).
//!
//! This cycle lands the sixth and FINAL work-unit,
//! `resolve_critical_multiplier`: a weapon's critical-hit damage
//! multiplier, read directly off its own `CRITMULT:` corpus token via the
//! identical `equipment_id_resolve` path the other token-reading
//! work-units use, parsing the corpus's `x<N>` value into the numeric
//! multiplier. Verified directly against the live corpus
//! (`core_rulebook/cr_equip_arms_armor.lst`: `KEY:Longsword (Base)`
//! carries `CRITMULT:x2`, `KEY:Longspear (Base)` carries `CRITMULT:x3`,
//! `KEY:Scythe (Base)` carries `CRITMULT:x4`). **This closes Epic 6** —
//! all six damage-class criteria (base-dice, STR-modifier,
//! weapon-enhancement, feat-effect, critical-threat-range,
//! critical-multiplier) are now landed.

use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;
use crate::rules_core::character_input::{ActiveState, CharacterInput};
use crate::rules_core::equipment_effects::{is_natural_attack_weapon, EquipmentEffects};
use crate::rules_core::equipment_resolver::{equipment_id_resolve, equipment_key_token};
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::feats::{feat_tables, FeatEffectBonus};
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

/// One resolved weapon's magical-enhancement contribution to attack and
/// damage. PF1's weapon-enhancement rule: a magic weapon's enhancement
/// bonus applies to **both** the attack roll and the damage roll (e.g. a
/// "+1 longsword" adds +1 to hit and +1 to damage) — confirmed via
/// `technical-design.md` §2.4's illustrative equipment-effects
/// deliverable ("Magic weapons with enhancement bonuses contribute to
/// `attack_bonus_delta`") and §2.5's `damage_modifier` doc comment ("STR
/// mod + weapon enhancement + ..."). Not every `equipmods` enhancement
/// source affects both rolls uniformly, though: a masterwork/special
/// -material record (e.g. `KEY:Material ~ Adamantine ~ Weapon`) carries a
/// narrower `TOHIT`-only `BONUS:WEAPON|...|TYPE=Enhancement` chain, while
/// a true magical "+N" record (e.g. `KEY:Special Ability ~ +1 ~ Weapon`)
/// carries the full `DAMAGE,TOHIT` chain — this engine reads that
/// distinction verbatim off
/// `equipment_effects::equipmods::WeaponEnhancementBonus::affects` rather
/// than assuming uniformity.
#[derive(Debug, Clone, PartialEq)]
pub struct DamageRollWeaponEnhancement {
    pub weapon_item_id: String,
    pub weapon_record_key: String,
    pub attack_bonus: i16,
    pub damage_bonus: i16,
    pub table_cell: Option<TableCellRef>,
}

/// The damage-total engine's third work-unit (SD-20 §1.6): weapon
/// -enhancement modifier. Composes with Epic 5's already-landed,
/// already-closed equipment-effect engine
/// (`equipment_effects::compute_equipment_effects`, per that module's own
/// doc comment "closing Epic 5") rather than re-deriving the corpus
/// `BONUS:WEAPON|...|TYPE=Enhancement` lookup independently — per this
/// cycle's brief, Epic 5's `equipment_effects::equipmods::compute_equipmods_effect`
/// is the closed, already-landed authority for that token family.
///
/// This bounded model has no explicit weapon-to-equipmod attachment link
/// — `EquipmentSelection` carries only a flat `item_id`, and
/// `compute_equipment_effects` resolves every equipped selection
/// independently (see that module's own doc comment). A loadout's
/// magical-enhancement equipmod item(s) (e.g. `Special Ability ~ +1 ~
/// Weapon`) are therefore summed across the *entire* already-computed
/// `EquipmentEffects.per_item`, matching the single-primary-weapon
/// tabletop convention this engine's fixtures use elsewhere (one weapon,
/// its enhancement equipmod(s) equipped alongside it). A loadout with
/// more than one weapon and more than one enhancement equipmod would
/// double-count; that is out of this narrow work-unit's bounded scope —
/// the same posture Epic 4's `skill_allocation.rs` module doc comment
/// documents for its own bounded class-skill set, for a future cycle to
/// widen if a real attachment model lands.
///
/// Returns `None` only when the weapon itself does not resolve against
/// the corpus at all (honest absence, matching
/// `resolve_base_damage_dice` / `resolve_str_damage_modifier`). A
/// resolvable weapon with no matching enhancement equipmod in the
/// loadout yields real `0` bonuses (an honest zero contribution — the
/// weapon is real, its enhancement value is genuinely nil), not `None`.
///
/// **`SD31-W17-INTEGRATE-001` fix (OPEN-ISSUES row 309, SD-31 wave 18):**
/// a `WeaponEnhancementBonus` whose `natural_attack_only` field is `true`
/// (the Amulet of Mighty Fists family's `WEAPONPROF=TYPE.Natural` chain,
/// `equipment_effects::equipmods`) now only contributes when
/// `weapon_item_id` itself resolves to a real natural-attack weapon
/// (`equipment_effects::is_natural_attack_weapon` — e.g. CRB's `Unarmed
/// Strike`). Wave 17 shipped this same loadout-wide sum with no such
/// check at all, so an equipped Amulet wrongly bonused every weapon in
/// the loadout, not just natural attacks; reverted at merge time and
/// re-landed correctly here. An ordinary `natural_attack_only: false`
/// bonus (a true magic "+N" weapon, a masterwork/material chain) is
/// unaffected by this check and still applies to whichever single weapon
/// this bounded model attaches it to (see this function's own doc
/// comment above on the no-attachment-model scope).
pub fn resolve_weapon_enhancement_modifier(
    weapon_item_id: &str,
    corpus: &SourcePackageContent,
    equipment_effects: &EquipmentEffects,
) -> Option<DamageRollWeaponEnhancement> {
    let (record, table_cell) = equipment_id_resolve(weapon_item_id, RuleSetId::Crb, corpus)?;
    let weapon_record_key = equipment_key_token(record)
        .unwrap_or(&record.name)
        .to_string();
    let weapon_is_natural_attack = is_natural_attack_weapon(record);

    let mut attack_bonus: i16 = 0;
    let mut damage_bonus: i16 = 0;
    for item in &equipment_effects.per_item {
        let Some(bonus) = &item.weapon_enhancement_bonus else {
            continue;
        };
        if bonus.natural_attack_only && !weapon_is_natural_attack {
            continue;
        }
        if let Some(tohit) = bonus.tohit_bonus {
            attack_bonus += tohit;
        }
        if let Some(damage) = bonus.damage_bonus {
            damage_bonus += damage;
        }
    }

    Some(DamageRollWeaponEnhancement {
        weapon_item_id: weapon_item_id.to_string(),
        weapon_record_key,
        attack_bonus,
        damage_bonus,
        table_cell,
    })
}

/// One resolved weapon's critical-threat-range, with its corpus
/// provenance. This is the critical-threat-range slice of the eventual
/// `DamageRoll` (`technical-design.md` §2.5) — `critical_multiplier` is
/// the final work-unit's field, not fabricated here. `critical_threat_range`
/// is the inclusive `(low, high)` natural-roll bounds within which the
/// weapon threatens a critical hit, e.g. `(19, 20)` for a longsword.
#[derive(Debug, Clone, PartialEq)]
pub struct DamageRollCriticalThreatRange {
    pub weapon_item_id: String,
    pub weapon_record_key: String,
    pub critical_threat_range: (u8, u8),
    pub table_cell: Option<TableCellRef>,
}

/// The damage-total engine's fifth work-unit (SD-20 §1.6):
/// critical-threat-range. Resolves a weapon selection's `item_id` against
/// the corpus (same resolver path `resolve_base_damage_dice` and
/// `resolve_str_damage_modifier` use) and reads its real `CRITRANGE:`
/// token, converting the corpus's raw threat-*width* value (the count of
/// consecutive top natural-roll numbers that threaten) into the inclusive
/// `(low, high)` bounds the eventual `DamageRoll.critical_threat_range`
/// field carries. Per PF1 (CRB p.187, "Critical Hits"): a natural 20
/// always threatens regardless of the weapon's own range, so a width of
/// `1` yields `(20, 20)`; a longsword's width of `2` yields `(19, 20)`; a
/// rapier's width of `3` yields `(18, 20)`.
///
/// Returns `None` when the item does not resolve against the corpus at
/// all, or resolves but carries no `CRITRANGE:` token (e.g. armor, or any
/// other non-weapon item) — both are honest absence, not a fabricated
/// threat range.
pub fn resolve_critical_threat_range(
    weapon_item_id: &str,
    corpus: &SourcePackageContent,
) -> Option<DamageRollCriticalThreatRange> {
    let (record, table_cell) = equipment_id_resolve(weapon_item_id, RuleSetId::Crb, corpus)?;
    let critical_threat_range = critical_threat_range_token(record)?;
    let weapon_record_key = equipment_key_token(record)
        .unwrap_or(&record.name)
        .to_string();

    Some(DamageRollCriticalThreatRange {
        weapon_item_id: weapon_item_id.to_string(),
        weapon_record_key,
        critical_threat_range,
        table_cell,
    })
}

fn critical_threat_range_token(record: &EquipmentRecord) -> Option<(u8, u8)> {
    record
        .tokens
        .iter()
        .find(|token| token.key == "CRITRANGE")
        .and_then(|token| token.value.parse::<u8>().ok())
        .filter(|width| (1..=20).contains(width))
        .map(|width| (20 - width + 1, 20))
}

/// One resolved weapon's critical-hit damage multiplier, with its corpus
/// provenance. This is the critical-multiplier slice of the eventual
/// `DamageRoll` (`technical-design.md` §2.5) — the sixth and final Epic 6
/// work-unit. `critical_multiplier` is the factor a confirmed critical
/// hit's damage is multiplied by, e.g. `2` for a longsword, `4` for a
/// scythe.
#[derive(Debug, Clone, PartialEq)]
pub struct DamageRollCriticalMultiplier {
    pub weapon_item_id: String,
    pub weapon_record_key: String,
    pub critical_multiplier: u8,
    pub table_cell: Option<TableCellRef>,
}

/// The damage-total engine's sixth and final work-unit (SD-20 §1.6):
/// critical-multiplier. Resolves a weapon selection's `item_id` against
/// the corpus (same resolver path every prior work-unit uses) and reads
/// its real `CRITMULT:` token, parsing the corpus's `x<N>` value (e.g.
/// `x2`, `x3`, `x4`) into the numeric multiplier the eventual
/// `DamageRoll.critical_multiplier` field carries. Per PF1 (CRB p.187,
/// "Critical Hits"): on a confirmed critical hit, the weapon's damage is
/// multiplied by this factor rather than a uniform x2 across every
/// weapon — a longsword's `CRITMULT:x2` doubles damage, a scythe's
/// `CRITMULT:x4` quadruples it.
///
/// Returns `None` when the item does not resolve against the corpus at
/// all, or resolves but carries no `CRITMULT:` token (e.g. armor, or any
/// other non-weapon item) — both are honest absence, not a fabricated
/// multiplier.
pub fn resolve_critical_multiplier(
    weapon_item_id: &str,
    corpus: &SourcePackageContent,
) -> Option<DamageRollCriticalMultiplier> {
    let (record, table_cell) = equipment_id_resolve(weapon_item_id, RuleSetId::Crb, corpus)?;
    let critical_multiplier = critical_multiplier_token(record)?;
    let weapon_record_key = equipment_key_token(record)
        .unwrap_or(&record.name)
        .to_string();

    Some(DamageRollCriticalMultiplier {
        weapon_item_id: weapon_item_id.to_string(),
        weapon_record_key,
        critical_multiplier,
        table_cell,
    })
}

fn critical_multiplier_token(record: &EquipmentRecord) -> Option<u8> {
    record
        .tokens
        .iter()
        .find(|token| token.key == "CRITMULT")
        .and_then(|token| token.value.strip_prefix('x'))
        .and_then(|digits| digits.parse::<u8>().ok())
        .filter(|multiplier| *multiplier >= 2)
}

/// One resolved feat's constant-valued damage contribution, with its
/// corpus provenance. This is the feat-effect slice of the eventual
/// `DamageRoll` (`technical-design.md` §2.5) — `critical_threat_range`
/// and `critical_multiplier` are later work-units' fields, not
/// fabricated here. Unlike the three earlier work-units'
/// `table_cell: Option<TableCellRef>` (whose corpus resolver,
/// `equipment_id_resolve`, can find a record but no cell metadata),
/// `table_cell` here is never optional — it is constructed directly from
/// the matched `FeatTableEntry.key`, the same always-`Some` shape
/// `feat_prereqs::combat::resolve_combat_feat_effect`'s own
/// `CombatFeatEffect.table_cell: TableCellRef` (non-`Option`) already
/// uses for this identical table.
#[derive(Debug, Clone, PartialEq)]
pub struct DamageRollFeatEffect {
    pub feat_key: String,
    pub damage_bonus: i16,
    pub table_cell: TableCellRef,
}

/// The damage-total engine's fourth work-unit (SD-20 §1.6): feat-effect
/// modifier — **bounded to feats whose `BONUS:` token is a directly
/// usable constant**, per this cycle's explicit scoping (see below).
/// Reads `rules_tables::crb::feats::feat_tables()` directly (no
/// `RulesTables` parameter, `technical-design.md` §2.0) — the same
/// direct-import pattern `feat_prereqs/combat.rs::resolve_combat_feat_effect`
/// already uses for this table. Deliberately does **not** compose with
/// Epic 3's `feat_prereqs.rs` / `FeatEffects` the way
/// `resolve_weapon_enhancement_modifier` composes with Epic 5's
/// `equipment_effects.rs`: `FeatEffects` (and every per-category effect
/// struct built from it) carries only `feat_id` / `description` /
/// `table_cell` — no numeric field at all (see the progress doc's
/// `damage:feat_effect` Open Blockers entry, resolved 2026-07-17 at
/// `3d962c2`) — so this work-unit reads the table store's own
/// `FeatTableEntry.effect` field directly instead, the same table Epic
/// 3's resolvers already read `key`/`category`/`name`/`description` from.
///
/// ## Scoping: constant-valued feats in scope; formula-based feats still
/// out of scope
///
/// `FeatTableEntry.effect`'s own doc comment explains why this can't be
/// a blanket resolution: many real `cr_feats.lst` `BONUS:` tokens are
/// PCGen formula expressions over runtime character state, not static
/// literals — e.g. Power Attack's damage bonus
/// (`BONUS:VAR|PowerAttackDamageModifier|PowerAttackDamageBase*floor(PowerAttackModifier)`)
/// depends on the wielder's base attack bonus (`BAB`). Resolving a
/// formula like that into a real number needs a full PCGen formula
/// evaluator (parsing `floor()`, variable lookups such as `BAB`, and
/// PCGen's `DEFINE:`-scoped runtime state) — a much larger undertaking
/// than this one cycle's bounded slice, and explicitly out of scope
/// here. Fabricating a plausible resolved integer for a formula-based
/// feat (e.g. hardcoding Power Attack's "+2 damage per 4 BAB" rule text)
/// was already rejected once by this exact work-unit's prior blocked
/// attempt (`cycle-2026-07-17T1738`) as counterfeit completion, and
/// stays rejected here.
///
/// **In scope today:** a feat whose `BONUS:` token qualifier list is
/// exactly `[<category>, "DAMAGE", "<integer>"]` — a bare `DAMAGE`
/// target (not a qualified/compound one like `DAMAGE-SHORTRANGE` or
/// `DAMAGE.ShieldBash`, which apply only under a condition this bounded
/// slice does not model) with a literal, directly-parseable value (not a
/// `VAR`-category token, since `VAR` defines a named formula variable
/// for other tokens to reference, not a direct roll bonus itself).
/// Verified against the real corpus (`core_rulebook/cr_feats.lst`, lines
/// 89 and 185): `KEY:Weapon Specialization` and `KEY:Greater Weapon
/// Specialization` both carry exactly `BONUS:WEAPONPROF=%LIST|DAMAGE|2`
/// (SOURCEPAGE p.137 / p.126: "You gain a +2 bonus on all damage rolls
/// you make using the selected weapon") — a genuine constant `+2`, not a
/// formula. Both resolve through this function.
///
/// **Still out of scope (formula-based; needs a future formula-evaluator
/// cycle to widen):** every feat whose `BONUS:` token category is `VAR`
/// (e.g. Power Attack, Arcane Strike, Shield Master) or whose target is
/// not the bare `DAMAGE` string (e.g. Point-Blank Shot's
/// `TOHIT-SHORTRANGE,DAMAGE-SHORTRANGE`, Double Slice's `DAMAGEMULT:0`).
/// These feats' `FeatTableEntry.effect` is real, landed data (`3d962c2`),
/// but this function honestly returns `None` for them rather than
/// resolving a wrong or fabricated number — the same "honest absence
/// over fabricated default" discipline every other resolver in this file
/// already follows.
///
/// Returns `None` when `feat_key` does not resolve to a real
/// `FeatTableEntry` in the catalog at all (matches `key` or `name`, the
/// same fallback `feat_prereqs`'s per-category resolvers use), when the
/// matched entry carries no `effect` data at all, or when none of its
/// `BONUS:` tokens are a constant-valued `DAMAGE` bonus per the scoping
/// above.
pub fn resolve_feat_damage_effect(feat_key: &str) -> Option<DamageRollFeatEffect> {
    let entry = feat_tables()
        .iter()
        .find(|entry| entry.key == feat_key || entry.name == feat_key)?;
    let effect = entry.effect?;
    let damage_bonus = effect.iter().find_map(constant_damage_bonus)?;

    Some(DamageRollFeatEffect {
        feat_key: entry.key.to_string(),
        damage_bonus,
        table_cell: TableCellRef {
            rule_set: RuleSetId::Crb,
            table: "feats".to_string(),
            row_key: entry.key.to_string(),
            column_key: String::new(),
        },
    })
}

/// A `BONUS:` token is a directly-usable constant damage bonus, per
/// `resolve_feat_damage_effect`'s scoping doc comment, only when its
/// qualifier list is exactly `[<category>, "DAMAGE", "<integer>"]` and
/// `<category>` is not `VAR` (a `VAR` token defines a named formula
/// variable, not a direct roll bonus — e.g. Power Attack's
/// `BONUS:VAR|PowerAttackDamageModifier|...`). Anything else (a
/// qualified/compound target, a non-numeric value, a `VAR` category, a
/// wrong-length qualifier list) is a formula or a non-constant-damage
/// bonus and is honestly excluded, not coerced.
fn constant_damage_bonus(bonus: &FeatEffectBonus) -> Option<i16> {
    let qualifiers = bonus.qualifiers;
    if qualifiers.len() != 3 {
        return None;
    }
    if qualifiers[0] == "VAR" || qualifiers[1] != "DAMAGE" {
        return None;
    }
    qualifiers[2].parse::<i16>().ok()
}

/// One equipped weapon's full damage breakdown — the wiring project's
/// Cycle 5a aggregator (`damage:aggregate_weapons`; see
/// `~/.claude/plans/adaptive-squishing-mccarthy.md`). Composes all six of
/// this module's narrow `resolve_*` work-units for a single weapon into
/// one structured record, plus the character's feat effects (which apply
/// per-character, not per-weapon).
///
/// **Known, bounded limitation — hand slot.** `resolve_str_damage_modifier`
/// takes a `WeaponHandSlot` (`Primary` vs. `OffHand`, which changes the
/// STR-modifier fraction applied). `resolve_weapon_damage_breakdown`
/// always resolves with `WeaponHandSlot::Primary`, because
/// `EquipmentSelection` (the character-input record this aggregator loops
/// over) has no hand-slot field today — there is no chosen-input signal
/// that says "this weapon is in the off hand." A loadout with a genuine
/// two-weapon-fighting off-hand weapon will get an inflated STR modifier
/// from this aggregator until a hand-slot field is added to
/// `EquipmentSelection` (a future, separate cycle's scope — not
/// fabricated here).
#[derive(Debug, Clone, PartialEq)]
pub struct WeaponDamageBreakdown {
    pub weapon_item_id: String,
    pub base_dice: Option<DamageRollBaseDice>,
    pub str_modifier: Option<DamageRollStrModifier>,
    pub weapon_enhancement: Option<DamageRollWeaponEnhancement>,
    pub critical_threat_range: Option<DamageRollCriticalThreatRange>,
    pub critical_multiplier: Option<DamageRollCriticalMultiplier>,
    pub feat_effects: Vec<DamageRollFeatEffect>,
}

/// The wiring project's Cycle 5a aggregator: loops `character`'s equipped
/// items, identifies which of them are weapons, and assembles a full
/// per-weapon `WeaponDamageBreakdown` for each.
///
/// **Identification mechanism:** an equipped item is a weapon when
/// `resolve_base_damage_dice` returns `Some` for it — a `None` result IS
/// the "not a weapon" signal (e.g. armor carries no `DAMAGE:` token), the
/// same honest-absence contract every `resolve_*` function in this module
/// already documents. Non-weapon equipped items are silently skipped —
/// they never appear in the output vec at all, not represented as a
/// weapon with `None` fields.
///
/// Only items whose `active_state` is `ActiveState::EquippedActive` are
/// considered; a selected-but-inactive or absent item is not part of the
/// current loadout's damage picture.
///
/// `feat_effects` is gathered **once per character**, not once per
/// weapon: `resolve_feat_damage_effect` takes no weapon parameter because
/// a feat like Weapon Specialization applies universally to whichever
/// weapon it names (this bounded slice does not model per-weapon feat
/// targeting), so the same resolved `feat_effects` vec is attached to
/// every `WeaponDamageBreakdown` in the output.
pub fn resolve_weapon_damage_breakdown(
    character: &CharacterInput,
    corpus: &SourcePackageContent,
    equipment_effects: &EquipmentEffects,
    str_modifier: i16,
) -> Vec<WeaponDamageBreakdown> {
    let feat_effects: Vec<DamageRollFeatEffect> = character
        .chosen
        .selected_feats
        .iter()
        .filter_map(|feat_key| resolve_feat_damage_effect(feat_key))
        .collect();

    character
        .chosen
        .equipment_selections
        .iter()
        .filter(|selection| selection.active_state == ActiveState::EquippedActive)
        .filter_map(|selection| {
            let weapon_item_id = selection.item_id.as_str();
            let base_dice = resolve_base_damage_dice(weapon_item_id, corpus)?;

            Some(WeaponDamageBreakdown {
                weapon_item_id: weapon_item_id.to_string(),
                base_dice: Some(base_dice),
                str_modifier: resolve_str_damage_modifier(
                    weapon_item_id,
                    corpus,
                    str_modifier,
                    WeaponHandSlot::Primary,
                ),
                weapon_enhancement: resolve_weapon_enhancement_modifier(
                    weapon_item_id,
                    corpus,
                    equipment_effects,
                ),
                critical_threat_range: resolve_critical_threat_range(weapon_item_id, corpus),
                critical_multiplier: resolve_critical_multiplier(weapon_item_id, corpus),
                feat_effects: feat_effects.clone(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcgen_import::ir_converter::convert_equipment_record;
    use crate::pcgen_import::lst_parser::equipment::parse_equipment_entries;
    use crate::rules_core::character_input::{ActiveState, EquipmentSelection};
    use crate::rules_core::equipment_effects::compute_equipment_effects;
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

    fn selection(item_id: &str) -> EquipmentSelection {
        EquipmentSelection {
            item_id: item_id.to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
        }
    }

    /// Real verbatim tokens: `KEY:Longsword (Base)`
    /// (`core_rulebook/cr_equip_arms_armor.lst`) plus `KEY:Special
    /// Ability ~ +1 ~ Weapon` (`core_rulebook/cr_equipmods.lst` line 219,
    /// `BONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement`).
    #[test]
    fn plus_one_weapon_enhancement_adds_to_both_attack_and_damage() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n\
+1 (Enhancement to Weapon)\tKEY:Special Ability ~ +1 ~ Weapon\tTYPE:Weapon\tPLUS:1\tCOST:0\tBONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement\n";
        let corpus = corpus_from(text);
        let equipped = vec![
            selection("Longsword (Base)"),
            selection("Special Ability ~ +1 ~ Weapon"),
        ];
        let effects = compute_equipment_effects(&equipped, &corpus);

        let resolved = resolve_weapon_enhancement_modifier("Longsword (Base)", &corpus, &effects)
            .expect("Longsword (Base) must resolve");
        assert_eq!(resolved.weapon_record_key, "Longsword (Base)");
        assert_eq!(resolved.attack_bonus, 1);
        assert_eq!(resolved.damage_bonus, 1);
    }

    /// Real verbatim tokens: `KEY:Material ~ Adamantine ~ Weapon`
    /// (`core_rulebook/cr_equipmods.lst` line 101,
    /// `BONUS:WEAPON|TOHIT|1|TYPE=Enhancement`) — a `TOHIT`-only chain,
    /// proving the engine reads the affected-roll set off the token
    /// rather than assuming every enhancement source hits both rolls.
    #[test]
    fn tohit_only_enhancement_does_not_add_to_damage() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n\
Adamantine\tKEY:Material ~ Adamantine ~ Weapon\tTYPE:BaseMaterial.MasterworkQuality.Weapon\tCOST:3000\tBONUS:WEAPON|TOHIT|1|TYPE=Enhancement\n";
        let corpus = corpus_from(text);
        let equipped = vec![
            selection("Longsword (Base)"),
            selection("Material ~ Adamantine ~ Weapon"),
        ];
        let effects = compute_equipment_effects(&equipped, &corpus);

        let resolved = resolve_weapon_enhancement_modifier("Longsword (Base)", &corpus, &effects)
            .expect("Longsword (Base) must resolve");
        assert_eq!(resolved.attack_bonus, 1);
        assert_eq!(resolved.damage_bonus, 0);
    }

    #[test]
    fn no_enhancement_equipped_yields_honest_zero_not_fabricated() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n";
        let corpus = corpus_from(text);
        let equipped = vec![selection("Longsword (Base)")];
        let effects = compute_equipment_effects(&equipped, &corpus);

        let resolved = resolve_weapon_enhancement_modifier("Longsword (Base)", &corpus, &effects)
            .expect("Longsword (Base) must resolve");
        assert_eq!(resolved.attack_bonus, 0);
        assert_eq!(resolved.damage_bonus, 0);
    }

    #[test]
    fn unresolvable_weapon_yields_none_not_fabricated() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n";
        let corpus = corpus_from(text);
        let effects = compute_equipment_effects(&[], &corpus);

        assert!(resolve_weapon_enhancement_modifier(
            "item:does-not-exist-in-this-corpus",
            &corpus,
            &effects
        )
        .is_none());
    }

    /// `SD31-W17-INTEGRATE-001` fix (OPEN-ISSUES row 309, SD-31 wave 18):
    /// real verbatim tokens for `KEY:Unarmed Strike`
    /// (`core_rulebook/cr_equip_arms_armor.lst` line 296, a real
    /// natural-attack weapon -- carries the `Natural` `TYPE:` segment) plus
    /// `KEY:Special Ability ~ +1 ~ Amulet of Mighty Fists`
    /// (`WEAPONPROF=TYPE.Natural`). The Amulet's bonus applies to the
    /// natural attack.
    #[test]
    fn amulet_of_mighty_fists_applies_to_a_real_natural_attack() {
        let text = "Unarmed Strike\tKEY:Unarmed Strike\tTYPE:Weapon.Resizable.Melee.Special.Unarmed.Monk.Bludgeoning.Finesseable.Close.Weapon Group Close.Weapon Group Monk.Weapon Group Natural.Natural.Light\tCOST:0\tWT:0\tCRITMULT:x2\tCRITRANGE:1\tDAMAGE:1d3\tWIELD:Light\n\
+1 to Hit and Damage\tKEY:Special Ability ~ +1 ~ Amulet of Mighty Fists\tTYPE:Amulet of Mighty Fists\tPLUS:1\tBONUS:WEAPONPROF=TYPE.Natural|TOHIT,DAMAGE|1|TYPE=Enhancement\n";
        let corpus = corpus_from(text);
        let equipped = vec![
            selection("Unarmed Strike"),
            selection("Special Ability ~ +1 ~ Amulet of Mighty Fists"),
        ];
        let effects = compute_equipment_effects(&equipped, &corpus);

        let resolved = resolve_weapon_enhancement_modifier("Unarmed Strike", &corpus, &effects)
            .expect("Unarmed Strike must resolve");
        assert_eq!(resolved.attack_bonus, 1, "a natural attack must receive the Amulet's bonus");
        assert_eq!(resolved.damage_bonus, 1);
    }

    /// The exact regression wave 17 shipped and review reverted: an
    /// equipped Amulet of Mighty Fists must NOT bonus an ordinary weapon.
    /// Same fixture as the passing case above, but resolved against the
    /// Longsword instead of the Unarmed Strike in the SAME loadout.
    #[test]
    fn amulet_of_mighty_fists_does_not_apply_to_an_ordinary_weapon() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n\
Unarmed Strike\tKEY:Unarmed Strike\tTYPE:Weapon.Resizable.Melee.Special.Unarmed.Monk.Bludgeoning.Finesseable.Close.Weapon Group Close.Weapon Group Monk.Weapon Group Natural.Natural.Light\tCOST:0\tWT:0\tCRITMULT:x2\tCRITRANGE:1\tDAMAGE:1d3\tWIELD:Light\n\
+1 to Hit and Damage\tKEY:Special Ability ~ +1 ~ Amulet of Mighty Fists\tTYPE:Amulet of Mighty Fists\tPLUS:1\tBONUS:WEAPONPROF=TYPE.Natural|TOHIT,DAMAGE|1|TYPE=Enhancement\n";
        let corpus = corpus_from(text);
        let equipped = vec![
            selection("Longsword (Base)"),
            selection("Unarmed Strike"),
            selection("Special Ability ~ +1 ~ Amulet of Mighty Fists"),
        ];
        let effects = compute_equipment_effects(&equipped, &corpus);

        let resolved = resolve_weapon_enhancement_modifier("Longsword (Base)", &corpus, &effects)
            .expect("Longsword (Base) must resolve");
        assert_eq!(
            resolved.attack_bonus, 0,
            "the Amulet of Mighty Fists must not bonus an ordinary weapon -- SD31-W17-INTEGRATE-001 row 309"
        );
        assert_eq!(resolved.damage_bonus, 0);
    }

    /// An ordinary (non-natural-attack-scoped) enhancement bonus is
    /// unaffected by the natural-attack-scope check -- proves the fix is
    /// additive, not a regression on the existing `WEAPON`-subject path.
    #[test]
    fn an_ordinary_enhancement_bonus_still_applies_regardless_of_natural_attack_scope() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n\
+1 (Enhancement to Weapon)\tKEY:Special Ability ~ +1 ~ Weapon\tTYPE:Weapon\tPLUS:1\tCOST:0\tBONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement\n";
        let corpus = corpus_from(text);
        let equipped = vec![
            selection("Longsword (Base)"),
            selection("Special Ability ~ +1 ~ Weapon"),
        ];
        let effects = compute_equipment_effects(&equipped, &corpus);

        let resolved = resolve_weapon_enhancement_modifier("Longsword (Base)", &corpus, &effects)
            .expect("Longsword (Base) must resolve");
        assert_eq!(resolved.attack_bonus, 1);
        assert_eq!(resolved.damage_bonus, 1);
    }

    /// Real verbatim tokens copied from `KEY:Longsword (Base)` in
    /// `core_rulebook/cr_equip_arms_armor.lst` — `CRITRANGE:2`.
    #[test]
    fn longsword_critrange_2_threatens_19_to_20() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n";
        let corpus = corpus_from(text);

        let resolved = resolve_critical_threat_range("Longsword (Base)", &corpus)
            .expect("Longsword (Base) must resolve");
        assert_eq!(resolved.critical_threat_range, (19, 20));
    }

    /// Real verbatim tokens copied from `KEY:Rapier (Base)` in
    /// `core_rulebook/cr_equip_arms_armor.lst` — `CRITRANGE:3`.
    #[test]
    fn rapier_critrange_3_threatens_18_to_20() {
        let text = "Rapier\tKEY:Rapier (Base)\tTYPE:Weapon.Melee.Martial\tCOST:20\tWT:2\tCRITMULT:x2\tCRITRANGE:3\tDAMAGE:1d6\n";
        let corpus = corpus_from(text);

        let resolved = resolve_critical_threat_range("Rapier (Base)", &corpus)
            .expect("Rapier (Base) must resolve");
        assert_eq!(resolved.critical_threat_range, (18, 20));
    }

    #[test]
    fn armor_record_has_no_critical_threat_range() {
        let text = "Leather Armor\tKEY:Leather Armor (Base)\tTYPE:Armor.Light\tCOST:10\tWT:15\tACCHECK:0\tMAXDEX:6\tSPELLFAILURE:10\tBONUS:COMBAT|AC|2|TYPE=Armor|PREVAREQ:DisableArmorBonus,0\n";
        let corpus = corpus_from(text);

        assert!(resolve_critical_threat_range("Leather Armor (Base)", &corpus).is_none());
    }

    #[test]
    fn critical_threat_range_unresolvable_item_yields_none_not_fabricated() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n";
        let corpus = corpus_from(text);

        assert!(
            resolve_critical_threat_range("item:does-not-exist-in-this-corpus", &corpus)
                .is_none()
        );
    }

    /// Real verbatim token from `KEY:Weapon Specialization`
    /// (`core_rulebook/cr_feats.lst` line 185): `BONUS:WEAPONPROF=%LIST|DAMAGE|2`.
    #[test]
    fn weapon_specialization_yields_its_real_constant_damage_bonus() {
        let resolved = resolve_feat_damage_effect("Weapon Specialization")
            .expect("Weapon Specialization is a real Combat feat with a constant BONUS: token");
        assert_eq!(resolved.feat_key, "Weapon Specialization");
        assert_eq!(resolved.damage_bonus, 2);
        assert_eq!(resolved.table_cell.table, "feats");
        assert_eq!(resolved.table_cell.row_key, "Weapon Specialization");
    }

    /// Power Attack's `BONUS:` tokens are all `VAR`-category PCGen
    /// formula expressions over BAB — out of this work-unit's bounded
    /// scope. Honest `None`, not a fabricated resolved integer.
    #[test]
    fn power_attack_formula_based_bonus_is_out_of_scope() {
        assert!(resolve_feat_damage_effect("Power Attack").is_none());
    }

    #[test]
    fn unrecognized_feat_key_yields_none_not_fabricated() {
        assert!(resolve_feat_damage_effect("Not A Real Feat In The Catalog").is_none());
    }

    /// Real verbatim tokens copied from `KEY:Longsword (Base)` in
    /// `core_rulebook/cr_equip_arms_armor.lst` — `CRITMULT:x2`.
    #[test]
    fn longsword_critmult_x2_yields_multiplier_2() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n";
        let corpus = corpus_from(text);

        let resolved = resolve_critical_multiplier("Longsword (Base)", &corpus)
            .expect("Longsword (Base) must resolve");
        assert_eq!(resolved.critical_multiplier, 2);
    }

    /// Real verbatim tokens copied from `KEY:Scythe (Base)` in
    /// `core_rulebook/cr_equip_arms_armor.lst` — `CRITMULT:x4`.
    #[test]
    fn scythe_critmult_x4_yields_multiplier_4() {
        let text = "Scythe\tKEY:Scythe (Base)\tTYPE:Weapon.Melee.Martial\tCOST:18\tWT:10\tCRITMULT:x4\tCRITRANGE:1\tDAMAGE:2d4\n";
        let corpus = corpus_from(text);

        let resolved = resolve_critical_multiplier("Scythe (Base)", &corpus)
            .expect("Scythe (Base) must resolve");
        assert_eq!(resolved.critical_multiplier, 4);
    }

    #[test]
    fn armor_record_has_no_critical_multiplier() {
        let text = "Leather Armor\tKEY:Leather Armor (Base)\tTYPE:Armor.Light\tCOST:10\tWT:15\tACCHECK:0\tMAXDEX:6\tSPELLFAILURE:10\tBONUS:COMBAT|AC|2|TYPE=Armor|PREVAREQ:DisableArmorBonus,0\n";
        let corpus = corpus_from(text);

        assert!(resolve_critical_multiplier("Leather Armor (Base)", &corpus).is_none());
    }

    #[test]
    fn critical_multiplier_unresolvable_item_yields_none_not_fabricated() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n";
        let corpus = corpus_from(text);

        assert!(
            resolve_critical_multiplier("item:does-not-exist-in-this-corpus", &corpus).is_none()
        );
    }

    #[test]
    fn constant_damage_bonus_examples() {
        assert_eq!(
            constant_damage_bonus(&FeatEffectBonus {
                qualifiers: &["WEAPONPROF=%LIST", "DAMAGE", "2"]
            }),
            Some(2)
        );
        assert_eq!(
            constant_damage_bonus(&FeatEffectBonus {
                qualifiers: &["VAR", "PowerAttackDamageBase", "2"]
            }),
            None,
            "a VAR-category token defines a formula variable, not a direct bonus"
        );
        assert_eq!(
            constant_damage_bonus(&FeatEffectBonus {
                qualifiers: &["COMBAT", "TOHIT-SHORTRANGE,DAMAGE-SHORTRANGE", "1"]
            }),
            None,
            "a compound/qualified target is not the bare DAMAGE this slice models"
        );
        assert_eq!(
            constant_damage_bonus(&FeatEffectBonus {
                qualifiers: &["HP", "CURRENTMAX", "max(3,TL)"]
            }),
            None,
            "a non-numeric value signals a formula, not a constant"
        );
    }
}
