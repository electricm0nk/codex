//! Epic 5 — equipment-effect engine (SD-20 §1.5).
//!
//! Extends SD-19's bounded equipment baseline
//! (`pilot_compute_corpus::DerivedEquipmentStats`, deliberately left
//! `Default`-valued per that module's own doc comment: "wrapping resolved
//! equipment with its (currently unpopulated — bounded-baseline non-goal)
//! derived stats") with real per-item armor/shield stats, one CRB
//! equipment category per cycle
//! (`scope-draft.md` §1.5 work-unit order: `arms_armor`, then `general`,
//! `magic_items`, `equipmods`). `arms_armor` landed first — see
//! `equipment_effects/arms_armor.rs`. `general` landed second — see
//! `equipment_effects/general.rs`; unlike `arms_armor`'s AC/max-dex/
//! spell-failure stats, the `general` category's real load-bearing field
//! is a per-item skill-check circumstance bonus
//! (`ResolvedEquipmentEffect::skill_bonus`), so `general` does not
//! populate `EquipmentStatEffect` at all (that type stays scoped to the
//! armor/shield fields `arms_armor` defined it for). `magic_items` landed
//! third — see `equipment_effects/magic_items.rs`; like `general`, its
//! real load-bearing field (a per-item ability-score enhancement bonus,
//! `ResolvedEquipmentEffect::ability_bonus`) does not fit
//! `EquipmentStatEffect` either, so it follows the same
//! shared-struct-extension pattern `general` established (a new
//! `ResolvedEquipmentEffect` field, not a new `EquipmentStatEffect`
//! field). This cycle lands `equipmods` — see
//! `equipment_effects/equipmods.rs`, **closing Epic 5** (all four CRB
//! equipment categories done); its real load-bearing field is a per-item
//! weapon to-hit/damage enhancement bonus
//! (`ResolvedEquipmentEffect::weapon_enhancement_bonus`), following the
//! same shared-struct-extension pattern.
//!
//! Adapts `technical-design.md` §2.4's illustrative
//! `compute_equipment_effects(equipped: &[EquipmentSelection], rules_tables:
//! &RulesTables) -> EquipmentEffects` seam to this repo's real types —
//! `RulesTables` does not exist anywhere in this codebase (same situation
//! `pilot_compute_corpus::compute_pilot_with_corpus`'s own doc comment
//! notes for the doctrine doc's illustrative `PilotReceipt`: "does not
//! exist in this repo"). Corpus resolution here goes through the exact
//! same `SourcePackageContent` + `equipment_resolver::equipment_id_resolve`
//! path every other SD-19/20 corpus-derived field uses, and category
//! membership comes from a `TableCellRef`-style lookup by the resolved
//! record's `KEY:` token against the canonical
//! `rules_tables::crb::equipment_tables` store — not re-derived from raw
//! corpus text.
//!
//! ## SD-35 `AT-35-E6-003-RULED` cycles 11 and 12 — where these values now come from
//!
//! Every "reads its own tokens directly off `record`" sentence in this module
//! and its `<category>.rs` children is the history of how each effect was
//! built, and each rule it states still holds. What changed is the side of the
//! ingest boundary the read happens on (`decisions.md` §11, §19): the token
//! spellings and the `BONUS:` chain grammar moved to
//! [`crate::pcgen_import::ir_converter::equipment_record_to_corpus`], and this
//! module reads settled values off
//! [`crate::rules_core::equipment_record::CorpusEquipmentRecord`]. Cycle 12
//! finished the job here: `is_natural_attack_weapon`, `is_weapon_record`,
//! `resolve_weapon_to_hit_bonus` and `resolve_eqm_weightdiv_effect` take a
//! converted record, and the parser-row `eqmod_referenced_records` is deleted
//! -- [`eqmod_referenced_converted_records`] is the only EQMOD resolution the
//! live side has.

pub mod arms_armor;
pub mod equipmods;
pub mod general;
pub mod intelligent_item;
pub mod magic_items;


