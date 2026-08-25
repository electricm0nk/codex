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

pub mod arms_armor;
pub mod equipmods;
pub mod general;
pub mod intelligent_item;
pub mod magic_items;

use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;
use crate::rules_core::character_input::EquipmentSelection;
use crate::rules_core::equipment_effects::equipmods::WeaponEnhancementBonus;
use crate::rules_core::equipment_effects::general::SkillCheckBonus;
use crate::rules_core::equipment_effects::intelligent_item::IntelligentItemContribution;
use crate::rules_core::equipment_effects::magic_items::AbilityScoreBonus;
use crate::rules_core::equipment_resolver::{equipment_id_resolve, equipment_key_token};
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::equipment_tables::{equipment_tables, EquipmentCategory};
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::source_content::SourcePackageContent;

/// Resolves a record's own `EQMOD:` token into the `equipment_modifier`
/// corpus records it references, across the whole loaded corpus.
///
/// SD-33 remediation wave 4 (`AT-33-E5-003`): the modifier a base item's
/// `EQMOD:` token names frequently lives in a *different* book than the
/// base item itself — e.g. Core Rulebook's own `Special Ability ~ +N ~
/// Armor` family, referenced by name from `inner_sea_races`/
/// `advanced_class_guide`/... items, or Ultimate Equipment's `Special
/// Ability ~ Martyring ~ Armor`, referenced from `inner_sea_races`. This
/// is why every caller must pass the FULL loaded corpus (every book the
/// resolution needs), not just the base item's own book.
///
/// PCGen's real `EQMOD:` grammar (confirmed against every real corpus
/// value this cycle read): `.` separates independently-attached
/// modifier instances (e.g. `A.B.C`); within one instance, `|` either
/// separates the modifier's own key from a trailing numeric parameter
/// (`Special Ability ~ Enhancement Cost|12600` — a cost override, not
/// itself a resolvable key) or lists alternative modifier keys for one
/// slot (`Material ~ Leather|Material ~ Steel`). Both shapes are handled
/// the same way here: every `|`-segment of every `.`-instance is tried
/// as a candidate modifier key via [`equipment_id_resolve`]. A candidate
/// that is not itself a real corpus key (a bare numeric parameter, or an
/// alternative this specific record did not take) simply fails to
/// resolve and is dropped — never fabricated, never guessed. This is
/// safe against double-counting: confirmed directly against every real
/// corpus record this cycle's own fix consumes that a non-`+N`/
/// enhancement modifier (a material, a cosmetic special quality) never
/// carries a `COMBAT|AC`/`VAR` chain of its own, so resolving extra,
/// non-enhancement candidates from the same instance contributes 0.
pub fn eqmod_referenced_records<'a>(
    record: &EquipmentRecord,
    rule_set: RuleSetId,
    corpus: &'a SourcePackageContent,
) -> Vec<&'a EquipmentRecord> {
    let Some(eqmod_value) = record.tokens.iter().find(|t| t.key == "EQMOD").map(|t| t.value.as_str())
    else {
        return Vec::new();
    };
    let mut resolved = Vec::new();
    for instance in eqmod_value.split('.') {
        for candidate in instance.split('|') {
            let candidate = candidate.trim();
            if candidate.is_empty() {
                continue;
            }
            if let Some((modifier_record, _table_cell)) = equipment_id_resolve(candidate, rule_set, corpus) {
                resolved.push(modifier_record);
            }
        }
    }
    resolved
}

