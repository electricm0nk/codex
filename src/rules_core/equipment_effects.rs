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
//! `rules_catalog::crb::equipment_tables` store — not re-derived from raw
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
//! `attached_weapon_enhancement` and `resolve_eqm_weightdiv_effect` take a
//! converted record, and the parser-row `eqmod_referenced_records` is deleted
//! -- [`eqmod_referenced_converted_records`] is the only EQMOD resolution the
//! live side has.

pub mod arms_armor;
pub mod equipmods;
pub mod general;
pub mod intelligent_item;
pub mod magic_items;


use crate::rules_core::character_input::{ActiveState, CharacterInput, EquipmentSelection};
use crate::rules_core::equipment_effects::equipmods::WeaponEnhancementBonus;
use crate::rules_core::equipment_effects::general::{SkillCheckBonus, VarBonus};
use crate::rules_core::equipment_effects::intelligent_item::IntelligentItemContribution;
use crate::rules_core::equipment_effects::magic_items::AbilityScoreBonus;
use crate::rules_core::equipment_record::{CorpusEquipmentRecord, StatedBonus};
use crate::rules_core::equipment_resolver::{
    equipment_converted_resolve, equipment_id_resolve,
};
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_catalog::crb::equipment_tables::{equipment_tables, EquipmentCategory};
use crate::rules_core::rules_catalog::RuleSetId;
use crate::rules_core::game_system::GameSystem;
use crate::rules_core::sheet_rule::stacking_types;
use crate::rules_core::skill_allocation::SHEET_SKILL_IDS;
use crate::rules_core::source_content::SourcePackageContent;

/// The Armor Class bonus types a touch attack ignores.
pub const NOT_AGAINST_TOUCH: [&str; 7] =
    ["Armor", "ArmorEnhancement", "Shield", "ShieldEnhancement", "NaturalArmor", "NaturalArmorEnhancement", "Enhancement"];

/// The record that stands in for what `record` does not state itself, one hop.
pub fn stand_in_record<'a>(
    record: &CorpusEquipmentRecord,
    corpus: &SourcePackageContent<'a>,
) -> Option<&'a CorpusEquipmentRecord> {
    let base = equipment_converted_resolve(record.base_item.as_deref()?, corpus)?;
    (base.identity != record.identity).then_some(base)
}

/// Whether a bonus adds whatever else is held: a penalty, an untyped bonus, and a type the game says
/// stacks with itself. Any other takes the higher of its type on one target, the rule of the sheet
/// engine's `Var` fold.
pub fn bonus_always_counts(value: i16, bonus_type: Option<&str>) -> bool {
    value < 0 || bonus_type.is_none_or(|word| stacking_types(GameSystem::Pathfinder1e).contains(&word))
}

/// One worn item's bonus that counts after stacking, and is already in the engine's totals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WornStatedBonus {
    pub item_name: String,
    pub target: String,
    pub bonus_type: Option<String>,
    pub value: i16,
}

/// What a character's worn stock items state on saves, ability scores, skills, attack, initiative,
/// land speed, CMB, CMD and Armor Class, after stacking, a skill set's bonus on each row of the set.
/// Armor Class is also in [`EquipmentEffects`]; it is listed here so that another source of a bonus
/// can fold with it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WornItemStatedBonuses(Vec<WornStatedBonus>);

impl WornItemStatedBonuses {
    pub fn rows(&self) -> &[WornStatedBonus] {
        &self.0
    }

    fn total(&self, target: &str) -> i16 {
        self.0.iter().filter(|row| row.target == target).map(|row| row.value).sum()
    }

    /// The save's total of what is worn: `save` is `fortitude`, `reflex` or `will`.
    pub fn save(&self, save: &str) -> i16 {
        self.total(&format!("save:{save}"))
    }

    /// The Melee total's share of what is worn.
    pub fn attack(&self) -> i16 {
        self.total("attack")
    }

