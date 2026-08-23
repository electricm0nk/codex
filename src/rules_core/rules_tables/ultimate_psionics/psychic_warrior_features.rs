//! SD-32 card 11 (T12) — real per-feature compute functions for the
//! Psychic Warrior (`untabled_base_class_chassis`), a fifth
//! magnitude-bearing `untabled_base_class_feature_roster` group worked
//! end-to-end as a class (`decisions.md §17`/`§27b` — novelty of shape is
//! grounds for sizing, not exclusion).
//!
//! Every formula below is transcribed from the corpus's own already-
//! ingested tokens (`data/corpus/ultimate_psionics/class_feature/
//! psychic_warrior/*.json`, each record's own `raw_tokens`, sourced from
//! `up_classes.lst`/`up_abilities_class.lst`).

/// `up_classes.lst:319`, `BONUS:VAR|WarriorPathLVL|CL` (unconditional for a
/// non-archetype Psychic Warrior): `Warrior's Path`'s own tracked level,
/// equal to class level — no separate `DEFINE`d magnitude exists on this
/// row itself (`up_abilities_class.lst:536`), the choice-of-path grant's
/// only numeric output is the level it tracks for downstream path-power
/// features. `None` below level 1 (the roster's own `min_level` for this
/// key).
pub fn warriors_path_level(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// `up_abilities_class.lst:540`, `Pathweaving`:
/// `BONUS:VAR|PathweavingTimes|(PathweavingLVL-12)/3`, and
/// `PathweavingLVL = CL` (`up_classes.lst:331`) — uses per day. `None`
/// below level 15 (the roster's own `min_level` for this key).
pub fn pathweaving_uses_per_day(level: u8) -> Option<i16> {
    if level < 15 {
        return None;
    }
    Some((i16::from(level) - 12) / 3)
}

/// `up_abilities_class.lst:541`, `Eternal Warrior`: `ASPECT:CheckCount|1`
/// — a flat 1/day capstone, not level-scaled. `None` below level 20 (the
/// roster's own `min_level` for this key, and PF1's own capstone level).
pub fn eternal_warrior_uses_per_day(level: u8) -> Option<i16> {
    if level < 20 {
        return None;
    }
    Some(1)
}

// --- SD-32 card 11 (T12) follow-up: `Psychic Warrior Manifesting`'s three
// magnitudes, same shape-3 grant convention and ladder as Cryptic/Dread
// (`PsychicWarriorPrimeStat`/`PsychicWarriorPLStatScore` read Wisdom).

pub fn psychic_warrior_power_points_total(level: u8, wis_mod: i16) -> Option<i16> {
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
    Some(base + (wis_mod * i16::from(level)) / 2)
}

pub fn psychic_warrior_powers_known(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

pub fn psychic_warrior_max_power_level(level: u8, wis_score: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    let mpl = i16::from(level);
    Some(((mpl + 2) / 3).min(6).min(wis_score - 10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warriors_path_level_equals_class_level() {
        assert_eq!(warriors_path_level(1), Some(1));
        assert_eq!(warriors_path_level(20), Some(20));
        assert_eq!(warriors_path_level(0), None);
    }

    #[test]
    fn pathweaving_grants_one_use_at_level_fifteen_the_first_grant() {
        assert_eq!(pathweaving_uses_per_day(15), Some(1));
        assert_eq!(pathweaving_uses_per_day(20), Some(2));
        assert_eq!(pathweaving_uses_per_day(14), None);
    }

    #[test]
    fn eternal_warrior_is_a_flat_one_use_at_the_capstone() {
        assert_eq!(eternal_warrior_uses_per_day(20), Some(1));
        assert_eq!(eternal_warrior_uses_per_day(19), None);
    }

    #[test]
    fn psychic_warrior_power_points_total_uses_the_base_ladder_and_wis_bonus() {
        assert_eq!(psychic_warrior_power_points_total(1, 0), Some(1));
        assert_eq!(psychic_warrior_power_points_total(20, 0), Some(12));
        assert_eq!(psychic_warrior_power_points_total(0, 0), None);
    }

    #[test]
    fn psychic_warrior_powers_known_equals_class_level() {
        assert_eq!(psychic_warrior_powers_known(1), Some(1));
        assert_eq!(psychic_warrior_powers_known(20), Some(20));
        assert_eq!(psychic_warrior_powers_known(0), None);
    }

    #[test]
    fn psychic_warrior_max_power_level_is_capped_by_the_lowest_of_three_terms() {
        assert_eq!(psychic_warrior_max_power_level(1, 10), Some(0));
        assert_eq!(psychic_warrior_max_power_level(20, 20), Some(6));
        assert_eq!(psychic_warrior_max_power_level(0, 20), None);
    }
}