/// Per-category stat contribution shared across every
/// `equipment_effects/<category>.rs` file. `None` means the category's
/// resolver has not populated that field (either because the underlying
/// corpus record carries no such token, or — for a category whose cycle
/// has not landed yet — because no resolver exists yet at all).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
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
        let Some((record, table_cell)) =
            equipment_id_resolve(&selection.item_id, RuleSetId::Crb, corpus)
        else {
            continue;
        };
        let to_hit_bonus = is_weapon_record(record).then(|| {
            let bonus = resolve_weapon_to_hit_bonus(record, &selection.applied_modifiers, corpus);
            weapon_count += 1;
            single_weapon_to_hit_bonus = bonus;
            bonus
        });
        let intelligent_item_contribution =
            resolve_intelligent_item_contribution(&selection.applied_modifiers, corpus);
        let key = equipment_key_token(record)
            .unwrap_or(&record.name)
            .to_string();
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
        let effect = resolve_category_effect(record, RuleSetId::Crb, corpus);
        let skill_bonus = general::compute_general_effect(record);
        let ability_bonus = magic_items::compute_magic_items_effect(record);
        let weapon_enhancement_bonus = equipmods::compute_equipmods_effect(record);
        let spell_resistance_bonus = equipmods::resolve_spell_resistance_bonus(record);
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
    weapon_record: &EquipmentRecord,
    applied_modifiers: &[String],
    corpus: &SourcePackageContent,
) -> i16 {
    let weapon_is_natural_attack = is_natural_attack_weapon(weapon_record);
    let mut total = 0;
    for modifier_item_id in applied_modifiers {
        let Some((record, _table_cell)) = equipment_id_resolve(modifier_item_id, RuleSetId::Crb, corpus) else {
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
        let Some((record, _table_cell)) = equipment_id_resolve(modifier_item_id, RuleSetId::Crb, corpus) else {
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
pub fn is_natural_attack_weapon(record: &EquipmentRecord) -> bool {
    record
        .tokens
        .iter()
        .find(|token| token.key == "TYPE")
        .is_some_and(|token| token.value.split('.').any(|segment| segment == "Natural"))
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
fn is_weapon_record(record: &EquipmentRecord) -> bool {
    let has_damage = record.tokens.iter().any(|token| token.key == "DAMAGE");
    let is_shield = record
        .tokens
        .iter()
        .find(|token| token.key == "TYPE")
        .is_some_and(|token| token.value.split('.').next() == Some("Shield"));
    has_damage && !is_shield
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
/// `eqmod_referenced_records`'s own doc comment) and sums their real
/// `COMBAT|AC` contribution in -- a magic armor/shield item's enhancement
/// bonus lives on that separate record, never on the base item's own
/// chain (see `arms_armor::apply_eqmod_armor_class_bonus`'s own doc
/// comment for the confirmed real-corpus evidence).
fn resolve_category_effect(
    record: &EquipmentRecord,
    rule_set: RuleSetId,
    corpus: &SourcePackageContent,
) -> EquipmentStatEffect {
    let mut effect = arms_armor::compute_arms_armor_effect(record);
    let eqmod_records = eqmod_referenced_records(record, rule_set, corpus);
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
    let (record, _table_cell) = equipment_id_resolve(item_id, RuleSetId::Crb, corpus)?;
    let base_weight: f32 = record
        .tokens
        .iter()
        .find(|token| token.key == "WT")
        .and_then(|token| token.value.parse().ok())?;
    let eqmod_records = eqmod_referenced_records(record, RuleSetId::Crb, corpus);
    let divisors: Vec<f32> = eqmod_records
        .iter()
        .filter_map(|modifier| {
            modifier.bonus_chains.iter().find_map(|bonus| {
                let qualifiers = &bonus.qualifiers;
                if qualifiers.len() >= 3 && qualifiers[0] == "EQM" && qualifiers[1] == "WEIGHTDIV" {
                    qualifiers[2].parse::<f32>().ok()
                } else {
                    None
                }
            })
        })
        .collect();
    if divisors.is_empty() {
        return None;
    }
    Some(
        divisors
            .iter()
            .fold(base_weight, |acc, divisor| if *divisor != 0.0 { acc / divisor } else { acc }),
    )
}

#[cfg(test)]
mod eqm_weightdiv_tests {
    use super::*;
    use crate::pcgen_import::ir_converter::convert_equipment_record;
    use crate::pcgen_import::lst_parser::equipment::parse_equipment_entries;
    use crate::rules_core::source_content::SourceRef;

    fn corpus_from(text: &str, source_file: &str) -> SourcePackageContent<'static> {
        let result = parse_equipment_entries(source_file, text);
        assert!(result.diagnostics.is_empty(), "fixture text must parse cleanly: {:?}", result.diagnostics);
        let source_ref = SourceRef { lst_file: source_file.to_string(), line: 1 };
        let mut corpus = SourcePackageContent::empty("advanced_race_guide", source_ref);
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }
        corpus
    }

    /// Real verbatim tokens: `KEY:Outfit (Explorer's)` (`WT:8`,
    /// `core_rulebook/cr_equip_general.lst`) with `EQMOD:Material ~
    /// Darkleaf Cloth ~ Clothing` baked in, plus the real modifier record
    /// (`advanced_race_guide/arg_equipmods.lst`, `BONUS:EQM|WEIGHTDIV|2`)
    /// -- the pair the live oracle confirmed this cycle (`WT:8` -> `4`).
    #[test]
    fn weightdiv_halves_a_real_hosts_weight() {
        let text = "\
Outfit (Explorer's) Darkleaf\tKEY:Outfit ~ Darkleaf Test\tTYPE:Goods.Clothing.Resizable.Starting\tCOST:10\tWT:8\tEQMOD:Material ~ Darkleaf Cloth ~ Clothing\n\
Darkleaf Cloth\tKEY:Material ~ Darkleaf Cloth ~ Clothing\tTYPE:BaseMaterial.MasterworkQuality.Cloth.Clothing\tCOST:500\tBONUS:EQM|WEIGHTDIV|2\n";
        let corpus = corpus_from(text, "arg_equip_general.lst");

        let weight = resolve_eqm_weightdiv_effect("Outfit ~ Darkleaf Test", &corpus);

        assert_eq!(weight, Some(4.0), "8 lbs divided by WEIGHTDIV:2 must be 4");
    }

    #[test]
    fn a_host_with_no_eqmod_yields_none_not_a_fabricated_weight() {
        let text = "Outfit (Explorer's)\tKEY:Outfit (Base) Test\tTYPE:Goods.Clothing\tCOST:10\tWT:8\n";
        let corpus = corpus_from(text, "cr_equip_general.lst");

        let weight = resolve_eqm_weightdiv_effect("Outfit (Base) Test", &corpus);

        assert_eq!(weight, None, "no EQMOD attached means no WEIGHTDIV chain to apply -- honest None");
    }
}

#[cfg(test)]
mod attack_bonus_delta_tests {
    use super::*;
    use crate::pcgen_import::ir_converter::convert_equipment_record;
    use crate::pcgen_import::lst_parser::equipment::parse_equipment_entries;
    use crate::rules_core::character_input::ActiveState;
    use crate::rules_core::source_content::SourceRef;

    // Real verbatim corpus tokens: Longsword/Chain Shirt/Masterwork quality
    // match `tests/sd20_contract_equipment_wiring.rs`'s own fixture exactly.
    // Dagger's DAMAGE:1d4 is the real CRB value. The `+1 (Enhancement to
    // Weapon)` line matches `equipmods.rs`'s own test fixture exactly. The
    // DAMAGE-only enhancement line uses the same `BONUS:WEAPON|<...>|<n>|
    // TYPE=Enhancement` grammar `equipmods.rs`'s own module doc comment
    // documents as a real, observed corpus shape (`TOHIT`, `DAMAGE`, or
    // `DAMAGE,TOHIT`) -- exercising the `DAMAGE`-only case specifically,
    // which no existing fixture in this codebase happens to cover.
    const FIXTURE_TEXT: &str = "\
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8
Dagger\tKEY:Dagger (Base)\tTYPE:Weapon.Melee.Simple\tCOST:2\tWT:1\tCRITMULT:x2\tDAMAGE:1d4
Chain Shirt\tKEY:Chain Shirt (Base)\tTYPE:Armor.Light\tCOST:100\tWT:25\tACCHECK:-2\tMAXDEX:4\tSPELLFAILURE:20\tBONUS:COMBAT|AC|4|TYPE=Armor|PREVAREQ:DisableArmorBonus,0
+1 (Enhancement to Weapon)\tKEY:Special Ability ~ +1 ~ Weapon\tTYPE:Weapon\tPLUS:1\tCOST:0\tBONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement
Masterwork (Weapon)\tKEY:Special Quality ~ Masterwork ~ Weapon\tTYPE:MasterworkQuality.Weapon\tCOST:0\tBONUS:WEAPON|TOHIT|1|TYPE=Enhancement
Flaming\tKEY:Special Ability ~ Flaming ~ Weapon\tTYPE:Weapon\tCOST:0\tBONUS:WEAPON|DAMAGE|2|TYPE=Enhancement
";

    fn corpus_with_fixture() -> SourcePackageContent<'static> {
        let result = parse_equipment_entries("cr_equip_arms_armor.lst", FIXTURE_TEXT);
        assert!(result.diagnostics.is_empty(), "fixture text must parse cleanly: {:?}", result.diagnostics);
        let source_ref = SourceRef { lst_file: "cr_equip_arms_armor.lst".to_string(), line: 1 };
        let mut corpus = SourcePackageContent::empty("core_rulebook", source_ref);
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }
        corpus
    }

    fn equipped(item_id: &str) -> EquipmentSelection {
        equipped_with_modifiers(item_id, &[])
    }

    fn equipped_with_modifiers(item_id: &str, applied_modifiers: &[&str]) -> EquipmentSelection {
        EquipmentSelection {
            item_id: item_id.to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: applied_modifiers.iter().map(|id| id.to_string()).collect(),
        }
    }

    #[test]
    fn exactly_one_weapon_plus_a_real_enhancement_yields_a_real_attack_bonus() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped_with_modifiers(
            "Longsword (Base)",
            &["Special Ability ~ +1 ~ Weapon"],
        )];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(effects.attack_bonus_delta, Some(1), "unambiguous single weapon: +1 TOHIT applies");
        assert_eq!(effects.per_item[0].to_hit_bonus, Some(1));
    }

    #[test]
    fn a_masterwork_tohit_only_bonus_also_counts() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped_with_modifiers(
            "Longsword (Base)",
            &["Special Quality ~ Masterwork ~ Weapon"],
        )];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(effects.attack_bonus_delta, Some(1));
    }

    #[test]
    fn a_damage_only_enhancement_does_not_affect_the_attack_bonus() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped_with_modifiers(
            "Longsword (Base)",
            &["Special Ability ~ Flaming ~ Weapon"],
        )];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(
            effects.attack_bonus_delta,
            Some(0),
            "a DAMAGE-only enhancement must not contribute to the TOHIT-affecting attack bonus, \
             but the field itself is still a real, unambiguous Some(0), not None"
        );
    }

    #[test]
    fn exactly_one_weapon_with_no_enhancement_yields_a_real_zero() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped("Longsword (Base)"), equipped("Chain Shirt (Base)")];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(
            effects.attack_bonus_delta,
            Some(0),
            "no ambiguity with one weapon; the real value is honestly zero, not absent"
        );
    }

    /// v0.6 alpha swarm sub-task 2: a modifier that exists in the corpus and
    /// is genuinely equipped, but is NOT listed in any weapon's
    /// `applied_modifiers`, must not silently attach to "the only weapon
    /// equipped" any more -- proves attachment is now required, not merely
    /// unambiguous-by-elimination (the old heuristic this replaced would
    /// have applied it here).
    #[test]
    fn an_equipped_but_unattached_modifier_does_not_apply_to_any_weapon() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped("Longsword (Base)"), equipped("Special Ability ~ +1 ~ Weapon")];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(
            effects.attack_bonus_delta,
            Some(0),
            "the +1 modifier is equipped but not attached to the Longsword via applied_modifiers, \
             so it must not count -- explicit attachment is required now"
        );
    }

    /// v0.6 alpha swarm sub-task 2: the real, closed 2+-weapon gap. The
    /// aggregate `attack_bonus_delta` stays honestly `None` (a single
    /// scalar can't represent two different weapons' bonuses at once --
    /// see its own doc comment), but each weapon's own `to_hit_bonus` in
    /// `per_item` is now real and unambiguous, since each modifier is
    /// explicitly attached to the specific weapon it enhances.
    #[test]
    fn two_weapons_each_with_their_own_attached_modifier_resolve_independently() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![
            equipped_with_modifiers("Longsword (Base)", &["Special Ability ~ +1 ~ Weapon"]),
            equipped("Dagger (Base)"),
        ];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(
            effects.attack_bonus_delta, None,
            "the aggregate scalar stays None for 2+ weapons -- it cannot represent both at once"
        );
        let longsword = effects
            .per_item
            .iter()
            .find(|item| item.item_id == "Longsword (Base)")
            .expect("Longsword must resolve");
        let dagger = effects
            .per_item
            .iter()
            .find(|item| item.item_id == "Dagger (Base)")
            .expect("Dagger must resolve");
        assert_eq!(longsword.to_hit_bonus, Some(1), "Longsword's own attached +1 resolves independently");
        assert_eq!(dagger.to_hit_bonus, Some(0), "Dagger has no attached modifier, a real zero");
    }

    #[test]
    fn zero_weapons_equipped_leaves_the_attack_bonus_honestly_absent() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped("Chain Shirt (Base)")];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(effects.attack_bonus_delta, None, "no weapon at all -- nothing to attach a bonus to");
    }

    /// Regression test for the real bug found by wiring the real corpus into
    /// the desktop app: a standard armor+shield+one-weapon loadout used to
    /// resolve as "2+ weapons, ambiguous" the moment a shield's own real
    /// `DAMAGE:` bash-attack token reached this path (the tiny desktop
    /// fixture bundle never carried a real shield record, so this was never
    /// exercised before). A shield must not count toward the single-weapon
    /// ambiguity check.
    #[test]
    fn a_shield_with_a_real_bash_damage_token_does_not_count_as_a_second_weapon() {
        const SHIELD_FIXTURE_TEXT: &str = "\
Heavy Wooden Shield\tKEY:Heavy Wooden Shield (Base)\tTYPE:Shield.Heavy.Weapon.Resizable.Melee.ShieldBash.Close.Weapon Group Close.Nonmetal\tCOST:7\tWT:10\tACCHECK:-2\tDAMAGE:1d4\tBONUS:COMBAT|AC|2|TYPE=Shield|PREVAREQ:DisableShieldBonus,0\n";
        let result = parse_equipment_entries("cr_equip_arms_armor.lst", SHIELD_FIXTURE_TEXT);
        assert!(result.diagnostics.is_empty(), "fixture text must parse cleanly: {:?}", result.diagnostics);
        let mut corpus = corpus_with_fixture();
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }

        let equipped_items = vec![equipped("Longsword (Base)"), equipped("Heavy Wooden Shield (Base)")];
        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(
            effects.attack_bonus_delta,
            Some(0),
            "exactly one real weapon (the Longsword) -- the shield's own bash-damage token must not count as a second"
        );
        let shield = effects
            .per_item
            .iter()
            .find(|item| item.item_id == "Heavy Wooden Shield (Base)")
            .expect("shield must resolve");
        assert_eq!(shield.armor_class_bonus, Some(2), "the shield's own real AC bonus still resolves");
    }
}

