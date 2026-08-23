//! SD-32 card 11 (T12), cycle 3 — real per-feature compute functions for
//! the Vitalist, the third of this cycle's four `ultimate_psionics`
//! classes. Every formula below is transcribed from the corpus's own
//! already-ingested `BONUS:VAR` tokens
//! (`data/corpus/ultimate_psionics/class_feature/vitalist/*.json`), not
//! from memory of the printed rulebook. Vitalist's prime stat
//! (`VitalistPrimeStat`, `up_classes.lst:464`) is Wisdom for every record
//! below that reads an ability score.

/// `up_abilities_class.lst:953`, `Collective`:
/// `BONUS:VAR|CollectiveMinds|max(CollectiveLVL/2,VitalistPrimeStat)`,
/// `CollectiveLVL = VitalistLVL` for a base Vitalist — the same
/// greater-of-level-or-stat shape as Tactician's Collective. `None` below
/// level 1 (the roster's own `min_level`).
pub fn collective_minds(level: u8, wisdom: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some((i16::from(level) / 2).max(wisdom))
}

/// `up_abilities_class.lst:957`, `Transfer Wounds`: the roster's own
/// tracked var is `TransferWoundsTimes`
/// (`BONUS:VAR|TransferWoundsTimes|(3+VitalistPrimeStat)`) — a pure
/// Wisdom-modifier value, no level term. `None` below level 1.
pub fn transfer_wounds_times_per_day(level: u8, wisdom: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(3 + wisdom)
}

/// `up_abilities_class.lst:958`, `Health Sense`: no roster-tracked var
/// (`text_only` is still `false` — a real `BONUS:VAR|HealthSenseLVL|
/// VitalistLVL` token exists, just no `%N`-substituted `DESC` to name it),
/// equal to class level with no ability term. `None` below level 2 (the
/// roster's own `min_level`).
pub fn health_sense_level(level: u8) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some(i16::from(level))
}

/// `up_abilities_class.lst:960`, `Steal Health`: the roster's own tracked
/// "var" field is the literal formula text `StealHealthLVL+
/// VitalistPrimeStat` (the census picked up the `DESC`'s substitution
/// expression itself rather than a bare token name) — `StealHealthLVL =
/// VitalistLVL`, so the real value is `level + Wisdom modifier`, the same
/// shape as Antipaladin's Cruelty DC / Dread's Terror. `None` below
/// level 3 (the roster's own `min_level`).
pub fn steal_health_damage(level: u8, wisdom: i16) -> Option<i16> {
    if level < 3 {
        return None;
    }
    Some(i16::from(level) + wisdom)
}

/// `up_abilities_class.lst:962`, `Request Aid`: the roster's own tracked
/// var is `RequestAidTimes` (`BONUS:VAR|RequestAidTimes|
/// (3+VitalistPrimeStat)`) — a sibling `RequestAidAmount` token (a flat 3)
/// exists on the same record but is not the roster's tracked var. `None`
/// below level 5 (the roster's own `min_level`).
pub fn request_aid_times_per_day(level: u8, wisdom: i16) -> Option<i16> {
    if level < 5 {
        return None;
    }
    Some(3 + wisdom)
}

/// `up_abilities_class.lst:963`, `Steal Life`:
/// `BONUS:VAR|StealLifeDC|10+VitalistPrimeStat+StealLifeLVL/2`,
/// `StealLifeLVL = VitalistLVL` — matches Antipaladin's `cruelty_dc`/
/// `channel_negative_energy_dc` and Marksman's `cover_fire_dc` shape
/// exactly (`10 + ability modifier + level/2`). `None` below level 14
/// (the roster's own `min_level`).
pub fn steal_life_dc(level: u8, wisdom: i16) -> Option<i16> {
    if level < 14 {
        return None;
    }
    Some(10 + wisdom + i16::from(level) / 2)
}

