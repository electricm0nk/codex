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
//! `carrying_capacity_thresholds_match_every_row_of_the_real_pcgen_load_lst_table`.

use crate::rules_core::character_input::{ActiveState, EquipmentSelection};
use crate::rules_core::equipment_resolver::{equipment_id_resolve, equipment_key_token};
use crate::rules_core::rules_tables::crb::equipment_tables::equipment_tables;
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::source_content::SourcePackageContent;

/// Max light/medium/heavy load in pounds for one Strength score, per PF1's
/// Table: Carrying Capacity (see module doc comment for the cited source).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CarryingCapacityThresholds {
    pub light_max_lbs: f64,
    pub medium_max_lbs: f64,
    pub heavy_max_lbs: f64,
}

/// Row `i` (0-indexed) is Strength score `i + 1`'s (light, medium, heavy)
/// maximum load in pounds, verbatim from the cited source table (Strength
/// 1-29). See module doc comment for the citation.
const CARRYING_CAPACITY_TABLE: [(f64, f64, f64); 29] = [
    (3.0, 6.0, 10.0),
    (6.0, 13.0, 20.0),
    (10.0, 20.0, 30.0),
    (13.0, 26.0, 40.0),
    (16.0, 33.0, 50.0),
    (20.0, 40.0, 60.0),
    (23.0, 46.0, 70.0),
    (26.0, 53.0, 80.0),
    (30.0, 60.0, 90.0),
    (33.0, 66.0, 100.0),
    (38.0, 76.0, 115.0),
    (43.0, 86.0, 130.0),
    (50.0, 100.0, 150.0),
    (58.0, 116.0, 175.0),
    // Str 15. The medium maximum here read 134.0 until a row-complete
    // cross-check against PCGen's `load.lst` caught it: `LOAD:15|200` with
    // `ENCUMBRANCE:Medium|2/3` derives 133, and the table's own doubling
    // structure agrees (Str 5 -> 33, Str 10 -> 66, Str 20 -> 266). 134 is
    // the *heavy* tier's lower bound in the printed prose table, one row
    // over from the medium maximum this array holds.
    (66.0, 133.0, 200.0),
    (76.0, 153.0, 230.0),
    (86.0, 173.0, 260.0),
    (100.0, 200.0, 300.0),
    (116.0, 233.0, 350.0),
    (133.0, 266.0, 400.0),
    (153.0, 306.0, 460.0),
    (173.0, 346.0, 520.0),
    (200.0, 400.0, 600.0),
    (233.0, 466.0, 700.0),
    (266.0, 533.0, 800.0),
    (306.0, 613.0, 920.0),
    (346.0, 693.0, 1040.0),
    (400.0, 800.0, 1200.0),
    (466.0, 933.0, 1400.0),
];

/// PF1 Table: Carrying Capacity, Strength score -> max light/medium/heavy
/// load. Strength scores below 1 are defensively floored to the Strength-1
/// row (PF1 does not define a player character's carrying capacity below
/// 1). Strength scores above 29 extrapolate via the source table's own
/// stated rule: find the 20-29 row sharing the same "ones" digit and
/// multiply its three values by 4 for every 10 points above that row's
/// Strength.
pub fn carrying_capacity_thresholds(strength_score: i16) -> CarryingCapacityThresholds {
    let clamped = strength_score.max(1);
    let (light, medium, heavy) = if clamped <= 29 {
        CARRYING_CAPACITY_TABLE[(clamped - 1) as usize]
    } else {
        let ones_digit = ((clamped - 20) % 10 + 10) % 10;
        let base_strength = 20 + ones_digit;
        let (base_light, base_medium, base_heavy) =
            CARRYING_CAPACITY_TABLE[(base_strength - 1) as usize];
        let tens_above = f64::from((clamped - base_strength) / 10);
        let multiplier = 4.0_f64.powf(tens_above);
        (base_light * multiplier, base_medium * multiplier, base_heavy * multiplier)
    };
    CarryingCapacityThresholds { light_max_lbs: light, medium_max_lbs: medium, heavy_max_lbs: heavy }
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

/// **Known limitation -- capacity is computed at Medium size.** PF1 scales
/// carrying capacity by creature size, and `load.lst` carries the real
/// multipliers (`SIZEMULT:S|0.75`, `L|2`, `H|4`, ... `SIZEMULT:F|0.125`).
/// They are not applied here, because creature size is not modelled
/// anywhere in this crate: there is no size field on `CharacterInput`, no
/// size enum in `rules_core`, and this repo's ingested `cr_races.lst`
/// contains only `.MOD` records carrying `SOURCEPAGE:` -- no `SIZE:` token
/// to resolve one from.
///
/// The practical consequence, stated plainly rather than buried: for a
/// Small character (Gnome and Halfling, both curated playable races) the
/// thresholds returned here are 4/3 of the true values, since PF1 gives
/// Small creatures 3/4 of a Medium creature's capacity. Every Medium race
/// is correct. Closing this needs a size model first -- inventing a
/// race-to-size mapping inside the encumbrance module would put a second,
/// unowned source of truth for creature size into the codebase.
pub fn compute_encumbrance(
    equipment_selections: &[EquipmentSelection],
    corpus: &SourcePackageContent,
    strength_score: i16,
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

    let thresholds = carrying_capacity_thresholds(strength_score);
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
            carrying_capacity_thresholds(10),
            CarryingCapacityThresholds { light_max_lbs: 33.0, medium_max_lbs: 66.0, heavy_max_lbs: 100.0 }
        );
        assert_eq!(
            carrying_capacity_thresholds(18),
            CarryingCapacityThresholds { light_max_lbs: 100.0, medium_max_lbs: 200.0, heavy_max_lbs: 300.0 }
        );
        assert_eq!(
            carrying_capacity_thresholds(29),
            CarryingCapacityThresholds { light_max_lbs: 466.0, medium_max_lbs: 933.0, heavy_max_lbs: 1400.0 }
        );
    }

    /// Strength 30 = Strength 20's row (same ones digit, one tier of +10
    /// above) multiplied by 4 -- the source table's own extrapolation rule.
    #[test]
    fn carrying_capacity_thresholds_extrapolate_beyond_strength_29() {
        assert_eq!(
            carrying_capacity_thresholds(30),
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

        let computation = compute_encumbrance(&equipment_selections, &corpus, 10);

        assert_eq!(computation.total_carried_weight_lbs, 15.0 + 5.0 + 4.0);
        assert!(computation.unresolved_item_ids.is_empty(), "{:?}", computation.unresolved_item_ids);
        assert_eq!(computation.per_item.len(), 3);
        assert_eq!(computation.thresholds, carrying_capacity_thresholds(10));
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

        let computation = compute_encumbrance(&equipment_selections, &corpus, 10);

        assert_eq!(computation.total_carried_weight_lbs, 15.0, "Absent items must not contribute weight");
        assert_eq!(computation.unresolved_item_ids, vec!["item:not_a_real_item".to_owned()]);
    }

    #[test]
    fn compute_encumbrance_classifies_medium_and_heavy_loads() {
        let corpus = corpus_from(FIXTURE_TEXT);
        // Strength 1: light max 3, medium max 6, heavy max 10. Leather Armor
        // alone (15 lbs) exceeds even the heavy max.
        let equipment_selections = vec![selection("item:leather_armor", ActiveState::EquippedActive)];

        let computation = compute_encumbrance(&equipment_selections, &corpus, 1);

        assert_eq!(computation.level, EncumbranceLevel::OverHeavyCapacity);
    }
}