/// Regression tests for the real bug found by SD-27's Advanced Race Guide
/// PCGen parity work: `compute_equipment_effects` gated every per-item effect
/// (AC bonus, max Dex, spell failure, armor check penalty, weapon
/// enhancement to-hit) behind a category lookup in the CRB-only compiled
/// `rules_tables::crb::equipment_tables()` store. A non-CRB item's key was
/// never in that table, so the item was `continue`'d out of the loop before
/// any of the per-category resolvers (which are all already book-agnostic --
/// see `arms_armor.rs`/`general.rs`/`magic_items.rs`/`equipmods.rs`'s own
/// doc comments) ever ran. See
/// `docs/release/v0.6/book-agnostic-backend-gaps-scoping.md` finding 1.
#[cfg(test)]
mod book_agnostic_resolution_tests {
    use super::*;
    use crate::pcgen_import::ir_converter::convert_equipment_record;
    use crate::pcgen_import::lst_parser::equipment::parse_equipment_entries;
    use crate::rules_core::character_input::ActiveState;
    use crate::rules_core::source_content::SourceRef;

    fn equipped(item_id: &str) -> EquipmentSelection {
        EquipmentSelection {
            item_id: item_id.to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
        }
    }

    fn equipped_with_modifiers(item_id: &str, applied_modifiers: &[&str]) -> EquipmentSelection {
        EquipmentSelection {
            item_id: item_id.to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: applied_modifiers.iter().map(|id| id.to_string()).collect(),
        }
    }

