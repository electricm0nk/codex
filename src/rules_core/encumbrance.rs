//! Carrying capacity / encumbrance (v0.6 alpha swarm, task 5).
//!
//! Before this file, carrying capacity and encumbrance were a complete gap
//! (`character_input.rs`'s own `ActiveState` doc comment: "no equipment
//! effect, encumbrance, or inventory behavior is computed here"; confirmed
//! by grep across `src/rules_core/` -- every prior "encumbrance"/"carrying
//! capacity" hit was a doc-comment disclaimer or corpus flavor text, never
//! computation). `equipment_effects.rs`'s per-item stats (AC bonus, max
//! Dex, spell failure) were already real and wired before this task; this
//! file adds the missing weight/capacity pillar alongside it, following the
//! same corpus-resolution pattern (`equipment_id_resolve` +
//! `rules_tables::crb::equipment_tables()` lookup by key).
//!
//! PF1's Table: Carrying Capacity (Strength score -> light/medium/heavy
//! load maximum weight in pounds) is not present in this repo's *ingested*
//! corpus (`data/pathfinder/.../core_rulebook/*.lst`) -- it is
//! core-rulebook prose/table content, not an equipment token stream.
//!
//! It *is*, however, available in machine-readable form in the same PCGen
//! checkout the ingested corpus comes from, as game-mode system data:
//! `/home/ubuntu/workspace/repos/pcgen/system/gameModes/Pathfinder/load.lst`.
//! That file is the authoritative source for this module:
//!
//!  - `LOAD:<Strength>|<value>` (lines 10-38) gives the *heavy* maximum for
//!    Strength 0-29.
//!  - `ENCUMBRANCE:Light|1/3`, `Medium|2/3`, `Heavy|1` give the light and
//!    medium maxima as fractions of that same value (truncated to whole
//!    pounds).
//!  - `SIZEMULT:<code>|<value>` (lines 1-8) scales the load value by
//!    creature size. Transcribed in `rules_core::size::SizeCategory::
//!    load_capacity_ratio`, which owns that half of the file.
//!  - `LOADMULT:4` gives the above-29 extrapolation: multiply the row with
//!    the same ones digit by 4 for every 10 points of Strength above it.
//!
//! The table below was originally hand-transcribed from Archives of
//! Nethys's SRD mirror (<https://www.aonprd.com/Rules.aspx?ID=118>) before
//! `load.lst` was identified. That transcription carried one real error --
//! Str 15's medium maximum read 134 (the *heavy* tier's printed lower
//! bound) instead of 133 -- which a three-row spot check missed. Every row
//! is now asserted against the `load.lst`-derived values by
//! `tests/v06_encumbrance.rs`'s
//! `carrying_capacity_thresholds_match_every_row_of_the_real_pcgen_load_lst_table`,
//! and the light/medium columns are no longer transcribed at all: they are
//! derived from the one `LOAD:` column, so the error class that produced
//! the Str-15 bug cannot recur in them.
//!
//! # Creature size
//!
//! Capacity is scaled by creature size (`SIZEMULT:`). This module does not
//! decide what size a character is -- `rules_core::size` owns the size
//! type and `rules_tables::crb::race_tables::race_size` owns the
//! race-to-size fact, read from each race record's own
//! `FACT:BaseSize|<code>` token. Callers pass a `SizeCategory` in.

use crate::rules_core::character_input::{ActiveState, EquipmentSelection};
use crate::rules_core::equipment_resolver::{equipment_id_resolve, equipment_key_token};
use crate::rules_core::rules_tables::crb::equipment_tables::equipment_tables;
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::size::SizeCategory;
use crate::rules_core::source_content::SourcePackageContent;

/// Max light/medium/heavy load in pounds for one Strength score, per PF1's
/// Table: Carrying Capacity (see module doc comment for the cited source).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CarryingCapacityThresholds {
    pub light_max_lbs: f64,
    pub medium_max_lbs: f64,
    pub heavy_max_lbs: f64,
}

