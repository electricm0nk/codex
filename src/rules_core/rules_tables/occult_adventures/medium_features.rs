//! SD-32 card 11 (T12), cycle 4 — real per-feature compute functions for
//! the Medium, one of the six `occult_adventures` classes sharing
//! `oa_abilities_class.lst`. Every formula below is transcribed from the
//! corpus's own already-ingested tokens
//! (`data/corpus/occult_adventures/class_feature/medium/*.json`).

/// `oa_abilities_class.lst:61`, `Spirit`:
/// `BONUS:VAR|SpiritBonus|1+MediumLVL/4`.
pub fn spirit_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1 + i16::from(level) / 4)
}

/// `oa_abilities_class.lst:62`, `Spirit Surge`:
/// `BONUS:VAR|SpiritSurgeDice|6+2*floor(MediumLVL/10)`.
pub fn spirit_surge_dice(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(6 + 2 * (i16::from(level) / 10))
}

/// `oa_abilities_class.lst:65`, `Haunt Channeler`:
/// `BONUS:VAR|HauntChannelDice|MediumLVL/2`.
pub fn haunt_channeler_dice(level: u8) -> Option<i16> {
    if level < 3 {
        return None;
    }
    Some(i16::from(level) / 2)
}

/// `oa_abilities_class.lst:65`, `Haunt Channeler`:
/// `BONUS:VAR|HauntChannelDC|20+MediumLVL/2`.
pub fn haunt_channeler_dc(level: u8) -> Option<i16> {
    if level < 3 {
        return None;
    }
    Some(20 + i16::from(level) / 2)
}

/// `oa_abilities_class.lst:66`, `Location Channel`:
/// `BONUS:VAR|LocationChannelDuration|MediumLVL`.
pub fn location_channel_duration_rounds(level: u8) -> Option<i16> {
    if level < 5 {
        return None;
    }
    Some(i16::from(level))
}

/// `oa_abilities_class.lst:66`, `Location Channel`:
/// `BONUS:VAR|LocationChannelDC|20+MediumLVL/2`.
pub fn location_channel_dc(level: u8) -> Option<i16> {
    if level < 5 {
        return None;
    }
    Some(20 + i16::from(level) / 2)
}

/// `oa_abilities_class.lst:69`, `Ask the Spirits`: no `BONUS:VAR` token —
/// the real magnitude lives in the record's own `SPELLS:` token,
/// `SPELLS:Medium|TIMES=ATWILL|CASTERLEVEL=MediumLVL|Contact Other
/// Plane,15+CHA` — the save DC of the granted spell-like ability.
pub fn ask_the_spirits_dc(level: u8, cha: i16) -> Option<i16> {
    if level < 13 {
        return None;
    }
    Some(15 + cha)
}

/// `oa_abilities_class.lst:70`, `Astral Journey`: same shape as Ask the
/// Spirits, from its own `SPELLS:` token —
/// `SPELLS:Medium|TIMES=ATWILL|CASTERLEVEL=MediumLVL|Astral
/// Projection,19+CHA`.
pub fn astral_journey_dc(level: u8, cha: i16) -> Option<i16> {
    if level < 14 {
        return None;
    }
    Some(19 + cha)
}

/// `oa_abilities_class.lst:71`, `Trance of Three`: the record's own
/// `BONUS:VAR|TraceOfThreeDuration|MediumLVL` (a corpus-verbatim typo of
/// its own `DEFINE:TranceOfThreeDuration`, the same class of quirk prior
/// cycles documented for Cryptic/Soulknife/Tactician).
pub fn trance_of_three_duration_rounds(level: u8) -> Option<i16> {
    if level < 15 {
        return None;
    }
    Some(i16::from(level))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spirit_bonus_steps_every_four_levels() {
        assert_eq!(spirit_bonus(1), Some(1));
        assert_eq!(spirit_bonus(20), Some(6));
        assert_eq!(spirit_bonus(0), None);
    }

    #[test]
    fn spirit_surge_dice_steps_every_ten_levels() {
        assert_eq!(spirit_surge_dice(1), Some(6));
        assert_eq!(spirit_surge_dice(20), Some(10));
        assert_eq!(spirit_surge_dice(0), None);
    }

    #[test]
    fn haunt_channeler_dice_and_dc_gate_at_level_three() {
        assert_eq!(haunt_channeler_dice(3), Some(1));
        assert_eq!(haunt_channeler_dc(3), Some(21));
        assert_eq!(haunt_channeler_dice(2), None);
        assert_eq!(haunt_channeler_dc(2), None);
    }

    #[test]
    fn location_channel_duration_and_dc_gate_at_level_five() {
        assert_eq!(location_channel_duration_rounds(5), Some(5));
        assert_eq!(location_channel_dc(5), Some(22));
        assert_eq!(location_channel_duration_rounds(4), None);
    }

    #[test]
    fn ask_the_spirits_dc_gates_at_level_thirteen() {
        assert_eq!(ask_the_spirits_dc(13, 3), Some(18));
        assert_eq!(ask_the_spirits_dc(12, 3), None);
    }

    #[test]
    fn astral_journey_dc_gates_at_level_fourteen() {
        assert_eq!(astral_journey_dc(14, 3), Some(22));
        assert_eq!(astral_journey_dc(13, 3), None);
    }

    #[test]
    fn trance_of_three_duration_gates_at_level_fifteen() {
        assert_eq!(trance_of_three_duration_rounds(15), Some(15));
        assert_eq!(trance_of_three_duration_rounds(14), None);
    }
}