    /// Not a literal single verbatim ARG record (no clean single-line
    /// arms_armor example exists in the real ARG source -- the book adds
    /// exotic weapons and accessories, not new base armor), but real,
    /// verified CRB armor token grammar (matches this file's own
    /// `Chain Shirt (Base)` fixture exactly), tagged as coming from
    /// `advanced_race_guide` to exercise the book-agnostic path
    /// specifically. Stated plainly rather than implied as verbatim.
    const ARG_ARMOR_FIXTURE_TEXT: &str = "\
Reinforced Leather (ARG)\tKEY:Reinforced Leather (ARG)\tTYPE:Armor.Light\tCOST:120\tWT:18\tACCHECK:-1\tMAXDEX:5\tSPELLFAILURE:15\tBONUS:COMBAT|AC|3|TYPE=Armor\n";

    // Real ARG equipmods token grammar, verbatim shape from
    // `arg_equipmods.lst`'s own `BONUS:WEAPON|...|TYPE=Enhancement` records.
    const ARG_EQUIPMOD_FIXTURE_TEXT: &str = "\
Longsword (ARG)\tKEY:Longsword (ARG)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n\
Keen (ARG)\tKEY:Special Ability ~ Keen ~ Weapon (ARG)\tTYPE:Weapon\tCOST:0\tBONUS:WEAPON|TOHIT|1|TYPE=Enhancement\n";