use crate::rules_core::character_input::EquipmentSelection;
use crate::rules_core::equipment_effects::equipmods::WeaponEnhancementBonus;
use crate::rules_core::equipment_effects::general::{SkillCheckBonus, VarBonus};
use crate::rules_core::equipment_effects::intelligent_item::IntelligentItemContribution;
use crate::rules_core::equipment_effects::magic_items::AbilityScoreBonus;
use crate::rules_core::equipment_record::CorpusEquipmentRecord;
use crate::rules_core::equipment_resolver::{
    equipment_converted_resolve, equipment_id_resolve,
};
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::equipment_tables::{equipment_tables, EquipmentCategory};
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::source_content::SourcePackageContent;

/// The settled records of the modifier items attached to this one, resolved
/// against `corpus` in the order the item names them.
///
/// SD-35 `AT-35-E6-003-RULED` cycle 11 built this as the sibling of the
/// parser-row `eqmod_referenced_records`; **cycle 12 deleted that sibling**,
/// its last two callers (`damage_total::resolve_eqmweapon_damagesize_effect`
/// and `resolve_eqm_weightdiv_effect` below) having moved here, so this is now
/// the only EQMOD resolution the live side has. The attachment grammar -- an
/// item may name several attachments, and each names its parts in one string,
/// including candidate segments that name no record at all -- stayed on the
/// converter with the values it was read alongside, so this function reads a
/// list of item identities
/// ([`CorpusEquipmentRecord::eqmod_references`]) and resolves each. A
/// candidate that names no corpus record is skipped, exactly as before: the
/// list is what the item states, not a promise that every entry is an item.
pub fn eqmod_referenced_converted_records<'a>(
    record: &CorpusEquipmentRecord,
    corpus: &SourcePackageContent<'a>,
) -> Vec<&'a CorpusEquipmentRecord> {
    record
        .eqmod_references
        .iter()
        .filter_map(|candidate| equipment_converted_resolve(candidate, corpus))
        .collect()
}

/// Per-category stat contribution shared across every
/// `equipment_effects/<category>.rs` file. `None` means the category's
/// resolver has not populated that field (either because the underlying
/// corpus record carries no such token, or — for a category whose cycle
/// has not landed yet — because no resolver exists yet at all).
#[derive(Debug, Clone, Copy, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EquipmentStatEffect {
    pub armor_class_bonus: Option<i16>,
    pub max_dex: Option<i16>,
    pub spell_failure: Option<f32>,
    /// The record's `ACCHECK:` token (v0.6 alpha swarm item 1, shape (c)):
    /// a negative or zero value, PF1's usual convention for a penalty. Read
    /// the same way `max_dex`/`spell_failure` already read their own
    /// tokens; `arms_armor.rs`'s own module doc comment already cited this
    /// exact token as present on the records it resolves, it was simply
    /// never extracted into this struct until now.
    pub armor_check_penalty: Option<i16>,
}