    /// `+1 Competence (Ioun Stone), -2 Untyped (Robe of Vermin)`, for the attack row's detail.
    pub fn attack_sources(&self) -> String {
        let words: Vec<String> = self
            .0
            .iter()
            .filter(|row| row.target == "attack")
            .map(|row| format!("{:+} {} ({})", row.value, row.bonus_type.as_deref().unwrap_or("Untyped"), row.item_name))
            .collect();
        words.join(", ")
    }

    /// What a bonus of `bonus_type` on `target` has to beat, when one of that type is already counted.
    pub fn counted(&self, target: &str, bonus_type: &str) -> Option<i16> {
        self.0
            .iter()
            .find(|row| row.target == target && row.value > 0 && row.bonus_type.as_deref() == Some(bonus_type))
            .map(|row| row.value)
    }

    /// A copy of `input` with the worn scores raised, for what the sheet computes from. The saved
    /// score does not change.
    pub fn raised(&self, input: &CharacterInput) -> CharacterInput {
        let mut raised = input.clone();
        let scores = &mut raised.chosen.ability_scores;
        scores.strength += self.total("ability:strength");
        scores.dexterity += self.total("ability:dexterity");
        scores.constitution += self.total("ability:constitution");
        scores.intelligence += self.total("ability:intelligence");
        scores.wisdom += self.total("ability:wisdom");
        scores.charisma += self.total("ability:charisma");
        raised
    }
}

/// The Skills panel's rows in a set a chain names, as `skill:<name>`: `all`, the book's `TYPE` words
/// (`base`, `knowledge`, `perform`), and a key ability's word. Craft, Perform and Profession are one
/// row each, whole. `tests/worn_vs_carried.rs` holds each set to the book's own skill records.
pub fn skill_set_rows(set: &str) -> Vec<String> {
    let every = || SHEET_SKILL_IDS.iter().map(|id| (*id).to_owned()).chain(["skill:craft", "skill:perform", "skill:profession"].map(str::to_owned));
    let named = |skills: &[&str]| skills.iter().map(|skill| format!("skill:{skill}")).collect::<Vec<_>>();
    let knowledge = || every().filter(|row| row.starts_with("skill:knowledge_"));
    match set {
        "all" | "base" => every().collect(),
        "knowledge" => knowledge().collect(),
        "craft" | "perform" | "profession" => named(&[set]),
        "strength" => named(&["climb", "swim"]),
        "dexterity" => named(&["acrobatics", "disable_device", "escape_artist", "fly", "ride", "sleight_of_hand", "stealth"]),
        "intelligence" => named(&["appraise", "craft", "linguistics", "spellcraft"]).into_iter().chain(knowledge()).collect(),
        "wisdom" => named(&["heal", "perception", "profession", "sense_motive", "survival"]),
        "charisma" => named(&["bluff", "diplomacy", "disguise", "handle_animal", "intimidate", "perform", "use_magic_device"]),
        _ => Vec::new(),
    }
}