    fn arg_corpus(text: &str, source_file: &str) -> SourcePackageContent<'static> {
        let result = parse_equipment_entries(source_file, text);
        assert!(result.diagnostics.is_empty(), "fixture text must parse cleanly: {:?}", result.diagnostics);
        let source_ref = SourceRef { lst_file: source_file.to_string(), line: 1 };
        let mut corpus = SourcePackageContent::empty("advanced_race_guide", source_ref);
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }
        corpus
    }

    #[test]
    fn a_non_crb_armor_item_resolves_all_four_arms_armor_stats() {
        let corpus = arg_corpus(ARG_ARMOR_FIXTURE_TEXT, "arg_equip_arms_armor.lst");
        let equipped_items = vec![equipped("Reinforced Leather (ARG)")];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(effects.per_item.len(), 1, "the item must not be silently dropped");
        let item = &effects.per_item[0];
        assert_eq!(item.armor_class_bonus, Some(3));
        assert_eq!(item.max_dex, Some(5));
        assert_eq!(item.spell_failure, Some(15.0));
        assert_eq!(item.armor_check_penalty, Some(-1));
        assert_eq!(item.category, EquipmentCategory::ArmsArmor);
        assert_eq!(effects.armor_class_delta, 3);
        assert_eq!(effects.max_dex_cap, Some(5));
        assert_eq!(effects.spell_failure_chance, Some(15.0));
        assert_eq!(effects.armor_check_penalty_total, -1);
    }

    #[test]
    fn a_non_crb_equipment_modifier_still_applies_its_tohit_bonus() {
        let corpus = arg_corpus(ARG_EQUIPMOD_FIXTURE_TEXT, "arg_equipmods.lst");
        let equipped_items = vec![equipped_with_modifiers(
            "Longsword (ARG)",
            &["Special Ability ~ Keen ~ Weapon (ARG)"],
        )];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(effects.attack_bonus_delta, Some(1), "a non-CRB weapon modifier must still apply");
        assert_eq!(effects.per_item[0].to_hit_bonus, Some(1));
    }

    /// SD-33 remediation wave 4 (`AT-33-E5-003`): end-to-end proof that
    /// `compute_equipment_effects` itself (not just
    /// `arms_armor::apply_eqmod_armor_class_bonus` in isolation) resolves
    /// a base item's `EQMOD:`-referenced modifier record ACROSS THE
    /// WHOLE CORPUS and sums its real `COMBAT|AC` contribution -- real
    /// verbatim tokens from `inner_sea_races:equipment:armor_of_grim_
    /// triumph` (base) and Core Rulebook's own `Special Ability ~ +1 ~
    /// Armor` (the modifier, in a DIFFERENT source line, proving
    /// resolution is not book-scoped). Before this fix,
    /// `item.armor_class_bonus` was `Some(6)` (the base item's own chain
    /// alone) -- a real, confirmed disagreement against the pinned
    /// oracle's `7` (`AT-33-E5-003.combined-oracle-results.json`).
    #[test]
    fn eqmod_referenced_modifier_sums_across_the_whole_corpus() {
        let text = "\
Armor of Grim Triumph\tKEY:Armor of Grim Triumph\tTYPE:Armor.Magic.Medium.ArmorProfMedium.Suit.Specific\tCOST:250\tWT:40\tACCHECK:-4\tEQMOD:Special Ability ~ Enhancement Cost|12600.Special Ability ~ +1 ~ Armor.Special Quality ~ Spikes ~ Armor.Material ~ Steel\tMAXDEX:3\tSPELLFAILURE:25\tBONUS:COMBAT|AC|6|TYPE=Armor\n\
+1 (Enhancement to Armor)\tKEY:Special Ability ~ +1 ~ Armor\tTYPE:Armor\tPLUS:1\tBONUS:COMBAT|AC|1|TYPE=ArmorEnhancement|PREVAREQ:DisableArmorBonus,0\n\
Armor Spikes\tKEY:Special Quality ~ Spikes ~ Armor\tTYPE:Armor\tCOST:50\n";
        let corpus = arg_corpus(text, "isr_equip_arms_armor.lst");
        let equipped_items = vec![equipped("Armor of Grim Triumph")];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(effects.per_item.len(), 1);
        assert_eq!(
            effects.per_item[0].armor_class_bonus,
            Some(7),
            "base item's own 6 plus the EQMOD-referenced +1 Armor modifier's own separate chain"
        );
        assert_eq!(effects.armor_class_delta, 7);
    }
}

