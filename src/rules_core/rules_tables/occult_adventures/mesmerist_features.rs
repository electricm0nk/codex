//! SD-32 card 11 (T12), cycle 4 — real per-feature compute functions for
//! the Mesmerist, one of the six `occult_adventures` classes sharing
//! `oa_abilities_class.lst`. Every formula below is transcribed from the
//! corpus's own already-ingested tokens
//! (`data/corpus/occult_adventures/class_feature/mesmerist/*.json`).
//!
//! Four of these records' own `BONUS:VAR` formulas add a `.../FCB` term
//! (a favored-class-bonus investment the player may choose at level-up).
//! This engine has no favored-class-bonus input anywhere in
//! `CharacterInput`, so every formula below grounds the FCB-less base value
//! (the correct value for a character who has not invested a favored-class
//! bonus into that feature) rather than fabricating an FCB score. This
//! omission is documented on each affected function, not silently dropped.

/// `oa_abilities_class.lst:85`, `Consummate Liar`:
/// `BONUS:VAR|ConsummateLiarBonus|max(MesmeristLVL/2,1)`.
pub fn consummate_liar_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(std::cmp::max(i16::from(level) / 2, 1))
}

/// `oa_abilities_class.lst:86`, `Hypnotic Stare`:
/// `BONUS:VAR|HypnoticStarePenalty|2` then overridden to `1` at level 8+
/// (`PREVARGTEQ:MesmeristLVL,8`).
pub fn hypnotic_stare_penalty(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(if level >= 8 { 1 } else { 2 })
}

/// `oa_abilities_class.lst:88`, `Mesmerist Tricks`:
/// `BONUS:VAR|MesmeristTricksUses|max(MesmeristLVL/2,1)+max(CHA,0)+
/// MesmeristTrickFCB/3` (FCB term dropped, see module doc).
pub fn mesmerist_tricks_uses(level: u8, cha: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(std::cmp::max(i16::from(level) / 2, 1) + std::cmp::max(cha, 0))
}

/// `oa_abilities_class.lst:88`, `Mesmerist Tricks`:
/// `BONUS:VAR|MesmeristTrickRange|100+(MesmeristLVL*10)`.
pub fn mesmerist_trick_range_feet(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(100 + i16::from(level) * 10)
}

/// `oa_abilities_class.lst:88`, `Mesmerist Tricks`:
/// `BONUS:VAR|MesmeristTrickDC|10+MesmeristLVL/2+CHA`.
pub fn mesmerist_trick_dc(level: u8, cha: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(10 + i16::from(level) / 2 + cha)
}

/// `oa_abilities_class.lst:88`, `Mesmerist Tricks`:
/// `BONUS:VAR|MesmeristTricksKnown|MesmeristLVL/2+1`.
pub fn mesmerist_tricks_known(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) / 2 + 1)
}

/// `oa_abilities_class.lst:89`, `Painful Stare`:
/// `BONUS:VAR|PainfulStareDam|max(MesmeristLVL/2,1)+PainfulStareDamFCB/4`
/// (FCB term dropped, see module doc).
pub fn painful_stare_damage(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(std::cmp::max(i16::from(level) / 2, 1))
}

/// `oa_abilities_class.lst:89`, `Painful Stare`:
/// `BONUS:VAR|PainfulStareBonusDice|MesmeristLVL/3`.
pub fn painful_stare_bonus_dice(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) / 3)
}

/// `oa_abilities_class.lst:90`, `Towering Ego`:
/// `BONUS:VAR|ToweringEgoBonus|max(CHA,0)+min(ToweringEgoFCB/3,2)` (FCB
/// term dropped, see module doc).
pub fn towering_ego_bonus(level: u8, cha: i16) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some(std::cmp::max(cha, 0))
}

/// `oa_abilities_class.lst:91`, `Bold Stare`:
/// `BONUS:VAR|BoldStaresKnown|(MesmeristLVL+1)/4`.
pub fn bold_stares_known(level: u8) -> Option<i16> {
    if level < 3 {
        return None;
    }
    Some((i16::from(level) + 1) / 4)
}