/// The worn selections' stated bonuses, stacked. A derived record stating none reads its stand-in's.
pub fn worn_item_stated_bonuses(selections: &[EquipmentSelection], corpus: &SourcePackageContent) -> WornItemStatedBonuses {
    let worn: Vec<EquipmentSelection> =
        selections.iter().filter(|selection| selection.active_state == ActiveState::EquippedActive).cloned().collect();
    let mut counted: Vec<WornStatedBonus> = Vec::new();
    for selection in &worn {
        let Some(record) = equipment_converted_resolve(&selection.item_id, corpus) else { continue };
        let stated: &[StatedBonus] = match stand_in_record(record, corpus) {
            Some(base) if record.stated_bonuses.is_empty() => &base.stated_bonuses,
            _ => &record.stated_bonuses,
        };
        for bonus in stated {
            let targets = match bonus.target.strip_prefix("skill_set:") {
                Some(set) => skill_set_rows(set),
                None => vec![bonus.target.clone()],
            };
            for target in targets {
                let row = WornStatedBonus {
                    item_name: record.identity.clone(),
                    target,
                    bonus_type: bonus.bonus_type.clone(),
                    value: bonus.value,
                };
                let same_type = counted.iter().position(|held| {
                    held.value > 0 && held.target == row.target && held.bonus_type == row.bonus_type
                });
                match same_type {
                    Some(index) if !bonus_always_counts(row.value, row.bonus_type.as_deref()) => {
                        if row.value > counted[index].value {
                            counted[index] = row;
                        }
                    }
                    _ => counted.push(row),
                }
            }
        }
    }
    for effect in &compute_equipment_effects(&worn, corpus).per_item {
        let value = effect.armor_class_bonus.unwrap_or(0) + effect.armor_class_enhancement.unwrap_or(0);
        if value != 0 {
            counted.push(WornStatedBonus {
                item_name: effect.equipment_record_key.clone(),
                target: "ac".to_owned(),
                bonus_type: effect.armor_class_bonus_type.clone(),
                value,
            });
        }
    }
    WornItemStatedBonuses(counted)
}

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
    /// The enhancement bonus this selection's own `applied_modifiers` add to its armor or shield
    /// value, already in `EquipmentEffects::armor_class_delta`. `None` when they add none.
    pub armor_class_enhancement: Option<i16>,
    /// The bonus type the two above count as. After stacking a bonus that lost to a higher one of
    /// its type reads `Some(0)` above, so the rows still sum to `armor_class_delta`.
    pub armor_class_bonus_type: Option<String>,
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

impl EquipmentEffects {
    fn armor_class_where(&self, keeps: impl Fn(&str) -> bool) -> i16 {
        self.per_item
            .iter()
            .filter(|item| item.armor_class_bonus_type.as_deref().is_some_and(&keeps))
            .map(|item| item.armor_class_bonus.unwrap_or(0) + item.armor_class_enhancement.unwrap_or(0))
            .sum()
    }

    /// The part of `armor_class_delta` a touch attack ignores.
    pub fn armor_class_not_against_touch(&self) -> i16 {
        self.armor_class_where(|word| NOT_AGAINST_TOUCH.contains(&word))
    }

    /// The part of `armor_class_delta` a flat-footed character loses.
    pub fn armor_class_lost_flat_footed(&self) -> i16 {
        self.armor_class_where(|word| word == "Dodge")
    }
}

