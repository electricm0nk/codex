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

use crate::rules_core::character_input::{ActiveState, CharacterInput};
use crate::rules_core::equipment_effects::{is_natural_attack_weapon, EquipmentEffects};
use crate::rules_core::equipment_record::CorpusEquipmentRecord;
use crate::rules_core::equipment_resolver::{
    equipment_converted_resolve, equipment_id_resolve,
};
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::feats::{feat_tables, EffectSelection, FeatEffectBonus};
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::source_content::SourcePackageContent;

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
