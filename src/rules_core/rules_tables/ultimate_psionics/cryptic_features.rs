//! SD-32 card 11 (T12) — real per-feature compute functions for the
//! Cryptic (`untabled_base_class_chassis`), the second magnitude-bearing
//! `untabled_base_class_feature_roster` group worked end-to-end as a class
//! (`antipaladin_features.rs` is the first; `decisions.md §17`/`§27b` —
//! novelty of shape is grounds for sizing, not exclusion).
//!
//! Every formula below is transcribed from the corpus's own already-
//! ingested `BONUS:VAR` tokens (`data/corpus/ultimate_psionics/
//! class_feature/cryptic/*.json`, each record's own `raw_tokens`, sourced
//! from `up_abilities_class.lst` — the roster's own `source_file` for every
//! Cryptic record), not from memory of the printed rulebook text. None of
//! Cryptic's six magnitude-bearing records reads `CrypticPrimeStat`
//! (`up_classes.lst:55`, `INT`), so no ability modifier is threaded here.

/// `up_abilities_class.lst:170`, `Altered Defense`: the "Absorb" option's DR
/// (`AlteredDefenseDR = AlteredDefenseBonus = floor((AlteredDefenseLVL+3)/4)`,
/// `AlteredDefenseLVL = CrypticLVL`). `None` below level 1 (the roster's own
/// `min_level` for this key).
pub fn altered_defense_damage_reduction(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some((i16::from(level) + 3) / 4)
}

/// `up_abilities_class.lst:172`, `Disrupt Pattern`:
/// `BONUS:VAR|DisruptPatternRange|30` — a flat 30-foot range, not
/// level-scaled. `None` below level 1.
pub fn disrupt_pattern_range_feet(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(30)
}

/// `up_abilities_class.lst:175`, `Enhanced Disruption`:
/// `BONUS:VAR|EnhancedDisruptionDice|floor((CrypticLVL-1)/2)` — bonus
/// damage dice added to Disrupt Pattern. `None` below level 3 (the
/// roster's own `min_level` for this key).
pub fn enhanced_disruption_bonus_dice(level: u8) -> Option<i16> {
    if level < 3 {
        return None;
    }
    Some((i16::from(level) - 1) / 2)
}

/// `up_abilities_class.lst:174`, `Hidden Pattern`:
/// `BONUS:VAR|HiddenPatternBonus|2*min(3,floor((CrypticLVL+1)/3))` — a
/// competence bonus on Stealth. `None` below level 2 (the roster's own
/// `min_level` for this key).
pub fn hidden_pattern_stealth_bonus(level: u8) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some(2 * (3.min((i16::from(level) + 1) / 3)))
}

/// `up_abilities_class.lst:173`, `Trapmaker`:
/// `BONUS:VAR|TrapmakerBonus|CrypticLVL` — a competence bonus on Craft
/// (traps) equal to class level. `None` below level 1.
pub fn trapmaker_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// `up_abilities_class.lst:181`, `Unchanging Pattern`:
/// `BONUS:VAR|UnchangingPatternPR|12+CrypticLVL` — power resistance.
/// `None` below level 18 (the roster's own `min_level` for this key).
pub fn unchanging_pattern_power_resistance(level: u8) -> Option<i16> {
    if level < 18 {
        return None;
    }
    Some(12 + i16::from(level))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn altered_defense_dr_steps_every_four_levels_from_level_one() {
        assert_eq!(altered_defense_damage_reduction(1), Some(1));
        assert_eq!(altered_defense_damage_reduction(5), Some(2));
        assert_eq!(altered_defense_damage_reduction(20), Some(5));
        assert_eq!(altered_defense_damage_reduction(0), None);
    }

    #[test]
    fn disrupt_pattern_range_is_a_flat_thirty_feet() {
        assert_eq!(disrupt_pattern_range_feet(1), Some(30));
        assert_eq!(disrupt_pattern_range_feet(20), Some(30));
        assert_eq!(disrupt_pattern_range_feet(0), None);
    }

    #[test]
    fn enhanced_disruption_grants_zero_dice_at_first_eligible_level_three() {
        assert_eq!(enhanced_disruption_bonus_dice(3), Some(1));
        assert_eq!(enhanced_disruption_bonus_dice(20), Some(9));
        assert_eq!(enhanced_disruption_bonus_dice(2), None);
    }

    #[test]
    fn hidden_pattern_bonus_caps_at_six() {
        assert_eq!(hidden_pattern_stealth_bonus(2), Some(2));
        assert_eq!(hidden_pattern_stealth_bonus(8), Some(6));
        assert_eq!(hidden_pattern_stealth_bonus(20), Some(6));
        assert_eq!(hidden_pattern_stealth_bonus(1), None);
    }

    #[test]
    fn trapmaker_bonus_equals_class_level() {
        assert_eq!(trapmaker_bonus(7), Some(7));
        assert_eq!(trapmaker_bonus(0), None);
    }

    #[test]
    fn unchanging_pattern_pr_is_twelve_plus_level_at_the_capstone() {
        assert_eq!(unchanging_pattern_power_resistance(18), Some(30));
        assert_eq!(unchanging_pattern_power_resistance(20), Some(32));
        assert_eq!(unchanging_pattern_power_resistance(17), None);
    }
}