/// One resolved equipment selection's computed effect.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedEquipmentEffect {
    pub item_id: String,
    pub equipment_record_key: String,
    pub category: EquipmentCategory,
    pub armor_class_bonus: Option<i16>,
    pub max_dex: Option<i16>,
    pub spell_failure: Option<f32>,
    /// Mirrors `EquipmentStatEffect::armor_check_penalty` exactly (v0.6
    /// alpha swarm item 1, shape (c)).
    pub armor_check_penalty: Option<i16>,
    /// The `general` category's per-item skill-check circumstance bonus
    /// (see `equipment_effects/general.rs`). `None` for every other
    /// category, and for a `general` record that carries no
    /// `BONUS:SKILL|...` token.
    pub skill_bonus: Option<SkillCheckBonus>,
    /// The `magic_items` category's per-item ability-score enhancement
    /// bonus (see `equipment_effects/magic_items.rs`). `None` for every
    /// other category, and for a `magic_items` record that carries no
    /// `BONUS:STAT|...` token.
    pub ability_bonus: Option<AbilityScoreBonus>,
    /// The `equipmods` category's per-item weapon to-hit/damage
    /// enhancement bonus (see `equipment_effects/equipmods.rs`). `None`
    /// for every other category, and for an `equipmods` record that
    /// carries no matching `BONUS:WEAPON|...|TYPE=Enhancement` token.
    pub weapon_enhancement_bonus: Option<WeaponEnhancementBonus>,
    /// The armor-slot "Spell Resistance" special ability family's flat
    /// `SR:<n>` contribution (see `equipmods::resolve_spell_resistance_
    /// bonus`'s own doc comment). `None` for every other category, and
    /// for an `equipmods` record that carries no literal-integer `SR:`
    /// token.
    pub spell_resistance_bonus: Option<i16>,
    /// `AT-34-E3-003` (bucket `M`, equipment sub-causes, cycle 4): every
    /// `BONUS:VAR|<name(s)>|<value>` chain on the record's own line, plus
    /// any referenced `EQMOD:` modifier's own `VAR` chains summed in by
    /// name (see `general::compute_var_effect` / `general::
    /// apply_eqmod_var_bonus`). Unlike `skill_bonus`/`ability_bonus`
    /// (each scoped to exactly one category's one named field), `VAR`
    /// chains appear across every category — a magic item's ki-pool
    /// counter, an armor material's ACP adjustment, a weapon's combat
    /// maneuver bonus — so this is a `Vec`, not a single `Option`, in the
    /// SAME zero-vs-honest-absence convention every other field here
    /// already uses: empty means the record's own closure carries no
    /// `VAR` chain at all, never a fabricated zero.
    pub var_bonus: Vec<VarBonus>,
    /// v0.6 alpha swarm items 1+27 sub-task 2: the real, per-weapon
    /// TOHIT-affecting bonus resolved from *this specific* selection's own
    /// `applied_modifiers` (see `character_input::EquipmentSelection`'s doc
    /// comment) -- `Some(n)` for every real weapon selection (a real `0`
    /// when it has no applied modifiers, or none affect TOHIT), `None` for
    /// every non-weapon selection (not applicable, not "no bonus"). Unlike
    /// `EquipmentEffects.attack_bonus_delta` (still gated to the
    /// single-weapon case for wire-contract stability -- see that field's
    /// own doc comment), this field is genuinely unambiguous for *any*
    /// number of equipped weapons, since each weapon's modifiers are now
    /// explicitly attached rather than inferred from "the only weapon
    /// equipped."
    pub to_hit_bonus: Option<i16>,
    /// SD-31 intelligent-item resolver (operator ruling 2026-08-19): this
    /// selection's own Intelligence/Wisdom/Charisma/Ego/alignment
    /// contribution, summed across this specific selection's
    /// `applied_modifiers` (see `character_input::EquipmentSelection`'s
    /// doc comment) -- the SAME per-selection attachment model
    /// `to_hit_bonus` uses, not the loadout-wide sum
    /// `weapon_enhancement_bonus`/`ability_bonus` use (see
    /// `equipment_effects::intelligent_item`'s own module doc comment for
    /// why: an intelligent item's own stat block belongs to the specific
    /// item it is attached to, not to every other equipped item). `None`
    /// when none of this selection's applied modifiers carry any
    /// `Intelligent Item ~ ...` token (the common case -- an ordinary,
    /// non-intelligent item).
    pub intelligent_item: Option<IntelligentItemContribution>,
    pub table_cell: Option<TableCellRef>,
}

