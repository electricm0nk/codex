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
//! (`rules_catalog::crb::feats::FeatTableEntry` had no numeric effect
//! field at all) since resolved by `3d962c2`.
//!
//! Adapts `technical-design.md` §2.5's illustrative `compute_damage`
//! seam to this repo's real types per §2.0 (`RulesTables` retired — no
//! `rules_tables: &RulesTables` parameter anywhere; a table-store read,
//! when this epic needs one, imports the specific
//! `rules_catalog::crb::<table>` item directly). The full
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
//!
//! ## SD-35 `AT-35-E6-003-RULED` cycle 12 — where these values now come from
//!
//! Every "reads its real `<TOKEN>:` token" sentence above is the history of
//! how each work-unit was built, and each rule it states still holds. What
//! changed is the side of the ingest boundary the read happens on
//! (`decisions.md` §11, §19): the `DAMAGE:`, `BASEITEM:`, `WIELD:`,
//! `CRITRANGE:`, `CRITMULT:` and `BONUS:EQMWEAPON|DAMAGESIZE` spellings moved
//! to [`crate::pcgen_import::ir_converter::equipment_record_to_corpus`], and
//! this module reads the settled values off
//! [`crate::rules_core::equipment_record::CorpusEquipmentRecord`], resolved by
//! [`crate::rules_core::equipment_resolver::equipment_id_resolve`]
//! -- the same one resolution, answering in settled values and provenance.
//! The parity of the move is proved over all 7,803 live corpus equipment
//! records by `equipment_record`'s own
//! `every_live_corpus_equipment_record_carries_the_same_weapon_values_the_token_reads_produced`.

use crate::rules_core::character_input::{ActiveState, CharacterInput, EquipmentSelection};
use crate::rules_core::equipment_effects::{attached_weapon_enhancement, is_natural_attack_weapon, stand_in_record, EquipmentEffects};
use crate::rules_core::equipment_record::CorpusEquipmentRecord;
use crate::rules_core::equipment_resolver::{
    equipment_converted_resolve, equipment_id_resolve,
};
use crate::rules_core::pilot_compute::{normalize_weapon_identity, ComputationExplanation};
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_catalog::crb::feats::{feat_tables, EffectSelection, FeatEffectBonus};
use crate::rules_core::rules_catalog::RuleSetId;
use crate::rules_core::sheet_rule::{slug, ProseFamily, ProsePiece, SheetRulePackage};
use crate::rules_core::source_content::SourcePackageContent;
use crate::saved_character::equipment_sets::{copy_keys, CopyKey};

/// A PF1 dice expression, e.g. `"1d8"` -> `{ count: 1, die_size: 8 }`,
/// `"2d6"` -> `{ count: 2, die_size: 6 }`. `count` dice, each with
/// `die_size` faces, summed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
/// all, or resolves (directly, or via a `BASEITEM:` chase — see
/// `base_item_damage_dice_token` below) but carries no `DAMAGE:` token
/// anywhere in that chain (e.g. armor, or any other non-weapon item) —
/// both are honest absence, not a fabricated dice expression.
pub fn resolve_base_damage_dice(
    weapon_item_id: &str,
    corpus: &SourcePackageContent,
) -> Option<DamageRollBaseDice> {
    let (record, table_cell) =
        equipment_id_resolve(weapon_item_id, RuleSetId::Crb, corpus)?;
    let base_dice = record
        .base_damage_dice
        .or_else(|| base_item_damage_dice(record, corpus))?;
    let weapon_record_key = record.identity.clone();

    Some(DamageRollBaseDice {
        weapon_item_id: weapon_item_id.to_string(),
        weapon_record_key,
        base_dice,
        table_cell,
    })
}

/// `AT-34-E3-003` (bucket `M`, equipment sub-cause
/// `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does`): a
/// PCGen alias row states no mechanical tokens of its own at all beyond a
/// `BASEITEM:` reference — the real corpus convention for "this record's
/// own stats are its base item's stats." `Crossbow (Light)`
/// (`core_rulebook/cr_equip_arms_armor.lst`) is the live instance: its own
/// row carries `BASEITEM:Light Crossbow (Base)` and no `DAMAGE:` token,
/// while `Light Crossbow (Base)` carries the real `DAMAGE:1d8`.
/// The stand-in item's IDENTITY is settled at ingest
/// ([`CorpusEquipmentRecord::base_item`], SD-35 `AT-35-E6-003-RULED`
/// cycle 12); the one-hop CHASE stays here, because it is a corpus
/// resolution and the converter has no corpus. It goes through the SAME
/// resolver `resolve_base_damage_dice` already calls for its primary
/// lookup — not a new resolution mechanism, a second call to the existing
/// one. A stand-in naming a record that does not resolve, or a chain more
/// than one hop deep, returns `None` rather than guessing or recursing.
fn base_item_damage_dice(
    record: &CorpusEquipmentRecord,
    corpus: &SourcePackageContent,
) -> Option<DiceExpression> {
    let base_item_key = record.base_item.as_deref()?;
    equipment_converted_resolve(base_item_key, corpus)?.base_damage_dice
}

/// The Pathfinder RPG single-die weapon-damage-size progression table
/// (CRB p.187's weapon-size-change note; the same table `BONUS:
/// EQMWEAPON|DAMAGESIZE` steps along -- real corpus example: Core
/// Rulebook's `Special Quality ~ Spikes ~ Shieldbash` (Shield Spikes),
/// `BONUS:EQMWEAPON|DAMAGESIZE|1`, confirmed live against the pinned
/// PCGen oracle this cycle stepping a `DAMAGE:1d4` shield-bash host to
/// `1d6`). Covers only the single-die steps every real corpus
/// `DAMAGESIZE` chain this cycle read actually uses -- `step_single_die`
/// returns `None` for a multi-die base or an out-of-table step rather
/// than fabricate a die past what this table proves.
const SINGLE_DIE_STEP_TABLE: [u8; 8] = [1, 2, 3, 4, 6, 8, 10, 12];

/// Steps a single-die `DiceExpression` up (positive `steps`) or down
/// (negative) `SINGLE_DIE_STEP_TABLE`'s progression. `None` for a
/// multi-die base (`count != 1`), a base die size not in the table, or a
/// step that would land outside it -- never a fabricated die.
pub fn step_single_die(base: DiceExpression, steps: i32) -> Option<DiceExpression> {
    if base.count != 1 {
        return None;
    }
    let idx = SINGLE_DIE_STEP_TABLE.iter().position(|&d| d == base.die_size)? as i32;
    let new_idx = idx + steps;
    if new_idx < 0 {
        return None;
    }
    SINGLE_DIE_STEP_TABLE
        .get(new_idx as usize)
        .map(|&die_size| DiceExpression { count: 1, die_size })
}

