//! SD-32 card 11 (T12), cycle 4 — real per-feature compute functions for
//! the Psychic, one of the six `occult_adventures` classes sharing
//! `oa_abilities_class.lst`. Every formula below is transcribed from the
//! corpus's own already-ingested tokens
//! (`data/corpus/occult_adventures/class_feature/psychic/*.json`).

/// `oa_abilities_class.lst:128`, `Phrenic Amplifications`:
/// `BONUS:ABILITYPOOL|Phrenic Amplification|1+((PsychicLVL-1)/4)`.
pub fn phrenic_amplifications_count(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1 + (i16::from(level) - 1) / 4)
}

/// `oa_abilities_class.lst:127`, `Phrenic Pool`:
/// `BONUS:VAR|PhrenicPool|(PsychicLVL/2)+PhrenicPoolAbility`.
/// `PhrenicPoolAbility` is set by the player's *chosen* Psychic Discipline
/// (`oa_abilities_class.lst:1188`-`1196`): Abomination/Dream/Pain/Rapport
/// use Charisma, Faith/Lore/Psychedelia/Self-Perfection/Tranquility use
/// Wisdom. This engine does not (yet) track which discipline a Psychic has
/// chosen, so the caller supplies whichever of the character's two mental
/// ability modifiers matches their build — the formula itself is grounded
/// exactly as the corpus states it, not approximated.
pub fn phrenic_pool(level: u8, phrenic_pool_ability_modifier: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) / 2 + phrenic_pool_ability_modifier)
}

/// `oa_abilities_class.lst:129`, `Psychic Discipline`:
/// `BONUS:ABILITYPOOL|Psychic Discipline|1` — a flat single choice, made
/// once at 1st level.
pub fn psychic_discipline_pool(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1)
}

/// `oa_abilities_class.lst:132`, `Major Amplifications`: no `BONUS:` token
/// exists, but the record's own DESC states the real cadence unambiguously
/// ("At 11th level and every 4 levels thereafter, a psychic can choose one
/// of the following major amplifications") — the same count shape as
/// `phrenic_amplifications_count` above, re-based at level 11.
pub fn major_amplifications_count(level: u8) -> Option<i16> {
    if level < 11 {
        return None;
    }
    Some(1 + (i16::from(level) - 11) / 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phrenic_amplifications_count_scales_every_four_levels() {
        assert_eq!(phrenic_amplifications_count(1), Some(1));
        assert_eq!(phrenic_amplifications_count(20), Some(1 + 19 / 4));
        assert_eq!(phrenic_amplifications_count(0), None);
    }

    #[test]
    fn phrenic_pool_combines_level_and_discipline_ability() {
        assert_eq!(phrenic_pool(20, 3), Some(13));
        assert_eq!(phrenic_pool(0, 3), None);
    }

    #[test]
    fn psychic_discipline_pool_is_a_flat_one_choice() {
        assert_eq!(psychic_discipline_pool(1), Some(1));
        assert_eq!(psychic_discipline_pool(0), None);
    }

    #[test]
    fn major_amplifications_count_gates_at_level_eleven() {
        assert_eq!(major_amplifications_count(11), Some(1));
        assert_eq!(major_amplifications_count(19), Some(1 + 8 / 4));
        assert_eq!(major_amplifications_count(10), None);
    }
}
