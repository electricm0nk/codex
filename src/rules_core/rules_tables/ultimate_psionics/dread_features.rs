//! SD-32 card 11 (T12) — real per-feature compute functions for the
//! Dread (`untabled_base_class_chassis`), the third magnitude-bearing
//! `untabled_base_class_feature_roster` group worked end-to-end as a class
//! (`antipaladin_features.rs`/`cryptic_features.rs` are the first two;
//! `decisions.md §17`/`§27b` — novelty of shape is grounds for sizing, not
//! exclusion).
//!
//! Every formula below is transcribed from the corpus's own already-
//! ingested `BONUS:VAR` tokens (`data/corpus/ultimate_psionics/
//! class_feature/dread/*.json`, each record's own `raw_tokens`, sourced
//! from `up_abilities_class.lst` — the roster's own `source_file` for every
//! Dread record). `DreadPrimeStat` is `CHA` (`up_classes.lst:115`,
//! `BONUS:VAR|DreadPrimeStat|CHA`), threaded here as `charisma_modifier`.

/// `up_abilities_class.lst:266`, `Devastating Touch`:
/// `BONUS:VAR|DevastatingTouchBonusDamage|DevastatingTouchLVL`, and
/// `DevastatingTouchLVL = DreadLVL` (level 266 two lines below) — the bonus
/// damage on the melee touch attack equals class level (added to the flat
/// 1d6 base, which is not level-scaled). `None` below level 1 (the
/// roster's own `min_level` for this key).
pub fn devastating_touch_bonus_damage(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// `up_abilities_class.lst:265`, `Fearsome Insight`:
/// `BONUS:VAR|FearsomeInsightBonus|max(1,floor(DreadLVL/2))` — an insight
/// bonus to Intimidate. `None` below level 1.
pub fn fearsome_insight_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some((i16::from(level) / 2).max(1))
}

/// `up_abilities_class.lst:267`, `Terror`:
/// `BONUS:VAR|TerrorTimes|TerrorLVL+DreadPrimeStat`, `TerrorLVL = DreadLVL`
/// — uses per day of a terror. `None` below level 2 (the roster's own
/// `min_level` for this key).
pub fn terror_uses_per_day(level: u8, charisma_modifier: i16) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some(i16::from(level) + charisma_modifier)
}

/// `up_abilities_class.lst:268`, `Aura of Fear`:
/// `BONUS:VAR|AuraOfFearPenalty|-4` — a flat penalty on saves against
/// fear for nearby enemies, not level-scaled. `None` below level 3 (the
/// roster's own `min_level` for this key).
pub fn aura_of_fear_penalty(level: u8) -> Option<i16> {
    if level < 3 {
        return None;
    }
    Some(-4)
}

/// `up_abilities_class.lst:272`, `Shadow Twin`:
/// `BONUS:VAR|ShadowTwinTimes|DreadPrimeStat` — uses per day, Charisma
/// modifier only (no level term, unlike `terror_uses_per_day`). `None`
/// below level 11 (the roster's own `min_level` for this key).
pub fn shadow_twin_uses_per_day(level: u8, charisma_modifier: i16) -> Option<i16> {
    if level < 11 {
        return None;
    }
    Some(charisma_modifier)
}

/// `up_abilities_class.lst:274`, `Fear Incarnate`: `DR:10/psionic` — a flat
/// capstone magnitude, no `BONUS:VAR` token (same shape as Antipaladin's
/// `aura_of_depravity_damage_reduction`). `None` below level 20 (the
/// roster's own `min_level` for this key, and PF1's own capstone level).
pub fn fear_incarnate_damage_reduction(level: u8) -> Option<i16> {
    if level < 20 {
        return None;
    }
    Some(10)
}

// --- SD-32 card 11 (T12) follow-up: `Dread Manifesting`'s three
// magnitudes, same shape-3 grant convention as Cryptic (identical ladder
// and `PowersKnown`/`MaxPowerLevel` shape) except `DreadPrimeStat`/
// `DreadPLStatScore` read Charisma, not Intelligence.

pub fn dread_power_points_total(level: u8, cha_mod: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    let base: i16 = match level {
        1 => 1,
        2 => 1,
        3..=5 => 2,
        6..=10 => 4,
        11..=15 => 8,
        _ => 12, // 16..=20
    };
    Some(base + (cha_mod * i16::from(level)) / 2)
}

pub fn dread_powers_known(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

pub fn dread_max_power_level(level: u8, cha_score: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    let mpl = i16::from(level);
    Some(((mpl + 2) / 3).min(6).min(cha_score - 10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn devastating_touch_bonus_damage_equals_class_level() {
        assert_eq!(devastating_touch_bonus_damage(1), Some(1));
        assert_eq!(devastating_touch_bonus_damage(20), Some(20));
        assert_eq!(devastating_touch_bonus_damage(0), None);
    }

    #[test]
    fn fearsome_insight_floors_at_one() {
        assert_eq!(fearsome_insight_bonus(1), Some(1));
        assert_eq!(fearsome_insight_bonus(2), Some(1));
        assert_eq!(fearsome_insight_bonus(20), Some(10));
    }

    #[test]
    fn terror_uses_combine_level_and_charisma() {
        assert_eq!(terror_uses_per_day(2, 3), Some(5));
        assert_eq!(terror_uses_per_day(1, 3), None);
    }

    #[test]
    fn aura_of_fear_is_a_flat_negative_four() {
        assert_eq!(aura_of_fear_penalty(3), Some(-4));
        assert_eq!(aura_of_fear_penalty(2), None);
    }

    #[test]
    fn shadow_twin_uses_ignore_level_and_use_charisma_only() {
        assert_eq!(shadow_twin_uses_per_day(11, 4), Some(4));
        assert_eq!(shadow_twin_uses_per_day(20, 4), Some(4));
        assert_eq!(shadow_twin_uses_per_day(10, 4), None);
    }

    #[test]
    fn fear_incarnate_is_a_flat_dr_ten_at_the_capstone() {
        assert_eq!(fear_incarnate_damage_reduction(20), Some(10));
        assert_eq!(fear_incarnate_damage_reduction(19), None);
    }

    #[test]
    fn dread_power_points_total_uses_the_base_ladder_and_cha_bonus() {
        assert_eq!(dread_power_points_total(1, 0), Some(1));
        assert_eq!(dread_power_points_total(20, 0), Some(12));
        assert_eq!(dread_power_points_total(5, 3), Some(2 + (3 * 5) / 2));
        assert_eq!(dread_power_points_total(0, 0), None);
    }

    #[test]
    fn dread_powers_known_equals_class_level() {
        assert_eq!(dread_powers_known(1), Some(1));
        assert_eq!(dread_powers_known(20), Some(20));
        assert_eq!(dread_powers_known(0), None);
    }

    #[test]
    fn dread_max_power_level_is_capped_by_the_lowest_of_three_terms() {
        assert_eq!(dread_max_power_level(1, 10), Some(0));
        assert_eq!(dread_max_power_level(20, 20), Some(6));
        assert_eq!(dread_max_power_level(0, 20), None);
    }
}
