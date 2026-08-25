//! SD-32 card 11 (T12), cycle 4 — real per-feature compute functions for
//! the Magus, `ultimate_magic`'s single magnitude-bearing class (its own
//! `_classes.lst`/`_abilities_class.lst` pair, not shared with any other
//! class). Every formula below is transcribed from the corpus's own
//! already-ingested tokens
//! (`data/corpus/ultimate_magic/class_feature/magus/*.json`).

/// `Arcane Pool`: `BONUS:VAR|MagusArcanePool|max(floor(MagusArcanePoolLVL/2),1)+INT`,
/// `MagusArcanePoolLVL` set from `MagusLVL` by a second `BONUS:VAR` row on
/// the same record.
pub fn arcane_pool(level: u8, int: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(std::cmp::max(i16::from(level) / 2, 1) + int)
}

/// `Arcane Pool`: `BONUS:VAR|MagusArcanePoolBonus|min(1+((MagusArcanePoolLVL-1)/4),5)`
/// — the enhancement bonus the pool can grant a weapon.
pub fn arcane_pool_enhancement_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(std::cmp::min(1 + (i16::from(level) - 1) / 4, 5))
}

/// `Armor Proficiency`: `BONUS:VAR|MagusArmorProficiencyLVL|MagusLVL` — a
/// pure level pass-through, the same shape as Vitalist's Health Sense.
pub fn armor_proficiency_level(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// `Magus Arcana`: `BONUS:VAR|Pool_Magus_Arcana|MagusArcanaLVL/3`,
/// `MagusArcanaLVL` set from `MagusLVL` by a sibling `BONUS:VAR` row.
pub fn magus_arcana_pool(level: u8) -> Option<i16> {
    if level < 3 {
        return None;
    }
    Some(i16::from(level) / 3)
}

/// `Bonus Feats`: `BONUS:ABILITYPOOL|Magus Bonus Feat|(MagusLVL+1)/6`.
pub fn bonus_feats_pool(level: u8) -> Option<i16> {
    if level < 5 {
        return None;
    }
    Some((i16::from(level) + 1) / 6)
}

/// `Fighter Training`: `BONUS:VAR|FighterWeaponQualifyLVL|MagusFighterTrainingLVL/2`,
/// `MagusFighterTrainingLVL` set from `MagusLVL`.
pub fn fighter_training_level(level: u8) -> Option<i16> {
    if level < 10 {
        return None;
    }
    Some(i16::from(level) / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arcane_pool_combines_level_and_intelligence() {
        assert_eq!(arcane_pool(1, 3), Some(1 + 3));
        assert_eq!(arcane_pool(20, 3), Some(10 + 3));
        assert_eq!(arcane_pool(0, 3), None);
    }

    #[test]
    fn arcane_pool_enhancement_bonus_caps_at_five() {
        assert_eq!(arcane_pool_enhancement_bonus(1), Some(1));
        assert_eq!(arcane_pool_enhancement_bonus(20), Some(5));
    }

    #[test]
    fn armor_proficiency_level_tracks_class_level() {
        assert_eq!(armor_proficiency_level(1), Some(1));
        assert_eq!(armor_proficiency_level(0), None);
    }

    #[test]
    fn magus_arcana_pool_gates_at_level_three() {
        assert_eq!(magus_arcana_pool(3), Some(1));
        assert_eq!(magus_arcana_pool(2), None);
    }

    #[test]
    fn bonus_feats_pool_gates_at_level_five() {
        assert_eq!(bonus_feats_pool(5), Some(1));
        assert_eq!(bonus_feats_pool(4), None);
    }

    #[test]
    fn fighter_training_level_gates_at_level_ten() {
        assert_eq!(fighter_training_level(10), Some(5));
        assert_eq!(fighter_training_level(9), None);
    }
}