/// Row `i` (0-indexed) is Strength score `i + 1`'s **heavy** maximum load
/// in pounds -- `load.lst`'s `LOAD:<Strength>|<value>` column, lines 10-38,
/// transcribed verbatim and nothing else.
///
/// The light and medium tiers are deliberately *not* stored. They are
/// derived from this one column via `load.lst`'s own
/// `ENCUMBRANCE:Light|1/3` and `ENCUMBRANCE:Medium|2/3` fractions, so the
/// module hand-copies exactly one column instead of three. That is not a
/// tidying preference: this table previously stored all three columns and
/// the Strength-15 medium value was wrong (134, where the source derives
/// 133) for exactly as long as it took a row-complete cross-check to
/// notice. Two of the three columns were redundant *and* were the ones
/// carrying the error.
///
/// Deriving is also what makes size scaling correct at all -- see
/// `carrying_capacity_thresholds`.
const PCGEN_LOAD_LST_HEAVY_BY_STRENGTH: [i64; 29] = [
    10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 115, 130, 150, 175, 200, 230, 260, 300, 350, 400, 460,
    520, 600, 700, 800, 920, 1040, 1200, 1400,
];

/// Splits one heavy-tier load value into the three tier maxima for a given
/// creature size, in exact integer arithmetic.
///
/// The order of operations is the whole point, and it is PCGen's:
/// `LoadFacet.getMaxLoad` computes `loadValue * mult * getLoadMultForSize(id)`
/// and `CharacterDisplay.getLoadToken` truncates the result exactly once
/// (`getMaxLoad(mult).intValue()`). So size scales the *load value*, and
/// truncation to whole pounds happens last -- per tier.
///
/// Applying size to already-truncated tier values instead would be wrong,
/// and visibly so: a Small Strength-10 character's light maximum is
/// `(100 * 3/4) / 3 = 25`, but scaling Medium's stored `33` gives
/// `24.75 -> 24`. PF1's published Small column says 25.
///
/// Kept in `i64` rather than `f64` because the truncation is load-bearing:
/// `300.0 * (2.0 / 3.0)` is `199.999...` in IEEE-754 and truncates to 199,
/// where the correct answer is 200. Integer arithmetic has no such edge.
fn tiers_for_heavy_load(heavy_load: i64, size: SizeCategory) -> CarryingCapacityThresholds {
    let (numerator, denominator) = size.load_capacity_ratio();
    let scaled = heavy_load * numerator;
    CarryingCapacityThresholds {
        // ENCUMBRANCE:Light|1/3
        light_max_lbs: (scaled / (denominator * 3)) as f64,
        // ENCUMBRANCE:Medium|2/3
        medium_max_lbs: (scaled * 2 / (denominator * 3)) as f64,
        // ENCUMBRANCE:Heavy|1
        heavy_max_lbs: (scaled / denominator) as f64,
    }
}

/// PF1 Table: Carrying Capacity, Strength score + creature size -> max
/// light/medium/heavy load.
///
/// Size is a required argument rather than an option with a Medium
/// default, on purpose. A defaulted size is what produced the bug this
/// signature replaces: every caller silently got Medium, and Gnome and
/// Halfling characters were handed `4/3` of their true capacity along with
/// the wrong load tier, max-Dex cap and armor check penalty. Making the
/// parameter explicit means a caller cannot get a Medium answer without
/// having said "Medium".
///
/// Strength scores below 1 are defensively floored to the Strength-1 row
/// (PF1 does not define a player character's carrying capacity below 1).
/// Strength scores above 29 extrapolate via the source table's own stated
/// rule: find the 20-29 row sharing the same "ones" digit and multiply by
/// 4 for every 10 points above that row's Strength (`load.lst`'s
/// `LOADMULT:4`).
pub fn carrying_capacity_thresholds(
    strength_score: i16,
    size: SizeCategory,
) -> CarryingCapacityThresholds {
    let clamped = strength_score.max(1);
    if clamped <= 29 {
        return tiers_for_heavy_load(PCGEN_LOAD_LST_HEAVY_BY_STRENGTH[(clamped - 1) as usize], size);
    }

    // Above the table's Strength-29 ceiling the three tier maxima are
    // scaled together, preserving this function's long-standing
    // extrapolation behaviour (multiply the derived thresholds, rather
    // than re-deriving tiers from a scaled load value -- the two differ by
    // a pound or two once truncation compounds, and changing which one
    // ships is not this task's to decide).
    let ones_digit = ((clamped - 20) % 10 + 10) % 10;
    let base_strength = 20 + ones_digit;
    let base = tiers_for_heavy_load(
        PCGEN_LOAD_LST_HEAVY_BY_STRENGTH[(base_strength - 1) as usize],
        size,
    );
    let tens_above = f64::from((clamped - base_strength) / 10);
    let multiplier = 4.0_f64.powf(tens_above);
    CarryingCapacityThresholds {
        light_max_lbs: base.light_max_lbs * multiplier,
        medium_max_lbs: base.medium_max_lbs * multiplier,
        heavy_max_lbs: base.heavy_max_lbs * multiplier,
    }
}

