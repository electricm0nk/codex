//! SD-32 card 11 (T12) — real per-feature compute functions for the
//! Marksman (`untabled_base_class_chassis`), the fourth magnitude-bearing
//! `untabled_base_class_feature_roster` group worked end-to-end as a class
//! (`decisions.md §17`/`§27b` — novelty of shape is grounds for sizing,
//! not exclusion).
//!
//! Every formula below is transcribed from the corpus's own already-
//! ingested `BONUS:VAR` tokens (`data/corpus/ultimate_psionics/
//! class_feature/marksman/*.json`, each record's own `raw_tokens`, sourced
//! from `up_abilities_class.lst` — the roster's own `source_file` for every
//! Marksman record). `MarksmanSecondaryStat` is `DEX` (`up_classes.lst:174`,
//! `BONUS:VAR|MarksmanSecondaryStat|DEX`), threaded here as
//! `dexterity_modifier`; `MarksmanPrimeStat` (`WIS`) is not read by any of
//! these five roster records.

/// `up_abilities_class.lst:331`, `Wind Reader`:
/// `BONUS:VAR|WindReaderTimes|3+WindReaderLVL`, `WindReaderLVL =
/// MarksmanLVL` — uses per day. `None` below level 1 (the roster's own
/// `min_level` for this key).
pub fn wind_reader_uses_per_day(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(3 + i16::from(level))
}

/// `up_abilities_class.lst:332`, `Evade Arrows`:
/// `BONUS:VAR|EvadeArrows|(MarksmanLVL+2)/4` — an AC bonus against ranged
/// attacks. `None` below level 2 (the roster's own `min_level` for this
/// key).
pub fn evade_arrows_ac_bonus(level: u8) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some((i16::from(level) + 2) / 4)
}

/// `up_abilities_class.lst:333`, `Favored Weapon`:
/// `BONUS:VAR|FavoredWeaponBase|(MarksmanLVL+2)/4` — the base competence
/// bonus fed into whichever weapon-group-specific Favored Weapon (Bows /
/// Crossbows / Spears / Thrown) row the character selects
/// (`BONUS:ABILITYPOOL|Favored Weapon|1`); this row's own `DESC:` is
/// empty (`VISIBLE:NO`), the magnitude lives entirely in the token.
/// `None` below level 2 (the roster's own `min_level` for this key).
pub fn favored_weapon_base_bonus(level: u8) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some((i16::from(level) + 2) / 4)
}

/// `up_abilities_class.lst:334`, `Cover Fire`:
/// `BONUS:VAR|CoverFireDC|10+MarksmanSecondaryStat+(CoverFireLVL/2)`,
/// `CoverFireLVL = MarksmanLVL` — DC to distract an opponent. `None`
/// below level 4 (the roster's own `min_level` for this key).
pub fn cover_fire_dc(level: u8, dexterity_modifier: i16) -> Option<i16> {
    if level < 4 {
        return None;
    }
    Some(10 + dexterity_modifier + i16::from(level) / 2)
}

/// `up_abilities_class.lst:338`, `Ranged Specialist`:
/// `BONUS:WEAPONPROF=TYPE.Ranged|CRITMULTADD|1` — a flat +1 critical
/// multiplier increase, not level-scaled. `None` below level 19 (the
/// roster's own `min_level` for this key).
pub fn ranged_specialist_critical_multiplier_bonus(level: u8) -> Option<i16> {
    if level < 19 {
        return None;
    }
    Some(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wind_reader_uses_per_day_is_three_plus_level() {
        assert_eq!(wind_reader_uses_per_day(1), Some(4));
        assert_eq!(wind_reader_uses_per_day(20), Some(23));
        assert_eq!(wind_reader_uses_per_day(0), None);
    }

    #[test]
    fn evade_arrows_and_favored_weapon_share_the_level_plus_two_over_four_formula() {
        assert_eq!(evade_arrows_ac_bonus(2), Some(1));
        assert_eq!(favored_weapon_base_bonus(2), Some(1));
        assert_eq!(evade_arrows_ac_bonus(20), Some(5));
        assert_eq!(favored_weapon_base_bonus(20), Some(5));
        assert_eq!(evade_arrows_ac_bonus(1), None);
        assert_eq!(favored_weapon_base_bonus(1), None);
    }

    #[test]
    fn cover_fire_dc_combines_level_and_dexterity() {
        assert_eq!(cover_fire_dc(4, 2), Some(10 + 2 + 2));
        assert_eq!(cover_fire_dc(3, 2), None);
    }

    #[test]
    fn ranged_specialist_is_a_flat_plus_one_at_the_capstone() {
        assert_eq!(ranged_specialist_critical_multiplier_bonus(19), Some(1));
        assert_eq!(ranged_specialist_critical_multiplier_bonus(18), None);
    }
}