/// Resolves a weapon's real base damage die (`resolve_base_damage_dice`)
/// stepped by every `BONUS:EQMWEAPON|DAMAGESIZE|<n>` chain carried by its
/// `EQMOD:`-referenced modifier records (SD-33 remediation wave 6,
/// `eqm-modifier-final` lane, `AT-33-E5-002` -- this shape had no
/// resolver at all before this cycle, confirmed absent by
/// `grep -rn "DAMAGESIZE" src/rules_core/equipment_effects*.rs
/// src/rules_core/damage_total.rs` returning nothing). The die-STEP
/// analogue of `resolve_weapon_enhancement_modifier`'s scalar bonus.
///
/// Returns `None` when the item has no base dice, carries no
/// `DAMAGESIZE` chain (a real "no step to apply" case a caller should
/// distinguish from a covered step by first calling
/// `resolve_base_damage_dice`), or the step lands outside
/// `step_single_die`'s covered table -- honest absence, never a
/// fabricated die.
pub fn resolve_eqmweapon_damagesize_effect(
    weapon_item_id: &str,
    corpus: &SourcePackageContent,
) -> Option<DiceExpression> {
    let record = equipment_converted_resolve(weapon_item_id, corpus)?;
    let base = record.base_damage_dice?;
    let eqmod_records =
        crate::rules_core::equipment_effects::eqmod_referenced_converted_records(record, corpus);
    let steps: i32 = eqmod_records.iter().map(|modifier| modifier.damage_size_steps).sum();
    if steps == 0 {
        return None;
    }
    step_single_die(base, steps)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
    let (record, table_cell) =
        equipment_id_resolve(weapon_item_id, RuleSetId::Crb, corpus)?;
    let wield_category = record.wield_category?;
    let weapon_record_key = record.identity.clone();
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
    let (record, table_cell) =
        equipment_id_resolve(weapon_item_id, RuleSetId::Crb, corpus)?;
    let weapon_record_key = record.identity.clone();
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
    let (record, table_cell) =
        equipment_id_resolve(weapon_item_id, RuleSetId::Crb, corpus)?;
    let critical_threat_range = record.critical_threat_range?;
    let weapon_record_key = record.identity.clone();

    Some(DamageRollCriticalThreatRange {
        weapon_item_id: weapon_item_id.to_string(),
        weapon_record_key,
        critical_threat_range,
        table_cell,
    })
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
    let (record, table_cell) =
        equipment_id_resolve(weapon_item_id, RuleSetId::Crb, corpus)?;
    let critical_multiplier = record.critical_multiplier?;
    let weapon_record_key = record.identity.clone();

    Some(DamageRollCriticalMultiplier {
        weapon_item_id: weapon_item_id.to_string(),
        weapon_record_key,
        critical_multiplier,
        table_cell,
    })
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
/// Reads `rules_catalog::crb::feats::feat_tables()` directly (no
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

/// A feat bonus is a directly-usable constant damage bonus, per
/// `resolve_feat_damage_effect`'s scoping doc comment, only when it is
/// untyped and unconditional, its target is the bare `DAMAGE` string, its
/// value is an integer, and its category is not the formula-variable
/// category (which defines a named variable for other tokens to reference
/// rather than a direct roll bonus — e.g. Power Attack). Anything else (a
/// qualified/compound target, a non-numeric value, a formula-variable
/// category, a wrong-length qualifier list) is a formula or a
/// non-constant-damage bonus and is honestly excluded, not coerced.
///
/// A row whose bonus is scoped to the character's chosen weapon
/// (`FeatEffectBonus.selection`) carries one fewer qualifier slot, because
/// the selection is no longer spelled inside the chain — SD-35
/// `AT-35-E6-003-SWEEP` cycle 12. Weapon Specialization and Greater Weapon
/// Specialization are that shape, and the match arms below are what keeps
/// their `+2` reaching the sheet.
fn constant_damage_bonus(bonus: &FeatEffectBonus) -> Option<i16> {
    // A typed or conditioned bonus is not a flat constant this slice can add.
    // Before SD-35 `AT-35-E6-003-SWEEP` cycle 7 the stacking label and the
    // guards were extra `qualifiers` elements, so the length check below
    // already excluded every one of them; that cycle moved them into
    // `bonus_type` and `conditions`, and this check keeps the excluded set
    // exactly what it was. It is a deliberate behaviour-preserving guard, not
    // a new rule: whether a typed damage bonus should contribute is a rules
    // question this exit cycle does not answer.
    if bonus.bonus_type.is_some() || !bonus.conditions.is_empty() {
        return None;
    }
    let qualifiers = bonus.qualifiers;

    // SD-35 `AT-35-E6-003-SWEEP` cycle 12 typed the character's own selection
    // out of the chain, which SHORTENS it by one slot on the rows that carry
    // one. Weapon Specialization and its Greater form are exactly such rows
    // (`WEAPONPROF=%LIST|DAMAGE|2` before the conversion), and they are the
    // two feats this function exists to resolve, so the arms below are what
    // preserve their `+2`. The excluded set is unchanged; each arm says why.
    let (category, target, value) = match bonus.selection {
        // The chosen weapon stood in the CATEGORY slot, which is now gone:
        // what remains is `[target, value]`, and the category is "the weapon
        // the character picked" rather than a fixed string. Never `VAR`.
        Some(EffectSelection::ChosenWeapon) => {
            if qualifiers.len() != 2 {
                return None;
            }
            (None, qualifiers[0], qualifiers[1])
        }
        // Every other selection stood in the target slot (Skill Focus, Spell
        // Focus) or in the value slot (Master Craftsman, Multitalented
        // Mastery). Neither is a constant damage bonus, and neither was one
        // before the conversion either: the first is not the bare `DAMAGE`
        // target, and the second never parsed as an integer.
        Some(_) => return None,
        None => {
            if qualifiers.len() != 3 {
                return None;
            }
            (Some(qualifiers[0]), qualifiers[1], qualifiers[2])
        }
    };
    if category == Some("VAR") || target != "DAMAGE" {
        return None;
    }
    value.parse::<i16>().ok()
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
    /// `false` for a carried (`SelectedInactive`) weapon, which only
    /// [`resolve_weapon_damage_breakdown_with_carried`] returns.
    pub in_hand: bool,
    /// `true` for the one Unarmed row the carried view ends with ([`unarmed_strike_row`]).
    pub unarmed: bool,
    /// The strike deals nonlethal damage: the Unarmed row of a character without Improved
    /// Unarmed Strike.
    pub nonlethal: bool,
    /// The weapon fires ammunition (its record, or the record standing in for it, is a `Projectile`).
    pub uses_ammunition: bool,
    /// Pieces of carried ammunition this weapon fires ([`ammunition_carried`]); 0 when it uses none.
    pub ammunition_carried: u32,
}

/// The corpus record every character's Unarmed row is built from (Core Rulebook
/// `Unarmed Strike`, `arms_armor/unarmed_strike.json`).
pub const UNARMED_STRIKE_ITEM_ID: &str = "item:unarmed_strike";

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
    weapon_rows(character, corpus, equipment_effects, str_modifier, ActiveState::EquippedActive, &[])
}