/// PF1's three named load tiers, plus a fourth honest state for weight
/// beyond even the heavy maximum (PF1: a character carrying more than its
/// heavy load "can't move" -- this crate does not model movement, so this
/// variant only names the state rather than computing any further effect).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncumbranceLevel {
    Light,
    Medium,
    Heavy,
    OverHeavyCapacity,
}

impl EncumbranceLevel {
    /// The maximum Dexterity bonus to AC this *load tier* allows, separate
    /// from any cap a worn armor imposes. `None` for a light load, which
    /// imposes no cap of its own.
    ///
    /// Grounded in the real PCGen engine's own implementation rather than
    /// reconstructed from memory: `PlayerCharacter.java:5362-5368`
    /// (`case MEDIUM -> 3; case HEAVY -> 1; case OVERLOAD -> 0;`, with the
    /// `default` branch -- Light -- applying no load cap).
    ///
    /// A caller combining this with armor's own `MAXDEX` must take the
    /// *lower* of the two, which is what PCGen's own loop does
    /// (`PlayerCharacter.java:5374-5385`): the load cap and each equipped
    /// item's cap both constrain, so the tightest wins.
    pub fn max_dex_cap(self) -> Option<i16> {
        match self {
            EncumbranceLevel::Light => None,
            EncumbranceLevel::Medium => Some(3),
            EncumbranceLevel::Heavy => Some(1),
            EncumbranceLevel::OverHeavyCapacity => Some(0),
        }
    }

    /// The armor check penalty this *load tier* imposes, separate from any
    /// penalty a worn armor imposes. `0` for a light load. Always
    /// non-positive.
    ///
    /// Grounded in `PlayerCharacter.java:5331`
    /// (`(load == Load.MEDIUM) ? -3 : (load == Load.HEAVY) ? -6 : 0`), which
    /// matches the third field of `load.lst`'s own `ENCUMBRANCE:` rows
    /// (`Light|1/3||0`, `Medium|2/3||-3`, `Heavy|1||-6`).
    ///
    /// `OverHeavyCapacity` is this crate's own deliberate choice, NOT a
    /// transcription -- flagged explicitly so nobody reads it as sourced.
    /// PCGen's ternary has no `OVERLOAD` branch and would fall through to
    /// `0`, which is plainly not "less penalising than a heavy load"; it
    /// reads as moot rather than intended, because PF1's actual rule for
    /// exceeding the heavy maximum is that the character *cannot move at
    /// all*. PCGen does model `OVERLOAD` as a real distinct state
    /// elsewhere (`case OVERLOAD -> 0` for the max-Dex cap). Reusing the
    /// heavy penalty is the conservative reading: an overloaded character
    /// is at least as hampered as a heavily loaded one. The full PF1
    /// consequence (no movement) is not modelled here -- this crate does
    /// not model movement; see `EncumbranceLevel`'s own doc comment.
    ///
    /// Note a real PF1 subtlety this does *not* fold in: the load penalty
    /// and equipped armor's penalty do not sum. PCGen takes the more
    /// punishing of the two (`bonus = Math.min(bonus, penaltyForLoad)`,
    /// `PlayerCharacter.java:5344`). Combining is the caller's job, so this
    /// method reports only the load's own contribution.
    pub fn armor_check_penalty(self) -> i16 {
        match self {
            EncumbranceLevel::Light => 0,
            EncumbranceLevel::Medium => -3,
            EncumbranceLevel::Heavy | EncumbranceLevel::OverHeavyCapacity => -6,
        }
    }
}