/// Aggregate equipment-effect result for one character's full equipped
/// loadout. `armor_class_delta` sums every resolved item's
/// `armor_class_bonus` (PF1 lets armor and shield bonuses stack, unlike
/// most other bonus types). `max_dex_cap` is the tightest (lowest)
/// `max_dex` among items that carry one — an unarmored or shieldless
/// loadout leaves it `None` (uncapped), exactly like the real rule.
/// `spell_failure_chance` sums every resolved item's `spell_failure` —
/// armor and shield arcane spell-failure chances stack additively per
/// PF1's rule. `armor_check_penalty_total` sums every resolved item's
/// `armor_check_penalty` the same way (v0.6 alpha swarm item 1, shape (c))
/// — PF1's rule: armor and shield check penalties add together when both
/// are worn, the same additive-stacking shape `armor_class_delta` already
/// uses. Any item whose category has not resolved a value (`None`)
/// contributes nothing to any aggregate rather than a fabricated zero.
///
/// `attack_bonus_delta` (v0.6 alpha swarm item 1, the bounded single-weapon
/// attack-bonus slice, greenlit as an engineering-scope call distinct from
/// item 27's claim-gating philosophy question) is `Some` only when
/// `equipped`'s `EquippedActive` selections resolve to *exactly one* real
/// weapon (a record carrying a `DAMAGE:` token, the same signal
/// `damage_total.rs`'s own `resolve_base_damage_dice` uses to identify a
/// weapon) -- in which case it mirrors that one weapon's own
/// `ResolvedEquipmentEffect.to_hit_bonus`. `None` for zero or two-or-more
/// weapons: not because the attachment is ambiguous any more (sub-task 2
/// closed that -- see `to_hit_bonus`'s own doc comment, resolved per-weapon
/// via `EquipmentSelection.applied_modifiers`), but because this single
/// scalar has no way to represent more than one weapon's bonus at once
/// (summing two different weapons' to-hit bonuses into one number would be
/// combat-meaningless, not merely imprecise). Kept single-weapon-scoped
/// deliberately, for wire-contract stability with the existing frontend
/// `attackBonusDelta` stat tile; `per_item`'s own `to_hit_bonus` is the
/// real, unambiguous per-weapon value for any weapon count.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EquipmentEffects {
    pub per_item: Vec<ResolvedEquipmentEffect>,
    pub armor_class_delta: i16,
    pub max_dex_cap: Option<i16>,
    pub spell_failure_chance: Option<f32>,
    pub armor_check_penalty_total: i16,
    pub attack_bonus_delta: Option<i16>,
    /// The HIGHEST `per_item[].spell_resistance_bonus` among everything
    /// equipped -- PF1's real rule ("if a creature has multiple sources
    /// of spell resistance, only the highest value applies"), unlike
    /// `armor_class_delta`'s additive stacking. `None` when nothing
    /// equipped grants Spell Resistance.
    pub spell_resistance_total: Option<i16>,
}