/// SD-31 intelligent-item resolver (operator ruling 2026-08-19) end-to-end
/// aggregation tests, and the `SD31-W17-INTEGRATE-001` (OPEN-ISSUES row
/// 309) natural-attack-scope fix.
#[cfg(test)]
mod intelligent_item_and_natural_attack_scope_tests {
    use super::*;
    use crate::pcgen_import::ir_converter::convert_equipment_record;
    use crate::pcgen_import::lst_parser::equipment::parse_equipment_entries;
    use crate::rules_core::character_input::ActiveState;
    use crate::rules_core::equipment_effects::intelligent_item::ItemAlignment;
    use crate::rules_core::source_content::SourceRef;

    // Real verbatim tokens: the Base record (line 354) + a real Ability
    // Score record (line 377, Wisdom 15) + a real Alignment record
    // (`ma_equipmods.lst` line 96, Chaotic Good) attached to a Longsword,
    // exactly the way a real character build attaches applied modifiers.
    const INTELLIGENT_ITEM_FIXTURE_TEXT: &str = "\
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8
Intelligent Magic Item Base\tKEY:Intelligent Item ~ Base\tTYPE:Weapon.Armor.Goods\tCOST:500\tBONUS:VAR|IntItemStatINT|10\tBONUS:VAR|IntItemStatWIS|10\tBONUS:VAR|IntItemStatCHA|10\n\
Int Item / Stat Wisdom 15\tKEY:Intelligent Item ~ Ability Score / Wisdom 15\tTYPE:Weapon.Armor.Goods\tCOST:1400\tBONUS:VAR|IntelligentItemEgo|2\tBONUS:VAR|IntItemStatWIS|5\n\
Legendary Intelligent Item / Align (CG)\tKEY:Legendary Item ~ Intelligent Item ~ Alignment / Chaotic Good\tTYPE:Mythic.Intelligent.Alignment\tCOST:0\tBONUS:VAR|IntItemAlignment|20\n\
Unarmed Strike\tKEY:Unarmed Strike\tTYPE:Weapon.Resizable.Melee.Special.Unarmed.Monk.Bludgeoning.Finesseable.Close.Weapon Group Close.Weapon Group Monk.Weapon Group Natural.Natural.Light\tCOST:0\tWT:0\tCRITMULT:x2\tCRITRANGE:1\tDAMAGE:1d3\tWIELD:Light\n\
+1 to Hit and Damage\tKEY:Special Ability ~ +1 ~ Amulet of Mighty Fists\tTYPE:Amulet of Mighty Fists\tPLUS:1\tBONUS:WEAPONPROF=TYPE.Natural|TOHIT,DAMAGE|1|TYPE=Enhancement\n\
+1 (Enhancement to Weapon)\tKEY:Special Ability ~ +1 ~ Weapon\tTYPE:Weapon\tPLUS:1\tCOST:0\tBONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement\n";