// --- SD-32 card 11 (T12) follow-up: `Vitalist Manifesting`'s three
// magnitudes, same shape-3 grant convention `psion_features` names,
// surfaced by that cycle's widened census. `vitalist_power_points.json`'s
// `BasePowerPoints` ladder is the SAME full-manifester progression
// `psion_features::psion_power_points_total` documents.

pub fn vitalist_power_points_total(level: u8, wis_mod: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    let base: i16 = match level {
        1 => 2,
        2 => 4,
        3 => 5,
        4 => 6,
        5 => 8,
        6 => 10,
        7 => 11,
        8 => 12,
        9 => 14,
        10 => 16,
        11 => 18,
        12 => 20,
        13 => 21,
        14 => 23,
        15 => 25,
        16 => 26,
        17 => 29,
        18 => 30,
        19 => 31,
        _ => 32, // level >= 20
    };
    Some(base + (wis_mod * i16::from(level)) / 2)
}

pub fn vitalist_powers_known(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    let pkl = i16::from(level);
    Some(1 + (pkl + 1) / 2)
}

pub fn vitalist_max_power_level(level: u8, wis_score: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    let mpl = i16::from(level);
    Some(((mpl + 1) / 2).min(9).min(wis_score - 10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collective_minds_is_the_greater_of_half_level_and_wisdom() {
        assert_eq!(collective_minds(1, 3), Some(3)); // level/2=0 < WIS 3
        assert_eq!(collective_minds(20, 3), Some(10)); // level/2=10 > WIS 3
        assert_eq!(collective_minds(0, 3), None);
    }

    #[test]
    fn transfer_wounds_times_reads_wisdom_only_no_level_term() {
        assert_eq!(transfer_wounds_times_per_day(1, 4), Some(7));
        assert_eq!(transfer_wounds_times_per_day(20, 4), Some(7));
        assert_eq!(transfer_wounds_times_per_day(0, 4), None);
    }

    #[test]
    fn health_sense_level_equals_class_level_no_ability_term() {
        assert_eq!(health_sense_level(2), Some(2));
        assert_eq!(health_sense_level(20), Some(20));
        assert_eq!(health_sense_level(1), None);
    }

    #[test]
    fn steal_health_damage_combines_level_and_wisdom() {
        assert_eq!(steal_health_damage(3, 4), Some(7));
        assert_eq!(steal_health_damage(20, 4), Some(24));
        assert_eq!(steal_health_damage(2, 4), None);
    }

    #[test]
    fn request_aid_times_reads_wisdom_only_no_level_term() {
        assert_eq!(request_aid_times_per_day(5, 4), Some(7));
        assert_eq!(request_aid_times_per_day(4, 4), None);
    }

    #[test]
    fn steal_life_dc_combines_level_and_wisdom_at_the_capstone_range() {
        assert_eq!(steal_life_dc(14, 4), Some(21)); // 10+4+7
        assert_eq!(steal_life_dc(20, 4), Some(24)); // 10+4+10
        assert_eq!(steal_life_dc(13, 4), None);
    }

    #[test]
    fn vitalist_power_points_total_matches_psion_s_full_manifester_ladder() {
        assert_eq!(vitalist_power_points_total(1, 0), Some(2));
        assert_eq!(vitalist_power_points_total(20, 0), Some(32));
        assert_eq!(vitalist_power_points_total(0, 0), None);
    }

    #[test]
    fn vitalist_powers_known_is_one_plus_half_level_rounded_up() {
        assert_eq!(vitalist_powers_known(1), Some(2)); // 1+(2/2)=2
        assert_eq!(vitalist_powers_known(20), Some(11)); // 1+(21/2)=11
        assert_eq!(vitalist_powers_known(0), None);
    }

    #[test]
    fn vitalist_max_power_level_is_capped_by_the_lowest_of_three_terms() {
        assert_eq!(vitalist_max_power_level(1, 10), Some(0));
        assert_eq!(vitalist_max_power_level(20, 20), Some(9));
        assert_eq!(vitalist_max_power_level(0, 20), None);
    }
}