/// The equipment-effect engine seam (`technical-design.md` §2.4, adapted
/// to real types — see module doc comment). Resolves every selection
/// against the corpus, looks up its category in the canonical CRB
/// equipment-table store, dispatches to that category's per-category
/// function, and aggregates the results.
pub fn compute_equipment_effects(
    equipped: &[EquipmentSelection],
    corpus: &SourcePackageContent,
) -> EquipmentEffects {
    let mut per_item = Vec::new();
    let mut armor_class_delta: i16 = 0;
    let mut max_dex_cap: Option<i16> = None;
    let mut spell_failure_chance: Option<f32> = None;
    let mut armor_check_penalty_total: i16 = 0;
    let mut weapon_count: u32 = 0;
    let mut single_weapon_to_hit_bonus: i16 = 0;
    let mut spell_resistance_total: Option<i16> = None;

    for selection in equipped {
        // SD-35 `AT-35-E6-003-RULED` cycle 11: one resolution answers with the
        // settled record every effect below reads and the parser row the two
        // consumers that have not moved yet still need, for the same item.
        let Some((converted, table_cell)) =
            equipment_id_resolve(&selection.item_id, RuleSetId::Crb, corpus)
        else {
            continue;
        };
        let to_hit_bonus = is_weapon_record(converted).then(|| {
            let bonus = resolve_weapon_to_hit_bonus(converted, &selection.applied_modifiers, corpus);
            weapon_count += 1;
            single_weapon_to_hit_bonus = bonus;
            bonus
        });
        let intelligent_item_contribution =
            resolve_intelligent_item_contribution(&selection.applied_modifiers, corpus);
        let key = converted.identity.clone();
        // Every per-category resolver (`arms_armor`/`general`/`magic_items`/
        // `equipmods`) already reads its own tokens directly off `record`
        // (book-agnostic -- see each module's own doc comment) and returns
        // `None`/default when the relevant token chain is absent. Calling
        // all four directly, instead of first gating on a category lookup
        // in the CRB-only compiled `equipment_tables()` store, means a
        // non-CRB item's real effects are no longer silently dropped before
        // any resolver even runs. `category` becomes a *descriptive* label
        // derived from which resolver(s) actually matched (confirmed unused
        // for branching anywhere downstream -- `apps/desktop`'s own wire
        // type treats it as a plain string) rather than the gate itself.
        let effect = resolve_category_effect(converted, corpus);
        let skill_bonus = general::compute_general_effect(converted);
        // SD-35 `AT-35-E6-003-RULED` cycle 10: the ability-score enhancement is
        // a settled value on the converted record, resolved by the same
        // identity rule that resolved the parser row beside it.
        let ability_bonus = magic_items::compute_magic_items_effect(converted);
        let mut weapon_enhancement_bonus = equipmods::compute_equipmods_effect(converted);
        // SD-33 remediation wave 6 (`AT-33-E5-003`'s escalated
        // `rending_claw_blades` blocker): fold the record's own `EQMOD:`-
        // referenced modifier records' weapon enhancement into the total,
        // mirroring `resolve_category_effect`'s already-shipped AC-
        // dimension pattern (wave 4) -- see
        // `equipmods::apply_eqmod_weapon_enhancement_bonus`'s own doc
        // comment.
        let weapon_eqmod_records = eqmod_referenced_converted_records(converted, corpus);
        equipmods::apply_eqmod_weapon_enhancement_bonus(&mut weapon_enhancement_bonus, &weapon_eqmod_records);
        let spell_resistance_bonus = equipmods::resolve_spell_resistance_bonus(converted);
        // `AT-34-E3-003` (bucket `M`, equipment sub-causes, cycle 4):
        // `general::compute_var_effect` already reads every `BONUS:VAR|...`
        // chain on a record -- it existed since the SD-20 `general`
        // category cycle but was never called from this, the real
        // dispatcher every category resolver above is called from (only
        // from the SD-33 oracle-comparison `src/bin/e5_*.rs` tools). Wiring
        // it here reaches every book's corpus by construction, same as
        // every resolver above. `weapon_eqmod_records` (despite its name)
        // is `eqmod_referenced_converted_records`'s generic, category-
        // agnostic result -- already reused by this exact record for the weapon
        // dimension above -- so no second EQMOD resolution pass is needed;
        // `apply_eqmod_var_bonus`'s own doc comment names the real
        // `panoply_of_the_fierani_knight` case this reuse now closes (an
        // armor item, not a weapon).
        let mut var_bonus = general::compute_var_effect(converted);
        general::apply_eqmod_var_bonus(&mut var_bonus, &weapon_eqmod_records);
        let has_arms_armor_effect = effect.armor_class_bonus.is_some()
            || effect.max_dex.is_some()
            || effect.spell_failure.is_some()
            || effect.armor_check_penalty.is_some();
        let category = if has_arms_armor_effect {
            EquipmentCategory::ArmsArmor
        } else if skill_bonus.is_some() {
            EquipmentCategory::General
        } else if ability_bonus.is_some() {
            EquipmentCategory::MagicItems
        } else if weapon_enhancement_bonus.is_some() || spell_resistance_bonus.is_some() {
            EquipmentCategory::Equipmods
        } else {
            // No category-defining token matched at all (e.g. a plain
            // weapon with no enhancement bonuses of its own) -- every
            // effect field is already correctly `None` above regardless of
            // this label; fall back to the CRB-only table for a
            // best-effort descriptive value, defaulting to `ArmsArmor`
            // (weapons/armor being the common no-extra-token case) if even
            // that lookup misses.
            equipment_tables()
                .iter()
                .find(|entry| entry.key == key)
                .map(|entry| entry.category)
                .unwrap_or(EquipmentCategory::ArmsArmor)
        };

        if let Some(bonus) = effect.armor_class_bonus {
            armor_class_delta += bonus;
        }
        if let Some(dex) = effect.max_dex {
            max_dex_cap = Some(max_dex_cap.map_or(dex, |current| current.min(dex)));
        }
        if let Some(failure) = effect.spell_failure {
            spell_failure_chance =
                Some(spell_failure_chance.map_or(failure, |current| current + failure));
        }
        if let Some(penalty) = effect.armor_check_penalty {
            armor_check_penalty_total += penalty;
        }
        if let Some(sr) = spell_resistance_bonus {
            // PF1's real rule: multiple SR sources do not stack, only the
            // highest applies (see `EquipmentEffects::spell_resistance_
            // total`'s own doc comment) -- `max`, never `+=`.
            spell_resistance_total = Some(spell_resistance_total.map_or(sr, |current| current.max(sr)));
        }

        per_item.push(ResolvedEquipmentEffect {
            item_id: selection.item_id.clone(),
            equipment_record_key: key,
            category,
            armor_class_bonus: effect.armor_class_bonus,
            max_dex: effect.max_dex,
            spell_failure: effect.spell_failure,
            armor_check_penalty: effect.armor_check_penalty,
            skill_bonus,
            ability_bonus,
            weapon_enhancement_bonus,
            spell_resistance_bonus,
            var_bonus,
            to_hit_bonus,
            intelligent_item: intelligent_item_contribution,
            table_cell,
        });
    }

    // v0.6 alpha swarm items 1+27 sub-task 2: `attack_bonus_delta` stays
    // single-weapon-scoped for wire-contract stability (see its own doc
    // comment) even though per-weapon resolution is no longer ambiguous
    // for any weapon count -- `per_item[].to_hit_bonus` carries the real
    // value for the 2+-weapon case.
    let attack_bonus_delta = (weapon_count == 1).then_some(single_weapon_to_hit_bonus);

    EquipmentEffects {
        per_item,
        armor_class_delta,
        max_dex_cap,
        spell_failure_chance,
        armor_check_penalty_total,
        attack_bonus_delta,
        spell_resistance_total,
    }
}