/// SD-41 E1.4: the in-hand rows of [`resolve_weapon_damage_breakdown`] (unchanged, first), then
/// one row per carried (`SelectedInactive`) weapon with `in_hand: false`, resolved with the same
/// arithmetic as a weapon in the main hand (a carried `+N` reaches it through its own selection's
/// `attached` bonus, since `equipment_effects` holds in-hand items only). `Absent` weapons have no row.
/// `quantities` are the pieces per copy the equipment-sets file states (E1.6, `decisions.md §30`);
/// a copy it does not name counts one bundle.
pub fn resolve_weapon_damage_breakdown_with_carried(
    character: &CharacterInput,
    corpus: &SourcePackageContent,
    equipment_effects: &EquipmentEffects,
    str_modifier: i16,
    quantities: &[(CopyKey, u32)],
) -> Vec<WeaponDamageBreakdown> {
    let mut rows = weapon_rows(character, corpus, equipment_effects, str_modifier, ActiveState::EquippedActive, quantities);
    rows.extend(weapon_rows(character, corpus, equipment_effects, str_modifier, ActiveState::SelectedInactive, quantities));
    rows.extend(unarmed_strike_row(character, corpus, equipment_effects, str_modifier));
    rows
}

/// SD-41 E1.5 (`decisions.md §22`): the Unarmed row every character has, the carried view's last.
/// The record's own die, critical and `WIELD:` (`Light`, so full Strength); one size step down for a
/// Small character (`1d3` -> `1d2`); the monk's die by level and size when the character has monk
/// levels ([`monk_unarmed_strike_damage_die_for_size`], the table the Core Rulebook and Unchained
/// Monk paths both read). Only natural-attack enhancements reach it (an Amulet of Mighty Fists), never
/// a held weapon's `+N`. Nonlethal unless the character holds Improved Unarmed Strike.
/// `None` only when the corpus lacks the record.
fn unarmed_strike_row(
    character: &CharacterInput,
    corpus: &SourcePackageContent,
    equipment_effects: &EquipmentEffects,
    str_modifier: i16,
) -> Option<WeaponDamageBreakdown> {
    use crate::rules_core::pilot_compute::{holds_improved_unarmed_strike, monk_unarmed_strike_damage_die_for_size, unarmed_strike_monk_level};
    use crate::rules_core::race_resolver::race_size_for_race_token;
    use crate::rules_core::size::SizeCategory;
    let mut base_dice = resolve_base_damage_dice(UNARMED_STRIKE_ITEM_ID, corpus)?;
    let size = race_size_for_race_token(&character.chosen.race_id);
    let monk_level = unarmed_strike_monk_level(character);
    // shortcut: only the Small and Medium columns are modelled (every playable race); another size has no die.
    let die = match (monk_level, size) {
        (1.., Some(size)) => monk_unarmed_strike_damage_die_for_size(monk_level, size)
            .map(|(face, count, _)| DiceExpression { count: count as u8, die_size: face as u8 }),
        (0, Some(SizeCategory::Medium)) => Some(base_dice.base_dice),
        (0, Some(SizeCategory::Small)) => step_single_die(base_dice.base_dice, -1),
        _ => None,
    };
    let weapon_enhancement = resolve_weapon_enhancement_modifier(UNARMED_STRIKE_ITEM_ID, corpus, equipment_effects).map(|mut part| {
        let natural = || equipment_effects.per_item.iter().filter_map(|item| item.weapon_enhancement_bonus.as_ref()).filter(|bonus| bonus.natural_attack_only);
        part.attack_bonus = natural().filter_map(|bonus| bonus.tohit_bonus).sum();
        part.damage_bonus = natural().filter_map(|bonus| bonus.damage_bonus).sum();
        part
    });
    Some(WeaponDamageBreakdown {
        weapon_item_id: UNARMED_STRIKE_ITEM_ID.to_owned(),
        base_dice: die.map(|die| {
            base_dice.base_dice = die;
            base_dice
        }),
        str_modifier: resolve_str_damage_modifier(UNARMED_STRIKE_ITEM_ID, corpus, str_modifier, WeaponHandSlot::Primary),
        weapon_enhancement,
        critical_threat_range: resolve_critical_threat_range(UNARMED_STRIKE_ITEM_ID, corpus),
        critical_multiplier: resolve_critical_multiplier(UNARMED_STRIKE_ITEM_ID, corpus),
        feat_effects: character.chosen.selected_feats.iter().filter_map(|feat| resolve_feat_damage_effect(feat)).collect(),
        in_hand: true,
        unarmed: true,
        nonlethal: !holds_improved_unarmed_strike(character),
        uses_ammunition: false,
        ammunition_carried: 0,
    })
}

/// SD-41 E1.6 (`decisions.md §23, §30`): the pieces of ammunition `character` carries for a weapon
/// of `family` (`None`: any ammunition fits): over the `EquippedActive` and `SelectedInactive`
/// copies whose record (or its stand-in) is ammunition of that family, each copy's quantity from
/// `quantities`, else its record's `base_quantity()` (one bundle).
pub fn ammunition_carried(
    character: &CharacterInput,
    corpus: &SourcePackageContent,
    family: Option<&str>,
    quantities: &[(CopyKey, u32)],
) -> u32 {
    let selections = &character.chosen.equipment_selections;
    let keys = copy_keys(selections.iter().map(|selection| (selection.item_id.as_str(), copy_bonus(selection))));
    selections
        .iter()
        .zip(keys)
        .filter(|(selection, _)| selection.active_state != ActiveState::Absent)
        .filter_map(|(selection, key)| {
            let record = equipment_converted_resolve(&selection.item_id, corpus)?;
            let ammunition = Some(record).filter(|r| r.is_ammunition()).or_else(|| stand_in_record(record, corpus))?;
            ammunition.is_ammunition_for(family).then(|| {
                quantities.iter().find(|(copy, _)| *copy == key).map_or_else(
                    || record.base_quantity.or(ammunition.base_quantity).unwrap_or(1),
                    |(_, pieces)| *pieces,
                )
            })
        })
        .sum()
}

/// A copy's `+N`, as the equipment-sets file keys it: the highest `Special Ability ~ +N ~ …` it holds.
fn copy_bonus(selection: &EquipmentSelection) -> u8 {
    use crate::rules_core::body_slots::Enhanceable;
    (1..=10u8)
        .filter(|n| {
            [Enhanceable::Armor, Enhanceable::Shield, Enhanceable::Weapon]
                .iter()
                .any(|kind| selection.applied_modifiers.contains(&kind.modifier_key(*n)))
        })
        .max()
        .unwrap_or(0)
}