/// Two bonuses of one type take the higher: each row that loses reads zero, and the total is the sum.
fn stack_armor_class(per_item: &mut [ResolvedEquipmentEffect]) -> i16 {
    let value = |item: &ResolvedEquipmentEffect| item.armor_class_bonus.unwrap_or(0) + item.armor_class_enhancement.unwrap_or(0);
    for index in 0..per_item.len() {
        let mine = value(&per_item[index]);
        let word = per_item[index].armor_class_bonus_type.clone();
        if bonus_always_counts(mine, word.as_deref()) {
            continue;
        }
        let beaten = per_item.iter().enumerate().any(|(other, item)| {
            item.armor_class_bonus_type == word && (value(item) > mine || (value(item) == mine && other < index))
        });
        if beaten {
            per_item[index].armor_class_bonus = Some(0);
            per_item[index].armor_class_enhancement = None;
        }
    }
    per_item.iter().map(value).sum()
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
            let bonus = attached_weapon_enhancement(converted, &selection.applied_modifiers, corpus).tohit;
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

        let armor_class_enhancement = attached_armor_class_enhancement(converted, &effect, &selection.applied_modifiers, corpus);
        let armor_class_bonus_type = effect.armor_class_bonus.and_then(|_| armor_class_type_of(converted, corpus));
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
            armor_class_enhancement,
            armor_class_bonus_type,
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
    let armor_class_delta = stack_armor_class(&mut per_item);

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
/// against `corpus` and takes the highest of each roll's `equipmods` bonus: every such chain is typed
/// `Enhancement`, and bonuses of one type do not stack (the rule
/// [`equipmods::apply_eqmod_weapon_enhancement_bonus`] applies to a record's own `EQMOD:` list). A
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
pub(crate) fn attached_weapon_enhancement(
    weapon_record: &CorpusEquipmentRecord,
    applied_modifiers: &[String],
    corpus: &SourcePackageContent,
) -> AttachedWeaponEnhancement {
    let weapon_is_natural_attack = is_natural_attack_weapon(weapon_record);
    let records: Vec<&CorpusEquipmentRecord> = applied_modifiers
        .iter()
        .filter_map(|modifier_item_id| equipment_converted_resolve(modifier_item_id, corpus))
        .filter(|record| {
            // `compute_equipmods_effect` already reads the record's own settled enhancement and
            // returns `None` if it has none -- that alone is the precise "is this an equipmods
            // enhancement" test, and it holds for a modifier of any book.
            equipmods::compute_equipmods_effect(record)
                .is_some_and(|bonus| !bonus.natural_attack_only || weapon_is_natural_attack)
        })
        .collect();
    let mut highest = None;
    equipmods::apply_eqmod_weapon_enhancement_bonus(&mut highest, &records);
    highest.map_or_else(AttachedWeaponEnhancement::default, |bonus: WeaponEnhancementBonus| AttachedWeaponEnhancement {
        tohit: bonus.tohit_bonus.unwrap_or(0),
        damage: bonus.damage_bonus.unwrap_or(0),
    })
}

/// What one weapon selection's own `applied_modifiers` add to its two rolls.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct AttachedWeaponEnhancement {
    pub tohit: i16,
    pub damage: i16,
}

/// What one armor or shield selection's own `applied_modifiers` add to its AC value: the highest
/// (one type, no stacking), and only from a modifier of the host's family, which is what each
/// record's own `ItemHas` rule says (`Special Ability ~ +1 ~ Armor` on armor, `~ Shield` on a
/// shield). A host that states no AC value is neither, so a modifier on a weapon adds none.
fn attached_armor_class_enhancement(
    host: &CorpusEquipmentRecord,
    host_effect: &EquipmentStatEffect,
    applied_modifiers: &[String],
    corpus: &SourcePackageContent,
) -> Option<i16> {
    host_effect.armor_class_bonus?;
    let host_is_shield = host.is_shield || stand_in_record(host, corpus).is_some_and(|base| base.is_shield);
    applied_modifiers
        .iter()
        .filter_map(|modifier_item_id| equipment_converted_resolve(modifier_item_id, corpus))
        .filter(|modifier| modifier.is_shield == host_is_shield)
        .filter_map(|modifier| modifier.armor_class_chain_bonus)
        .max()
}

/// The type the record's Armor Class counts as: its own, else its stand-in's, else that of a
/// modifier the record names, which is where the number came from.
fn armor_class_type_of(record: &CorpusEquipmentRecord, corpus: &SourcePackageContent) -> Option<String> {
    record
        .armor_class_bonus_type
        .clone()
        .or_else(|| stand_in_record(record, corpus).and_then(|base| base.armor_class_bonus_type.clone()))
        .or_else(|| {
            eqmod_referenced_converted_records(record, corpus)
                .iter()
                .find(|modifier| modifier.armor_class_chain_bonus.is_some())
                .and_then(|modifier| modifier.armor_class_bonus_type.clone())
        })
}

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
    if let Some(base) = stand_in_record(record, corpus) {
        let stated = base.stat_effect;
        effect.armor_class_bonus = effect.armor_class_bonus.or(stated.armor_class_bonus);
        effect.max_dex = effect.max_dex.or(stated.max_dex);
        effect.spell_failure = effect.spell_failure.or(stated.spell_failure);
        effect.armor_check_penalty = effect.armor_check_penalty.or(stated.armor_check_penalty);
    }
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