/// Resolves `applied_modifiers` (item_ids attached to one specific weapon
/// selection -- see `character_input::EquipmentSelection`'s doc comment)
/// against `corpus` and sums every TOHIT-affecting `equipmods` bonus. A
/// modifier item_id that doesn't resolve, or resolves to a non-`Equipmods`
/// category, or an `Equipmods` record with no matching
/// `BONUS:WEAPON|...|TYPE=Enhancement` chain, contributes nothing --
/// mirrors the main loop's own resolve-or-skip discipline, not a
/// fabricated value.
///
/// **`SD31-W18-INTEGRATE-001` fix (adversarial review, `OPEN-ISSUES.md`
/// row 309 re-opened):** `weapon_record` is the specific weapon this
/// to-hit bonus is being resolved for. Wave 18's row-309 re-land guarded
/// `damage_total::resolve_weapon_enhancement_modifier` against a
/// `natural_attack_only` bonus (the Amulet of Mighty Fists family)
/// leaking onto an ordinary weapon's DAMAGE roll, but left THIS function
/// -- the one `compute_equipment_effects` actually calls for every
/// weapon's `to_hit_bonus`/`attack_bonus_delta`, via
/// `selection.applied_modifiers`, the attachment shape the shipped
/// desktop app's own `attach_equipment_modifier_at_root` uses -- with no
/// scope check at all. Confirmed live before this fix: an Amulet of
/// Mighty Fists +5 attached to an ordinary Longsword yielded
/// `to_hit_bonus = Some(5)` / `attack_bonus_delta = Some(5)`, a real
/// player-facing regression reaching `character_hub.rs` ->
/// `CharacterSheet.tsx`'s "Attack Bonus" stat tile. Same guard as
/// `resolve_weapon_enhancement_modifier`: skip a `natural_attack_only`
/// bonus unless `weapon_record` itself is a real natural attack.
fn resolve_weapon_to_hit_bonus(
    weapon_record: &CorpusEquipmentRecord,
    applied_modifiers: &[String],
    corpus: &SourcePackageContent,
) -> i16 {
    let weapon_is_natural_attack = is_natural_attack_weapon(weapon_record);
    let mut total = 0;
    for modifier_item_id in applied_modifiers {
        let Some(record) = equipment_converted_resolve(modifier_item_id, corpus) else {
            continue;
        };
        // `compute_equipmods_effect` already reads the record's own
        // `BONUS:WEAPON|...|TYPE=Enhancement` chain directly (book-agnostic)
        // and returns `None` if it's absent -- that check alone is the real,
        // precise "is this an equipmods enhancement" test. The CRB-only
        // `equipment_tables()` category gate this used to require first was
        // strictly redundant with it, and silently dropped any non-CRB
        // modifier's bonus before this check ever ran.
        if let Some(bonus) = equipmods::compute_equipmods_effect(record) {
            if bonus.natural_attack_only && !weapon_is_natural_attack {
                continue;
            }
            if let Some(tohit) = bonus.tohit_bonus {
                total += tohit;
            }
        }
    }
    total
}

