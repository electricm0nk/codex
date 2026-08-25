//! SD-32 card 11 (T12), cycle 4 — real per-feature compute functions for
//! the Vigilante, `ultimate_intrigue`'s single magnitude-bearing class.
//! Every formula below is transcribed from the corpus's own already-
//! ingested tokens
//! (`data/corpus/ultimate_intrigue/class_feature/vigilante/*.json`).

/// `Seamless Guise`: `BONUS:VAR|SeamlessGuiseBonus|20` — a flat literal.
pub fn seamless_guise_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(20)
}

/// `Social Talent`: `BONUS:VAR|SocialTalentCount|(VigilanteLVL+1)/2`.
pub fn social_talent_count(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some((i16::from(level) + 1) / 2)
}

/// `Vigilante Specialization`: `BONUS:ABILITYPOOL|Vigilante
/// Specialization|1` — a flat single choice made once.
pub fn vigilante_specialization_pool(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1)
}

/// `Vigilante Talent`: `BONUS:VAR|VigilanteTalentCount|VigilanteLVL/2`.
pub fn vigilante_talent_count(level: u8) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some(i16::from(level) / 2)
}

/// `Vigilante Talent`: `BONUS:VAR|VigilanteTalentDC|10+VigilanteLVL/2+CHA`.
pub fn vigilante_talent_dc(level: u8, cha: i16) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some(10 + i16::from(level) / 2 + cha)
}

/// `Unshakable`: `BONUS:VAR|VigilanteUnshakableDCBonus|VigilanteLVL` — a
/// pure level pass-through.
pub fn unshakable_dc_bonus(level: u8) -> Option<i16> {
    if level < 3 {
        return None;
    }
    Some(i16::from(level))
}

/// `Frightening Appearance`: no `BONUS:VAR` token — the record's own
/// `%1`-substituted DESC states the DC formula directly:
/// `10+VigilanteLVL/2+CHA`.
pub fn frightening_appearance_dc(level: u8, cha: i16) -> Option<i16> {
    if level < 11 {
        return None;
    }
    Some(10 + i16::from(level) / 2 + cha)
}

/// `Stunning Appearance`: same DC formula as Frightening Appearance
/// (`10+VigilanteLVL/2+CHA`), from its own `%1`-substituted DESC.
pub fn stunning_appearance_dc(level: u8, cha: i16) -> Option<i16> {
    if level < 17 {
        return None;
    }
    Some(10 + i16::from(level) / 2 + cha)
}

/// `Stunning Appearance`: the DESC's `%2` substitution, `VigilanteLVL` —
/// the HD threshold above which the target gets a +4 bonus on the save.
pub fn stunning_appearance_hd_threshold(level: u8) -> Option<i16> {
    if level < 17 {
        return None;
    }
    Some(i16::from(level))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seamless_guise_bonus_is_a_flat_twenty() {
        assert_eq!(seamless_guise_bonus(1), Some(20));
        assert_eq!(seamless_guise_bonus(0), None);
    }

    #[test]
    fn social_talent_count_scales_with_level() {
        assert_eq!(social_talent_count(1), Some(1));
        assert_eq!(social_talent_count(20), Some(10));
    }

    #[test]
    fn vigilante_specialization_pool_is_a_flat_one_choice() {
        assert_eq!(vigilante_specialization_pool(1), Some(1));
        assert_eq!(vigilante_specialization_pool(0), None);
    }

    #[test]
    fn vigilante_talent_count_and_dc_gate_at_level_two() {
        assert_eq!(vigilante_talent_count(2), Some(1));
        assert_eq!(vigilante_talent_dc(2, 3), Some(14));
        assert_eq!(vigilante_talent_count(1), None);
    }

    #[test]
    fn unshakable_dc_bonus_gates_at_level_three() {
        assert_eq!(unshakable_dc_bonus(3), Some(3));
        assert_eq!(unshakable_dc_bonus(2), None);
    }

    #[test]
    fn frightening_appearance_dc_gates_at_level_eleven() {
        assert_eq!(frightening_appearance_dc(11, 3), Some(18));
        assert_eq!(frightening_appearance_dc(10, 3), None);
    }

    #[test]
    fn stunning_appearance_dc_and_hd_threshold_gate_at_level_seventeen() {
        assert_eq!(stunning_appearance_dc(17, 3), Some(21));
        assert_eq!(stunning_appearance_hd_threshold(17), Some(17));
        assert_eq!(stunning_appearance_dc(16, 3), None);
    }
}