/// `oa_abilities_class.lst:92`, `Touch Treatment`:
/// `BONUS:VAR|TouchTreatmentsUses|3+CHA`.
pub fn touch_treatment_uses(level: u8, cha: i16) -> Option<i16> {
    if level < 3 {
        return None;
    }
    Some(3 + cha)
}

/// `oa_abilities_class.lst:93`, `Manifold Tricks`:
/// `BONUS:VAR|ManifoldTricksCount|2+(MesmeristLVL-5)/4`.
pub fn manifold_tricks_count(level: u8) -> Option<i16> {
    if level < 5 {
        return None;
    }
    Some(2 + (i16::from(level) - 5) / 4)
}

/// `oa_abilities_class.lst:94`, `Mental Potency`:
/// `BONUS:VAR|MentalPotencyBonus|min(MesmeristLVL/5,4)`.
pub fn mental_potency_bonus(level: u8) -> Option<i16> {
    if level < 5 {
        return None;
    }
    Some(std::cmp::min(i16::from(level) / 5, 4))
}

/// `oa_abilities_class.lst:95`, `Glib Lie`:
/// `BONUS:VAR|GlibLieDC|15+MesmeristLVL`.
pub fn glib_lie_dc(level: u8) -> Option<i16> {
    if level < 11 {
        return None;
    }
    Some(15 + i16::from(level))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consummate_liar_bonus_floors_at_one() {
        assert_eq!(consummate_liar_bonus(1), Some(1));
        assert_eq!(consummate_liar_bonus(20), Some(10));
        assert_eq!(consummate_liar_bonus(0), None);
    }

    #[test]
    fn hypnotic_stare_penalty_drops_at_level_eight() {
        assert_eq!(hypnotic_stare_penalty(1), Some(2));
        assert_eq!(hypnotic_stare_penalty(8), Some(1));
        assert_eq!(hypnotic_stare_penalty(0), None);
    }

    #[test]
    fn mesmerist_tricks_uses_combines_level_and_charisma() {
        assert_eq!(mesmerist_tricks_uses(20, 3), Some(13));
        assert_eq!(mesmerist_tricks_uses(0, 3), None);
    }

    #[test]
    fn mesmerist_trick_range_scales_by_ten_feet_per_level() {
        assert_eq!(mesmerist_trick_range_feet(1), Some(110));
        assert_eq!(mesmerist_trick_range_feet(20), Some(300));
    }

    #[test]
    fn mesmerist_trick_dc_combines_level_and_charisma() {
        assert_eq!(mesmerist_trick_dc(20, 3), Some(23));
    }

    #[test]
    fn mesmerist_tricks_known_scales_with_level() {
        assert_eq!(mesmerist_tricks_known(1), Some(1));
        assert_eq!(mesmerist_tricks_known(20), Some(11));
    }

    #[test]
    fn painful_stare_damage_and_bonus_dice_scale_with_level() {
        assert_eq!(painful_stare_damage(1), Some(1));
        assert_eq!(painful_stare_bonus_dice(20), Some(6));
    }

    #[test]
    fn towering_ego_bonus_gates_at_level_two() {
        assert_eq!(towering_ego_bonus(2, 4), Some(4));
        assert_eq!(towering_ego_bonus(1, 4), None);
    }

    #[test]
    fn bold_stares_known_gates_at_level_three() {
        assert_eq!(bold_stares_known(3), Some(1));
        assert_eq!(bold_stares_known(2), None);
    }

    #[test]
    fn touch_treatment_uses_gates_at_level_three() {
        assert_eq!(touch_treatment_uses(3, 3), Some(6));
        assert_eq!(touch_treatment_uses(2, 3), None);
    }

    #[test]
    fn manifold_tricks_count_gates_at_level_five() {
        assert_eq!(manifold_tricks_count(5), Some(2));
        assert_eq!(manifold_tricks_count(4), None);
    }

    #[test]
    fn mental_potency_bonus_caps_at_four() {
        assert_eq!(mental_potency_bonus(5), Some(1));
        assert_eq!(mental_potency_bonus(20), Some(4));
    }

    #[test]
    fn glib_lie_dc_gates_at_level_eleven() {
        assert_eq!(glib_lie_dc(11), Some(26));
        assert_eq!(glib_lie_dc(10), None);
    }
}