/// Resolves `applied_modifiers` against `corpus` and sums every attached
/// modifier's `intelligent_item::compute_intelligent_item_effect`
/// contribution (SD-31 intelligent-item resolver, operator ruling
/// 2026-08-19). Mirrors `resolve_weapon_to_hit_bonus`'s own resolve-or-skip
/// discipline: a modifier item_id that doesn't resolve, or resolves to a
/// record carrying none of this family's tokens, contributes nothing.
/// Numeric fields (ability-score/Ego deltas) sum across every attached
/// modifier that carries them -- a real build attaches exactly one
/// `Intelligent Item ~ Ability Score / <Ability> <N>` per mental ability
/// (PCGen's own `REPLACES:` chain on these records prevents selecting two
/// values for the same ability at once, see module doc comment), so
/// summing is safe and matches how `Intelligent Item ~ Base`'s own
/// literal 10-point baseline composes with a selected Ability Score
/// modifier's delta. `alignment` takes the last attached modifier that
/// carries one (a real build attaches at most one, per the same
/// `REPLACES:` discipline on the alignment family). Returns `None` when no
/// attached modifier contributes anything at all.
fn resolve_intelligent_item_contribution(
    applied_modifiers: &[String],
    corpus: &SourcePackageContent,
) -> Option<IntelligentItemContribution> {
    let mut total = IntelligentItemContribution::default();
    let mut found = false;
    for modifier_item_id in applied_modifiers {
        let Some(record) = equipment_converted_resolve(modifier_item_id, corpus) else {
            continue;
        };
        if let Some(contribution) = intelligent_item::compute_intelligent_item_effect(record) {
            found = true;
            total.intelligence_bonus += contribution.intelligence_bonus;
            total.wisdom_bonus += contribution.wisdom_bonus;
            total.charisma_bonus += contribution.charisma_bonus;
            total.ego_bonus += contribution.ego_bonus;
            if contribution.alignment.is_some() {
                total.alignment = contribution.alignment;
            }
        }
    }
    found.then_some(total)
}

/// Whether `record` carries PF1's natural-attack `TYPE:` marker -- an
/// exact `Natural` dot-segment (e.g. CRB's `Unarmed Strike`/`Flurry of
/// Blows`, `TYPE:...Weapon Group Natural.Natural.Light` --
/// `core_rulebook/cr_equip_arms_armor.lst` lines 292/296 -- confirmed the
/// `Weapon Group Natural` segment is a DIFFERENT, non-exact-matching
/// string, so this reads the real `Natural` segment specifically, not a
/// substring match that would also fire on `Weapon Group Natural`).
///
/// **`SD31-W17-INTEGRATE-001` fix (OPEN-ISSUES row 309):** this is the
/// scope-aware signal `resolve_weapon_enhancement_modifier`
/// (`damage_total.rs`) now checks before applying a
/// `WeaponEnhancementBonus` whose `natural_attack_only` field is `true`
/// (the Amulet of Mighty Fists family's own `WEAPONPROF=TYPE.Natural`
/// chain, see `equipmods.rs`) -- without it, wave 17 found an equipped
/// Amulet of Mighty Fists wrongly bonused every weapon a character
/// wielded, not just its natural attacks.
pub fn is_natural_attack_weapon(record: &CorpusEquipmentRecord) -> bool {
    record.is_natural_attack
}