fn classify_encumbrance(total_weight_lbs: f64, thresholds: &CarryingCapacityThresholds) -> EncumbranceLevel {
    if total_weight_lbs <= thresholds.light_max_lbs {
        EncumbranceLevel::Light
    } else if total_weight_lbs <= thresholds.medium_max_lbs {
        EncumbranceLevel::Medium
    } else if total_weight_lbs <= thresholds.heavy_max_lbs {
        EncumbranceLevel::Heavy
    } else {
        EncumbranceLevel::OverHeavyCapacity
    }
}

/// One resolved carried item's contribution to the loadout's total weight
/// and total gp value. Both are read from the same `equipment_tables()`
/// entry, so cost costs no second corpus resolution.
#[derive(Debug, Clone, PartialEq)]
pub struct CarriedItem {
    pub item_id: String,
    pub weight_lbs: f64,
    /// The record's real corpus `COST:` token in gold pieces. `None` is a
    /// genuine corpus absence, never a fabricated zero -- a `(Base)`
    /// template record with no independent price, or an equipment modifier
    /// whose cost is a formula over the base item rather than a fixed
    /// number (see `EquipmentTableEntry::cost_gp`). An item priced `None`
    /// still contributes its weight; it simply cannot contribute to
    /// `EncumbranceComputation::total_carried_cost_gp`.
    pub cost_gp: Option<f64>,
}

/// Real, corpus-grounded carrying-capacity/encumbrance computation for a
/// character's full carried loadout (v0.6 alpha swarm, task 5).
///
/// Weight is summed across every selection with `ActiveState::EquippedActive`
/// or `ActiveState::SelectedInactive` (both represent items the character
/// actually possesses/carries; `Absent` means not carried at all -- mirrors
/// `ActiveState`'s own doc comment). Each selection is resolved to a real
/// corpus record via the same `equipment_id_resolve` + `equipment_tables()`
/// key lookup `equipment_effects::compute_equipment_effects` already
/// establishes, then that record's `weight_lbs` (parsed from the corpus's
/// own `WT:` token, `rules_tables::crb::equipment_tables`) is added to the
/// total. An item that does not resolve to a corpus record, or resolves but
/// carries no `weight_lbs` value, is recorded in `unresolved_item_ids`
/// rather than silently contributing a fabricated zero -- so a caller can
/// tell "this loadout weighs exactly 0 lbs" apart from "this loadout's
/// weight could not be fully verified."
#[derive(Debug, Clone, PartialEq)]
pub struct EncumbranceComputation {
    pub per_item: Vec<CarriedItem>,
    pub total_carried_weight_lbs: f64,
    /// Total gp value of every carried item that carries a real corpus
    /// `COST:` token. Items priced `None` (see `CarriedItem::cost_gp`)
    /// contribute nothing rather than a fabricated zero, so this is a
    /// floor on the loadout's value, not necessarily its full value --
    /// `per_item` retains the per-item detail needed to tell the two
    /// apart.
    pub total_carried_cost_gp: f64,
    pub thresholds: CarryingCapacityThresholds,
    pub level: EncumbranceLevel,
    pub unresolved_item_ids: Vec<String>,
    /// The max-Dex cap imposed by `level` alone (`EncumbranceLevel::
    /// max_dex_cap`). `None` under a light load. Does NOT account for any
    /// worn armor's own cap -- a consumer showing an effective cap must
    /// take the lower of this and `EquipmentEffects.max_dex_cap`.
    pub load_max_dex_cap: Option<i16>,
    /// The armor check penalty imposed by `level` alone
    /// (`EncumbranceLevel::armor_check_penalty`); `0` under a light load.
    /// Does NOT account for worn armor's own penalty, and the two do not
    /// sum -- see `EncumbranceLevel::armor_check_penalty`.
    pub load_armor_check_penalty: i16,
}

