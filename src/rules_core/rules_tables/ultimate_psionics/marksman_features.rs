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

// --- SD-32 card 11 (T12) follow-up: `Marksman Manifesting`'s three
// magnitudes, the same shape-3 grant convention `psion_features` names,
// surfaced by that cycle's widened census. `marksman_power_points.json`'s
// `BasePowerPoints` ladder ("highest satisfied threshold wins") is a
// slower (1/6-manifester) progression than the full-manifester classes.
// `MarksmanPowersKnown` has TWO `BONUS:VAR` terms sharing one target
// (`min(9,floor((3*MarksmanPKL-1)/4))` unconditional, plus
// `floor((MarksmanLVL-13)/2)` once `MarksmanPKL>=15`) -- resolved by the
// same SUM semantics `psion_features::psion_powers_known` documents
// (`bonus_stack_reader.rs`, citing `pcgen/core/PlayerCharacter.java:2136`).
// `MarksmanMaxPowerLevel`'s single term carries its own
// `PREVARGTEQ:MarksmanMPL,2` gate -- 0 below level 2.

pub fn marksman_power_points_total(level: u8, wis_mod: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    let base: i16 = match level {
        1..=3 => 1,
        4..=7 => 2,
        8..=10 => 3,
        11..=13 => 4,
        14..=17 => 5,
        _ => 6, // 18..=20
    };
    Some(base + (wis_mod * i16::from(level)) / 2)
}

pub fn marksman_powers_known(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    let pkl = i16::from(level);
    let base = ((3 * pkl - 1) / 4).min(9);
    let bonus = if pkl >= 15 { (pkl - 13) / 2 } else { 0 };
    Some(base + bonus)
}

pub fn marksman_max_power_level(level: u8, wis_score: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    let mpl = i16::from(level);
    if mpl < 2 {
        return Some(0);
    }
    Some(((mpl + 3) / 4).min(4).min(wis_score - 10))
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

    #[test]
    fn marksman_power_points_total_uses_the_slower_ladder_and_wis_bonus() {
        assert_eq!(marksman_power_points_total(1, 0), Some(1));
        assert_eq!(marksman_power_points_total(20, 0), Some(6));
        assert_eq!(marksman_power_points_total(0, 0), None);
    }

    #[test]
    fn marksman_powers_known_only_the_base_term_below_level_fifteen() {
        assert_eq!(marksman_powers_known(1), Some(0)); // (3-1)/4=0
        assert_eq!(marksman_powers_known(14), Some(9)); // min(9,(41)/4=10)=9
        assert_eq!(marksman_powers_known(0), None);
    }

    #[test]
    fn marksman_powers_known_sums_both_terms_from_level_fifteen() {
        // Level 15: base min(9,floor(44/4)=11)=9, bonus floor(2/2)=1 -> 10.
        assert_eq!(marksman_powers_known(15), Some(10));
        // Level 20: base 9 (already saturated), bonus floor(7/2)=3 -> 12.
        assert_eq!(marksman_powers_known(20), Some(12));
    }

    #[test]
    fn marksman_max_power_level_is_zero_below_level_two_then_capped() {
        assert_eq!(marksman_max_power_level(1, 20), Some(0));
        assert_eq!(marksman_max_power_level(2, 20), Some(1)); // min(4,floor(5/4)=1,10)
        assert_eq!(marksman_max_power_level(20, 20), Some(4)); // min(4,5,10)
        assert_eq!(marksman_max_power_level(0, 20), None);
    }
}
