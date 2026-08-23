//! SD-32 card 11 (T12), cycle 3 — real per-feature compute functions for
//! the Wilder, the fourth and last of this cycle's four `ultimate_psionics`
//! classes (closing all nine of that source book's magnitude-bearing
//! classes named at the start of this cycle: Aegis, Cryptic, Dread,
//! Marksman, Psychic Warrior, Soulknife, Tactician, Vitalist, Wilder).
//! Every formula below is transcribed from the corpus's own already-
//! ingested `BONUS:VAR` tokens
//! (`data/corpus/ultimate_psionics/class_feature/wilder/*.json`), not from
//! memory of the printed rulebook. None of Wilder's five magnitude-bearing
//! records reads `WilderPrimeStat` (`up_classes.lst` — Charisma), so no
//! ability modifier is threaded here; `WilderML`
//! (`up_abilities_class.lst:1033`) resolves to plain class level for a
//! base (non-archetype) Wilder.

/// `up_abilities_class.lst:1038`, `Psychic Enervation`:
/// `BONUS:VAR|PsychicEnervationPercent|15` — a flat 15% chance, not
/// level-scaled. `None` below level 1 (the roster's own `min_level`).
pub fn psychic_enervation_percent(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(15)
}

/// `up_abilities_class.lst:1039`, `Surge Blast`:
/// `BONUS:VAR|SurgeBlastRange|30` — a flat 30-foot range, not
/// level-scaled. `None` below level 1.
pub fn surge_blast_range_feet(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(30)
}

/// `up_abilities_class.lst:1037`, `Wild Surge`:
/// `BONUS:VAR|WildSurge|1+floor((SurgeLVL+1)/4)`, `SurgeLVL = WilderML =
/// WilderLVL` for a base Wilder. `None` below level 1.
pub fn wild_surge_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1 + (i16::from(level) + 1) / 4)
}

/// `up_abilities_class.lst:1040`, `Elude Attack`:
/// `BONUS:VAR|EludeAttack|floor((WilderLVL+2)/4)`. `None` below level 2
/// (the roster's own `min_level`).
pub fn elude_attack_ac_bonus(level: u8) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some((i16::from(level) + 2) / 4)
}

/// `up_abilities_class.lst:1041`, `Surging Euphoria`: the roster's own
/// tracked var is `SurgingEuphoriaDuration`
/// (`BONUS:VAR|SurgingEuphoriaDuration|WildSurge`) — the duration in
/// rounds equals the character's own current Wild Surge bonus, so this
/// reuses [`wild_surge_bonus`] rather than re-deriving the same formula. A
/// sibling `SurgingEuphoria` token (`floor((WilderLVL+4)/8)`, the morale
/// bonus itself) exists on the same record but is not the roster's tracked
/// var. `None` below level 4 (the roster's own `min_level`).
pub fn surging_euphoria_duration_rounds(level: u8) -> Option<i16> {
    if level < 4 {
        return None;
    }
    wild_surge_bonus(level)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn psychic_enervation_is_a_flat_fifteen_percent() {
        assert_eq!(psychic_enervation_percent(1), Some(15));
        assert_eq!(psychic_enervation_percent(20), Some(15));
        assert_eq!(psychic_enervation_percent(0), None);
    }

    #[test]
    fn surge_blast_range_is_a_flat_thirty_feet() {
        assert_eq!(surge_blast_range_feet(1), Some(30));
        assert_eq!(surge_blast_range_feet(0), None);
    }

    #[test]
    fn wild_surge_bonus_steps_every_four_levels_from_level_one() {
        assert_eq!(wild_surge_bonus(1), Some(1));
        assert_eq!(wild_surge_bonus(20), Some(6));
        assert_eq!(wild_surge_bonus(0), None);
    }

    #[test]
    fn elude_attack_ac_bonus_steps_every_four_levels_from_level_two() {
        assert_eq!(elude_attack_ac_bonus(2), Some(1));
        assert_eq!(elude_attack_ac_bonus(20), Some(5));
        assert_eq!(elude_attack_ac_bonus(1), None);
    }

    #[test]
    fn surging_euphoria_duration_mirrors_wild_surge_bonus() {
        assert_eq!(surging_euphoria_duration_rounds(4), wild_surge_bonus(4));
        assert_eq!(surging_euphoria_duration_rounds(20), Some(6));
        assert_eq!(surging_euphoria_duration_rounds(3), None);
    }
}