/// Carrying capacity is scaled by `size` per PF1's size rules; see
/// `carrying_capacity_thresholds`.
///
/// This closes the "capacity is computed at Medium size" limitation this
/// function's doc comment used to carry. Creature size now has a real
/// owner (`rules_core::size::SizeCategory`, with the race mapping in
/// `rules_tables::crb::race_tables::race_size`), so this module consumes a
/// size rather than assuming one -- which was the specific reason the
/// original implementation stopped short of fixing it.
///
/// # Still deferred, deliberately
///
/// PF1 scales more than capacity by size -- weapon damage dice, AC,
/// attack rolls, CMB/CMD, Stealth and Fly checks. **None of those apply
/// size anywhere in this crate**, and none are touched here. Each needs
/// its own corpus verification and each would change shipped numbers, so
/// they stay named gaps rather than half-applied ones. In particular a
/// Small character's weapon damage dice are still computed at Medium in
/// `resolve_weapon_damage_breakdown`.
pub fn compute_encumbrance(
    equipment_selections: &[EquipmentSelection],
    corpus: &SourcePackageContent,
    strength_score: i16,
    size: SizeCategory,
) -> EncumbranceComputation {
    let mut per_item = Vec::new();
    let mut unresolved_item_ids = Vec::new();
    let mut total_carried_weight_lbs = 0.0_f64;
    let mut total_carried_cost_gp = 0.0_f64;

    for selection in equipment_selections {
        if selection.active_state == ActiveState::Absent {
            continue;
        }

        let Some((record, _table_cell)) =
            equipment_id_resolve(&selection.item_id, RuleSetId::Crb, corpus)
        else {
            unresolved_item_ids.push(selection.item_id.clone());
            continue;
        };
        let key = equipment_key_token(record).unwrap_or(&record.name).to_string();
        let table_entry = equipment_tables().iter().find(|entry| entry.key == key);

        // Weight is what makes an item *carried* for encumbrance purposes,
        // so a record with no `WT:` token is unresolved. Cost is
        // supplementary: a real corpus absence there (a formula-priced
        // modifier, an unpriced `(Base)` template) must not evict an item
        // whose weight is perfectly well known.
        let Some(weight_lbs) = table_entry.and_then(|entry| entry.weight_lbs) else {
            unresolved_item_ids.push(selection.item_id.clone());
            continue;
        };
        let cost_gp = table_entry.and_then(|entry| entry.cost_gp);

        total_carried_weight_lbs += weight_lbs;
        total_carried_cost_gp += cost_gp.unwrap_or(0.0);
        per_item.push(CarriedItem { item_id: selection.item_id.clone(), weight_lbs, cost_gp });
    }

    let thresholds = carrying_capacity_thresholds(strength_score, size);
    let level = classify_encumbrance(total_carried_weight_lbs, &thresholds);

    EncumbranceComputation {
        per_item,
        total_carried_weight_lbs,
        total_carried_cost_gp,
        thresholds,
        level,
        unresolved_item_ids,
        load_max_dex_cap: level.max_dex_cap(),
        load_armor_check_penalty: level.armor_check_penalty(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcgen_import::ir_converter::convert_equipment_record;
    use crate::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
    use crate::rules_core::source_content::SourceRef;

    /// Real verbatim rows mirroring `tests/sd20_equipment_arms_armor.rs`'s
    /// own fixture (same three CRB records, same `WT:` values) -- kept
    /// in-module rather than shared so this file has no dependency on
    /// `tests/**`, which this task does not own.
    const FIXTURE_TEXT: &str = "\
Leather Armor\tKEY:Leather Armor (Base)\tTYPE:Armor.Light\tCOST:10\tWT:15\tACCHECK:0\tMAXDEX:6\tSPELLFAILURE:10\tBONUS:COMBAT|AC|2|TYPE=Armor|PREVAREQ:DisableArmorBonus,0
Buckler\tKEY:Buckler (Base)\tTYPE:Shield.Buckler\tCOST:5\tWT:5\tACCHECK:-1\tSPELLFAILURE:5\tBONUS:COMBAT|AC|1|TYPE=Shield|PREVAREQ:DisableShieldBonus,0
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8
";

    fn corpus_from(text: &str) -> SourcePackageContent<'static> {
        let result = parse_equipment_entries("cr_equip_arms_armor.lst", text);
        assert!(result.diagnostics.is_empty(), "fixture text must parse cleanly: {:?}", result.diagnostics);
        let source_ref = SourceRef { lst_file: "cr_equip_arms_armor.lst".to_string(), line: 1 };
        let mut corpus = SourcePackageContent::empty("core_rulebook", source_ref);
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }
        corpus
    }

    fn selection(item_id: &str, state: ActiveState) -> EquipmentSelection {
        EquipmentSelection {
            item_id: item_id.to_owned(),
            equipped_or_active: state == ActiveState::EquippedActive,
            active_state: state,
            applied_modifiers: Vec::new(),
        }
    }

    #[test]
    fn carrying_capacity_thresholds_match_the_cited_table_at_known_rows() {
        assert_eq!(
            carrying_capacity_thresholds(10, SizeCategory::Medium),
            CarryingCapacityThresholds { light_max_lbs: 33.0, medium_max_lbs: 66.0, heavy_max_lbs: 100.0 }
        );
        assert_eq!(
            carrying_capacity_thresholds(18, SizeCategory::Medium),
            CarryingCapacityThresholds { light_max_lbs: 100.0, medium_max_lbs: 200.0, heavy_max_lbs: 300.0 }
        );
        assert_eq!(
            carrying_capacity_thresholds(29, SizeCategory::Medium),
            CarryingCapacityThresholds { light_max_lbs: 466.0, medium_max_lbs: 933.0, heavy_max_lbs: 1400.0 }
        );
    }

    /// Strength 30 = Strength 20's row (same ones digit, one tier of +10
    /// above) multiplied by 4 -- the source table's own extrapolation rule.
    #[test]
    fn carrying_capacity_thresholds_extrapolate_beyond_strength_29() {
        assert_eq!(
            carrying_capacity_thresholds(30, SizeCategory::Medium),
            CarryingCapacityThresholds { light_max_lbs: 133.0 * 4.0, medium_max_lbs: 266.0 * 4.0, heavy_max_lbs: 400.0 * 4.0 }
        );
    }

    #[test]
    fn compute_encumbrance_sums_real_corpus_weight_for_carried_items() {
        let corpus = corpus_from(FIXTURE_TEXT);
        let equipment_selections = vec![
            selection("item:leather_armor", ActiveState::EquippedActive),
            selection("item:buckler", ActiveState::SelectedInactive),
            selection("item:longsword", ActiveState::EquippedActive),
        ];

        let computation = compute_encumbrance(&equipment_selections, &corpus, 10, SizeCategory::Medium);

        assert_eq!(computation.total_carried_weight_lbs, 15.0 + 5.0 + 4.0);
        assert!(computation.unresolved_item_ids.is_empty(), "{:?}", computation.unresolved_item_ids);
        assert_eq!(computation.per_item.len(), 3);
        assert_eq!(computation.thresholds, carrying_capacity_thresholds(10, SizeCategory::Medium));
        // 24 lbs total is within Strength 10's light max (33 lbs).
        assert_eq!(computation.level, EncumbranceLevel::Light);
    }

    #[test]
    fn compute_encumbrance_excludes_absent_selections_and_flags_unresolvable_ones() {
        let corpus = corpus_from(FIXTURE_TEXT);
        let equipment_selections = vec![
            selection("item:leather_armor", ActiveState::EquippedActive),
            selection("item:buckler", ActiveState::Absent),
            selection("item:not_a_real_item", ActiveState::EquippedActive),
        ];

        let computation = compute_encumbrance(&equipment_selections, &corpus, 10, SizeCategory::Medium);

        assert_eq!(computation.total_carried_weight_lbs, 15.0, "Absent items must not contribute weight");
        assert_eq!(computation.unresolved_item_ids, vec!["item:not_a_real_item".to_owned()]);
    }

    #[test]
    fn compute_encumbrance_classifies_medium_and_heavy_loads() {
        let corpus = corpus_from(FIXTURE_TEXT);
        // Strength 1: light max 3, medium max 6, heavy max 10. Leather Armor
        // alone (15 lbs) exceeds even the heavy max.
        let equipment_selections = vec![selection("item:leather_armor", ActiveState::EquippedActive)];

        let computation = compute_encumbrance(&equipment_selections, &corpus, 1, SizeCategory::Medium);

        assert_eq!(computation.level, EncumbranceLevel::OverHeavyCapacity);
    }
}