    fn corpus_with_fixture() -> SourcePackageContent<'static> {
        let result = parse_equipment_entries("cr_equipmods.lst", INTELLIGENT_ITEM_FIXTURE_TEXT);
        assert!(result.diagnostics.is_empty(), "fixture text must parse cleanly: {:?}", result.diagnostics);
        let source_ref = SourceRef { lst_file: "cr_equipmods.lst".to_string(), line: 1 };
        let mut corpus = SourcePackageContent::empty("core_rulebook", source_ref);
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }
        corpus
    }

    fn equipped_with_modifiers(item_id: &str, applied_modifiers: &[&str]) -> EquipmentSelection {
        EquipmentSelection {
            item_id: item_id.to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: applied_modifiers.iter().map(|id| id.to_string()).collect(),
        }
    }

    #[test]
    fn an_intelligent_longsword_yields_its_real_summed_stat_block() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped_with_modifiers(
            "Longsword (Base)",
            &[
                "Intelligent Item ~ Base",
                "Intelligent Item ~ Ability Score / Wisdom 15",
                "Legendary Item ~ Intelligent Item ~ Alignment / Chaotic Good",
            ],
        )];

        let effects = compute_equipment_effects(&equipped_items, &corpus);
        let item = effects
            .per_item
            .iter()
            .find(|item| item.item_id == "Longsword (Base)")
            .expect("Longsword must resolve");
        let intelligent_item = item
            .intelligent_item
            .as_ref()
            .expect("an intelligent longsword must yield a contribution");

        assert_eq!(intelligent_item.intelligence_bonus, 10, "10 (Base) + 0 (no INT modifier attached)");
        assert_eq!(intelligent_item.wisdom_bonus, 15, "10 (Base) + 5 (Wisdom 15's own delta) == 15");
        assert_eq!(intelligent_item.charisma_bonus, 10, "10 (Base) + 0");
        assert_eq!(intelligent_item.ego_bonus, 2, "Wisdom 15's own literal Ego contribution");
        assert_eq!(intelligent_item.alignment, Some(ItemAlignment::ChaoticGood));
    }

    #[test]
    fn an_ordinary_longsword_with_no_intelligent_item_modifiers_has_none() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped_with_modifiers("Longsword (Base)", &["Special Ability ~ +1 ~ Weapon"])];

        let effects = compute_equipment_effects(&equipped_items, &corpus);
        let item = &effects.per_item[0];

        assert_eq!(item.intelligent_item, None, "a plain +1 weapon carries no intelligent-item token at all");
    }

    /// Regression guard: an intelligent item's stat block is scoped to the
    /// SPECIFIC selection it's attached to (`applied_modifiers`), not
    /// summed across the whole loadout the way `weapon_enhancement_bonus`
    /// bounded model does -- a second, ordinary weapon equipped alongside
    /// the intelligent longsword must not pick up its stat block.
    #[test]
    fn a_second_unattached_weapon_does_not_inherit_the_first_items_intelligence() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![
            equipped_with_modifiers(
                "Longsword (Base)",
                &["Intelligent Item ~ Base", "Intelligent Item ~ Ability Score / Wisdom 15"],
            ),
            equipped_with_modifiers("Unarmed Strike", &[]),
        ];

        let effects = compute_equipment_effects(&equipped_items, &corpus);
        let unarmed = effects
            .per_item
            .iter()
            .find(|item| item.item_id == "Unarmed Strike")
            .expect("Unarmed Strike must resolve");

        assert_eq!(
            unarmed.intelligent_item, None,
            "the Longsword's own intelligent-item stat block must not leak onto an unattached weapon"
        );
    }

    #[test]
    fn unarmed_strike_is_a_real_natural_attack() {
        let corpus = corpus_with_fixture();
        let (record, _) = equipment_id_resolve("Unarmed Strike", RuleSetId::Crb, &corpus)
            .expect("Unarmed Strike must resolve");
        assert!(is_natural_attack_weapon(record));
    }

    #[test]
    fn a_longsword_is_not_a_natural_attack() {
        let corpus = corpus_with_fixture();
        let (record, _) = equipment_id_resolve("Longsword (Base)", RuleSetId::Crb, &corpus)
            .expect("Longsword must resolve");
        assert!(!is_natural_attack_weapon(record));
    }

    /// `SD31-W17-INTEGRATE-001` (OPEN-ISSUES row 309): the Amulet of
    /// Mighty Fists family's `WEAPONPROF=TYPE.Natural` chain is now
    /// recognized (`equipmods.rs`) and tagged `natural_attack_only: true`
    /// -- proves the tag survives resolution through
    /// `compute_equipment_effects`, ready for `damage_total.rs`'s
    /// consumer to honour.
    #[test]
    fn an_amulet_of_mighty_fists_modifier_is_tagged_natural_attack_only() {
        let corpus = corpus_with_fixture();
        // Equipped as its own top-level selection, matching how
        // `equipmods.rs`'s own weapon-enhancement fixtures equip
        // "Special Ability ~ +1 ~ Weapon" directly -- `weapon_enhancement_bonus`
        // reads the SELECTION's own record, unlike `to_hit_bonus`/
        // `intelligent_item`, which read `applied_modifiers`.
        let equipped_items = vec![equipped_with_modifiers("Special Ability ~ +1 ~ Amulet of Mighty Fists", &[])];

        let effects = compute_equipment_effects(&equipped_items, &corpus);
        let amulet = &effects.per_item[0];
        let bonus = amulet
            .weapon_enhancement_bonus
            .as_ref()
            .expect("the Amulet of Mighty Fists chain must now resolve to a real bonus");
        assert!(bonus.natural_attack_only, "the Amulet of Mighty Fists family scopes to natural attacks only");
        assert_eq!(bonus.tohit_bonus, Some(1));
        assert_eq!(bonus.damage_bonus, Some(1));
    }

    /// `SD31-W18-INTEGRATE-001` (adversarial review, `OPEN-ISSUES.md` row
    /// 309 re-opened): `damage_total::resolve_weapon_enhancement_modifier`
    /// was scope-guarded in wave 18, but `resolve_weapon_to_hit_bonus` --
    /// the function `compute_equipment_effects` actually calls for
    /// `to_hit_bonus`/`attack_bonus_delta` via `selection.applied_modifiers`,
    /// the SAME attachment shape the shipped desktop app's
    /// `attach_equipment_modifier_at_root` uses -- was not. Confirmed live
    /// before this fix: an Amulet of Mighty Fists +1 attached to an
    /// ordinary Longsword yielded `to_hit_bonus = Some(1)` /
    /// `attack_bonus_delta = Some(1)`, leaking the natural-attack-only
    /// bonus onto a non-natural weapon. Mutation-provable: removing the
    /// `bonus.natural_attack_only && !weapon_is_natural_attack` skip in
    /// `resolve_weapon_to_hit_bonus` reproduces `Some(1)` here.
    #[test]
    fn amulet_of_mighty_fists_attached_as_a_modifier_does_not_leak_onto_an_ordinary_longsword() {
        let corpus = corpus_with_fixture();
        let equipped_items =
            vec![equipped_with_modifiers("Longsword (Base)", &["Special Ability ~ +1 ~ Amulet of Mighty Fists"])];

        let effects = compute_equipment_effects(&equipped_items, &corpus);
        let longsword = &effects.per_item[0];

        assert_eq!(
            longsword.to_hit_bonus,
            Some(0),
            "a natural-attack-only bonus attached to a non-natural weapon must not apply"
        );
        assert_eq!(
            effects.attack_bonus_delta,
            Some(0),
            "the single-weapon attack_bonus_delta scalar must reflect the same scope guard"
        );
    }

    /// Positive control for the same guard: a REAL natural attack
    /// (Unarmed Strike) with the Amulet attached as a modifier DOES
    /// receive the bonus -- proves the fix scopes the bonus, it does not
    /// simply zero it out everywhere.
    #[test]
    fn amulet_of_mighty_fists_attached_as_a_modifier_applies_to_a_real_natural_attack() {
        let corpus = corpus_with_fixture();
        let equipped_items =
            vec![equipped_with_modifiers("Unarmed Strike", &["Special Ability ~ +1 ~ Amulet of Mighty Fists"])];

        let effects = compute_equipment_effects(&equipped_items, &corpus);
        let unarmed = &effects.per_item[0];

        assert_eq!(unarmed.to_hit_bonus, Some(1), "a natural-attack-only bonus must apply to a real natural attack");
        assert_eq!(effects.attack_bonus_delta, Some(1));
    }
}

