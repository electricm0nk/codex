//! SD-32 card 11 (T12), cycle 3 — real per-feature compute functions for
//! the Tactician, the second of this cycle's four `ultimate_psionics`
//! classes. Every formula below is transcribed from the corpus's own
//! already-ingested `BONUS:VAR`/`BONUS:ABILITYPOOL` tokens
//! (`data/corpus/ultimate_psionics/class_feature/tactician/*.json`), not
//! from memory of the printed rulebook. Tactician's prime stat
//! (`TacticianPrimeStat`, `up_classes.lst:406`) is Intelligence; its
//! secondary stat (`TacticianSecondaryStat`, `up_classes.lst:407`) is
//! Charisma.

/// `up_abilities_class.lst:912`, `Collective`: the roster's own tracked
/// "var" field for this record is actually a `PREABILITY` gate clause
/// picked up by the census script's last-`|`-segment heuristic (the same
/// class of quirk the prior cycle documented for Cryptic's Enhanced
/// Disruption and Soulknife's Psychic Strike), not a real token name. The
/// record's real first-substituted magnitude is `TacticianCollectiveMinds`
/// (`BONUS:VAR|TacticianCollectiveMinds|max(TacticianPrimeStat,
/// TacticianCollectiveLVL/2)`, `TacticianCollectiveLVL = TacticianLVL` for
/// a base Tactician) — the number of minds joined to the collective.
/// `None` below level 1 (the roster's own `min_level`).
pub fn collective_minds(level: u8, intelligence: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(intelligence.max(i16::from(level) / 2))
}

/// `up_abilities_class.lst:913`, `Coordinated Strike`: the roster's own
/// tracked var is `TacticianCoordinatedStrikeTimes`
/// (`BONUS:VAR|TacticianCoordinatedStrikeTimes|3+TacticianPrimeStat`) — a
/// sibling `TacticianCoordinatedStrikeBonus` token exists on the same
/// record but is not the roster's tracked var. `None` below level 1.
pub fn coordinated_strike_times_per_day(level: u8, intelligence: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(3 + intelligence)
}

/// `up_abilities_class.lst:916`, `Strategy`:
/// `BONUS:VAR|StrategyTimes|3+TacticianSecondaryStat` — reads the
/// *secondary* stat (Charisma), not the prime stat. `None` below level 4
/// (the roster's own `min_level`).
pub fn strategy_times_per_day(level: u8, charisma: i16) -> Option<i16> {
    if level < 4 {
        return None;
    }
    Some(3 + charisma)
}

/// `up_abilities_class.lst:917`, `Improved Share`:
/// `BONUS:VAR|ImprovedSharePowers|1+floor((TacticianLVL+1)/6)`. `None`
/// below level 5 (the roster's own `min_level`).
pub fn improved_share_powers(level: u8) -> Option<i16> {
    if level < 5 {
        return None;
    }
    Some(1 + (i16::from(level) + 1) / 6)
}

/// `up_abilities_class.lst:919`, `Teamwork Feats`: no `DEFINE`/plain
/// `BONUS:VAR` on this record at all — the record's real magnitude is the
/// `BONUS:ABILITYPOOL|Tactician Bonus Teamwork Feat|TacticianLVL/6` pool
/// size (the roster's own `var` field is `None` for this record, same as
/// Vitalist's Health Sense; a real formula still exists in `raw_tokens`).
/// `None` below level 6 (the roster's own `min_level`).
pub fn teamwork_feats_bonus_pool(level: u8) -> Option<i16> {
    if level < 6 {
        return None;
    }
    Some(i16::from(level) / 6)
}

/// `up_abilities_class.lst:924`, `Master Strategist`:
/// `BONUS:VAR|MasterStrategistBonus|TacticianPrimeStat` — a pure
/// Intelligence-modifier value with no level term. `None` below level 20
/// (the roster's own `min_level`).
pub fn master_strategist_bonus(level: u8, intelligence: i16) -> Option<i16> {
    if level < 20 {
        return None;
    }
    Some(intelligence)
}

// --- SD-32 card 11 (T12) follow-up: `Tactician Manifesting`'s three
// magnitudes, same shape-3 grant convention `psion_features` names,
// surfaced by that cycle's widened census. `tactician_power_points.json`'s
// `BasePowerPoints` ladder is the SAME full-manifester progression
// `psion_features::psion_power_points_total` documents (2,4,5,6,8,10,11,
// 12,14,16,18,20,21,23,25,26,29,30,31,32).

pub fn tactician_power_points_total(level: u8, int_mod: i16) -> Option<i16> {
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
    Some(base + (int_mod * i16::from(level)) / 2)
}

pub fn tactician_powers_known(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

pub fn tactician_max_power_level(level: u8, int_score: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    let mpl = i16::from(level);
    Some(((mpl + 1) / 2).min(9).min(int_score - 10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collective_minds_is_the_greater_of_intelligence_and_half_level() {
        assert_eq!(collective_minds(1, 2), Some(2)); // INT 2 > 0
        assert_eq!(collective_minds(20, 2), Some(10)); // level/2=10 > INT 2
        assert_eq!(collective_minds(0, 2), None);
    }

    #[test]
    fn coordinated_strike_times_reads_prime_stat_only() {
        assert_eq!(coordinated_strike_times_per_day(1, 3), Some(6));
        assert_eq!(coordinated_strike_times_per_day(0, 3), None);
    }

    #[test]
    fn strategy_times_reads_secondary_stat_not_prime() {
        assert_eq!(strategy_times_per_day(4, 4), Some(7));
        assert_eq!(strategy_times_per_day(3, 4), None);
    }

    #[test]
    fn improved_share_powers_steps_every_six_levels_from_level_five() {
        assert_eq!(improved_share_powers(5), Some(2));
        assert_eq!(improved_share_powers(20), Some(4));
        assert_eq!(improved_share_powers(4), None);
    }

    #[test]
    fn teamwork_feats_pool_steps_every_six_levels_from_level_six() {
        assert_eq!(teamwork_feats_bonus_pool(6), Some(1));
        assert_eq!(teamwork_feats_bonus_pool(20), Some(3));
        assert_eq!(teamwork_feats_bonus_pool(5), None);
    }

    #[test]
    fn master_strategist_bonus_is_gated_to_the_capstone() {
        assert_eq!(master_strategist_bonus(20, 4), Some(4));
        assert_eq!(master_strategist_bonus(19, 4), None);
    }

    #[test]
    fn tactician_power_points_total_matches_psion_s_full_manifester_ladder() {
        assert_eq!(tactician_power_points_total(1, 0), Some(2));
        assert_eq!(tactician_power_points_total(20, 0), Some(32));
        assert_eq!(tactician_power_points_total(5, 3), Some(8 + (3 * 5) / 2));
        assert_eq!(tactician_power_points_total(0, 0), None);
    }

    #[test]
    fn tactician_powers_known_equals_class_level() {
        assert_eq!(tactician_powers_known(1), Some(1));
        assert_eq!(tactician_powers_known(20), Some(20));
        assert_eq!(tactician_powers_known(0), None);
    }

    #[test]
    fn tactician_max_power_level_is_capped_by_the_lowest_of_three_terms() {
        assert_eq!(tactician_max_power_level(1, 10), Some(0)); // min(9,1,0)
        assert_eq!(tactician_max_power_level(20, 20), Some(9)); // min(9,10,10)
        assert_eq!(tactician_max_power_level(0, 20), None);
    }
}