/// One breakdown per weapon selection in `state`, in input order.
fn weapon_rows(
    character: &CharacterInput,
    corpus: &SourcePackageContent,
    equipment_effects: &EquipmentEffects,
    str_modifier: i16,
    state: ActiveState,
    quantities: &[(CopyKey, u32)],
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
        .filter(|selection| selection.active_state == state)
        .filter_map(|selection| {
            let weapon_item_id = selection.item_id.as_str();
            let base_dice = resolve_base_damage_dice(weapon_item_id, corpus)?;
            let ammunition_user = equipment_converted_resolve(weapon_item_id, corpus).and_then(|record| {
                Some(record).filter(|r| r.uses_ammunition()).or_else(|| stand_in_record(record, corpus).filter(|base| base.uses_ammunition()))
            });

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
                )
                .map(|mut enhancement| {
                    // This selection's own `+N`, which no other weapon in the loadout shares. One
                    // bonus type, so the higher stands, not the sum.
                    let attached = equipment_converted_resolve(weapon_item_id, corpus).map(|record| {
                        attached_weapon_enhancement(record, &selection.applied_modifiers, corpus)
                    });
                    if let Some(attached) = attached {
                        enhancement.attack_bonus = enhancement.attack_bonus.max(attached.tohit);
                        enhancement.damage_bonus = enhancement.damage_bonus.max(attached.damage);
                    }
                    enhancement
                }),
                critical_threat_range: resolve_critical_threat_range(weapon_item_id, corpus),
                critical_multiplier: resolve_critical_multiplier(weapon_item_id, corpus),
                feat_effects: feat_effects.clone(),
                in_hand: state == ActiveState::EquippedActive,
                unarmed: false,
                nonlethal: false,
                uses_ammunition: ammunition_user.is_some(),
                ammunition_carried: ammunition_user
                    .map_or(0, |weapon| ammunition_carried(character, corpus, weapon.ammunition_family(), quantities)),
            })
        })
        .collect()
}

/// SD-40 E2.14: one weapon's damage as the sheet prints it, the die and one signed sum (`1d8+4`),
/// as the `combat.weapon_damage_total.<weapon>` row. The sum is facets the load response already
/// holds, added and nothing else: the Strength part [`resolve_weapon_damage_breakdown`] read
/// (`strength`: its value and the corpus `WIELD:` word), the weapon's enhancement to damage, the
/// weapon's feat damage row (`feat_row`: [`weapon_feat_damage_row_id`], the Specialization feats
/// that name this weapon) and each item damage bonus in `damage_bonuses`. A feat's own entry in
/// `damage_bonuses` (`resolve_feat_damage_effect` knows it) is not added again: the feat row
/// already holds it, for the weapon the feat chose. Its value is the sum; its detail opens with
/// the printed `1d8+4` and names every part. `None` when the weapon has no dice: absence, never `0`.
///
/// The Strength part is the primary hand's (`WeaponHandSlot::Primary`): no input records a hand,
/// so an off-hand weapon's half Strength is not applied, and the detail says so.
pub fn weapon_damage_total_row(
    weapon_record_key: &str,
    dice: Option<DiceExpression>,
    strength: Option<(i16, String)>,
    enhancement: Option<i16>,
    feat_row: Option<i16>,
    damage_bonuses: &[(String, i16)],
) -> Option<ComputationExplanation> {
    let dice = dice?;
    let mut parts: Vec<String> = Vec::new();
    let mut sum = 0_i16;
    if let Some((value, wield)) = &strength {
        sum += value;
        parts.push(format!("Strength {value:+} (primary hand, {wield} WIELD)"));
    }
    if let Some(value) = enhancement {
        sum += value;
        parts.push(format!("enhancement {value:+}"));
    }
    match feat_row {
        Some(value) => {
            sum += value;
            parts.push(format!("feats naming this weapon {value:+}"));
        }
        None => parts.push("no feat damage row for this weapon".to_owned()),
    }
    for (source, value) in damage_bonuses.iter().filter(|(source, _)| resolve_feat_damage_effect(source).is_none()) {
        sum += value;
        parts.push(format!("{source} {value:+}"));
    }
    let die = format!("{}d{}", dice.count, dice.die_size);
    let printed = if sum == 0 { die.clone() } else { format!("{die}{sum:+}") };
    Some(ComputationExplanation {
        id: format!("combat.weapon_damage_total.{}", normalize_weapon_identity(weapon_record_key)),
        value: sum,
        detail: format!(
            "{printed}: damage with the equipped {weapon_record_key}: the {die} die + {} = {printed}. \
             The Strength part is the primary hand's: no input records which hand holds a weapon, so \
             an off-hand weapon's half Strength is not applied.",
            parts.join(" + ")
        ),
    })
}

/// The id of the row holding a weapon's feat damage (`combat.weapon_damage_bonus.<weapon>`), the
/// part [`weapon_damage_total_row`] reads as `feat_row`.
pub fn weapon_feat_damage_row_id(weapon_record_key: &str) -> String {
    format!("combat.weapon_damage_bonus.{}", normalize_weapon_identity(weapon_record_key))
}

/// SD-40 E2.14: the range increment, in feet, the weapon's record states (`RANGE:10` on
/// `Dagger (Base)`), read from its sheet-rule record's Range line; a variant that states none
/// (`Dagger (Adamantine)`) reads its base item's. `None` for a weapon with no range (a melee
/// weapon) or one whose range is not a whole number of feet.
pub fn weapon_range_increment(
    package: &SheetRulePackage,
    corpus: &SourcePackageContent,
    weapon_record_key: &str,
) -> Option<u16> {
    let stated = |key: &str| {
        let rule = package.rule(package.find("equipment", &slug(key))?)?;
        let words: String = rule
            .prose
            .iter()
            .filter(|segment| matches!(&segment.family, ProseFamily::StatBlock(name) if name == "Range"))
            .flat_map(|segment| &segment.pieces)
            .filter_map(|piece| match piece {
                ProsePiece::Text(text) => Some(text.as_str()),
                _ => None,
            })
            .collect();
        words.trim().trim_end_matches("ft.").trim().parse::<u16>().ok()
    };
    stated(weapon_record_key).or_else(|| {
        let base = equipment_converted_resolve(weapon_record_key, corpus)?.base_item.as_deref()?;
        stated(base)
    })
}

#[cfg(test)]
mod tests {
    use super::*;







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





    #[test]
    fn constant_damage_bonus_examples() {
        assert_eq!(
            constant_damage_bonus(&FeatEffectBonus {
                qualifiers: &["DAMAGE", "2"],
                bonus_type: None,
                conditions: &[],
                selection: Some(EffectSelection::ChosenWeapon),
            }),
            Some(2)
        );
        assert_eq!(
            constant_damage_bonus(&FeatEffectBonus {
                qualifiers: &["VAR", "PowerAttackDamageBase", "2"],
                bonus_type: None,
                conditions: &[],
                selection: None,
            }),
            None,
            "a VAR-category token defines a formula variable, not a direct bonus"
        );
        assert_eq!(
            constant_damage_bonus(&FeatEffectBonus {
                qualifiers: &["COMBAT", "TOHIT-SHORTRANGE,DAMAGE-SHORTRANGE", "1"],
                bonus_type: None,
                conditions: &[],
                selection: None,
            }),
            None,
            "a compound/qualified target is not the bare DAMAGE this slice models"
        );
        assert_eq!(
            constant_damage_bonus(&FeatEffectBonus {
                qualifiers: &["HP", "CURRENTMAX", "max(3,TL)"],
                bonus_type: None,
                conditions: &[],
                selection: None,
            }),
            None,
            "a non-numeric value signals a formula, not a constant"
        );
    }
}

/// SD-33 remediation wave 6 (`eqm-modifier-final` lane, `AT-33-E5-002`):
/// the `EQMWEAPON|DAMAGESIZE` shape's own tests -- a genuinely unhandled
/// shape before this cycle (confirmed absent by
/// `grep -rn "DAMAGESIZE" src/rules_core/equipment_effects*.rs
/// src/rules_core/damage_total.rs` returning nothing pre-cycle).
#[cfg(test)]
mod eqmweapon_damagesize_tests {
    use super::*;