/// `SD31-W21-EQUIPMOD-001`: the armor-slot "Spell Resistance" special
/// ability family (`equipmods::resolve_spell_resistance_bonus`) end to
/// end through `compute_equipment_effects` -- the same `equipment_key_is_
/// wired` shape `v06_work_inventory.rs` probes to ground an
/// `equipment_modifier` unit.
#[cfg(test)]
mod spell_resistance_tests {
    use super::*;
    use crate::pcgen_import::ir_converter::convert_equipment_record;
    use crate::pcgen_import::lst_parser::equipment::parse_equipment_entries;
    use crate::rules_core::character_input::ActiveState;
    use crate::rules_core::source_content::SourceRef;

    // Real verbatim tokens copied from `core_rulebook/cr_equipmods.lst`
    // lines 343 (`/ 13 ~ Armor`) and 346 (`/ 19 ~ Armor`), plus a plain
    // Longsword with no SR-shaped token anywhere, for the negative
    // control.
    const FIXTURE_TEXT: &str = "\
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8
Spell Resistance 13\tFORMATCAT:FRONT\tNAMEOPT:NORMAL\tKEY:Special Ability ~ Spell Resistance / 13 ~ Armor\tTYPE:Armor.Bracer.ArmorLike\tPLUS:2\tVISIBLE:QUALIFY\tPREMULT:2,[PRETYPE:1,ArmorEnhancement],[PRETYPE:1,Armor,Bracer]\tSR:13\tSPROP:grants spell resistance 13
Spell Resistance 19\tFORMATCAT:FRONT\tNAMEOPT:NORMAL\tKEY:Special Ability ~ Spell Resistance / 19 ~ Armor\tTYPE:Armor.Bracer.ArmorLike\tPLUS:8\tVISIBLE:QUALIFY\tPREMULT:2,[PRETYPE:1,ArmorEnhancement],[PRETYPE:1,Armor,Bracer]\tSR:19\tSPROP:grants spell resistance 19
";

    fn corpus_with_fixture() -> SourcePackageContent<'static> {
        let result = parse_equipment_entries("cr_equipmods.lst", FIXTURE_TEXT);
        assert!(result.diagnostics.is_empty(), "fixture text must parse cleanly: {:?}", result.diagnostics);
        let source_ref = SourceRef { lst_file: "cr_equipmods.lst".to_string(), line: 1 };
        let mut corpus = SourcePackageContent::empty("core_rulebook", source_ref);
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }
        corpus
    }

    fn equipped(item_id: &str) -> EquipmentSelection {
        EquipmentSelection {
            item_id: item_id.to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
        }
    }

    #[test]
    fn spell_resistance_13_armor_yields_a_real_per_item_and_aggregate_bonus() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped("Special Ability ~ Spell Resistance / 13 ~ Armor")];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(effects.per_item[0].spell_resistance_bonus, Some(13));
        assert_eq!(effects.spell_resistance_total, Some(13));
    }

    /// PF1's real rule: "If a creature has multiple sources of spell
    /// resistance, only the highest value applies" -- unlike
    /// `armor_class_delta` (armor and shield AC bonuses genuinely stack),
    /// two simultaneous SR sources must yield the HIGHEST single value,
    /// never their sum. Equipping both the 13 and 19 tiers together must
    /// read `Some(19)`, not `Some(32)`.
    #[test]
    fn two_spell_resistance_sources_yield_the_highest_not_the_sum() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![
            equipped("Special Ability ~ Spell Resistance / 13 ~ Armor"),
            equipped("Special Ability ~ Spell Resistance / 19 ~ Armor"),
        ];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(
            effects.spell_resistance_total,
            Some(19),
            "PF1: multiple SR sources take the highest value, they do not stack"
        );
    }

    #[test]
    fn an_ordinary_weapon_has_no_spell_resistance_bonus() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped("Longsword (Base)")];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(effects.per_item[0].spell_resistance_bonus, None);
        assert_eq!(effects.spell_resistance_total, None);
    }
}
