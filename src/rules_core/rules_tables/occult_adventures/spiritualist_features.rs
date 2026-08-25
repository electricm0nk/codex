//! SD-32 card 11 (T12), cycle 4 — real per-feature compute functions for
//! the Spiritualist, one of the six `occult_adventures` classes sharing
//! `oa_abilities_class.lst`. Every formula below is transcribed from the
//! corpus's own already-ingested tokens
//! (`data/corpus/occult_adventures/class_feature/spiritualist/*.json`).

/// `oa_abilities_class.lst:148`, `Phantom`:
/// `BONUS:VAR|PhantomMasterLVL|SpiritualistLVL` — the phantom's master
/// level tracks the Spiritualist's own class level directly.
pub fn phantom_master_level(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// `oa_abilities_class.lst:149`, `Shared Consciousness`:
/// `BONUS:ABILITYPOOL|Phantom Emotional Focus|1` — a flat single choice.
pub fn shared_consciousness_focus_pool(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1)
}

/// `oa_abilities_class.lst:158`, `Calm Spirit`: no `BONUS:` token exists,
/// but the record's own DESC states the real cadence unambiguously ("once
/// per day... At 11th level and every 4 levels thereafter, the
/// spiritualist can use this ability an additional time per day (to a
/// maximum of 4 times per day at 19th level)"). The stated cap is reached
/// naturally by the formula at level 19-20 without an explicit `min()`.
pub fn calm_spirit_uses_per_day(level: u8) -> Option<i16> {
    if level < 7 {
        return None;
    }
    Some(1 + (i16::from(level) - 7) / 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phantom_master_level_tracks_class_level() {
        assert_eq!(phantom_master_level(1), Some(1));
        assert_eq!(phantom_master_level(20), Some(20));
        assert_eq!(phantom_master_level(0), None);
    }

    #[test]
    fn shared_consciousness_focus_pool_is_a_flat_one_choice() {
        assert_eq!(shared_consciousness_focus_pool(1), Some(1));
        assert_eq!(shared_consciousness_focus_pool(0), None);
    }

    #[test]
    fn calm_spirit_uses_per_day_gates_at_seven_and_caps_at_four_by_nineteen() {
        assert_eq!(calm_spirit_uses_per_day(7), Some(1));
        assert_eq!(calm_spirit_uses_per_day(19), Some(4));
        assert_eq!(calm_spirit_uses_per_day(20), Some(4));
        assert_eq!(calm_spirit_uses_per_day(6), None);
    }
}