    #[test]
    fn single_die_step_table_covers_the_real_progression() {
        assert_eq!(
            step_single_die(DiceExpression { count: 1, die_size: 4 }, 1),
            Some(DiceExpression { count: 1, die_size: 6 }),
            "1d4 stepped up one must be 1d6"
        );
        assert_eq!(
            step_single_die(DiceExpression { count: 1, die_size: 6 }, -1),
            Some(DiceExpression { count: 1, die_size: 4 }),
            "1d6 stepped down one must be 1d4"
        );
        assert_eq!(
            step_single_die(DiceExpression { count: 1, die_size: 12 }, 1),
            None,
            "stepping past the table's top must be honest None, not a fabricated die"
        );
        assert_eq!(
            step_single_die(DiceExpression { count: 2, die_size: 6 }, 1),
            None,
            "a multi-die base (2d6) is outside this table's covered cases"
        );
    }


}

/// SD-40 E2.14: one damage total per weapon in hand, and the range increment its record states.
#[cfg(test)]
mod weapon_damage_total_tests {
    use super::*;
    use crate::rules_core::character_input::{load_character_input_fixture, EquipmentSelection};
    use crate::rules_core::corpus_loader::{load_equipment_corpus, BookCorpusRoot};
    use crate::rules_core::equipment_effects::compute_equipment_effects;
    use std::path::Path;