/// Whether `record` is an actively-wielded weapon for the single-weapon
/// to-hit/attack-bonus ambiguity check above -- carries a `DAMAGE:` corpus
/// token AND is not a shield.
///
/// **Real correction (2026-07-30, found via wiring the real corpus into
/// the desktop app for the first time):** this doc comment used to claim
/// "armor, shields, and every other non-weapon item carry no `DAMAGE:`
/// token at all" -- true only of the tiny 4-record desktop fixture bundle
/// this codebase tested against until now. A real corpus shield (e.g. CRB's
/// `Heavy Wooden Shield (Base)`) carries a genuine `DAMAGE:1d4` token for
/// its shield-bash attack, per PF1's own published rules (`SPROP`: "You can
/// bash an opponent with a heavy shield"). Counting a worn-but-not-bashing
/// shield as a second "weapon" made every standard armor+shield+one-weapon
/// loadout falsely ambiguous the moment real shield data reached this path.
/// A shield bash is a distinct, optional combat choice most builds never
/// make -- not the default state of wearing a shield -- so a record whose
/// `TYPE:` token's first segment is `Shield` is excluded here, matching
/// `EquipmentCategory`'s own `arms_armor` grouping (armor and shields
/// share a category; only shields carry this specific ambiguity).
fn is_weapon_record(record: &CorpusEquipmentRecord) -> bool {
    record.states_base_damage && !record.is_shield
}

/// `arms_armor::compute_arms_armor_effect` reads `MAXDEX:`/`SPELLFAILURE:`/
/// `ACCHECK:`/the `BONUS:COMBAT|AC` chain straight off `record` -- book
/// -agnostic already, and correctly returns every field `None` for a record
/// that carries none of those tokens (a weapon, a `general`/`magic_items`/
/// `equipmods` record), so it's safe to call unconditionally rather than
/// gating on a category lookup first (see `compute_equipment_effects`'s own
/// comment on this).
///
/// SD-33 remediation wave 4 (`AT-33-E5-003`): also resolves `record`'s own
/// `EQMOD:`-referenced modifier records (across the whole `corpus`, see
/// `eqmod_referenced_converted_records`'s own doc comment) and sums their real
/// `COMBAT|AC` contribution in -- a magic armor/shield item's enhancement
/// bonus lives on that separate record, never on the base item's own
/// chain (see `arms_armor::apply_eqmod_armor_class_bonus`'s own doc
/// comment for the confirmed real-corpus evidence).
fn resolve_category_effect(
    record: &CorpusEquipmentRecord,
    corpus: &SourcePackageContent,
) -> EquipmentStatEffect {
    let mut effect = arms_armor::compute_arms_armor_effect(record);
    let eqmod_records = eqmod_referenced_converted_records(record, corpus);
    arms_armor::apply_eqmod_armor_class_bonus(&mut effect, &eqmod_records);
    effect
}

/// Resolves an item's real base `WT:` token divided by every
/// `BONUS:EQM|WEIGHTDIV|<n>` chain carried by its `EQMOD:`-referenced
/// modifier records (SD-33 remediation wave 6, `eqm-modifier-final`
/// lane, `AT-33-E5-002` -- the `EQM|WEIGHTDIV` shape had no resolver at
/// all before this cycle, confirmed absent by
/// `grep -rn "WEIGHTDIV" src/rules_core/`). Real corpus example:
/// Advanced Race Guide's `Material ~ Darkleaf Cloth ~ Clothing`,
/// `BONUS:EQM|WEIGHTDIV|2` -- confirmed live against the pinned PCGen
/// oracle this cycle: an `Outfit (Explorer's)`-shaped host (`WT:8`)
/// with this modifier's real `KEY:` baked into its own item line
/// exports `EQ.MERGELOC.x.WT=4`.
///
/// Multiple attached instances compound by successive division (each
/// divides the running weight, not the base weight independently) --
/// this cycle's population only ever carries one instance, so that
/// choice is unexercised by any real corpus record and stated here as
/// the documented behavior, not proven by a corpus example.
///
/// Returns `None` when the item carries no `WT:` token, or resolves to
/// no `EQMOD:`-referenced modifier carrying this chain -- honest
/// absence, never a fabricated weight.
pub fn resolve_eqm_weightdiv_effect(item_id: &str, corpus: &SourcePackageContent) -> Option<f32> {
    let record = equipment_converted_resolve(item_id, corpus)?;
    let base_weight = record.weight_lbs? as f32;
    let eqmod_records = eqmod_referenced_converted_records(record, corpus);
    let divisors: Vec<f32> =
        eqmod_records.iter().filter_map(|modifier| modifier.weight_divisor).collect();
    if divisors.is_empty() {
        return None;
    }
    Some(
        divisors
            .iter()
            .fold(base_weight, |acc, divisor| if *divisor != 0.0 { acc / divisor } else { acc }),
    )
}