    fn core_rulebook() -> SourcePackageContent<'static> {
        load_equipment_corpus(&[BookCorpusRoot { book_id: "core_rulebook", dir: Path::new("data/corpus/core_rulebook") }])
    }

    fn breakdown(item: &str, modifiers: &[&str], feats: &[&str], str_modifier: i16) -> WeaponDamageBreakdown {
        let corpus = core_rulebook();
        let mut input = load_character_input_fixture(
            "case_id=case:test\nsource_package_id=core_rulebook\nrace_id=race:human\nclass_level=class:fighter:3\n\
             ability=strength:19\nability=dexterity:13\nability=constitution:14\nability=intelligence:14\n\
             ability=wisdom:12\nability=charisma:8\n",
        )
        .character_input
        .expect("the fixture loads");
        input.chosen.selected_feats = feats.iter().map(|feat| (*feat).to_owned()).collect();
        input.chosen.equipment_selections = vec![EquipmentSelection {
            item_id: item.to_owned(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: modifiers.iter().map(|m| (*m).to_owned()).collect(),
        }];
        let effects = compute_equipment_effects(&input.chosen.equipment_selections, &corpus);
        resolve_weapon_damage_breakdown(&input, &corpus, &effects, str_modifier).remove(0)
    }

    fn row_of(weapon: &WeaponDamageBreakdown, feat_row: i16, items: &[(&str, i16)]) -> Option<ComputationExplanation> {
        let mut feats: Vec<(String, i16)> =
            weapon.feat_effects.iter().map(|feat| (feat.feat_key.clone(), feat.damage_bonus)).collect();
        feats.extend(items.iter().map(|(name, value)| ((*name).to_owned(), *value)));
        weapon_damage_total_row(
            weapon.base_dice.as_ref().map(|dice| dice.weapon_record_key.as_str()).unwrap_or(&weapon.weapon_item_id),
            weapon.base_dice.as_ref().map(|dice| dice.base_dice),
            weapon.str_modifier.as_ref().map(|part| (part.str_damage_modifier, format!("{:?}", part.wield_category))),
            weapon.weapon_enhancement.as_ref().map(|part| part.damage_bonus),
            Some(feat_row),
            &feats,
        )
    }

    /// Aldric's longsword (Strength 19, +4; one-handed in the primary hand; no `+N`; his feat
    /// damage row reads 0): the printed value is the die and the one signed sum, `1d8+4`.
    #[test]
    fn weapon_damage_total_of_aldrics_longsword_is_the_die_and_one_sum() {
        let sword = breakdown("Longsword (Base)", &[], &[], 4);
        let row = row_of(&sword, 0, &[]).expect("a weapon with dice has a total");
        let facets = sword.str_modifier.as_ref().unwrap().str_damage_modifier
            + sword.weapon_enhancement.as_ref().map_or(0, |part| part.damage_bonus);
        assert_eq!(row.id, "combat.weapon_damage_total.longsword");
        assert_eq!(weapon_feat_damage_row_id("Longsword (Base)"), "combat.weapon_damage_bonus.longsword");
        assert_eq!(row.value, facets);
        assert_eq!(row.value, 4);
        assert!(row.detail.starts_with("1d8+4:"), "{}", row.detail);
        assert!(row.detail.contains("primary hand") && row.detail.contains("OneHanded"), "{}", row.detail);
    }

    /// Every part sums: Strength +4, the sword's own `+1`, the feat row's Weapon Specialization
    /// `+2` and an item's `+3` -> `1d8+10`. The breakdown's own Weapon Specialization entry (`+2`
    /// on every weapon) is not added a second time. A negative sum keeps its sign (`1d8-1`); a
    /// zero sum prints the die alone.
    #[test]
    fn weapon_damage_total_sums_strength_enhancement_feats_and_items() {
        let sword = breakdown("Longsword (Base)", &["Special Ability ~ +1 ~ Weapon"], &["Weapon Specialization"], 4);
        assert_eq!(sword.feat_effects.iter().map(|feat| feat.damage_bonus).sum::<i16>(), 2, "{:?}", sword.feat_effects);
        let row = row_of(&sword, 2, &[("Gauntlet of Everything", 3)]).unwrap();
        assert_eq!(row.value, 10, "{}", row.detail);
        assert!(row.detail.starts_with("1d8+10:"), "{}", row.detail);
        assert!(row.detail.contains("feats naming this weapon +2") && row.detail.contains("Gauntlet of Everything +3"), "{}", row.detail);
        assert!(row_of(&breakdown("Longsword (Base)", &[], &[], -1), 0, &[]).unwrap().detail.starts_with("1d8-1:"));
        assert!(row_of(&breakdown("Longsword (Base)", &[], &[], 0), 0, &[]).unwrap().detail.starts_with("1d8:"));
    }

    /// A weapon the Core Rulebook table does not hold by its own name (`Dagger (Adamantine)`) has
    /// its feat damage row on the corpus path too, so its total holds Weapon Specialization
    /// (Dagger): `+2`, read from the base weapon the feat chose.
    #[test]
    fn weapon_damage_total_feat_row_of_a_variant_weapon() {
        let corpus = core_rulebook();
        let mut input = load_character_input_fixture(
            "case_id=case:test\nsource_package_id=core_rulebook\nrace_id=race:human\nclass_level=class:fighter:4\n\
             ability=strength:19\nability=dexterity:13\nability=constitution:14\nability=intelligence:14\n\
             ability=wisdom:12\nability=charisma:8\n",
        )
        .character_input
        .expect("the fixture loads");
        input.chosen.selected_feats = vec!["feat:weapon_specialization".to_owned()];
        input.chosen.selected_choices.push(crate::rules_core::character_input::SelectedChoice {
            choice_set_id: "choice:weapon_specialization_target".to_owned(),
            selection_id: "weapon:Dagger".to_owned(),
        });
        input.chosen.equipment_selections = vec![EquipmentSelection {
            item_id: "Dagger (Adamantine)".to_owned(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
        }];
        let base = crate::rules_core::pilot_compute::compute_pilot_base_chassis(&input);
        let rows = crate::rules_core::pilot_compute_corpus::corpus_weapon_attack_rows(&base, &input, &corpus);
        let id = weapon_feat_damage_row_id("Dagger (Adamantine)");
        let row = rows.iter().find(|row| row.id == id).unwrap_or_else(|| panic!("no {id} in {rows:#?}"));
        assert_eq!(row.value, 2, "{}", row.detail);
    }

    /// A weapon with no dice has no total row: absence, never `0`.
    #[test]
    fn weapon_damage_total_needs_dice() {
        assert_eq!(weapon_damage_total_row("Net (Base)", None, None, None, None, &[]), None);
    }

    /// The range increment is the record's own `RANGE:` (the dagger's 10 ft.), or its base item's
    /// for a variant that states none; a melee weapon has none.
    #[test]
    fn weapon_damage_total_range_increment_is_the_records() {
        let package = crate::rules_core::sheet_rule_package::package().as_ref().expect("the converted package loads");
        let corpus = core_rulebook();
        assert_eq!(weapon_range_increment(package, &corpus, "Dagger (Base)"), Some(10));
        assert_eq!(weapon_range_increment(package, &corpus, "Dagger (Adamantine)"), Some(10));
        assert_eq!(weapon_range_increment(package, &corpus, "Longsword (Base)"), None);
    }
}

/// SD-41 E1.4: a carried (`SelectedInactive`) weapon gets a row from the carried view, marked
/// `in_hand: false`, with the same arithmetic as the weapon held in the main hand; the in-hand
/// view is unchanged and `Absent` weapons appear in neither.
#[cfg(test)]
mod carried_weapon_tests {
    use super::*;
    use crate::rules_core::character_input::{load_character_input_fixture, EquipmentSelection};
    use crate::rules_core::corpus_loader::{load_equipment_corpus, BookCorpusRoot};
    use crate::rules_core::equipment_effects::compute_equipment_effects;
    use std::path::Path;

    type Resolver = fn(&CharacterInput, &SourcePackageContent, &EquipmentEffects, i16) -> Vec<WeaponDamageBreakdown>;

    fn select(item: &str, state: ActiveState, modifiers: &[&str]) -> EquipmentSelection {
        EquipmentSelection {
            item_id: item.to_owned(),
            equipped_or_active: state == ActiveState::EquippedActive,
            active_state: state,
            applied_modifiers: modifiers.iter().map(|m| (*m).to_owned()).collect(),
        }
    }

    /// The callers' shape: equipment effects over the in-hand selections only.
    fn rows(selections: Vec<EquipmentSelection>, resolve: Resolver) -> Vec<WeaponDamageBreakdown> {
        let corpus = load_equipment_corpus(&[BookCorpusRoot { book_id: "core_rulebook", dir: Path::new("data/corpus/core_rulebook") }]);
        let mut input = load_character_input_fixture(
            "case_id=case:test\nsource_package_id=core_rulebook\nrace_id=race:human\nclass_level=class:fighter:3\n\
             ability=strength:19\nability=dexterity:13\nability=constitution:14\nability=intelligence:14\n\
             ability=wisdom:12\nability=charisma:8\n",
        )
        .character_input
        .expect("the fixture loads");
        input.chosen.selected_feats = vec!["Weapon Specialization".to_owned()];
        input.chosen.equipment_selections = selections;
        let held: Vec<EquipmentSelection> = input
            .chosen
            .equipment_selections
            .iter()
            .filter(|s| s.active_state == ActiveState::EquippedActive)
            .cloned()
            .collect();
        let effects = compute_equipment_effects(&held, &corpus);
        resolve(&input, &corpus, &effects, 4)
    }

    const PLUS_ONE: &[&str] = &["Special Ability ~ +1 ~ Weapon"];

    /// A carried `+1` longsword: one weapon row, `in_hand: false`, and every other field equal to the same
    /// sword held in the main hand (its `+1` reaches it through its own selection).
    #[test]
    fn carried_weapon_with_carried_gets_a_row_not_in_hand() {
        let carried = rows(vec![select("Longsword (Base)", ActiveState::SelectedInactive, PLUS_ONE)], |i, c, e, s| resolve_weapon_damage_breakdown_with_carried(i, c, e, s, &[]));
        assert_eq!(carried.len(), 2, "the sword, then the Unarmed row (E1.5): {carried:#?}");
        assert!(!carried[0].in_hand);
        let enhancement = carried[0].weapon_enhancement.as_ref().expect("a weapon has an enhancement part");
        assert_eq!((enhancement.attack_bonus, enhancement.damage_bonus), (1, 1));

        let held = rows(vec![select("Longsword (Base)", ActiveState::EquippedActive, PLUS_ONE)], resolve_weapon_damage_breakdown);
        assert_eq!(held.len(), 1);
        assert!(held[0].in_hand);
        assert_eq!(WeaponDamageBreakdown { in_hand: true, ..carried[0].clone() }, held[0], "same arithmetic as in hand");
    }

    /// The old entry point keeps its contract: in-hand weapons only, each `in_hand: true`.
    #[test]
    fn carried_weapon_old_entry_point_unchanged_in_hand_only() {
        let old = rows(
            vec![
                select("Longsword (Base)", ActiveState::EquippedActive, &[]),
                select("Dagger (Base)", ActiveState::SelectedInactive, &[]),
            ],
            resolve_weapon_damage_breakdown,
        );
        assert_eq!(old.iter().map(|row| (row.weapon_item_id.as_str(), row.in_hand)).collect::<Vec<_>>(), [("Longsword (Base)", true)]);
        assert!(rows(vec![select("Dagger (Base)", ActiveState::SelectedInactive, PLUS_ONE)], resolve_weapon_damage_breakdown).is_empty());
    }

    /// The carried view lists the in-hand rows first, byte-identical to the old entry point's, then
    /// the carried rows (input order within each group).
    #[test]
    fn carried_weapon_in_hand_rows_identical_between_entry_points() {
        let loadout = || {
            vec![
                select("Dagger (Base)", ActiveState::SelectedInactive, PLUS_ONE),
                select("Longsword (Base)", ActiveState::EquippedActive, PLUS_ONE),
                select("Breastplate (Base)", ActiveState::EquippedActive, &[]),
                select("Heavy Mace (Base)", ActiveState::EquippedActive, &[]),
            ]
        };
        let old = rows(loadout(), resolve_weapon_damage_breakdown);
        let new = rows(loadout(), |i, c, e, s| resolve_weapon_damage_breakdown_with_carried(i, c, e, s, &[]));
        assert_eq!(old.len(), 2, "{old:#?}");
        assert_eq!(new.len(), 4, "the Unarmed row last (E1.5): {new:#?}");
        assert_eq!(new[..2], old[..]);
        assert_eq!((new[2].weapon_item_id.as_str(), new[2].in_hand), ("Dagger (Base)", false));
    }

    /// An `Absent` weapon has no row in either view; a carried non-weapon has none either (the carried view holds only its Unarmed row).
    #[test]
    fn carried_weapon_absent_has_no_row_in_either() {
        let loadout = || {
            vec![
                select("Longsword (Base)", ActiveState::Absent, PLUS_ONE),
                select("Breastplate (Base)", ActiveState::SelectedInactive, &[]),
            ]
        };
        assert!(rows(loadout(), resolve_weapon_damage_breakdown).is_empty());
        let carried = rows(loadout(), |i, c, e, s| resolve_weapon_damage_breakdown_with_carried(i, c, e, s, &[]));
        assert!(carried.iter().all(|row| row.unarmed), "only the Unarmed row (E1.5): {carried:#?}");
    }
}

/// SD-41 E1.5: the carried view always ends with one Unarmed row, built from the Core Rulebook
/// `Unarmed Strike` record: `1d3` nonlethal for a Medium character, the record's die one step down
/// for a Small one, the monk's die when the character has monk levels, lethal with Improved Unarmed
/// Strike. Its attack and feat damage rows are the per-weapon melee path's over the same record.
#[cfg(test)]
mod unarmed_row_tests {
    use super::*;
    use crate::rules_core::character_input::{load_character_input_fixture, EquipmentSelection};
    use crate::rules_core::corpus_loader::{load_equipment_corpus, BookCorpusRoot};
    use crate::rules_core::equipment_effects::compute_equipment_effects;
    use crate::rules_core::pilot_compute::{compute_pilot_base_chassis, unarmed_strike_combat_rows};
    use std::path::Path;

    fn input(race: &str, class: &str, feats: &[&str], selections: Vec<EquipmentSelection>) -> CharacterInput {
        let mut input = load_character_input_fixture(&format!(
            "case_id=case:test\nsource_package_id=core_rulebook\nrace_id=race:{race}\nclass_level=class:{class}\n\
             ability=strength:19\nability=dexterity:13\nability=constitution:14\nability=intelligence:14\n\
             ability=wisdom:12\nability=charisma:8\n"
        ))
        .character_input
        .expect("the fixture loads");
        input.chosen.selected_feats = feats.iter().map(|feat| (*feat).to_owned()).collect();
        input.chosen.equipment_selections = selections;
        input
    }

    fn select(item: &str, state: ActiveState, modifiers: &[&str]) -> EquipmentSelection {
        EquipmentSelection {
            item_id: item.to_owned(),
            equipped_or_active: state == ActiveState::EquippedActive,
            active_state: state,
            applied_modifiers: modifiers.iter().map(|m| (*m).to_owned()).collect(),
        }
    }

    fn rows(input: &CharacterInput) -> Vec<WeaponDamageBreakdown> {
        let corpus = load_equipment_corpus(&[BookCorpusRoot { book_id: "core_rulebook", dir: Path::new("data/corpus/core_rulebook") }]);
        let held: Vec<EquipmentSelection> = input
            .chosen
            .equipment_selections
            .iter()
            .filter(|s| s.active_state == ActiveState::EquippedActive)
            .cloned()
            .collect();
        let effects = compute_equipment_effects(&held, &corpus);
        resolve_weapon_damage_breakdown_with_carried(input, &corpus, &effects, 4, &[])
    }

    fn unarmed(input: &CharacterInput) -> WeaponDamageBreakdown {
        let rows = rows(input);
        let last = rows.last().expect("the Unarmed row is always there").clone();
        assert!(last.unarmed && last.in_hand, "{last:#?}");
        assert_eq!(rows.iter().filter(|row| row.unarmed).count(), 1, "{rows:#?}");
        last
    }

    fn dice(row: &WeaponDamageBreakdown) -> Option<(u8, u8)> {
        row.base_dice.as_ref().map(|dice| (dice.base_dice.count, dice.base_dice.die_size))
    }

    /// Aldric (Human Fighter 3, Strength 19): a `+1` longsword in hand and a dagger carried, then the
    /// Unarmed row last: `1d3`, nonlethal, no share of the sword's `+1`; its attack row is base
    /// attack +3 and Strength +4; its damage total prints `1d3+4`. The old entry point has no
    /// Unarmed row.
    #[test]
    fn unarmed_row_is_last_and_nonlethal_for_aldric() {
        let aldric = input(
            "human",
            "fighter:3",
            &[],
            vec![
                select("Longsword (Base)", ActiveState::EquippedActive, &["Special Ability ~ +1 ~ Weapon"]),
                select("Dagger (Base)", ActiveState::SelectedInactive, &[]),
            ],
        );
        let all = rows(&aldric);
        assert_eq!(
            all.iter().map(|row| (row.weapon_item_id.as_str(), row.in_hand, row.unarmed)).collect::<Vec<_>>(),
            [("Longsword (Base)", true, false), ("Dagger (Base)", false, false), (UNARMED_STRIKE_ITEM_ID, true, true)]
        );
        let fist = unarmed(&aldric);
        assert!(fist.nonlethal);
        assert!(!all[0].nonlethal && !all[1].nonlethal);
        assert_eq!(dice(&fist), Some((1, 3)));
        assert_eq!(fist.weapon_enhancement.as_ref().map(|part| (part.attack_bonus, part.damage_bonus)), Some((0, 0)));

        let combat = unarmed_strike_combat_rows(&compute_pilot_base_chassis(&aldric), &aldric);
        let attack = combat.iter().find(|row| row.id == "combat.weapon_attack_bonus.unarmedstrike").expect("an attack row");
        assert_eq!(attack.value, 7, "{}", attack.detail);
        let key = fist.base_dice.as_ref().unwrap().weapon_record_key.clone();
        let feat_row = combat.iter().find(|row| row.id == weapon_feat_damage_row_id(&key)).map(|row| row.value);
        let total = weapon_damage_total_row(
            &key,
            fist.base_dice.as_ref().map(|dice| dice.base_dice),
            fist.str_modifier.as_ref().map(|part| (part.str_damage_modifier, format!("{:?}", part.wield_category))),
            fist.weapon_enhancement.as_ref().map(|part| part.damage_bonus),
            feat_row,
            &[],
        )
        .expect("the Unarmed row has dice");
        assert_eq!(total.id, "combat.weapon_damage_total.unarmedstrike");
        assert!(total.detail.starts_with("1d3+4:"), "{}", total.detail);

        let corpus = load_equipment_corpus(&[BookCorpusRoot { book_id: "core_rulebook", dir: Path::new("data/corpus/core_rulebook") }]);
        let effects = compute_equipment_effects(&aldric.chosen.equipment_selections[..1], &corpus);
        assert!(resolve_weapon_damage_breakdown(&aldric, &corpus, &effects, 4).iter().all(|row| !row.unarmed));
    }

    /// A Monk reads the monk's die by level (Medium: 1d8 at 4th, 2d6 at 12th) and is lethal
    /// (the Monk's Improved Unarmed Strike grant).
    #[test]
    fn unarmed_row_monk_progression_and_lethal() {
        let monk4 = unarmed(&input("human", "monk:4", &[], Vec::new()));
        assert_eq!(dice(&monk4), Some((1, 8)));
        assert!(!monk4.nonlethal);
        assert_eq!(dice(&unarmed(&input("human", "monk:12", &[], Vec::new()))), Some((2, 6)));
    }

    /// A Small character reads the record's die one size step down, `1d2`; a Small monk the
    /// monk's Small column (1d6 at 4th).
    #[test]
    fn unarmed_row_small_character_is_1d2() {
        let small = unarmed(&input("halfling", "fighter:1", &[], Vec::new()));
        assert_eq!(dice(&small), Some((1, 2)));
        assert!(small.nonlethal);
        assert_eq!(dice(&unarmed(&input("gnome", "monk:4", &[], Vec::new()))), Some((1, 6)));
    }

    /// Improved Unarmed Strike, chosen, makes the strike lethal and leaves the die; the Unchained
    /// Monk's grant does too.
    #[test]
    fn unarmed_row_improved_unarmed_strike_is_lethal() {
        let chosen = unarmed(&input("human", "fighter:3", &["Improved Unarmed Strike"], Vec::new()));
        assert!(!chosen.nonlethal);
        assert_eq!(dice(&chosen), Some((1, 3)));
        let unchained = unarmed(&input("human", "unchained_monk:1", &[], Vec::new()));
        assert!(!unchained.nonlethal);
        assert_eq!(dice(&unchained), Some((1, 6)));
    }
}

/// SD-41 E1.6 (`decisions.md §23, §30`): a weapon uses ammunition when its settled record (or the
/// record standing in for it) is a `Projectile`; `ammunition_carried` counts the pieces of carried
/// ammunition of the weapon's `CONTAINS` family, a copy without a quantity counting one bundle.
#[cfg(test)]
mod ammunition_tests {
    use super::*;
    use crate::rules_core::character_input::{load_character_input_fixture, EquipmentSelection};
    use crate::rules_core::corpus_loader::{load_equipment_corpus, BookCorpusRoot};
    use crate::rules_core::equipment_effects::compute_equipment_effects;
    use crate::saved_character::equipment_sets::CopyKey;
    use std::path::Path;

    const FLIGHT_ARROWS: &str = "Arrows (Flight/20)";

    fn corpus() -> SourcePackageContent<'static> {
        let books = ["core_rulebook", "advanced_players_guide", "ultimate_equipment"];
        let dirs: Vec<String> = books.iter().map(|book| format!("data/corpus/{book}")).collect();
        let roots: Vec<BookCorpusRoot> =
            books.iter().zip(&dirs).map(|(book, dir)| BookCorpusRoot { book_id: book, dir: Path::new(dir) }).collect();
        load_equipment_corpus(&roots)
    }

    fn select(item: &str, state: ActiveState) -> EquipmentSelection {
        EquipmentSelection {
            item_id: item.to_owned(),
            equipped_or_active: state == ActiveState::EquippedActive,
            active_state: state,
            applied_modifiers: Vec::new(),
        }
    }

    /// Aldric's chassis (Human Fighter 3, Strength 19) holding `selections`, through the carried view.
    fn rows(selections: Vec<EquipmentSelection>, quantities: &[(CopyKey, u32)]) -> Vec<WeaponDamageBreakdown> {
        let corpus = corpus();
        let mut input = load_character_input_fixture(
            "case_id=case:test\nsource_package_id=core_rulebook\nrace_id=race:human\nclass_level=class:fighter:3\n\
             ability=strength:19\nability=dexterity:13\nability=constitution:14\nability=intelligence:14\n\
             ability=wisdom:12\nability=charisma:8\n",
        )
        .character_input
        .expect("the fixture loads");
        input.chosen.equipment_selections = selections;
        let held: Vec<EquipmentSelection> =
            input.chosen.equipment_selections.iter().filter(|s| s.active_state == ActiveState::EquippedActive).cloned().collect();
        let effects = compute_equipment_effects(&held, &corpus);
        resolve_weapon_damage_breakdown_with_carried(&input, &corpus, &effects, 4, quantities)
    }

    fn row<'a>(rows: &'a [WeaponDamageBreakdown], item: &str) -> &'a WeaponDamageBreakdown {
        rows.iter().find(|row| row.weapon_item_id == item).unwrap_or_else(|| panic!("no row for {item}: {rows:#?}"))
    }

    /// Longbow, light crossbow, pistol and sling carry the `Projectile` type (the bow and crossbow
    /// through the base record they copy); a longsword does not.
    #[test]
    fn uses_ammunition_by_projectile_tag() {
        for (item, uses) in [("Longbow", true), ("Crossbow (Light)", true), ("Pistol", true), ("Sling", true), ("Longsword", false)] {
            let rows = rows(vec![select(item, ActiveState::EquippedActive)], &[]);
            assert_eq!(row(&rows, item).uses_ammunition, uses, "{item}");
        }
    }

    /// A thrown dagger or javelin and the Unarmed row use none, and count none even with arrows carried.
    #[test]
    fn uses_ammunition_thrown_and_unarmed_do_not() {
        let rows = rows(
            vec![
                select("Dagger", ActiveState::EquippedActive),
                select("Javelin", ActiveState::SelectedInactive),
                select(FLIGHT_ARROWS, ActiveState::SelectedInactive),
            ],
            &[],
        );
        for item in ["Dagger", "Javelin", UNARMED_STRIKE_ITEM_ID] {
            let row = row(&rows, item);
            assert_eq!((row.uses_ammunition, row.ammunition_carried), (false, 0), "{item}");
        }
    }

    /// Aldric with a longbow and 3 × "Arrow, Flight (20)" carried: 60 arrows. A copy given a piece
    /// quantity counts that; bolts (another family) and Not Carried arrows count nothing.
    #[test]
    fn uses_ammunition_carried_count_sums_bundles() {
        let loadout = || {
            vec![
                select("Longbow", ActiveState::EquippedActive),
                select(FLIGHT_ARROWS, ActiveState::SelectedInactive),
                select(FLIGHT_ARROWS, ActiveState::SelectedInactive),
                select(FLIGHT_ARROWS, ActiveState::EquippedActive),
                select(FLIGHT_ARROWS, ActiveState::Absent),
                select("Bolts (Crossbow/10)", ActiveState::SelectedInactive),
            ]
        };
        assert_eq!(row(&rows(loadout(), &[]), "Longbow").ammunition_carried, 60);
        let five = CopyKey { item_id: FLIGHT_ARROWS.to_owned(), bonus: 0, ordinal: 1 };
        assert_eq!(row(&rows(loadout(), &[(five, 5)]), "Longbow").ammunition_carried, 45, "20 + 5 + 20");
        let crossbow = rows(vec![select("Crossbow (Light)", ActiveState::EquippedActive), select(FLIGHT_ARROWS, ActiveState::SelectedInactive)], &[]);
        assert_eq!(row(&crossbow, "Crossbow (Light)").ammunition_carried, 0, "arrows are not bolts");
    }
}
